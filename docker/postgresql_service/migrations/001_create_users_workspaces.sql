-- Migration: 001_create_users_workspaces
-- Description: Create users and workspaces tables with workspace_members for multi-tenant access
-- Requirements: 1.1, 2.1, 2.2
-- Created: 2025-01-21

-- ============================================================================
-- Users Table
-- ============================================================================
-- Stores user accounts with email validation and account type

CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    full_name VARCHAR(255),
    account_type VARCHAR(50) NOT NULL DEFAULT 'personal',
    email_verified_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    CONSTRAINT valid_email CHECK (email ~* '^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$'),
    CONSTRAINT valid_account_type CHECK (account_type IN ('personal', 'company'))
);

COMMENT ON TABLE users IS 'Stores user accounts with email validation';

-- ============================================================================
-- Workspaces Table
-- ============================================================================
-- Stores workspaces for personal and company contexts

CREATE TABLE IF NOT EXISTS workspaces (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    type VARCHAR(50) NOT NULL DEFAULT 'personal',
    owner_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    logo_url TEXT,
    settings JSONB DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    CONSTRAINT valid_workspace_type CHECK (type IN ('personal', 'company'))
);

COMMENT ON TABLE workspaces IS 'Stores workspaces for personal and company contexts';

-- ============================================================================
-- Workspace Members Table
-- ============================================================================
-- Stores membership relationships for company workspaces

CREATE TABLE IF NOT EXISTS workspace_members (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    workspace_id UUID NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role VARCHAR(50) NOT NULL DEFAULT 'member',
    invitation_status VARCHAR(50) NOT NULL DEFAULT 'pending',
    invited_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    joined_at TIMESTAMP WITH TIME ZONE,
    
    CONSTRAINT unique_workspace_member UNIQUE (workspace_id, user_id),
    CONSTRAINT valid_role CHECK (role IN ('owner', 'admin', 'member')),
    CONSTRAINT valid_invitation_status CHECK (invitation_status IN ('pending', 'accepted', 'declined', 'expired'))
);

COMMENT ON TABLE workspace_members IS 'Stores membership relationships for company workspaces';

-- ============================================================================
-- Row-Level Security Policies for Workspaces
-- ============================================================================

ALTER TABLE workspaces ENABLE ROW LEVEL SECURITY;

CREATE POLICY workspace_isolation ON workspaces
    USING (
        owner_id = current_setting('app.current_user_id')::UUID
        OR id IN (
            SELECT workspace_id FROM workspace_members 
            WHERE user_id = current_setting('app.current_user_id')::UUID
            AND invitation_status = 'accepted'
        )
    );

-- ============================================================================
-- Indexes
-- ============================================================================

CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
CREATE INDEX IF NOT EXISTS idx_workspaces_owner ON workspaces(owner_id);
CREATE INDEX IF NOT EXISTS idx_workspace_members_user ON workspace_members(user_id);
CREATE INDEX IF NOT EXISTS idx_workspace_members_workspace ON workspace_members(workspace_id);
CREATE INDEX IF NOT EXISTS idx_workspace_members_status ON workspace_members(invitation_status);

-- ============================================================================
-- Triggers
-- ============================================================================

CREATE TRIGGER update_users_updated_at 
    BEFORE UPDATE ON users
    FOR EACH ROW 
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_workspaces_updated_at 
    BEFORE UPDATE ON workspaces
    FOR EACH ROW 
    EXECUTE FUNCTION update_updated_at_column();
