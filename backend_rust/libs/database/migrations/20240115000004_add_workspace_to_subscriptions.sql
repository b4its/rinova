-- Migration: Add workspace_id and stripe_customer_id to subscriptions
-- Requirements: 3.3 (workspace subscriptions)

-- Add workspace_id column (nullable; NULL = personal subscription)
ALTER TABLE subscriptions
ADD COLUMN IF NOT EXISTS workspace_id UUID REFERENCES workspaces(id) ON DELETE CASCADE;

-- Add stripe_customer_id column (for Stripe integration)
ALTER TABLE subscriptions
ADD COLUMN IF NOT EXISTS stripe_customer_id VARCHAR(255);

-- Add unique constraint: a workspace can have at most one subscription
-- (only enforced when workspace_id IS NOT NULL)
CREATE UNIQUE INDEX IF NOT EXISTS idx_subscriptions_workspace
ON subscriptions(workspace_id) WHERE workspace_id IS NOT NULL;

-- Index for looking up workspace subscriptions
CREATE INDEX IF NOT EXISTS idx_subscriptions_workspace_user
ON subscriptions(user_id, workspace_id);

COMMENT ON COLUMN subscriptions.workspace_id IS 'If set, this is a workspace subscription (company). If NULL, it is a personal subscription.';
COMMENT ON COLUMN subscriptions.stripe_customer_id IS 'Stripe customer ID for payment processing';
