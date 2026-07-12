//! Component repository for MongoDB operations

use anyhow::{Context, Result};
use mongodb::{
    bson::{doc, oid::ObjectId},
    Client, Collection,
};
use uuid::Uuid;

use crate::models::{AddComponentRequest, ComponentNode, ProjectComponents, UpdateComponentRequest};

/// Repository for component CRUD operations in MongoDB
pub struct ComponentRepository {
    collection: Collection<ProjectComponents>,
}

impl ComponentRepository {
    /// Create a new component repository
    pub async fn new(mongodb_uri: &str, database_name: &str) -> Result<Self> {
        let client = Client::new(mongodb_uri)
            .await
            .context("Failed to connect to MongoDB")?;

        let database = client.database(database_name);
        let collection = database.collection("project_components");

        // Create indexes
        collection
            .create_index(
                mongodb::IndexModel::builder()
                    .keys(doc! { "project_id": 1 })
                    .build(),
            )
            .await?;

        collection
            .create_index(
                mongodb::IndexModel::builder()
                    .keys(doc! { "workspace_id": 1 })
                    .build(),
            )
            .await?;

        Ok(ComponentRepository { collection })
    }

    /// Create component document for a new project
    pub async fn create_project_document(&self, project_id: Uuid, workspace_id: Uuid) -> Result<ProjectComponents> {
        let document = ProjectComponents::new(project_id, workspace_id);

        let result = self
            .collection
            .insert_one(&document)
            .await?;

        let mut document = document;
        document.id = result.inserted_id.as_object_id();

        Ok(document)
    }

    /// Get all components for a project
    pub async fn get_project_components(&self, project_id: Uuid) -> Result<Option<ProjectComponents>> {
        let filter = doc! { "project_id": project_id.to_string() };

        let document = self
            .collection
            .find_one(filter)
            .await?;

        Ok(document)
    }

    /// Get a specific component by ID
    pub async fn get_component(&self, project_id: Uuid, component_id: &str) -> Result<Option<ComponentNode>> {
        let document = self.get_project_components(project_id).await?;

        Ok(document.and_then(|doc| doc.components.get(component_id).cloned()))
    }

    /// Add a component to a project
    pub async fn add_component(&self, project_id: Uuid, request: AddComponentRequest) -> Result<ComponentNode> {
        let filter = doc! { "project_id": project_id.to_string() };

        // Get the current document
        let mut document = self
            .get_project_components(project_id)
            .await?
            .context("Project components document not found")?;

        let mut component = request.component.clone();

        // Set parent ID
        component.parent_id = request.parent_id.clone();

        // Add to components map
        document.components.insert(component.id.clone(), component.clone());

        // Add to parent's children list
        if let Some(ref parent_id) = request.parent_id {
            if let Some(parent) = document.components.get_mut(parent_id) {
                if let Some(pos) = request.position {
                    parent.children.insert(pos, component.id.clone());
                } else {
                    parent.children.push(component.id.clone());
                }
            }
        } else {
            // Add to root
            if let Some(pos) = request.position {
                document.root_ids.insert(pos, component.id.clone());
            } else {
                document.root_ids.push(component.id.clone());
            }
        }

        // Update version and timestamp
        document.version += 1;
        document.updated_at = chrono::Utc::now();

        // Save back to database
        self.collection
            .replace_one(filter, &document)
            .await?;

        Ok(component)
    }

    /// Update a component
    pub async fn update_component(
        &self,
        project_id: Uuid,
        component_id: &str,
        request: UpdateComponentRequest,
    ) -> Result<ComponentNode> {
        let filter = doc! { "project_id": project_id.to_string() };

        let mut document = self
            .get_project_components(project_id)
            .await?
            .context("Project components document not found")?;

        let component = document
            .components
            .get_mut(component_id)
            .context("Component not found")?;

        // Apply updates
        if let Some(props) = request.props {
            component.props = props;
        }

        if let Some(styles) = request.styles {
            component.styles = styles;
        }

        if let Some(animations) = request.animations {
            component.animations = animations;
        }

        // Update version and timestamp
        document.version += 1;
        document.updated_at = chrono::Utc::now();

        // Save back to database
        self.collection
            .replace_one(filter, &document)
            .await?;

        Ok(component.clone())
    }

    /// Delete a component and its children
    pub async fn delete_component(&self, project_id: Uuid, component_id: &str) -> Result<Vec<String>> {
        let filter = doc! { "project_id": project_id.to_string() };

        let mut document = self
            .get_project_components(project_id)
            .await?
            .context("Project components document not found")?;

        // Collect all component IDs to delete (including children)
        let mut to_delete = vec![component_id.to_string()];
        let mut queue = vec![component_id.to_string()];

        while let Some(id) = queue.pop() {
            if let Some(component) = document.components.get(&id) {
                for child_id in &component.children {
                    to_delete.push(child_id.clone());
                    queue.push(child_id.clone());
                }
            }
        }

        // Remove from parent's children list
        if let Some(component) = document.components.get(component_id) {
            if let Some(ref parent_id) = component.parent_id {
                if let Some(parent) = document.components.get_mut(parent_id) {
                    parent.children.retain(|id| id != component_id);
                }
            } else {
                // Remove from root
                document.root_ids.retain(|id| id != component_id);
            }
        }

        // Remove all components
        for id in &to_delete {
            document.components.remove(id);
        }

        // Update version and timestamp
        document.version += 1;
        document.updated_at = chrono::Utc::now();

        // Save back to database
        self.collection
            .replace_one(filter, &document)
            .await?;

        Ok(to_delete)
    }

    /// Move a component to a new parent or position
    pub async fn move_component(
        &self,
        project_id: Uuid,
        component_id: &str,
        new_parent_id: Option<String>,
        new_position: Option<usize>,
    ) -> Result<ComponentNode> {
        let filter = doc! { "project_id": project_id.to_string() };

        let mut document = self
            .get_project_components(project_id)
            .await?
            .context("Project components document not found")?;

        let component = document
            .components
            .get(component_id)
            .context("Component not found")?
            .clone();

        // Remove from old parent
        if let Some(ref old_parent_id) = component.parent_id {
            if let Some(old_parent) = document.components.get_mut(old_parent_id) {
                old_parent.children.retain(|id| id != component_id);
            }
        } else {
            document.root_ids.retain(|id| id != component_id);
        }

        // Add to new parent
        if let Some(ref new_parent_id) = new_parent_id {
            if let Some(new_parent) = document.components.get_mut(new_parent_id) {
                if let Some(pos) = new_position {
                    new_parent.children.insert(pos, component_id.to_string());
                } else {
                    new_parent.children.push(component_id.to_string());
                }
            }
        } else {
            if let Some(pos) = new_position {
                document.root_ids.insert(pos, component_id.to_string());
            } else {
                document.root_ids.push(component_id.to_string());
            }
        }

        // Update component's parent ID
        if let Some(comp) = document.components.get_mut(component_id) {
            comp.parent_id = new_parent_id;
        }

        // Update version and timestamp
        document.version += 1;
        document.updated_at = chrono::Utc::now();

        // Save back to database
        self.collection
            .replace_one(filter, &document)
            .await?;

        // Return updated component
        Ok(document.components.get(component_id).cloned().unwrap())
    }

    /// Delete entire project component document
    pub async fn delete_project_document(&self, project_id: Uuid) -> Result<()> {
        let filter = doc! { "project_id": project_id.to_string() };

        self.collection
            .delete_one(filter)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: Integration tests require a test MongoDB instance
}
