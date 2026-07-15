-- Migration: 004_add_workspace_to_subscriptions
-- Description: Add workspace_id and stripe_customer_id to subscriptions table
-- Requirements: 3.3 (workspace subscriptions)

-- ============================================================================
-- Add columns to subscriptions table
-- ============================================================================

-- Add workspace_id column (nullable; NULL = personal subscription)
ALTER TABLE subscriptions
ADD COLUMN IF NOT EXISTS workspace_id UUID REFERENCES workspaces(id) ON DELETE CASCADE;

-- Add stripe_customer_id column
ALTER TABLE subscriptions
ADD COLUMN IF NOT EXISTS stripe_customer_id VARCHAR(255);

-- ============================================================================
-- Indexes
-- ============================================================================

-- A workspace can have at most one active subscription
CREATE UNIQUE INDEX IF NOT EXISTS idx_subscriptions_workspace
ON subscriptions(workspace_id) WHERE workspace_id IS NOT NULL;

-- Index for workspace subscription lookups
CREATE INDEX IF NOT EXISTS idx_subscriptions_workspace_user
ON subscriptions(user_id, workspace_id);

-- ============================================================================
-- Comments
-- ============================================================================

COMMENT ON COLUMN subscriptions.workspace_id IS 'If set, this is a workspace subscription (company use). If NULL, it is a personal subscription.';
COMMENT ON COLUMN subscriptions.stripe_customer_id IS 'Stripe customer ID for payment processing';
