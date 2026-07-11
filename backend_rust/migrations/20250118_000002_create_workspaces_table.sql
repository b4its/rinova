-- Migration: Create workspaces table
-- Created at: 2025-01-18
-- Requirements: 2.1, 2.2 - Multi-Tenant Workspace Management

-- Workspaces table for multi-tenant architecture
CREATE TABLE IF NOT EXISTS workspaces (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    type VARCHAR(50) NOT NULL DEFAULT 'personal',
    owner_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    logo_url TEXT,
    settings JSONB DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    -- Workspace type constraint
    CONSTRAINT valid_workspace_type CHECK (type IN ('personal', 'company'))
);

-- Indexes for workspace lookups
CREATE INDEX IF NOT EXISTS idx_workspaces_owner ON workspaces(owner_id);
CREATE INDEX IF NOT EXISTS idx_workspaces_type ON workspaces(type);

-- Apply updated_at trigger to workspaces table
DROP TRIGGER IF EXISTS update_workspaces_updated_at ON workspaces;
CREATE TRIGGER update_workspaces_updated_at 
    BEFORE UPDATE ON workspaces
    FOR EACH ROW 
    EXECUTE FUNCTION update_updated_at_column();

-- Comments for documentation
COMMENT ON TABLE workspaces IS 'Workspaces for multi-tenant isolation: Personal or Company types';
COMMENT ON COLUMN workspaces.type IS 'Workspace type: personal (default for each user) or company (for team collaboration)';
COMMENT ON COLUMN workspaces.owner_id IS 'Reference to the user who owns this workspace';
COMMENT ON COLUMN workspaces.settings IS 'JSONB field for workspace-specific settings';
