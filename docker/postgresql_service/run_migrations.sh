#!/bin/bash
# PostgreSQL Migration Runner
# Runs migrations in order and tracks them in schema_migrations table

set -e

MIGRATIONS_DIR="/docker-entrypoint-initdb.d/migrations"
MIGRATION_TABLE="schema_migrations"

echo "Starting database migrations..."

# Check if migrations directory exists
if [ ! -d "$MIGRATIONS_DIR" ]; then
    echo "ERROR: Migrations directory not found: $MIGRATIONS_DIR"
    exit 1
fi

# Get list of migration files sorted by name
MIGRATION_FILES=$(ls -1 "$MIGRATIONS_DIR"/*.sql 2>/dev/null | sort -V)

if [ -z "$MIGRATION_FILES" ]; then
    echo "No migration files found in $MIGRATIONS_DIR"
    exit 0
fi

# Run each migration
for MIGRATION_FILE in $MIGRATION_FILES; do
    MIGRATION_NAME=$(basename "$MIGRATION_FILE" .sql)
    
    # Check if migration has already been applied
    ALREADY_APPLIED=$(psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" -t -c \
        "SELECT COUNT(*) FROM $MIGRATION_TABLE WHERE version = '$MIGRATION_NAME';" 2>/dev/null || echo "0")
    
    if [ "$ALREADY_APPLIED" -gt 0 ]; then
        echo "Migration $MIGRATION_NAME already applied, skipping..."
        continue
    fi
    
    echo "Applying migration: $MIGRATION_NAME"
    
    # Extract description from migration file (first comment line after -- Description:)
    DESCRIPTION=$(grep -m1 "^-- Description:" "$MIGRATION_FILE" | sed 's/-- Description: //' || echo "")
    
    # Run the migration
    psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" -f "$MIGRATION_FILE"
    
    if [ $? -eq 0 ]; then
        # Record the migration
        psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" -c \
            "INSERT INTO $MIGRATION_TABLE (version, description) VALUES ('$MIGRATION_NAME', '$DESCRIPTION');"
        echo "Migration $MIGRATION_NAME completed successfully."
    else
        echo "ERROR: Migration $MIGRATION_NAME failed!"
        exit 1
    fi
done

echo "All migrations completed successfully."
