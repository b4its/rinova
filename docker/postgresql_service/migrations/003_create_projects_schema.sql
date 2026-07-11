-- Migration: 003_create_projects_schema
-- Description: Create projects table with workspace reference, published_sites table for hosting metadata,
--              and audit_logs table for action tracking with row-level security for project isolation
-- Requirements: 6.1, 10.5
-- Created: 2025-01-21

-- ============================================================================
-- Projects Table
-- ============================================================================
-- Stores website projects created by users within workspaces
-- Each project belongs to a workspace and has an owner user

CREATE TABLE IF NOT EXISTS projects (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    workspace_id UUID NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    owner_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'draft',
    metadata JSONB DEFAULT '{}',
    last_published_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    CONSTRAINT valid_project_status CHECK (status IN ('draft', 'published', 'archived'))
);

-- Add comment to table
COMMENT ON TABLE projects IS 'Stores website projects created by users within workspaces';
COMMENT ON COLUMN projects.id IS 'Unique identifier for the project';
COMMENT ON COLUMN projects.workspace_id IS 'Reference to the workspace this project belongs to';
COMMENT ON COLUMN projects.owner_id IS 'Reference to the user who owns this project';
COMMENT ON COLUMN projects.name IS 'Display name of the project';
COMMENT ON COLUMN projects.status IS 'Current status: draft, published, or archived';
COMMENT ON COLUMN projects.metadata IS 'JSON metadata for project configuration';
COMMENT ON COLUMN projects.last_published_at IS 'Timestamp of the most recent publication';

-- ============================================================================
-- Published Sites Table
-- ============================================================================
-- Stores hosting metadata for published projects
-- Tracks subdomain, custom domain, and SSL status

CREATE TABLE IF NOT EXISTS published_sites (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    subdomain VARCHAR(255) NOT NULL UNIQUE,
    custom_domain VARCHAR(255),
    ssl_status VARCHAR(50) DEFAULT 'pending',
    published_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    CONSTRAINT valid_ssl_status CHECK (ssl_status IN ('pending', 'active', 'failed'))
);

-- Add comments to table
COMMENT ON TABLE published_sites IS 'Stores hosting metadata for published websites';
COMMENT ON COLUMN published_sites.project_id IS 'Reference to the published project';
COMMENT ON COLUMN published_sites.subdomain IS 'Unique subdomain for the published site (e.g., project-id.rinova.app)';
COMMENT ON COLUMN published_sites.custom_domain IS 'Optional custom domain configured by user';
COMMENT ON COLUMN published_sites.ssl_status IS 'SSL certificate status: pending, active, or failed';
COMMENT ON COLUMN published_sites.published_at IS 'Timestamp when the site was published';

-- ============================================================================
-- Audit Logs Table
-- ============================================================================
-- Stores audit trail for user actions within the platform
-- Tracks action type, resource, and metadata for compliance and debugging

CREATE TABLE IF NOT EXISTS audit_logs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    action VARCHAR(100) NOT NULL,
    resource_type VARCHAR(100) NOT NULL,
    resource_id UUID,
    metadata JSONB DEFAULT '{}',
    ip_address INET,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Add comments to table
COMMENT ON TABLE audit_logs IS 'Stores audit trail for user actions for compliance and debugging';
COMMENT ON COLUMN audit_logs.user_id IS 'Reference to the user who performed the action (nullable for deleted users)';
COMMENT ON COLUMN audit_logs.action IS 'Action type: create, update, delete, publish, etc.';
COMMENT ON COLUMN audit_logs.resource_type IS 'Type of resource affected: project, workspace, component, etc.';
COMMENT ON COLUMN audit_logs.resource_id IS 'ID of the affected resource';
COMMENT ON COLUMN audit_logs.metadata IS 'JSON metadata with action-specific details';
COMMENT ON COLUMN audit_logs.ip_address IS 'IP address of the client that performed the action';

-- ============================================================================
-- Row-Level Security Policies
-- ============================================================================
-- Enable RLS on projects table to enforce workspace isolation
-- Users can only see projects in workspaces they have access to

-- Enable RLS on projects table
ALTER TABLE projects ENABLE ROW LEVEL SECURITY;

-- Policy: Users can see projects in workspaces they own or are members of
CREATE POLICY project_isolation ON projects
    USING (
        workspace_id IN (
            SELECT id FROM workspaces 
            WHERE owner_id = current_setting('app.current_user_id')::UUID
            OR id IN (
                SELECT workspace_id FROM workspace_members 
                WHERE user_id = current_setting('app.current_user_id')::UUID
                AND invitation_status = 'accepted'
            )
        )
    );

-- Policy: Users can insert projects only in workspaces they have access to
CREATE POLICY project_insert_policy ON projects
    WITH CHECK (
        workspace_id IN (
            SELECT id FROM workspaces 
            WHERE owner_id = current_setting('app.current_user_id')::UUID
            OR id IN (
                SELECT workspace_id FROM workspace_members 
                WHERE user_id = current_setting('app.current_user_id')::UUID
                AND invitation_status = 'accepted'
                AND role IN ('owner', 'admin', 'member')
            )
        )
    );

-- Policy: Users can update projects only in workspaces they have edit access to
CREATE POLICY project_update_policy ON projects
    USING (
        workspace_id IN (
            SELECT id FROM workspaces 
            WHERE owner_id = current_setting('app.current_user_id')::UUID
            OR id IN (
                SELECT workspace_id FROM workspace_members 
                WHERE user_id = current_setting('app.current_user_id')::UUID
                AND invitation_status = 'accepted'
                AND role IN ('owner', 'admin', 'member')
            )
        )
    );

-- Policy: Users can delete projects only in workspaces they have admin access to
CREATE POLICY project_delete_policy ON projects
    USING (
        workspace_id IN (
            SELECT id FROM workspaces 
            WHERE owner_id = current_setting('app.current_user_id')::UUID
            OR id IN (
                SELECT workspace_id FROM workspace_members 
                WHERE user_id = current_setting('app.current_user_id')::UUID
                AND invitation_status = 'accepted'
                AND role IN ('owner', 'admin')
            )
        )
    );

-- ============================================================================
-- Indexes for Performance
-- ============================================================================

-- Projects table indexes
CREATE INDEX IF NOT EXISTS idx_projects_workspace ON projects(workspace_id);
CREATE INDEX IF NOT EXISTS idx_projects_owner ON projects(owner_id);
CREATE INDEX IF NOT EXISTS idx_projects_status ON projects(status);
CREATE INDEX IF NOT EXISTS idx_projects_created_at ON projects(created_at DESC);

-- Published sites table indexes
CREATE INDEX IF NOT EXISTS idx_published_sites_project ON published_sites(project_id);
CREATE INDEX IF NOT EXISTS idx_published_sites_subdomain ON published_sites(subdomain);
CREATE INDEX IF NOT EXISTS idx_published_sites_custom_domain ON published_sites(custom_domain) WHERE custom_domain IS NOT NULL;

-- Audit logs table indexes
CREATE INDEX IF NOT EXISTS idx_audit_logs_user ON audit_logs(user_id);
CREATE INDEX IF NOT EXISTS idx_audit_logs_created ON audit_logs(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_audit_logs_action ON audit_logs(action);
CREATE INDEX IF NOT EXISTS idx_audit_logs_resource ON audit_logs(resource_type, resource_id);

-- ============================================================================
-- Triggers
-- ============================================================================

-- Apply updated_at trigger to projects table
CREATE TRIGGER update_projects_updated_at 
    BEFORE UPDATE ON projects
    FOR EACH ROW 
    EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- Sample Audit Log Actions (for documentation)
-- ============================================================================
-- The audit_logs table will track the following action types:
-- 
-- Project actions:
--   - project.created: A new project was created
--   - project.updated: Project settings were modified
--   - project.deleted: A project was deleted
--   - project.published: A project was published
--   - project.archived: A project was archived
--
-- Workspace actions:
--   - workspace.created: A new workspace was created
--   - workspace.updated: Workspace settings were modified
--   - workspace.deleted: A workspace was deleted
--   - workspace.member_invited: A member was invited to workspace
--   - workspace.member_joined: A member joined a workspace
--
-- Publishing actions:
--   - publish.started: Publishing process started
--   - publish.completed: Publishing completed successfully
--   - publish.failed: Publishing failed
--   - domain.added: Custom domain added
--   - domain.removed: Custom domain removed
--   - ssl.configured: SSL certificate configured
