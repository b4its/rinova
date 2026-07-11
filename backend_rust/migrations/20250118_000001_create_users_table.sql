-- Migration: Create users table
-- Created at: 2025-01-18
-- Requirements: 1.1 - User Registration and Authentication

-- Enable UUID extension (if not already enabled)
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
    
    -- Email validation constraint (RFC 5322 format)
    CONSTRAINT valid_email CHECK (email ~* '^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$'),
    -- Account type constraint
    CONSTRAINT valid_account_type CHECK (account_type IN ('personal', 'company'))
);

-- Index for email lookups (used in authentication)
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);

-- Updated_at trigger function (idempotent)
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Apply updated_at trigger to users table
DROP TRIGGER IF EXISTS update_users_updated_at ON users;
CREATE TRIGGER update_users_updated_at 
    BEFORE UPDATE ON users
    FOR EACH ROW 
    EXECUTE FUNCTION update_updated_at_column();

-- Comment for documentation
COMMENT ON TABLE users IS 'Stores user accounts with email validation and account type';
COMMENT ON COLUMN users.email IS 'User email address, must be valid RFC 5322 format';
COMMENT ON COLUMN users.password_hash IS 'Bcrypt hashed password with cost factor 12';
COMMENT ON COLUMN users.account_type IS 'Account type: personal or company';
COMMENT ON COLUMN users.email_verified_at IS 'Timestamp when email was verified, NULL if not verified';
