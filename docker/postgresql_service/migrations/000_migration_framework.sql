-- Migration: 000_migration_framework
-- Description: Create migration tracking table and utility functions
-- Created: 2025-01-21

-- ============================================================================
-- Schema Migrations Table
-- ============================================================================
-- Tracks which migrations have been applied to the database

CREATE TABLE IF NOT EXISTS schema_migrations (
    version VARCHAR(255) PRIMARY KEY,
    applied_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    description TEXT
);

COMMENT ON TABLE schema_migrations IS 'Tracks database schema migrations';

-- ============================================================================
-- Utility Functions
-- ============================================================================

-- Function to check if a migration has been applied
CREATE OR REPLACE FUNCTION migration_applied(p_version VARCHAR)
RETURNS BOOLEAN AS $$
BEGIN
    RETURN EXISTS (SELECT 1 FROM schema_migrations WHERE version = p_version);
END;
$$ LANGUAGE plpgsql;

-- Function to record a migration
CREATE OR REPLACE FUNCTION record_migration(p_version VARCHAR, p_description TEXT DEFAULT NULL)
RETURNS VOID AS $$
BEGIN
    INSERT INTO schema_migrations (version, description)
    VALUES (p_version, p_description)
    ON CONFLICT (version) DO NOTHING;
END;
$$ LANGUAGE plpgsql;

-- ============================================================================
-- Updated_at Trigger Function
-- ============================================================================
-- Automatically updates the updated_at column on any table that has one

CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- ============================================================================
-- Set Current User Context Function
-- ============================================================================
-- Sets the current user ID for row-level security policies
-- This should be called by the application after authentication

CREATE OR REPLACE FUNCTION set_current_user_id(p_user_id UUID)
RETURNS VOID AS $$
BEGIN
    PERFORM set_config('app.current_user_id', p_user_id::TEXT, FALSE);
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

-- ============================================================================
-- Get Current User Context Function
-- ============================================================================
-- Returns the current user ID from the session context

CREATE OR REPLACE FUNCTION get_current_user_id()
RETURNS UUID AS $$
BEGIN
    RETURN current_setting('app.current_user_id', TRUE)::UUID;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;
