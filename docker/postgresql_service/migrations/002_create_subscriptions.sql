-- Migration: 002_create_subscriptions
-- Description: Create subscriptions table with plan_type constraints and indexes
-- Requirements: 3.1, 3.2
-- Created: 2025-01-21

-- ============================================================================
-- Subscriptions Table
-- ============================================================================
-- Stores user subscription plans and billing information

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

COMMENT ON TABLE subscriptions IS 'Stores user subscription plans and billing information';
COMMENT ON COLUMN subscriptions.plan_type IS 'Subscription plan: freemium, enterprise, or exclusive';
COMMENT ON COLUMN subscriptions.status IS 'Subscription status: active, canceled, expired, or past_due';

-- ============================================================================
-- Indexes
-- ============================================================================

CREATE INDEX IF NOT EXISTS idx_subscriptions_user ON subscriptions(user_id);
CREATE INDEX IF NOT EXISTS idx_subscriptions_status ON subscriptions(status);
CREATE INDEX IF NOT EXISTS idx_subscriptions_stripe ON subscriptions(stripe_subscription_id) WHERE stripe_subscription_id IS NOT NULL;

-- ============================================================================
-- Triggers
-- ============================================================================

CREATE TRIGGER update_subscriptions_updated_at 
    BEFORE UPDATE ON subscriptions
    FOR EACH ROW 
    EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- Plan Limits Documentation
-- ============================================================================
-- Freemium Plan:
--   - Max 2 active projects
--   - Basic drag & drop editor
--   - No premium components
--   - Rate limit: 100 req/min
--
-- Enterprise Plan:
--   - Max 10 active projects
--   - Animation Editor, Custom CSS, SEO Tools
--   - Asset Marketplace access
--   - Rate limit: 1000 req/min
--
-- Exclusive Plan:
--   - Unlimited projects
--   - One-Click Publish with hosting
--   - Custom domain support
--   - Analytics Dashboard
--   - Rate limit: 5000 req/min
