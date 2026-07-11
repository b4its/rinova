# Panduan Deployment - Rinova Website Builder

Dokumentasi ini menjelaskan cara menjalankan dan mendeploy Rinova Website Builder secara detail.

## Daftar Isi

1. [Prasyarat](#prasyarat)
2. [Environment Variables](#environment-variables)
3. [Menjalankan dengan Docker Compose](#menjalankan-dengan-docker-compose)
4. [Development Lokal](#development-lokal)
5. [Production Deployment](#production-deployment)
6. [Troubleshooting](#troubleshooting)
7. [Monitoring & Logging](#monitoring--logging)

---

## Prasyarat

### Hardware Requirements

#### Minimum (Development)
- CPU: 4 cores
- RAM: 8GB
- Storage: 50GB SSD
- Network: 10 Mbps

#### Recommended (Production)
- CPU: 8+ cores
- RAM: 16GB+
- Storage: 100GB+ SSD
- Network: 100 Mbps+

### Software Requirements

#### Wajib
- **Docker Engine**: v24.0 atau lebih baru
- **Docker Compose**: v2.0 atau lebih baru

#### Opsional (untuk development lokal)
- **Node.js**: v18.0 atau lebih baru
- **Rust**: v1.75 atau lebih baru
- **PostgreSQL**: v15 atau lebih baru
- **MongoDB**: v6.0 atau lebih baru
- **Redis**: v7 atau lebih baru

### Verifikasi Instalasi Docker

```bash
# Cek versi Docker
docker --version
# Output: Docker version 24.0.x, build xxxxx

# Cek versi Docker Compose
docker compose version
# Output: Docker Compose version v2.x.x

# Cek Docker daemon berjalan
docker ps
# Output: CONTAINER ID   IMAGE     COMMAND   ...
```

---

## Environment Variables

### Membuat File .env

```bash
# Copy dari template
cp .env.example .env

# Edit file
nano .env
```

### Daftar Lengkap Environment Variables

#### Database Configuration

```env
# PostgreSQL Configuration
POSTGRES_USER=rinova
POSTGRES_PASSWORD=rinova_secret_change_in_production
POSTGRES_DB=rinova

# MongoDB Configuration
MONGO_USER=rinova
MONGO_PASSWORD=rinova_secret_change_in_production
MONGO_DB=rinova

# Redis Configuration
REDIS_PASSWORD=rinova_redis_secret_change_in_production
```

#### Authentication & Security

```env
# JWT Secret (WAJIB GANTI DI PRODUCTION)
JWT_SECRET=your_super_secret_jwt_key_minimum_32_characters_long

# Generate secure secret dengan:
# openssl rand -base64 32
```

#### Email Configuration (SMTP)

```env
# SMTP untuk email notifications
SMTP_HOST=smtp.gmail.com
SMTP_PORT=587
SMTP_USER=your_email@gmail.com
SMTP_PASSWORD=your_app_password
```

Untuk Gmail, gunakan App Password:
1. Enable 2FA di Google Account
2. Generate App Password di Security settings
3. Gunakan App Password sebagai SMTP_PASSWORD

#### Payment Configuration (Stripe)

```env
# Stripe API Keys
STRIPE_SECRET_KEY=sk_live_your_stripe_secret_key
STRIPE_WEBHOOK_SECRET=whsec_your_webhook_secret

# Untuk testing, gunakan test keys:
# STRIPE_SECRET_KEY=sk_test_...
```

Dapatkan keys dari: https://dashboard.stripe.com/apikeys

#### Blockchain Configuration

```env
# Hyperledger Besu Private Key (untuk signing transactions)
BESU_PRIVATE_KEY=0x8f2a55949038a9610f50fb23b5883af3b4ecb3c3bb792cbcefbd1542c692be63

# Smart Contract Addresses (setelah deployment)
OWNERSHIP_CONTRACT_ADDRESS=0x1234567890abcdef1234567890abcdef12345678
AUDIT_CONTRACT_ADDRESS=0xabcdef1234567890abcdef1234567890abcdef12
```

**PENTING**: Jangan pernah commit private key ke repository!

#### Storage Configuration

```env
# IPFS untuk file storage
IPFS_API_URL=http://ipfs:5001

# AWS S3 untuk hosting static websites
HOSTING_S3_BUCKET=rinova-hosting
AWS_ACCESS_KEY_ID=your_access_key
AWS_SECRET_ACCESS_KEY=your_secret_key
AWS_REGION=us-east-1
```

#### Frontend Configuration

```env
# API endpoints
API_URL=http://localhost:8080
WS_URL=ws://localhost:8080

# CDN URL (optional)
CDN_URL=https://cdn.rinova.io

# Sentry for error tracking (optional)
SENTRY_DSN=https://xxx@sentry.io/xxx
```

---

## Menjalankan dengan Docker Compose

### Opsi 1: Menjalankan Semua Services (Full Stack)

```bash
# 1. Pastikan berada di root project
cd /path/to/rinova

# 2. Buat file .env
cp .env.example .env
nano .env  # Edit sesuai kebutuhan

# 3. Build dan jalankan semua services
docker compose --profile full up -d

# 4. Monitor logs
docker compose logs -f

# 5. Cek status services
docker compose ps
```

**Proses Build**:
- Build Rust services membutuhkan waktu 5-15 menit
- Ini hanya terjadi pada first-time build
- Subsequent builds akan menggunakan cache

**Output yang Diharapkan**:
```
NAME                    STATUS              PORTS
rinova-postgres         running (healthy)   0.0.0.0:5432->5432/tcp
rinova-mongo            running (healthy)   0.0.0.0:27017->27017/tcp
rinova-redis            running (healthy)   0.0.0.0:6379->6379/tcp
rinova-besu             running (healthy)   0.0.0.0:8545-8546->8545-8546/tcp
rinova-zmq-pub          running             0.0.0.0:5555-5556->5555-5556/tcp
rinova-gateway          running (healthy)   0.0.0.0:8080->8080/tcp
rinova-user-service     running (healthy)   0.0.0.0:8001->8001/tcp
rinova-subscription     running (healthy)   0.0.0.0:8002->8002/tcp
rinova-project-service  running (healthy)   0.0.0.0:8003->8003/tcp
rinova-publishing       running (healthy)   0.0.0.0:8004->8004/tcp
rinova-blockchain       running (healthy)   0.0.0.0:8005->8005/tcp
rinova-notification     running (healthy)   0.0.0.0:8006->8006/tcp
rinova-frontend         running (healthy)   0.0.0.0:3000->3000/tcp
```

### Opsi 2: Menjalankan Services Spesifik

#### Database Only
```bash
docker compose --profile database up -d
```

Services yang berjalan: PostgreSQL, MongoDB, Redis

#### Backend Only (dengan database)
```bash
docker compose --profile backend up -d
```

Services yang berjalan: Database + API Gateway + User Service + Subscription Service + Project Service + Publishing Service + Notification Service + ZeroMQ

#### Frontend Only
```bash
docker compose --profile frontend up -d
```

Services yang berjalan: Nuxt Frontend (membutuhkan backend berjalan)

#### Blockchain Only
```bash
docker compose --profile blockchain up -d
```

Services yang berjalan: Hyperledger Besu + Blockchain Service

### Mengelola Services

#### Melihat Logs

```bash
# Semua services
docker compose logs -f

# Service tertentu
docker compose logs -f user-service
docker compose logs -f api-gateway

# 100 baris terakhir
docker compose logs --tail=100 user-service
```

#### Restart Service

```bash
# Restart semua
docker compose restart

# Restart service tertentu
docker compose restart user-service
```

#### Stop Services

```bash
# Stop semua services
docker compose --profile full down

# Stop dan hapus volumes
docker compose --profile full down -v
```

#### Rebuild Service

```bash
# Rebuild semua services
docker compose --profile full build --no-cache

# Rebuild service tertentu
docker compose build --no-cache user-service
```

### Verifikasi Services Berjalan

#### Check Health Status

```bash
# API Gateway
curl http://localhost:8080/health

# User Service
curl http://localhost:8001/health

# Subscription Service
curl http://localhost:8002/health

# ... dst
```

#### Test Database Connections

```bash
# PostgreSQL
docker exec -it rinova-postgres psql -U rinova -d rinova -c "SELECT version();"

# MongoDB
docker exec -it rinova-mongo mongosh -u rinova -p rinova_secret --authenticationDatabase admin rinova --eval "db.version()"

# Redis
docker exec -it rinova-redis redis-cli -a rinova_redis_secret ping
```

#### Test Blockchain Connection

```bash
# Hyperledger Besu RPC
curl -X POST -H "Content-Type: application/json" \
  --data '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}' \
  http://localhost:8545
```

---

## Development Lokal

### Setup Development Environment

#### 1. Clone Repository

```bash
git clone https://github.com/yourusername/rinova.git
cd rinova
```

#### 2. Install Dependencies

```bash
# Frontend dependencies
cd frontend_nuxt
npm install

# Backend dependencies (Rust akan otomatis install saat cargo build)
cd ../backend_rust
cargo build
```

#### 3. Setup Databases

**Opsi A: Jalankan database via Docker**

```bash
# Jalankan hanya database services
docker compose --profile database up -d
```

**Opsi B: Install databases locally**

PostgreSQL:
```bash
# Ubuntu/Debian
sudo apt install postgresql postgresql-contrib

# macOS
brew install postgresql@15
brew services start postgresql@15

# Create database
createdb rinova
```

MongoDB:
```bash
# Ubuntu/Debian
sudo apt install mongodb

# macOS
brew install mongodb-community
brew services start mongodb-community
```

Redis:
```bash
# Ubuntu/Debian
sudo apt install redis-server

# macOS
brew install redis
brew services start redis
```

#### 4. Run Migrations

```bash
cd backend_rust/migrations
./run_migrations.sh
```

Atau manual:
```bash
# PostgreSQL migrations
psql -U rinova -d rinova -f 20250118_000001_create_users_table.sql
psql -U rinova -d rinova -f 20250118_000002_create_workspaces_table.sql
# ... dst
```

#### 5. Run Services Locally

**Terminal 1 - User Service:**
```bash
cd backend_rust
cargo run --bin user_service
```

**Terminal 2 - Subscription Service:**
```bash
cd backend_rust
cargo run --bin subscription_service
```

**Terminal 3 - API Gateway:**
```bash
cd backend_rust
cargo run --bin api_gateway
```

**Terminal 4 - Frontend:**
```bash
cd frontend_nuxt
npm run dev
```

### Hot Reload Development

#### Frontend (Nuxt)

Nuxt memiliki built-in hot reload:
```bash
cd frontend_nuxt
npm run dev
```

Perubahan pada file akan otomatis ter-refresh di browser.

#### Backend (Rust)

Gunakan `cargo-watch` untuk auto-reload:

```bash
# Install cargo-watch
cargo install cargo-watch

# Run dengan watch
cargo watch -x "run --bin user_service"
```

---

## Production Deployment

### Prasyarat Production

1. **Domain Name**: Contoh: `rinova.io`, `api.rinova.io`
2. **SSL Certificate**: Gunakan Let's Encrypt (gratis)
3. **CDN**: CloudFlare, AWS CloudFront, atau similar
4. **Monitoring**: Prometheus + Grafana
5. **Log Aggregation**: ELK Stack atau Loki
6. **Backup Strategy**: Database backups harian

### Deployment Steps

#### 1. Setup Server

```bash
# Update system
sudo apt update && sudo apt upgrade -y

# Install Docker
curl -fsSL https://get.docker.com -o get-docker.sh
sudo sh get-docker.sh

# Add user to docker group
sudo usermod -aG docker $USER

# Install Docker Compose
sudo curl -L "https://github.com/docker/compose/releases/latest/download/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
sudo chmod +x /usr/local/bin/docker-compose
```

#### 2. Setup SSL dengan Let's Encrypt

```bash
# Install certbot
sudo apt install certbot

# Generate certificate
sudo certbot certonly --standalone -d rinova.io -d api.rinova.io

# Certificates akan disimpan di:
# /etc/letsencrypt/live/rinova.io/fullchain.pem
# /etc/letsencrypt/live/rinova.io/privkey.pem
```

#### 3. Konfigurasi Production Environment

Buat `.env.production`:

```env
# Database (GANTI DENGAN PASSWORD KUAT!)
POSTGRES_USER=rinova_prod
POSTGRES_PASSWORD=<generate-strong-password>
POSTGRES_DB=rinova

MONGO_USER=rinova_prod
MONGO_PASSWORD=<generate-strong-password>
MONGO_DB=rinova

REDIS_PASSWORD=<generate-strong-password>

# JWT (MINIMAL 32 KARAKTER RANDOM)
JWT_SECRET=<generate-with-openssl-rand-base64-32>

# Production API URLs
API_URL=https://api.rinova.io
WS_URL=wss://api.rinova.io

# Stripe Production Keys
STRIPE_SECRET_KEY=sk_live_xxx
STRIPE_WEBHOOK_SECRET=whsec_xxx

# SMTP Production
SMTP_HOST=smtp.sendgrid.net
SMTP_PORT=587
SMTP_USER=apikey
SMTP_PASSWORD=your_sendgrid_api_key

# AWS S3 untuk Hosting
HOSTING_S3_BUCKET=rinova-prod-hosting
AWS_ACCESS_KEY_ID=AKIAxxx
AWS_SECRET_ACCESS_KEY=xxx
AWS_REGION=us-east-1

# Blockchain (JANGAN COMMIT KE REPO!)
BESU_PRIVATE_KEY=0x...
```

#### 4. Deploy dengan Docker Compose

```bash
# Pull latest code
git pull origin main

# Build dan deploy
docker compose -f docker-compose.prod.yml --profile full up -d --build

# Run migrations
docker exec -it rinova-postgres psql -U rinova_prod -d rinova -f /path/to/migrations

# Verify deployment
docker compose ps
curl https://api.rinova.io/health
```

#### 5. Setup Reverse Proxy (Nginx)

Buat `/etc/nginx/sites-available/rinova`:

```nginx
# Frontend
server {
    listen 80;
    server_name rinova.io www.rinova.io;
    return 301 https://$server_name$request_uri;
}

server {
    listen 443 ssl http2;
    server_name rinova.io www.rinova.io;

    ssl_certificate /etc/letsencrypt/live/rinova.io/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/rinova.io/privkey.pem;

    # Security headers
    add_header Strict-Transport-Security "max-age=31536000; includeSubDomains" always;
    add_header X-Frame-Options "SAMEORIGIN" always;
    add_header X-Content-Type-Options "nosniff" always;
    add_header X-XSS-Protection "1; mode=block" always;

    # Frontend
    location / {
        proxy_pass http://localhost:3000;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;
    }
}

# API Gateway
server {
    listen 80;
    server_name api.rinova.io;
    return 301 https://$server_name$request_uri;
}

server {
    listen 443 ssl http2;
    server_name api.rinova.io;

    ssl_certificate /etc/letsencrypt/live/rinova.io/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/rinova.io/privkey.pem;

    # Security headers
    add_header Strict-Transport-Security "max-age=31536000; includeSubDomains" always;
    add_header X-Frame-Options "SAMEORIGIN" always;
    add_header X-Content-Type-Options "nosniff" always;

    # Rate limiting
    limit_req_zone $binary_remote_addr zone=api_limit:10m rate=10r/s;

    location / {
        limit_req zone=api_limit burst=20 nodelay;
        
        proxy_pass http://localhost:8080;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_cache_bypass $http_upgrade;
    }
}
```

Enable site:
```bash
sudo ln -s /etc/nginx/sites-available/rinova /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl restart nginx
```

#### 6. Setup Systemd Services (Optional)

Buat `/etc/systemd/system/rinova.service`:

```ini
[Unit]
Description=Rinova Website Builder
Requires=docker.service
After=docker.service

[Service]
Type=oneshot
RemainAfterExit=yes
WorkingDirectory=/opt/rinova
ExecStart=/usr/local/bin/docker-compose -f docker-compose.prod.yml --profile full up -d
ExecStop=/usr/local/bin/docker-compose -f docker-compose.prod.yml --profile full down
TimeoutStartSec=0

[Install]
WantedBy=multi-user.target
```

Enable service:
```bash
sudo systemctl daemon-reload
sudo systemctl enable rinova
sudo systemctl start rinova
```

#### 7. Setup Backup

Buat script backup harian `/opt/rinova/backup.sh`:

```bash
#!/bin/bash
DATE=$(date +%Y%m%d_%H%M%S)
BACKUP_DIR="/opt/backups/rinova"

mkdir -p $BACKUP_DIR

# PostgreSQL backup
docker exec rinova-postgres pg_dump -U rinova_prod rinova > $BACKUP_DIR/postgres_$DATE.sql

# MongoDB backup
docker exec rinova-mongo mongodump --uri="mongodb://rinova_prod:password@localhost:27017/rinova?authSource=admin" --out=$BACKUP_DIR/mongo_$DATE

# Compress
tar -czf $BACKUP_DIR/backup_$DATE.tar.gz $BACKUP_DIR/*.sql $BACKUP_DIR/mongo_$DATE

# Remove old backups (keep 30 days)
find $BACKUP_DIR -name "backup_*.tar.gz" -mtime +30 -delete

# Upload to S3 (optional)
aws s3 cp $BACKUP_DIR/backup_$DATE.tar.gz s3://rinova-backups/
```

Cron job:
```bash
# Tambahkan ke crontab
crontab -e

# Backup setiap hari jam 2 pagi
0 2 * * * /opt/rinova/backup.sh >> /var/log/rinova-backup.log 2>&1
```

---

## Kubernetes Deployment (Advanced)

Untuk deployment skala besar, gunakan Kubernetes.

### Prasyarat

- Kubernetes cluster (EKS, GKE, AKS, atau self-managed)
- kubectl configured
- Helm 3.x

### Deploy dengan Helm

```bash
# Add Helm repo
helm repo add rinova https://helm.rinova.io

# Install
helm install rinova rinova/rinova \
  --namespace rinova \
  --create-namespace \
  --values values-prod.yaml
```

Konfigurasi `values-prod.yaml` tersedia di repository.

---

## Troubleshooting

### Container Tidak Bisa Start

```bash
# Check logs
docker compose logs user-service

# Check container status
docker compose ps

# Restart service
docker compose restart user-service

# Rebuild jika perlu
docker compose build --no-cache user-service
docker compose up -d user-service
```

### Database Connection Error

```bash
# Check database berjalan
docker compose ps postgresql mongodb redis

# Check database logs
docker compose logs postgresql

# Test connection
docker exec -it rinova-postgres psql -U rinova -d rinova -c "SELECT 1;"
```

### Permission Denied Errors

```bash
# Fix ownership
sudo chown -R $USER:$USER .

# Fix docker permissions
sudo usermod -aG docker $USER
# Logout dan login kembali
```

### Port Already in Use

```bash
# Cek apa yang menggunakan port
sudo lsof -i :8080

# Kill process
sudo kill -9 <PID>

# Atau ubah port di docker-compose.yml
```

### Out of Memory

```bash
# Check memory usage
docker stats

# Increase Docker memory limit
# Di Docker Desktop: Settings > Resources > Memory

# Atau set limit di docker-compose.yml
services:
  user-service:
    deploy:
      resources:
        limits:
          memory: 2G
```

### Blockchain Connection Failed

```bash
# Check Besu berjalan
docker compose logs besu

# Check Besu RPC
curl -X POST http://localhost:8545 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"net_version","params":[],"id":1}'

# Restart Besu
docker compose restart besu
```

---

## Monitoring & Logging

### Health Monitoring

```bash
# Monitor semua services
watch -n 5 'docker compose ps'

# Check health endpoints
for port in 8080 8001 8002 8003 8004 8005 8006; do
  echo "Port $port:"
  curl -s http://localhost:$port/health | jq .
done
```

### Log Management

```bash
# View logs dengan timestamp
docker compose logs -f --timestamps user-service

# Export logs ke file
docker compose logs > logs_$(date +%Y%m%d).txt

# Search logs
docker compose logs user-service | grep "ERROR"
```

### Performance Monitoring

```bash
# Container resource usage
docker stats

# Container disk usage
docker system df

# Clean up unused resources
docker system prune -a
```

### Prometheus & Grafana (Optional)

Deploy monitoring stack:

```bash
# Deploy Prometheus dan Grafana
docker compose -f docker-compose.monitoring.yml up -d

# Akses Grafana
# http://localhost:3001
# Default login: admin/admin
```

Dashboard templates tersedia di `/monitoring/grafana/dashboards/`.

---

## Maintenance

### Update Services

```bash
# Pull latest code
git pull origin main

# Rebuild dan restart
docker compose --profile full up -d --build

# Run migrations jika ada
docker exec -it rinova-postgres psql -U rinova -d rinova -f /migrations/new_migration.sql
```

### Scale Services

```bash
# Scale user service
docker compose up -d --scale user-service=3

# Load balancing akan otomatis via API Gateway
```

### Backup & Restore

```bash
# Backup database
docker exec rinova-postgres pg_dump -U rinova rinova > backup_$(date +%Y%m%d).sql

# Restore database
cat backup_20240101.sql | docker exec -i rinova-postgres psql -U rinova rinova
```

---

## Security Checklist

- [ ] Ganti semua default passwords di `.env`
- [ ] JWT_SECRET minimal 32 karakter random
- [ ] Enable HTTPS di production
- [ ] Setup firewall (hanya expose port 80, 443, 22)
- [ ] Enable rate limiting
- [ ] Setup fail2ban untuk SSH
- [ ] Regular security updates
- [ ] Backup encryption
- [ ] Rotate database passwords secara berkala
- [ ] Audit log aktif
- [ ] Remove unused Docker images

---

## Support

Jika mengalami masalah:

1. Check logs: `docker compose logs -f`
2. Check dokumentasi troubleshooting di atas
3. Buat issue di GitHub: [github.com/rinova/issues](https://github.com/rinova/issues)
4. Email: support@rinova.io
5. Discord: [discord.gg/rinova](https://discord.gg/rinova)

---

**Terakhir diperbarui**: 2024-01-01
