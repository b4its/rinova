# Build stage
FROM node:20-alpine AS builder

WORKDIR /app

# Copy package files
COPY package*.json ./

# Install dependencies
RUN npm ci

# Copy source files
COPY . .

# Build the application
RUN npm run build

# Production stage
FROM node:20-alpine AS runner

WORKDIR /app

# Set to production
ENV NODE_ENV=production

# Copy built files
COPY --from=builder /app/.output ./.output
COPY --from=builder /app/package.json ./package.json

# Expose port
EXPOSE 3000

# Start the server
CMD ["node", ".output/server/index.mjs"]
