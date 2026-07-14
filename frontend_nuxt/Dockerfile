# Rinova Frontend (Nuxt 4) - Multi-stage build using pnpm
# Produces a minimal Node runtime image serving the Nitro server output.

# =============================================================================
# Stage 1: Base - Node + pnpm via corepack
# =============================================================================
FROM node:22-alpine AS base
RUN corepack enable && corepack prepare pnpm@11.10.0 --activate
WORKDIR /app

# =============================================================================
# Stage 2: Dependencies - install with a cached pnpm store
# =============================================================================
FROM base AS deps
COPY package.json pnpm-lock.yaml pnpm-workspace.yaml* ./
# Note: no BuildKit cache mount here — the classic Docker builder is in use.
RUN pnpm install --frozen-lockfile

# =============================================================================
# Stage 3: Builder - compile the Nuxt app
# =============================================================================
FROM base AS builder

# Build-time public config (baked into the client bundle)
ARG NUXT_PUBLIC_API_URL=http://localhost:8080
ARG NUXT_PUBLIC_WS_URL=ws://localhost:8080
ARG NUXT_PUBLIC_SENTRY_DSN=
ARG NUXT_PUBLIC_CDN_URL=
ENV NUXT_PUBLIC_API_URL=${NUXT_PUBLIC_API_URL} \
    NUXT_PUBLIC_WS_URL=${NUXT_PUBLIC_WS_URL} \
    NUXT_PUBLIC_SENTRY_DSN=${NUXT_PUBLIC_SENTRY_DSN} \
    NUXT_PUBLIC_CDN_URL=${NUXT_PUBLIC_CDN_URL}

COPY --from=deps /app/node_modules ./node_modules
COPY . .
RUN pnpm build

# =============================================================================
# Stage 4: Production - minimal runtime with only the built output
# =============================================================================
FROM node:22-alpine AS production
WORKDIR /app

# curl is used by the docker-compose healthcheck
RUN apk add --no-cache curl \
    && addgroup -S rinova && adduser -S rinova -G rinova

ENV NODE_ENV=production \
    NUXT_HOST=0.0.0.0 \
    NUXT_PORT=3000 \
    NITRO_HOST=0.0.0.0 \
    NITRO_PORT=3000

# The Nitro build output is fully self-contained (bundled node_modules).
COPY --from=builder --chown=rinova:rinova /app/.output ./.output

USER rinova
EXPOSE 3000

HEALTHCHECK --interval=30s --timeout=10s --start-period=30s --retries=3 \
    CMD curl -f http://localhost:3000/health || exit 1

CMD ["node", ".output/server/index.mjs"]
