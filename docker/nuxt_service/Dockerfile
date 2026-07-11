# Rinova Website Builder - Nuxt Frontend Dockerfile
# Multi-stage build for optimized production SSR deployment
# Supports: Server-Side Rendering with Node.js runtime, CDN static assets, environment injection

# =============================================================================
# Stage 1: Dependencies - Install npm packages
# =============================================================================
FROM node:20-alpine AS deps

# Install build dependencies for native modules
# - python3: Required for node-gyp
# - make, g++: Required for compiling native addons
RUN apk add --no-cache python3 make g++

WORKDIR /app

# Copy package files first for better layer caching
COPY package.json package-lock.json* ./

# Install dependencies with ci for reproducible builds
# --prefer-offline: Use cached packages when available
# --no-audit: Skip audit for faster install
RUN npm ci --prefer-offline --no-audit

# =============================================================================
# Stage 2: Builder - Build the Nuxt application
# =============================================================================
FROM node:20-alpine AS builder

WORKDIR /app

# Copy dependencies from deps stage
COPY --from=deps /app/node_modules ./node_modules

# Copy source files
COPY . .

# Build arguments for environment injection at build time
# These are used for NUXT_PUBLIC_* variables that need to be embedded at build
ARG NUXT_PUBLIC_API_URL=http://localhost:8080
ARG NUXT_PUBLIC_WS_URL=ws://localhost:8080
ARG NUXT_PUBLIC_SENTRY_DSN=""
ARG NUXT_PUBLIC_CDN_URL=""

# Set environment variables for build
# NUXT_ENV variables are available during build time
ENV NUXT_PUBLIC_API_URL=${NUXT_PUBLIC_API_URL} \
    NUXT_PUBLIC_WS_URL=${NUXT_PUBLIC_WS_URL} \
    NUXT_PUBLIC_SENTRY_DSN=${NUXT_PUBLIC_SENTRY_DSN} \
    NUXT_PUBLIC_CDN_URL=${NUXT_PUBLIC_CDN_URL} \
    NODE_ENV=production

# Build the Nuxt application
# Output goes to .output directory
RUN npm run build

# =============================================================================
# Stage 3: Production - Minimal runtime image
# =============================================================================
FROM node:20-alpine AS production

# Install runtime dependencies
# - ca-certificates: TLS/SSL certificate validation
# - tzdata: Timezone support
# - curl: Health checks and HTTP requests
RUN apk add --no-cache ca-certificates tzdata curl

# Create non-root user for security
RUN addgroup -S rinova && adduser -S rinova -G rinova

WORKDIR /app

# Copy built application from builder
# .output contains both server and static files
COPY --from=builder /app/.output ./.output
COPY --from=builder /app/package.json ./package.json

# Create directory for static assets (for CDN sync)
RUN mkdir -p /app/.output/public && \
    chown -R rinova:rinova /app

# Switch to non-root user
USER rinova

# Expose port 3000 for SSR server
EXPOSE 3000

# Environment variables for runtime configuration
# These can be overridden at runtime via docker-compose or k8s
ENV NUXT_HOST=0.0.0.0 \
    NUXT_PORT=3000 \
    NODE_ENV=production \
    # Runtime environment injection for API endpoints
    NUXT_PUBLIC_API_URL=${NUXT_PUBLIC_API_URL:-http://localhost:8080} \
    NUXT_PUBLIC_WS_URL=${NUXT_PUBLIC_WS_URL:-ws://localhost:8080} \
    NUXT_PUBLIC_SENTRY_DSN=${NUXT_PUBLIC_SENTRY_DSN:-""} \
    NUXT_PUBLIC_CDN_URL=${NUXT_PUBLIC_CDN_URL:-""}

# Health check for SSR server
# Verifies the server is responding on /health endpoint
HEALTHCHECK --interval=30s --timeout=10s --start-period=10s --retries=3 \
    CMD curl -f http://localhost:3000/health || exit 1

# Labels for container metadata
LABEL maintainer="Rinova Team" \
      service="nuxt-frontend" \
      description="Rinova Website Builder - Nuxt SSR Frontend" \
      version="1.0.0"

# Run the Nuxt SSR server
# .output/server/index.mjs is the entry point for SSR
CMD ["node", ".output/server/index.mjs"]

# =============================================================================
# Stage 4: Static Assets - For CDN deployment (optional)
# =============================================================================
FROM alpine:3.19 AS static-assets

# This stage extracts static assets for CDN sync
# Use this stage to copy static files to CDN storage
WORKDIR /assets

# Copy static assets from builder
COPY --from=builder /app/.output/public ./

# Default command - list assets for verification
CMD ["sh", "-c", "find . -type f | head -20"]

# =============================================================================
# Development Stage - For local development with hot reload
# =============================================================================
FROM node:20-alpine AS development

WORKDIR /app

# Install development dependencies
RUN apk add --no-cache python3 make g++ git

# Copy package files
COPY package.json package-lock.json* ./

# Install all dependencies including devDependencies
RUN npm ci

# Copy source files
COPY . .

# Environment for development
ENV NODE_ENV=development \
    NUXT_HOST=0.0.0.0 \
    NUXT_PORT=3000

# Expose port
EXPOSE 3000

# Health check for development server
HEALTHCHECK --interval=30s --timeout=10s --start-period=30s --retries=3 \
    CMD curl -f http://localhost:3000/health || exit 1

# Run development server with hot reload
CMD ["npm", "run", "dev"]
