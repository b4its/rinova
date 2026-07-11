-- Migration: Create projects, published_sites, and audit_logs tables
-- Requirements: 6.1, 10.5

-- Projects table
CREATE TABLE IF NOT EXISTS projects (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
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
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    subdomain VARCHAR(255) NOT NULL UNIQUE,
    custom_domain VARCHAR(255),
    ssl_status VARCHAR(50) DEFAULT 'pending',
    published_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    CONSTRAINT valid_ssl_status CHECK (ssl_status IN ('pending', 'active', 'failed'))
);

-- Audit logs
CREATE TABLE IF NOT EXISTS audit_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    action VARCHAR(100) NOT NULL,
    resource_type VARCHAR(100) NOT NULL,
    resource_id UUID,
    metadata JSONB DEFAULT '{}',
    ip_address INET,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Indexes for projects
CREATE INDEX IF NOT EXISTS idx_projects_workspace ON projects(workspace_id);
CREATE INDEX IF NOT EXISTS idx_projects_owner ON projects(owner_id);
CREATE INDEX IF NOT EXISTS idx_projects_status ON projects(status);
CREATE INDEX IF NOT EXISTS idx_published_sites_project ON published_sites(project_id);
CREATE INDEX IF NOT EXISTS idx_published_sites_subdomain ON published_sites(subdomain);
CREATE INDEX IF NOT EXISTS idx_audit_logs_user ON audit_logs(user_id);
CREATE INDEX IF NOT EXISTS idx_audit_logs_created ON audit_logs(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_audit_logs_resource ON audit_logs(resource_type, resource_id);

-- Add updated_at trigger for projects
CREATE TRIGGER update_projects_updated_at BEFORE UPDATE ON projects
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Row-Level Security for projects
ALTER TABLE projects ENABLE ROW LEVEL SECURITY;

CREATE POLICY project_isolation ON projects
    USING (workspace_id IN (SELECT id FROM workspaces WHERE owner_id = current_setting('app.current_user_id', true)::UUID
           OR id IN (SELECT workspace_id FROM workspace_members 
                     WHERE user_id = current_setting('app.current_user_id', true)::UUID
                     AND invitation_status = 'accepted')));

-- Comments for documentation
COMMENT ON TABLE projects IS 'Stores website projects with workspace isolation via RLS';
COMMENT ON TABLE published_sites IS 'Stores published site metadata with subdomain and custom domain';
COMMENT ON TABLE audit_logs IS 'Stores audit trail for user actions with ON DELETE SET NULL for user_id';
COMMENT ON COLUMN projects.workspace_id IS 'References workspaces.id with CASCADE delete';
COMMENT ON COLUMN projects.owner_id IS 'References users.id with CASCADE delete for user cleanup';
