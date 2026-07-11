-- Rinova Website Builder - PostgreSQL Initialization
-- Multi-tenant schema with row-level security

-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- Users table
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

-- Workspaces table
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

-- Workspace members (for company workspaces)
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

-- Subscriptions
CREATE TABLE IF NOT EXISTS subscriptions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    plan_type VARCHAR(50) NOT NULL DEFAULT 'freemium',
    status VARCHAR(50) NOT NULL DEFAULT 'active',
    current_period_start TIMESTAMP WITH TIME ZONE,
    current_period_end TIMESTAMP WITH TIME ZONE,
    stripe_subscription_id VARCHAR(255),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    CONSTRAINT valid_plan_type CHECK (plan_type IN ('freemium', 'enterprise', 'exclusive')),
    CONSTRAINT valid_subscription_status CHECK (status IN ('active', 'canceled', 'expired', 'past_due'))
);

-- Projects
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

-- Published sites
CREATE TABLE IF NOT EXISTS published_sites (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    subdomain VARCHAR(255) NOT NULL UNIQUE,
    custom_domain VARCHAR(255),
    ssl_status VARCHAR(50) DEFAULT 'pending',
    published_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    CONSTRAINT valid_ssl_status CHECK (ssl_status IN ('pending', 'active', 'failed'))
);

-- Audit logs
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

-- Row-Level Security for multi-tenancy
ALTER TABLE projects ENABLE ROW LEVEL SECURITY;
ALTER TABLE workspaces ENABLE ROW LEVEL SECURITY;

CREATE POLICY workspace_isolation ON workspaces
    USING (owner_id = current_setting('app.current_user_id')::UUID
           OR id IN (SELECT workspace_id FROM workspace_members 
                     WHERE user_id = current_setting('app.current_user_id')::UUID
                     AND invitation_status = 'accepted'));

CREATE POLICY project_isolation ON projects
    USING (workspace_id IN (SELECT id FROM workspaces WHERE owner_id = current_setting('app.current_user_id')::UUID
           OR id IN (SELECT workspace_id FROM workspace_members 
                     WHERE user_id = current_setting('app.current_user_id')::UUID
                     AND invitation_status = 'accepted')));

-- Indexes for performance
CREATE INDEX idx_workspaces_owner ON workspaces(owner_id);
CREATE INDEX idx_workspace_members_user ON workspace_members(user_id);
CREATE INDEX idx_workspace_members_workspace ON workspace_members(workspace_id);
CREATE INDEX idx_projects_workspace ON projects(workspace_id);
CREATE INDEX idx_projects_owner ON projects(owner_id);
CREATE INDEX idx_subscriptions_user ON subscriptions(user_id);
CREATE INDEX idx_audit_logs_user ON audit_logs(user_id);
CREATE INDEX idx_audit_logs_created ON audit_logs(created_at DESC);
CREATE INDEX idx_users_email ON users(email);

-- Updated_at trigger function
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Apply updated_at trigger to all tables
CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_workspaces_updated_at BEFORE UPDATE ON workspaces
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_subscriptions_updated_at BEFORE UPDATE ON subscriptions
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_projects_updated_at BEFORE UPDATE ON projects
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Initial admin user (password: admin123 - change in production)
-- INSERT INTO users (email, password_hash, full_name, account_type, email_verified_at)
-- VALUES ('admin@rinova.app', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5GyYIBkOkMmOa', 'Admin', 'personal', NOW());
