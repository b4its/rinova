//! Component model for MongoDB storage

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Component node in the component tree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentNode {
    /// Unique component ID
    pub id: String,
    /// Component type (e.g., "Hero", "Button", "Text")
    #[serde(rename = "type")]
    pub component_type: String,
    /// Parent component ID (None for root)
    pub parent_id: Option<String>,
    /// Component properties
    pub props: serde_json::Value,
    /// Component styles per breakpoint
    pub styles: ComponentStyles,
    /// Animation configurations
    #[serde(default)]
    pub animations: Vec<AnimationConfig>,
    /// Child component IDs
    #[serde(default)]
    pub children: Vec<String>,
}

impl ComponentNode {
    /// Create a new component node
    pub fn new(id: String, component_type: String) -> Self {
        ComponentNode {
            id,
            component_type,
            parent_id: None,
            props: serde_json::json!({}),
            styles: ComponentStyles::default(),
            animations: vec![],
            children: vec![],
        }
    }
}

/// Component styles for different breakpoints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentStyles {
    /// Desktop styles (default)
    pub desktop: serde_json::Value,
    /// Tablet styles (optional)
    pub tablet: Option<serde_json::Value>,
    /// Mobile styles (optional)
    pub mobile: Option<serde_json::Value>,
}

impl Default for ComponentStyles {
    fn default() -> Self {
        ComponentStyles {
            desktop: serde_json::json!({}),
            tablet: None,
            mobile: None,
        }
    }
}

/// Animation configuration for a component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationConfig {
    /// Animation type (fade, slide, bounce, etc.)
    pub animation_type: String,
    /// Animation duration in milliseconds
    #[serde(default = "default_duration")]
    pub duration_ms: u32,
    /// Animation delay in milliseconds
    #[serde(default)]
    pub delay_ms: u32,
    /// Easing function
    #[serde(default = "default_easing")]
    pub easing: String,
    /// Number of iterations (0 = infinite)
    #[serde(default = "default_iterations")]
    pub iterations: u32,
    /// Animation trigger (on_load, on_scroll, on_hover, on_click)
    #[serde(default = "default_trigger")]
    pub trigger: String,
}

fn default_duration() -> u32 {
    300
}

fn default_easing() -> String {
    "ease".to_string()
}

fn default_iterations() -> u32 {
    1
}

fn default_trigger() -> String {
    "on_load".to_string()
}

/// Project components document stored in MongoDB
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectComponents {
    /// MongoDB document ID
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<mongodb::bson::oid::ObjectId>,
    /// Project ID (UUID as string)
    pub project_id: String,
    /// Workspace ID (UUID as string)
    pub workspace_id: String,
    /// Component tree (keyed by component ID)
    pub components: std::collections::HashMap<String, ComponentNode>,
    /// Root component IDs (for pages)
    pub root_ids: Vec<String>,
    /// Component order for rendering
    #[serde(default)]
    pub component_order: Vec<String>,
    /// Document version
    #[serde(default)]
    pub version: u32,
    /// When this document was created
    pub created_at: DateTime<Utc>,
    /// When this document was last updated
    pub updated_at: DateTime<Utc>,
}

impl ProjectComponents {
    /// Create a new project components document
    pub fn new(project_id: Uuid, workspace_id: Uuid) -> Self {
        let now = Utc::now();
        ProjectComponents {
            id: None,
            project_id: project_id.to_string(),
            workspace_id: workspace_id.to_string(),
            components: std::collections::HashMap::new(),
            root_ids: vec![],
            component_order: vec![],
            version: 1,
            created_at: now,
            updated_at: now,
        }
    }
}

/// Request to add a component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddComponentRequest {
    /// Component to add
    pub component: ComponentNode,
    /// Parent component ID (None for root)
    pub parent_id: Option<String>,
    /// Position among siblings (None to append)
    pub position: Option<usize>,
}

/// Request to update a component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateComponentRequest {
    /// Updated properties (partial update)
    pub props: Option<serde_json::Value>,
    /// Updated styles (partial update)
    pub styles: Option<ComponentStyles>,
    /// Updated animations
    pub animations: Option<Vec<AnimationConfig>>,
}

/// Request to move a component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveComponentRequest {
    /// New parent component ID (None for root)
    pub new_parent_id: Option<String>,
    /// New position among siblings (None to append)
    pub new_position: Option<usize>,
}

/// Response for component operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentResponse {
    pub component: ComponentNode,
    pub message: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_node_new() {
        let component = ComponentNode::new("comp-1".to_string(), "Button".to_string());

        assert_eq!(component.id, "comp-1");
        assert_eq!(component.component_type, "Button");
        assert!(component.parent_id.is_none());
        assert!(component.children.is_empty());
    }

    #[test]
    fn test_project_components_new() {
        let project_id = Uuid::new_v4();
        let workspace_id = Uuid::new_v4();
        let components = ProjectComponents::new(project_id, workspace_id);

        assert_eq!(components.project_id, project_id.to_string());
        assert!(components.components.is_empty());
        assert_eq!(components.version, 1);
    }

    #[test]
    fn test_animation_config_defaults() {
        let config = AnimationConfig {
            animation_type: "fade".to_string(),
            duration_ms: default_duration(),
            delay_ms: 0,
            easing: default_easing(),
            iterations: default_iterations(),
            trigger: default_trigger(),
        };

        assert_eq!(config.duration_ms, 300);
        assert_eq!(config.easing, "ease");
        assert_eq!(config.iterations, 1);
        assert_eq!(config.trigger, "on_load");
    }
}
