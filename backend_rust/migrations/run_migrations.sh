#!/bin/bash
# Migration Runner Script for Rinova PostgreSQL
# Usage: ./run_migrations.sh [database_url]

set -e

# Default database URL from environment or docker-compose
DB_URL="${DATABASE_URL:-postgresql://rinova:rinova_secret@localhost:5432/rinova}"

if [ -n "$1" ]; then
    DB_URL="$1"
fi

echo "Running migrations against: $(echo $DB_URL | sed 's/:[^:@]*@/:***@/')"

# Migration directory
MIGRATION_DIR="$(dirname "$0")"

# List of migrations in order
MIGRATIONS=(
    "20250118_000001_create_users_table.sql"
    "20250118_000002_create_workspaces_table.sql"
    "20250118_000003_create_workspace_members_table.sql"
    "20250118_000004_add_row_level_security.sql"
)

# Create migrations tracking table if not exists
psql "$DB_URL" -c "
CREATE TABLE IF NOT EXISTS schema_migrations (
    version VARCHAR(255) PRIMARY KEY,
    applied_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
"

# Apply each migration
for migration in "${MIGRATIONS[@]}"; do
    # Check if already applied
    applied=$(psql "$DB_URL" -t -c "SELECT COUNT(*) FROM schema_migrations WHERE version = '$migration';" | tr -d ' ')
    
    if [ "$applied" -eq "0" ]; then
        echo "Applying migration: $migration"
        psql "$DB_URL" -f "$MIGRATION_DIR/$migration"
        psql "$DB_URL" -c "INSERT INTO schema_migrations (version) VALUES ('$migration');"
        echo "✓ Applied: $migration"
    else
        echo "⊗ Skipped (already applied): $migration"
    fi
done

echo ""
echo "All migrations completed successfully!"
