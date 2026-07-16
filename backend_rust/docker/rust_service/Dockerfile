# Rinova Website Builder - Rust Backend Services Dockerfile
# Multi-stage build for optimized production image
# Supports: user_service, subscription_service, project_service, 
#            publishing_service, notification_service, api_gateway, marketplace_service

# =============================================================================
# Stage 1: Builder - Compile Rust application
# =============================================================================
FROM rust:1.88-alpine AS builder

# Install build dependencies
# - musl-dev: Static linking for Alpine
# - pkgconfig: Library configuration
# - openssl-dev: SSL/TLS support
# - g++: C++ compiler for zmq-sys
# - zeromq-dev: ZeroMQ development files
RUN apk add --no-cache \
    musl-dev \
    pkgconfig \
    openssl-dev \
    openssl-libs-static \
    protobuf \
    g++ \
    zeromq-dev \
    cmake

# Set working directory
WORKDIR /app

# Copy workspace configuration first (for better layer caching)
COPY Cargo.toml Cargo.lock ./

# Copy all workspace members
COPY libs ./libs
COPY services ./services

# Build argument for service name
ARG SERVICE_NAME
ENV SERVICE_NAME=${SERVICE_NAME}

# Use sqlx offline mode for compile-time checks without database
# Note: This requires .sqlx/sqlx-data.json to be present
# If not present, sqlx will use runtime checks instead
ENV SQLX_OFFLINE=false

# Build the service in release mode with optimizations
RUN cargo build --release --bin ${SERVICE_NAME}

# =============================================================================
# Stage 2: Runtime - Minimal production image
# =============================================================================
FROM alpine:3.19 AS runtime

# Install runtime dependencies only
# - ca-certificates: TLS/SSL certificate validation
# - tzdata: Timezone support
# - curl: Health checks
# - libzmq: ZeroMQ library for runtime
RUN apk add --no-cache \
    ca-certificates \
    tzdata \
    curl \
    libzmq

# Create non-root user for security
RUN addgroup -S rinova && adduser -S rinova -G rinova

# Set working directory
WORKDIR /app

# Build argument for service name (passed from builder)
ARG SERVICE_NAME
ARG SERVICE_PORT=8000
ARG SERVICE_TYPE=service

# Environment variables for service configuration
# These can be overridden at runtime
ENV SERVICE_NAME=${SERVICE_NAME} \
    SERVICE_PORT=${SERVICE_PORT} \
    SERVICE_TYPE=${SERVICE_TYPE} \
    RUST_LOG=info \
    RUST_BACKTRACE=0 \
    # Network configuration
    SERVICE_HOST=0.0.0.0 \
    # Database connection defaults (override in docker-compose)
    DATABASE_URL="" \
    MONGODB_URI="" \
    REDIS_URL="" \
    # Message bus
    ZMQ_PUBLISHER_URL="" \
    ZMQ_SUBSCRIBER_URL="" \
    # Service discovery URLs
    USER_SERVICE_URL="" \
    SUBSCRIPTION_SERVICE_URL="" \
    PROJECT_SERVICE_URL="" \
    PUBLISHING_SERVICE_URL="" \
    NOTIFICATION_SERVICE_URL="" \
    # Security
    JWT_SECRET="" \


# Copy the compiled binary from builder
COPY --from=builder /app/target/release/${SERVICE_NAME} /app/service

# Create config directory for any configuration files
RUN mkdir -p /app/config && chown -R rinova:rinova /app

# Switch to non-root user for security
USER rinova

# Expose the service port
EXPOSE ${SERVICE_PORT}

# Health check configuration
HEALTHCHECK --interval=30s --timeout=10s --start-period=10s --retries=3 \
    CMD curl -f http://localhost:${SERVICE_PORT}/health || exit 1

# Set entrypoint
ENTRYPOINT ["/app/service"]

# =============================================================================
# Stage 3: Development - For local development with hot reload
# =============================================================================
FROM rust:1.88-alpine AS development

# Install development dependencies
RUN apk add --no-cache \
    musl-dev \
    pkgconfig \
    openssl-dev \
    git \
    g++ \
    zeromq-dev \
    cmake

WORKDIR /app

# Copy source files
COPY . .

# Build argument for service name
ARG SERVICE_NAME
ENV SERVICE_NAME=${SERVICE_NAME} \
    RUST_LOG=debug \
    RUST_BACKTRACE=1 \
    SQLX_OFFLINE=true

# Expose port for development
ARG SERVICE_PORT=8000
EXPOSE ${SERVICE_PORT}

# Run in development mode
CMD cargo run --bin ${SERVICE_NAME}
