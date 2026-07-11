-- Migration: Create subscriptions table
-- Requirements: 3.1, 3.2

-- Create subscriptions table with plan_type constraints
CREATE TABLE IF NOT EXISTS subscriptions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
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

-- Add indexes for user_id and status lookups
CREATE INDEX IF NOT EXISTS idx_subscriptions_user ON subscriptions(user_id);
CREATE INDEX IF NOT EXISTS idx_subscriptions_status ON subscriptions(status);
CREATE INDEX IF NOT EXISTS idx_subscriptions_user_status ON subscriptions(user_id, status);

-- Add updated_at trigger
CREATE TRIGGER update_subscriptions_updated_at BEFORE UPDATE ON subscriptions
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Comment for documentation
COMMENT ON TABLE subscriptions IS 'Stores user subscription plans with plan_type: freemium, enterprise, exclusive';
COMMENT ON COLUMN subscriptions.plan_type IS 'Subscription tier: freemium (default), enterprise, exclusive';
COMMENT ON COLUMN subscriptions.status IS 'Subscription status: active, canceled, expired, past_due';
COMMENT ON COLUMN subscriptions.user_id IS 'References users.id with CASCADE delete for user cleanup';
