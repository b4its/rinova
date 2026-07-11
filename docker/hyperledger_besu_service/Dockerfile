# Hyperledger Besu Node Dockerfile
# Multi-stage build for Rinova private blockchain network

# Stage 1: Build stage (for custom plugins if needed)
FROM eclipse-temurin:17-jdk-alpine AS builder

WORKDIR /build

# Download Besu (if we need custom build)
# RUN apk add --no-cache curl && \
#     curl -L -o besu.tar.gz https://hyperledger.jfrog.io/hyperledger/besu-binaries/besu/${BESU_VERSION}/besu-${BESU_VERSION}.tar.gz && \
#     tar -xzf besu.tar.gz

# Stage 2: Runtime stage
FROM hyperledger/besu:latest

LABEL maintainer="Rinova Development Team"
LABEL description="Hyperledger Besu node for Rinova private blockchain"
LABEL version="1.0.0"

# Create directories for keys and data
USER root
RUN mkdir -p /opt/besu/keys /opt/besu/data /opt/besu/permissions && \
    chown -R besu:besu /opt/besu

# Copy configuration files
COPY genesis.json /opt/besu/config/genesis.json
COPY config.toml /opt/besu/config/config.toml
COPY keys/nodekey /opt/besu/keys/nodekey
COPY permissions/permissions_config.toml /opt/besu/permissions/permissions_config.toml

# Set proper permissions
RUN chmod 600 /opt/besu/keys/nodekey && \
    chown besu:besu /opt/besu/keys/nodekey

USER besu

# Expose ports
# 8545 - JSON-RPC HTTP
# 8546 - WebSocket
# 30303 - P2P discovery
# 9545 - Prometheus metrics
EXPOSE 8545 8546 30303 9545

# Health check
HEALTHCHECK --interval=10s --timeout=5s --retries=5 --start-period=60s \
    CMD curl -f -X POST -H 'Content-Type: application/json' \
        -d '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}' \
        http://localhost:8545 || exit 1

# Default entrypoint
ENTRYPOINT ["/opt/besu/bin/besu"]
CMD ["--config-file=/opt/besu/config/config.toml"]
