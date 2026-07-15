-- Rinova Website Builder - PostgreSQL Initialization
-- Multi-tenant schema with row-level security
--
-- IMPORTANT: This file provides a complete schema for Docker initialization.
-- For incremental migrations, use the files in backend_rust/migrations/
--
-- Migration files (applied in order):
-- 1. 20250118_000001_create_users_table.sql
-- 2. 20250118_000002_create_workspaces_table.sql
-- 3. 20250118_000003_create_workspace_members_table.sql
-- 4. 20250118_000004_add_row_level_security.sql
-- 5. 20250118_000005_create_subscriptions_table.sql
-- 6. 20250118_000006_create_projects_table.sql
-- 7. 20250118_000007_create_published_sites_table.sql
-- 8. 20250118_000008_create_audit_logs_table.sql

-- ============================================================================
-- EXTENSIONS
-- ============================================================================

-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- ============================================================================
-- SCHEMA MIGRATIONS TRACKING
-- ============================================================================

-- Track applied migrations
CREATE TABLE IF NOT EXISTS schema_migrations (
    version VARCHAR(255) PRIMARY KEY,
    applied_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- ============================================================================
-- USERS TABLE (Migration 000001)
-- Requirements: 1.1 - User Registration and Authentication
-- ============================================================================

CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    full_name VARCHAR(255),
    account_type VARCHAR(50) NOT NULL DEFAULT 'personal',
    role VARCHAR(50) NOT NULL DEFAULT 'user',
    email_verified_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    -- Email validation constraint (RFC 5322 format)
    CONSTRAINT valid_email CHECK (email ~* '^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$'),
    -- Account type constraint
    CONSTRAINT valid_account_type CHECK (account_type IN ('personal', 'company'))
);

-- Index for email lookups (used in authentication)
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);

-- ============================================================================
-- WORKSPACES TABLE (Migration 000002)
-- Requirements: 2.1, 2.2 - Multi-Tenant Workspace Management
-- ============================================================================

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

-- ============================================================================
-- WORKSPACE MEMBERS TABLE (Migration 000003)
-- Requirements: 2.1, 2.2, 2.3, 2.4, 2.6 - Multi-Tenant Workspace Management
-- ============================================================================

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

-- ============================================================================
-- SUBSCRIPTIONS TABLE (Migration 000005)
-- Requirements: 3.1, 3.2 - Subscription Plan Management
-- ============================================================================

CREATE TABLE IF NOT EXISTS subscriptions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    workspace_id UUID REFERENCES workspaces(id) ON DELETE CASCADE,
    plan_type VARCHAR(50) NOT NULL DEFAULT 'freemium',
    status VARCHAR(50) NOT NULL DEFAULT 'active',
    current_period_start TIMESTAMP WITH TIME ZONE,
    current_period_end TIMESTAMP WITH TIME ZONE,
    stripe_subscription_id VARCHAR(255),
    stripe_customer_id VARCHAR(255),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    CONSTRAINT valid_plan_type CHECK (plan_type IN ('freemium', 'enterprise', 'exclusive')),
    CONSTRAINT valid_subscription_status CHECK (status IN ('active', 'canceled', 'expired', 'past_due'))
);

CREATE INDEX IF NOT EXISTS idx_subscriptions_user ON subscriptions(user_id);
CREATE INDEX IF NOT EXISTS idx_subscriptions_status ON subscriptions(status);

-- ============================================================================
-- PROJECTS TABLE (Migration 000006)
-- Requirements: 6.1, 10.5 - Component Management and Publishing
-- ============================================================================

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

CREATE INDEX IF NOT EXISTS idx_projects_workspace ON projects(workspace_id);
CREATE INDEX IF NOT EXISTS idx_projects_owner ON projects(owner_id);
CREATE INDEX IF NOT EXISTS idx_projects_status ON projects(status);

-- ============================================================================
-- PUBLISHED SITES TABLE (Migration 000007)
-- Requirements: 10.5, 11.1 - One-Click Publish and Domain Management
-- ============================================================================

CREATE TABLE IF NOT EXISTS published_sites (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    subdomain VARCHAR(255) NOT NULL UNIQUE,
    custom_domain VARCHAR(255),
    ssl_status VARCHAR(50) DEFAULT 'pending',
    published_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    CONSTRAINT valid_ssl_status CHECK (ssl_status IN ('pending', 'active', 'failed'))
);

CREATE INDEX IF NOT EXISTS idx_published_sites_project ON published_sites(project_id);
CREATE INDEX IF NOT EXISTS idx_published_sites_subdomain ON published_sites(subdomain);

-- ============================================================================
-- AUDIT LOGS TABLE (Migration 000008)
-- Requirements: 9.1, 13.6 - Audit Trail and Notification History
-- ============================================================================

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

CREATE INDEX IF NOT EXISTS idx_audit_logs_user ON audit_logs(user_id);
CREATE INDEX IF NOT EXISTS idx_audit_logs_created ON audit_logs(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_audit_logs_action ON audit_logs(action);
CREATE INDEX IF NOT EXISTS idx_audit_logs_resource ON audit_logs(resource_type, resource_id);

-- ============================================================================
-- ROW-LEVEL SECURITY (Migration 000004)
-- Requirements: 2.7, 15.4 - Multi-Tenant Workspace Management and Data Storage
-- ============================================================================

-- Enable Row-Level Security on workspaces table
ALTER TABLE workspaces ENABLE ROW LEVEL SECURITY;

-- Enable Row-Level Security on projects table
ALTER TABLE projects ENABLE ROW LEVEL SECURITY;

-- Enable Row-Level Security on workspace_members table
ALTER TABLE workspace_members ENABLE ROW LEVEL SECURITY;

-- Drop existing policies if they exist (for idempotency)
DROP POLICY IF EXISTS workspace_isolation ON workspaces;
DROP POLICY IF EXISTS workspace_select_policy ON workspaces;
DROP POLICY IF EXISTS workspace_insert_policy ON workspaces;
DROP POLICY IF EXISTS workspace_update_policy ON workspaces;
DROP POLICY IF EXISTS workspace_delete_policy ON workspaces;
DROP POLICY IF EXISTS project_isolation ON projects;
DROP POLICY IF EXISTS project_select_policy ON projects;
DROP POLICY IF EXISTS project_insert_policy ON projects;
DROP POLICY IF EXISTS project_update_policy ON projects;
DROP POLICY IF EXISTS project_delete_policy ON projects;
DROP POLICY IF EXISTS workspace_members_select_policy ON workspace_members;
DROP POLICY IF EXISTS workspace_members_insert_policy ON workspace_members;
DROP POLICY IF EXISTS workspace_members_update_policy ON workspace_members;
DROP POLICY IF EXISTS workspace_members_delete_policy ON workspace_members;

-- Create RLS policies for workspaces
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

CREATE POLICY workspace_insert_policy ON workspaces
    FOR INSERT
    WITH CHECK (owner_id = current_setting('app.current_user_id', true)::UUID);

CREATE POLICY workspace_update_policy ON workspaces
    FOR UPDATE
    USING (owner_id = current_setting('app.current_user_id', true)::UUID);

CREATE POLICY workspace_delete_policy ON workspaces
    FOR DELETE
    USING (owner_id = current_setting('app.current_user_id', true)::UUID);

-- Create RLS policies for projects
CREATE POLICY project_select_policy ON projects
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

CREATE POLICY project_insert_policy ON projects
    FOR INSERT
    WITH CHECK (
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

CREATE POLICY project_update_policy ON projects
    FOR UPDATE
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

CREATE POLICY project_delete_policy ON projects
    FOR DELETE
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

-- Create RLS policies for workspace_members
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

CREATE POLICY workspace_members_delete_policy ON workspace_members
    FOR DELETE
    USING (
        workspace_id IN (
            SELECT id FROM workspaces
            WHERE owner_id = current_setting('app.current_user_id', true)::UUID
        )
        OR user_id = current_setting('app.current_user_id', true)::UUID
    );

-- ============================================================================
-- HELPER FUNCTIONS
-- ============================================================================

-- Updated_at trigger function
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Function to set the current user context for RLS
CREATE OR REPLACE FUNCTION set_current_user_id(user_id UUID)
RETURNS VOID AS $$
BEGIN
    PERFORM set_config('app.current_user_id', user_id::TEXT, false);
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

-- Function to get the current user context
CREATE OR REPLACE FUNCTION get_current_user_id()
RETURNS UUID AS $$
BEGIN
    RETURN current_setting('app.current_user_id', true)::UUID;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

-- ============================================================================
-- TRIGGERS
-- ============================================================================

-- Apply updated_at trigger to all tables
DROP TRIGGER IF EXISTS update_users_updated_at ON users;
CREATE TRIGGER update_users_updated_at 
    BEFORE UPDATE ON users
    FOR EACH ROW 
    EXECUTE FUNCTION update_updated_at_column();

DROP TRIGGER IF EXISTS update_workspaces_updated_at ON workspaces;
CREATE TRIGGER update_workspaces_updated_at 
    BEFORE UPDATE ON workspaces
    FOR EACH ROW 
    EXECUTE FUNCTION update_updated_at_column();

DROP TRIGGER IF EXISTS update_subscriptions_updated_at ON subscriptions;
CREATE TRIGGER update_subscriptions_updated_at 
    BEFORE UPDATE ON subscriptions
    FOR EACH ROW 
    EXECUTE FUNCTION update_updated_at_column();

DROP TRIGGER IF EXISTS update_projects_updated_at ON projects;
CREATE TRIGGER update_projects_updated_at 
    BEFORE UPDATE ON projects
    FOR EACH ROW 
    EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- RECORD APPLIED MIGRATIONS
-- ============================================================================

INSERT INTO schema_migrations (version) VALUES
    ('20250118_000001_create_users_table.sql'),
    ('20250118_000002_create_workspaces_table.sql'),
    ('20250118_000003_create_workspace_members_table.sql'),
    ('20250118_000004_add_row_level_security.sql'),
    ('20250118_000005_create_subscriptions_table.sql'),
    ('20250118_000006_create_projects_table.sql'),
    ('20250118_000007_create_published_sites_table.sql'),
    ('20250118_000008_create_audit_logs_table.sql'),
    ('20240115000001_create_users_workspaces.sql'),
    ('20240115000002_create_subscriptions.sql'),
    ('20240115000003_create_projects.sql'),
    ('20240115000004_add_workspace_to_subscriptions.sql')
ON CONFLICT (version) DO NOTHING;

-- ============================================================================
-- COMMENTS FOR DOCUMENTATION
-- ============================================================================

COMMENT ON TABLE users IS 'Stores user accounts with email validation and account type';
COMMENT ON TABLE workspaces IS 'Workspaces for multi-tenant isolation: Personal or Company types';
COMMENT ON TABLE workspace_members IS 'Junction table for workspace membership with role-based access control';
COMMENT ON TABLE subscriptions IS 'User subscription plans: freemium, enterprise, exclusive';
COMMENT ON TABLE projects IS 'Website projects belonging to workspaces';
COMMENT ON TABLE published_sites IS 'Published website metadata for hosting';
COMMENT ON TABLE audit_logs IS 'System audit trail for user actions';

COMMENT ON FUNCTION set_current_user_id(UUID) IS 'Set the current user context for Row-Level Security';
COMMENT ON FUNCTION get_current_user_id() IS 'Get the current user context for Row-Level Security';

-- ============================================================================
-- SEED DATA
-- ============================================================================
-- All seed accounts use password: Password123!
-- (hash: $2b$12$kwCyZwwpHHPyNNQBt20xqOU/OtrdFJSa7VHfEES59gtxlQ/twzuIe)

INSERT INTO users (email, password_hash, full_name, account_type, role, email_verified_at)
VALUES
    ('admin@rinova.id',    '$2b$12$kwCyZwwpHHPyNNQBt20xqOU/OtrdFJSa7VHfEES59gtxlQ/twzuIe', 'Super Admin',     'personal', 'superuser', NOW()),
    ('admin1@rinova.io',   '$2b$12$kwCyZwwpHHPyNNQBt20xqOU/OtrdFJSa7VHfEES59gtxlQ/twzuIe', 'Admin One',       'personal', 'superuser', NOW()),
    ('admin2@rinova.io',   '$2b$12$kwCyZwwpHHPyNNQBt20xqOU/OtrdFJSa7VHfEES59gtxlQ/twzuIe', 'Admin Two',       'personal', 'user',      NOW()),
    ('admin3@rinova.io',   '$2b$12$kwCyZwwpHHPyNNQBt20xqOU/OtrdFJSa7VHfEES59gtxlQ/twzuIe', 'Admin Three',     'personal', 'user',      NOW()),
    ('admin4@rinova.io',   '$2b$12$kwCyZwwpHHPyNNQBt20xqOU/OtrdFJSa7VHfEES59gtxlQ/twzuIe', 'Admin Four',      'personal', 'user',      NOW()),
    ('admin5@rinova.io',   '$2b$12$kwCyZwwpHHPyNNQBt20xqOU/OtrdFJSa7VHfEES59gtxlQ/twzuIe', 'Admin Five',      'personal', 'user',      NOW()),
    ('enterprise1@rinova.io', '$2b$12$kwCyZwwpHHPyNNQBt20xqOU/OtrdFJSa7VHfEES59gtxlQ/twzuIe', 'Enterprise One',  'personal', 'user',  NOW()),
    ('enterprise2@rinova.io', '$2b$12$kwCyZwwpHHPyNNQBt20xqOU/OtrdFJSa7VHfEES59gtxlQ/twzuIe', 'Enterprise Two',  'personal', 'user',  NOW()),
    ('enterprise3@rinova.io', '$2b$12$kwCyZwwpHHPyNNQBt20xqOU/OtrdFJSa7VHfEES59gtxlQ/twzuIe', 'Enterprise Three','personal', 'user',  NOW()),
    ('enterprise4@rinova.io', '$2b$12$kwCyZwwpHHPyNNQBt20xqOU/OtrdFJSa7VHfEES59gtxlQ/twzuIe', 'Enterprise Four', 'personal', 'user',  NOW()),
    ('enterprise5@rinova.io', '$2b$12$kwCyZwwpHHPyNNQBt20xqOU/OtrdFJSa7VHfEES59gtxlQ/twzuIe', 'Enterprise Five', 'personal', 'user',  NOW()),
    ('exclusive1@rinova.io', '$2b$12$kwCyZwwpHHPyNNQBt20xqOU/OtrdFJSa7VHfEES59gtxlQ/twzuIe', 'Exclusive One',   'personal', 'user',  NOW()),
    ('exclusive2@rinova.io', '$2b$12$kwCyZwwpHHPyNNQBt20xqOU/OtrdFJSa7VHfEES59gtxlQ/twzuIe', 'Exclusive Two',   'personal', 'user',  NOW()),
    ('exclusive3@rinova.io', '$2b$12$kwCyZwwpHHPyNNQBt20xqOU/OtrdFJSa7VHfEES59gtxlQ/twzuIe', 'Exclusive Three', 'personal', 'user',  NOW()),
    ('exclusive4@rinova.io', '$2b$12$kwCyZwwpHHPyNNQBt20xqOU/OtrdFJSa7VHfEES59gtxlQ/twzuIe', 'Exclusive Four',  'personal', 'user',  NOW()),
    ('exclusive5@rinova.io', '$2b$12$kwCyZwwpHHPyNNQBt20xqOU/OtrdFJSa7VHfEES59gtxlQ/twzuIe', 'Exclusive Five',  'personal', 'user',  NOW()),
    ('test@rinova.local',  '$2b$12$kwCyZwwpHHPyNNQBt20xqOU/OtrdFJSa7VHfEES59gtxlQ/twzuIe', 'Test User',       'personal', 'user',  NOW())
ON CONFLICT (email) DO NOTHING;

-- Create personal workspace for every user
INSERT INTO workspaces (name, type, owner_id, created_at, updated_at)
SELECT 'Personal Workspace', 'personal', id, NOW(), NOW() FROM users
ON CONFLICT DO NOTHING;

-- Add owner as workspace member
INSERT INTO workspace_members (workspace_id, user_id, role, invitation_status, joined_at)
SELECT w.id, w.owner_id, 'owner', 'accepted', NOW()
FROM workspaces w
ON CONFLICT (workspace_id, user_id) DO NOTHING;

-- Create freemium subscriptions
INSERT INTO subscriptions (user_id, plan_type, status, current_period_start, current_period_end)
SELECT id, 'freemium', 'active', NOW(), NOW() + INTERVAL '1 month'
FROM users
ON CONFLICT DO NOTHING;
