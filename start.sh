#!/bin/bash

# Rinova Website Builder - Development Environment Startup Script

set -e

echo "🚀 Starting Rinova Website Builder Development Environment..."

# Check if .env exists
if [ ! -f .env ]; then
    echo "⚠️  .env file not found. Creating from .env.example..."
    cp .env.example .env
    echo "✅ Created .env file. Please update with your configuration."
    echo ""
fi

# Create necessary directories
echo "📁 Creating necessary directories..."
mkdir -p backend_rust/src
mkdir -p frontend_nuxt/components
mkdir -p frontend_nuxt/composables
mkdir -p frontend_nuxt/stores
mkdir -p frontend_nuxt/pages

# Pull required Docker images
echo "📦 Pulling Docker images..."
docker-compose pull

# Start services
echo "🔧 Starting Docker services..."
docker-compose up -d postgresql mongodb redis besu zmq-publisher

# Wait for databases to be ready
echo "⏳ Waiting for databases to be ready..."
sleep 10

# Check PostgreSQL
until docker-compose exec -T postgresql pg_isready -U rinova; do
    echo "Waiting for PostgreSQL..."
    sleep 2
done
echo "✅ PostgreSQL is ready"

# Check MongoDB
until docker-compose exec -T mongodb mongosh --eval "db.runCommand('ping').ok" --quiet; do
    echo "Waiting for MongoDB..."
    sleep 2
done
echo "✅ MongoDB is ready"

# Check Redis
until docker-compose exec -T redis redis-cli -a rinova_redis_secret_change_in_production ping | grep -q PONG; do
    echo "Waiting for Redis..."
    sleep 2
done
echo "✅ Redis is ready"

# Check Hyperledger Besu
until curl -s -X POST -H "Content-Type: application/json" --data '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}' http://localhost:8545 > /dev/null 2>&1; do
    echo "Waiting for Hyperledger Besu..."
    sleep 2
done
echo "✅ Hyperledger Besu is ready"

echo ""
echo "🎉 Development environment is ready!"
echo ""
echo "Services running:"
echo "  - PostgreSQL:     localhost:5432"
echo "  - MongoDB:        localhost:27017"
echo "  - Redis:          localhost:6379"
echo "  - Hyperledger Besu: localhost:8545 (JSON-RPC), localhost:8546 (WebSocket)"
echo "  - ZeroMQ:         localhost:5555 (Publisher), localhost:5556 (Subscriber)"
echo ""
echo "To start backend services:"
echo "  cd backend_rust && cargo run"
echo ""
echo "To start frontend:"
echo "  cd frontend_nuxt && npm run dev"
echo ""
echo "To stop all services:"
echo "  docker-compose down"
echo ""
