# Rinova Website Builder - Nuxt Frontend Dockerfile
# Multi-stage build for optimized production SSR deployment

# =============================================================================
# Stage 1: Dependencies - Install npm packages
# =============================================================================
FROM node:20-alpine AS deps

# Install build dependencies for native modules
RUN apk add --no-cache python3 make g++

WORKDIR /app

# Copy package files first for better layer caching
COPY package.json package-lock.json* ./

# Install dependencies with ci for reproducible builds
RUN npm ci --prefer-offline --no-audit || npm install

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
ARG NUXT_PUBLIC_API_URL=http://localhost:8080
ARG NUXT_PUBLIC_WS_URL=ws://localhost:8080

# Set environment variables for build
ENV NUXT_PUBLIC_API_URL=${NUXT_PUBLIC_API_URL} \
    NUXT_PUBLIC_WS_URL=${NUXT_PUBLIC_WS_URL} \
    NODE_ENV=production

# Build the Nuxt application
RUN npm run build

# =============================================================================
# Stage 3: Production - Minimal runtime image
# =============================================================================
FROM node:20-alpine AS production

# Install runtime dependencies
RUN apk add --no-cache ca-certificates tzdata curl

# Create non-root user for security
RUN addgroup -S rinova && adduser -S rinova -G rinova

WORKDIR /app

# Copy built application from builder
COPY --from=builder /app/.output ./.output
COPY --from=builder /app/package.json ./package.json

# Create directory for static assets
RUN mkdir -p /app/.output/public && \
    chown -R rinova:rinova /app

# Switch to non-root user
USER rinova

# Expose port 3000 for SSR server
EXPOSE 3000

# Environment variables for runtime configuration
ENV NUXT_HOST=0.0.0.0 \
    NUXT_PORT=3000 \
    NODE_ENV=production

# Health check for SSR server
HEALTHCHECK --interval=30s --timeout=10s --start-period=10s --retries=3 \
    CMD curl -f http://localhost:3000/ || exit 1

# Labels for container metadata
LABEL maintainer="Rinova Team" \
      service="nuxt-frontend" \
      description="Rinova Website Builder - Nuxt SSR Frontend" \
      version="1.0.0"

# Run the Nuxt SSR server
CMD ["node", ".output/server/index.mjs"]
