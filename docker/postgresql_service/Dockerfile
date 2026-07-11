# Rinova Website Builder - PostgreSQL Service
# Multi-tenant database with row-level security

FROM postgres:16-alpine

# Set environment variables
ENV POSTGRES_DB=rinova
ENV POSTGRES_USER=rinova
ENV POSTGRES_PASSWORD=rinova_secret

# Copy initialization scripts
# Docker will automatically run files in /docker-entrypoint-initdb.d/
COPY init.sql /docker-entrypoint-initdb.d/01_init.sql
COPY migrations/ /docker-entrypoint-initdb.d/migrations/

# Expose PostgreSQL port
EXPOSE 5432

# Health check
HEALTHCHECK --interval=10s --timeout=5s --start-period=30s --retries=3 \
    CMD pg_isready -U $POSTGRES_USER -d $POSTGRES_DB || exit 1

# Optional: Add custom PostgreSQL configuration
# COPY postgresql.conf /etc/postgresql/postgresql.conf
# CMD ["postgres", "-c", "config_file=/etc/postgresql/postgresql.conf"]
