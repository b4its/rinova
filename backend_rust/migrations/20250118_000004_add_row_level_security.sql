-- Migration: Add Row-Level Security policies for tenant isolation
-- Created at: 2025-01-18
-- Requirements: 2.7, 15.4 - Multi-Tenant Workspace Management and Data Storage

-- Enable Row-Level Security on workspaces table
ALTER TABLE workspaces ENABLE ROW LEVEL SECURITY;

-- Enable Row-Level Security on projects table (if exists, will be created in later migration)
-- This is here for reference and will be applied when projects table is created
-- ALTER TABLE projects ENABLE ROW LEVEL SECURITY;

-- Drop existing policies if they exist (for idempotency)
DROP POLICY IF EXISTS workspace_isolation ON workspaces;
DROP POLICY IF EXISTS workspace_member_isolation ON workspaces;
DROP POLICY IF EXISTS workspace_select_policy ON workspaces;
DROP POLICY IF EXISTS workspace_insert_policy ON workspaces;
DROP POLICY IF EXISTS workspace_update_policy ON workspaces;
DROP POLICY IF EXISTS workspace_delete_policy ON workspaces;

-- Create comprehensive RLS policies for workspaces
-- Policy: Users can view workspaces they own or are a member of with accepted status
CREATE POLICY workspace_select_policy ON workspaces
    FOR SELECT
    USING (
        owner_id = current_setting('app.current_user_id', true)::UUID
        OR id IN (
            SELECT workspace_id 
            FROM workspace_members 
            WHERE user_id = current_setting('app.current_user_id', true)::UUID
            AND invitation_status = 'accepted'
        )
    );

-- Policy: Users can create workspaces (owner is automatically set to current user)
CREATE POLICY workspace_insert_policy ON workspaces
    FOR INSERT
    WITH CHECK (owner_id = current_setting('app.current_user_id', true)::UUID);

-- Policy: Only workspace owners can update workspaces
CREATE POLICY workspace_update_policy ON workspaces
    FOR UPDATE
    USING (owner_id = current_setting('app.current_user_id', true)::UUID);

-- Policy: Only workspace owners can delete workspaces
CREATE POLICY workspace_delete_policy ON workspaces
    FOR DELETE
    USING (owner_id = current_setting('app.current_user_id', true)::UUID);

-- Enable Row-Level Security on workspace_members table
ALTER TABLE workspace_members ENABLE ROW LEVEL SECURITY;

-- Drop existing policies if they exist (for idempotency)
DROP POLICY IF EXISTS workspace_members_select_policy ON workspace_members;
DROP POLICY IF EXISTS workspace_members_insert_policy ON workspace_members;
DROP POLICY IF EXISTS workspace_members_update_policy ON workspace_members;
DROP POLICY IF EXISTS workspace_members_delete_policy ON workspace_members;

-- Policy: Users can view members of workspaces they have access to
CREATE POLICY workspace_members_select_policy ON workspace_members
    FOR SELECT
    USING (
        workspace_id IN (
            SELECT id FROM workspaces
            WHERE owner_id = current_setting('app.current_user_id', true)::UUID
            OR id IN (
                SELECT workspace_id 
                FROM workspace_members 
                WHERE user_id = current_setting('app.current_user_id', true)::UUID
                AND invitation_status = 'accepted'
            )
        )
    );

-- Policy: Workspace owners and admins can add members
CREATE POLICY workspace_members_insert_policy ON workspace_members
    FOR INSERT
    WITH CHECK (
        workspace_id IN (
            SELECT id FROM workspaces
            WHERE owner_id = current_setting('app.current_user_id', true)::UUID
            OR id IN (
                SELECT workspace_id 
                FROM workspace_members 
                WHERE user_id = current_setting('app.current_user_id', true)::UUID
                AND role IN ('owner', 'admin')
                AND invitation_status = 'accepted'
            )
        )
    );

-- Policy: Workspace owners and admins can update member roles
CREATE POLICY workspace_members_update_policy ON workspace_members
    FOR UPDATE
    USING (
        workspace_id IN (
            SELECT id FROM workspaces
            WHERE owner_id = current_setting('app.current_user_id', true)::UUID
            OR id IN (
                SELECT workspace_id 
                FROM workspace_members 
                WHERE user_id = current_setting('app.current_user_id', true)::UUID
                AND role IN ('owner', 'admin')
                AND invitation_status = 'accepted'
            )
        )
    );

-- Policy: Workspace owners can remove members, users can remove themselves
CREATE POLICY workspace_members_delete_policy ON workspace_members
    FOR DELETE
    USING (
        workspace_id IN (
            SELECT id FROM workspaces
            WHERE owner_id = current_setting('app.current_user_id', true)::UUID
        )
        OR user_id = current_setting('app.current_user_id', true)::UUID
    );

-- Create a function to set the current user context for RLS
-- This function should be called at the beginning of each database session
CREATE OR REPLACE FUNCTION set_current_user_id(user_id UUID)
RETURNS VOID AS $$
BEGIN
    PERFORM set_config('app.current_user_id', user_id::TEXT, false);
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

-- Create a function to get the current user context
CREATE OR REPLACE FUNCTION get_current_user_id()
RETURNS UUID AS $$
BEGIN
    RETURN current_setting('app.current_user_id', true)::UUID;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

-- Comments for documentation
COMMENT ON POLICY workspace_select_policy ON workspaces IS 'RLS: Users can view workspaces they own or are accepted members of';
COMMENT ON POLICY workspace_insert_policy ON workspaces IS 'RLS: Users can create workspaces as owner';
COMMENT ON POLICY workspace_update_policy ON workspaces IS 'RLS: Only workspace owners can update workspaces';
COMMENT ON POLICY workspace_delete_policy ON workspaces IS 'RLS: Only workspace owners can delete workspaces';
COMMENT ON POLICY workspace_members_select_policy ON workspace_members IS 'RLS: Users can view members of accessible workspaces';
COMMENT ON POLICY workspace_members_insert_policy ON workspaces IS 'RLS: Workspace owners and admins can invite members';
COMMENT ON POLICY workspace_members_update_policy ON workspaces IS 'RLS: Workspace owners and admins can update member roles';
COMMENT ON POLICY workspace_members_delete_policy ON workspaces IS 'RLS: Workspace owners can remove members, users can leave workspaces';
COMMENT ON FUNCTION set_current_user_id(UUID) IS 'Set the current user context for Row-Level Security';
COMMENT ON FUNCTION get_current_user_id() IS 'Get the current user context for Row-Level Security';
