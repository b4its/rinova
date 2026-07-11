# Rinova Website Builder - Nuxt Frontend Dockerfile
# Multi-stage build for optimized production image

# Build stage
FROM node:20-alpine AS builder

WORKDIR /app

# Copy package files
COPY package.json package-lock.json ./

# Install dependencies
RUN npm ci

# Copy source files
COPY . .

# Build the Nuxt application
RUN npm run build

# Production stage
FROM node:20-alpine

# Install runtime dependencies
RUN apk add --no-cache ca-certificates tzdata

# Create non-root user
RUN addgroup -S rinova && adduser -S rinova -G rinova

WORKDIR /app

# Copy built files from builder
COPY --from=builder /app/.output ./.output
COPY --from=builder /app/package.json ./
COPY --from=builder /app/nuxt.config.ts ./

# Set ownership
RUN chown -R rinova:rinova /app

# Switch to non-root user
USER rinova

# Expose port
EXPOSE 3000

# Set environment variables
ENV NUXT_HOST=0.0.0.0
ENV NUXT_PORT=3000
ENV NODE_ENV=production

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD wget --no-verbose --tries=1 --spider http://localhost:3000/health || exit 1

# Run the Nuxt server
CMD ["node", ".output/server/index.mjs"]
