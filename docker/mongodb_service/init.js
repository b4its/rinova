// Rinova Website Builder - MongoDB Initialization
// Collections and indexes for document storage

// Switch to rinova database
db = db.getSiblingDB('rinova');

// Create collections
db.createCollection('project_components');
db.createCollection('custom_css');
db.createCollection('component_library');
db.createCollection('audit_trail_records');
db.createCollection('analytics_events');

// Project Components Collection
// Stores the component tree for each project
db.project_components.createIndex(
    { project_id: 1, workspace_id: 1 },
    { unique: true, name: 'idx_project_workspace' }
);

db.project_components.createIndex(
    { 'components.id': 1 },
    { name: 'idx_component_id' }
);

db.project_components.createIndex(
    { updated_at: -1 },
    { name: 'idx_updated_at' }
);

// Custom CSS Collection
db.custom_css.createIndex(
    { project_id: 1, css_scope: 1 },
    { unique: true, name: 'idx_project_scope' }
);

db.custom_css.createIndex(
    { project_id: 1, component_id: 1 },
    { name: 'idx_project_component' }
);

// Component Library (Asset Marketplace)
db.component_library.createIndex(
    { component_id: 1 },
    { unique: true, name: 'idx_component_id_unique' }
);

db.component_library.createIndex(
    { category: 1, is_premium: 1 },
    { name: 'idx_category_premium' }
);

db.component_library.createIndex(
    { creator_id: 1 },
    { name: 'idx_creator' }
);

db.component_library.createIndex(
    { downloads: -1 },
    { name: 'idx_downloads' }
);

db.component_library.createIndex(
    { rating: -1 },
    { name: 'idx_rating' }
);

// Audit Trail Records (synced from blockchain)
db.audit_trail_records.createIndex(
    { project_id: 1, published_at: -1 },
    { name: 'idx_project_published' }
);

db.audit_trail_records.createIndex(
    { transaction_hash: 1 },
    { unique: true, name: 'idx_tx_hash' }
);

db.audit_trail_records.createIndex(
    { publish_hash: 1 },
    { name: 'idx_publish_hash' }
);

// Analytics Events Collection
db.analytics_events.createIndex(
    { site_id: 1, timestamp: -1 },
    { name: 'idx_site_timestamp' }
);

db.analytics_events.createIndex(
    { visitor_id: 1, session_id: 1 },
    { name: 'idx_visitor_session' }
);

db.analytics_events.createIndex(
    { event_type: 1, timestamp: -1 },
    { name: 'idx_event_timestamp' }
);

// TTL index for analytics data (retain for 12 months)
db.analytics_events.createIndex(
    { timestamp: 1 },
    { expireAfterSeconds: 31536000, name: 'idx_ttl_analytics' }
);

// Insert sample component library data
db.component_library.insertMany([
    {
        component_id: 'hero-gradient',
        name: 'Gradient Hero Section',
        description: 'Modern hero section with gradient background',
        category: 'hero',
        price_usd: 0,
        is_premium: false,
        included_in_plans: ['freemium', 'enterprise', 'exclusive'],
        creator_id: 'rinova-official',
        component_schema: {
            type: 'Hero',
            defaultProps: {
                heading: 'Welcome',
                subheading: 'Start building your website',
                gradient: 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)'
            },
            defaultStyles: {
                desktop: { minHeight: '400px', display: 'flex', alignItems: 'center' }
            }
        },
        preview_image_url: 'https://cdn.rinova.app/components/hero-gradient.png',
        downloads: 0,
        rating: 0,
        created_at: new Date()
    },
    {
        component_id: 'cta-premium',
        name: 'Premium Call-to-Action',
        description: 'Animated CTA button with hover effects',
        category: 'button',
        price_usd: 9.99,
        is_premium: true,
        included_in_plans: ['enterprise', 'exclusive'],
        creator_id: 'rinova-official',
        component_schema: {
            type: 'Button',
            defaultProps: {
                text: 'Get Started',
                variant: 'primary'
            },
            defaultStyles: {
                desktop: { padding: '16px 32px', fontSize: '18px' }
            },
            animations: [
                { type: 'scale', trigger: 'hover', duration: 200 }
            ]
        },
        preview_image_url: 'https://cdn.rinova.app/components/cta-premium.png',
        downloads: 0,
        rating: 0,
        created_at: new Date()
    }
]);

print('MongoDB initialization completed successfully');
