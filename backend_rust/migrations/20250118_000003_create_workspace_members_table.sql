-- Migration: Create workspace_members table for multi-tenant access
-- Created at: 2025-01-18
-- Requirements: 2.1, 2.2, 2.3, 2.4, 2.6 - Multi-Tenant Workspace Management

-- Workspace members table (junction table for company workspace membership)
CREATE TABLE IF NOT EXISTS workspace_members (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    workspace_id UUID NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role VARCHAR(50) NOT NULL DEFAULT 'member',
    invitation_status VARCHAR(50) NOT NULL DEFAULT 'pending',
    invited_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    joined_at TIMESTAMP WITH TIME ZONE,
    
    -- Ensure a user can only be a member of a workspace once
    CONSTRAINT unique_workspace_member UNIQUE (workspace_id, user_id),
    -- Role constraint (Requirement 2.9: Owner, Admin, Member)
    CONSTRAINT valid_role CHECK (role IN ('owner', 'admin', 'member')),
    -- Invitation status constraint (Requirement 2.4: 7-day expiry handled at application level)
    CONSTRAINT valid_invitation_status CHECK (invitation_status IN ('pending', 'accepted', 'declined', 'expired'))
);

-- Indexes for workspace member lookups
CREATE INDEX IF NOT EXISTS idx_workspace_members_user ON workspace_members(user_id);
CREATE INDEX IF NOT EXISTS idx_workspace_members_workspace ON workspace_members(workspace_id);
CREATE INDEX IF NOT EXISTS idx_workspace_members_status ON workspace_members(invitation_status);
CREATE INDEX IF NOT EXISTS idx_workspace_members_role ON workspace_members(role);

-- Comments for documentation
COMMENT ON TABLE workspace_members IS 'Junction table for workspace membership with role-based access control';
COMMENT ON COLUMN workspace_members.role IS 'User role in workspace: owner (full access), admin (user management), member (project edit)';
COMMENT ON COLUMN workspace_members.invitation_status IS 'Invitation status: pending, accepted, declined, expired (7-day expiry)';
COMMENT ON COLUMN workspace_members.invited_at IS 'Timestamp when invitation was sent';
COMMENT ON COLUMN workspace_members.joined_at IS 'Timestamp when user accepted invitation and joined workspace';
