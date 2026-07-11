# Rinova Website Builder

Platform SaaS berbasis microservice untuk membuat website profesional dengan editor drag & drop berkinerja tinggi (200 FPS++).

## 🚀 Fitur Utama

- **Editor Drag & Drop**: Editor visual dengan performa tinggi (200 FPS++)
- **Multi-Tenant Workspace**: Dukungan workspace Personal dan Company dengan isolasi data
- **Blockchain Integration**: Proof of ownership menggunakan Hyperledger Besu
- **Asset Marketplace**: Marketplace untuk komponen premium
- **Subscription Management**: Sistem subscription dengan 3 tier (Freemium, Enterprise, Exclusive)
- **One-Click Publishing**: Deploy website ke hosting dengan satu klik
- **Analytics Dashboard**: Dashboard analytics untuk monitoring performa website

## 📋 Prasyarat

### Software yang Dibutuhkan
- Docker Engine 24.0+
- Docker Compose v2.0+
- Node.js 18+ (untuk development lokal)
- Rust 1.75+ (untuk development lokal)
- PostgreSQL 15+ (untuk development lokal)
- MongoDB 6.0+ (untuk development lokal)
- Redis 7+ (untuk development lokal)

### Hardware Minimum
- CPU: 4 cores
- RAM: 8GB
- Storage: 50GB SSD

### Hardware Rekomendasi (Production)
- CPU: 8+ cores
- RAM: 16GB+
- Storage: 100GB+ SSD

## 🏗️ Arsitektur Sistem

### Technology Stack

| Component | Technology | Version |
|-----------|------------|---------|
| Frontend | Nuxt.js + Vue.js | Latest |
| Backend Services | Rust | 1.75+ |
| API Gateway | Rust (Actix-web/Axum) | 1.75+ |
| Relational Database | PostgreSQL | 15+ |
| Document Database | MongoDB | 6.0+ |
| Cache | Redis | 7+ |
| Message Bus | ZeroMQ | 4.3+ |
| Blockchain | Hyperledger Besu | Latest |
| Smart Contracts | Solidity | 0.8+ |

### Microservices Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      Client Layer                            │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐     │
│  │   Browser    │  │    Mobile    │  │     API      │     │
│  │   (Nuxt)     │  │   (Browser)  │  │   Clients    │     │
│  └──────────────┘  └──────────────┘  └──────────────┘     │
└─────────────────────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────┐
│                    API Gateway Layer                         │
│  ┌──────────────────────────────────────────────────────┐  │
│  │         API Gateway (Rust - Port 8080)               │  │
│  │  • JWT Authentication                                 │  │
│  │  • Rate Limiting                                      │  │
│  │  • Request Routing                                    │  │
│  │  • Load Balancing                                     │  │
│  └──────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────┐
│                   Microservices Layer                        │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │
│  │User Service │  │Subscription │  │  Project    │        │
│  │  (Port 8001)│  │  Service    │  │  Service    │        │
│  │             │  │  (Port 8002)│  │  (Port 8003)│        │
│  └─────────────┘  └─────────────┘  └─────────────┘        │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │
│  │ Publishing  │  │ Blockchain  │  │Notification │        │
│  │  Service    │  │  Service    │  │  Service    │        │
│  │  (Port 8004)│  │  (Port 8005)│  │  (Port 8006)│        │
│  └─────────────┘  └─────────────┘  └─────────────┘        │
└─────────────────────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────┐
│                     Data Layer                               │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │
│  │ PostgreSQL  │  │   MongoDB   │  │    Redis    │        │
│  │  (Port 5432)│  │ (Port 27017)│  │  (Port 6379)│        │
│  └─────────────┘  └─────────────┘  └─────────────┘        │
└─────────────────────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────┐
│                  Message Bus Layer                           │
│  ┌──────────────────────────────────────────────────────┐  │
│  │         ZeroMQ Publisher (Ports 5555, 5556)          │  │
│  └──────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────┐
│                  Blockchain Layer                            │
│  ┌──────────────────────────────────────────────────────┐  │
│  │       Hyperledger Besu Network (IBFT 2.0)            │  │
│  │  • JSON-RPC: Port 8545                                │  │
│  │  • WebSocket: Port 8546                               │  │
│  │  • P2P: Port 30303                                    │  │
│  └──────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

## 🎯 Service Descriptions

### 1. API Gateway (Port 8080)
Gateway utama yang menangani:
- JWT Authentication & Authorization
- Rate Limiting (1000 requests/minute per user)
- Request Routing ke microservices
- Load Balancing

### 2. User Service (Port 8001)
Mengelola:
- User Registration & Authentication
- User Profile Management
- Email Verification
- Password Reset

### 3. Subscription Service (Port 8002)
Mengelola:
- Subscription Plans (Freemium, Enterprise, Exclusive)
- Payment Integration (Stripe)
- Feature Flagging
- Project Limits Enforcement

### 4. Project Service (Port 8003)
Mengelola:
- Project CRUD Operations
- Component Management
- Canvas State Serialization
- Real-time Collaboration

### 5. Publishing Service (Port 8004)
Mengelola:
- Project Validation
- Build & Deploy Pipeline
- Domain Management
- Static Site Generation

### 6. Blockchain Service (Port 8005)
Mengelola:
- Asset Ownership Recording (ERC-721)
- Audit Trail Recording
- Smart Contract Interaction
- Ownership Verification

### 7. Notification Service (Port 8006)
Mengelola:
- Real-time Notifications
- Email Notifications
- WebSocket Events
- Event Aggregation

## 🚀 Quick Start

### Opsi 1: Menjalankan Semua Services dengan Docker Compose

```bash
# Clone repository
git clone <repository-url>
cd rinova

# Copy environment file
cp .env.example .env

# Edit environment variables sesuai kebutuhan
nano .env

# Jalankan semua services
docker compose --profile full up -d

# Monitor logs
docker compose logs -f

# Cek status services
docker compose ps
```

### Opsi 2: Menjalankan Services Spesifik

```bash
# Jalankan hanya database services
docker compose --profile database up -d

# Jalankan hanya backend services (dengan database)
docker compose --profile backend up -d

# Jalankan hanya frontend
docker compose --profile frontend up -d

# Jalankan dengan blockchain
docker compose --profile blockchain up -d
```

### Akses Aplikasi

Setelah semua services berjalan:

- **Frontend**: http://localhost:3000
- **API Gateway**: http://localhost:8080
- **API Documentation**: http://localhost:8080/docs
- **PostgreSQL**: localhost:5432
- **MongoDB**: localhost:27017
- **Redis**: localhost:6379
- **Hyperledger Besu RPC**: http://localhost:8545

## 🔧 Konfigurasi Environment Variables

Buat file `.env` di root project dengan konfigurasi berikut:

```env
# Database Configuration
POSTGRES_USER=rinova
POSTGRES_PASSWORD=rinova_secret
POSTGRES_DB=rinova

MONGO_USER=rinova
MONGO_PASSWORD=rinova_secret
MONGO_DB=rinova

REDIS_PASSWORD=rinova_redis_secret

# JWT Configuration
JWT_SECRET=your_jwt_secret_key_change_in_production

# SMTP Configuration (untuk email notifications)
SMTP_HOST=smtp.gmail.com
SMTP_PORT=587
SMTP_USER=your_email@gmail.com
SMTP_PASSWORD=your_app_password

# Stripe Configuration (untuk pembayaran)
STRIPE_SECRET_KEY=sk_test_your_stripe_secret_key
STRIPE_WEBHOOK_SECRET=whsec_your_webhook_secret

# Blockchain Configuration
BESU_PRIVATE_KEY=0x8f2a55949038a9610f50fb23b5883af3b4ecb3c3bb792cbcefbd1542c692be63
OWNERSHIP_CONTRACT_ADDRESS=0x...
AUDIT_CONTRACT_ADDRESS=0x...

# IPFS Configuration (untuk file storage)
IPFS_API_URL=http://ipfs:5001

# AWS S3 Configuration (untuk hosting)
HOSTING_S3_BUCKET=your-bucket-name
AWS_ACCESS_KEY_ID=your_access_key
AWS_SECRET_ACCESS_KEY=your_secret_key
AWS_REGION=us-east-1

# Frontend Configuration
API_URL=http://localhost:8080
WS_URL=ws://localhost:8080
```

## 📊 Subscription Plans

### Freemium Plan
- 2 Projects Maximum
- Basic Components Only
- Community Support
- No Custom Domain

### Enterprise Plan ($29/month)
- 10 Projects Maximum
- Advanced Editor Features (Animation, Custom CSS, SEO Tools)
- Asset Marketplace Access
- Priority Support
- No Custom Domain

### Exclusive Plan ($99/month)
- Unlimited Projects
- All Enterprise Features
- One-Click Publishing
- Custom Domain Support
- Analytics Dashboard
- Premium Support

## 🔒 Security Features

- JWT-based Authentication dengan masa berlaku 7 hari
- Row-Level Security (RLS) untuk multi-tenancy
- Rate Limiting (1000 requests/minute per user)
- Account Lockout (5 failed attempts)
- HTTPS enforcement
- Secure HTTP-only cookies
- Input validation dan sanitization
- SQL injection prevention
- XSS protection

## 🧪 Testing

### Run All Tests
```bash
# Backend tests
cd backend_rust
cargo test --all

# Frontend tests
cd frontend_nuxt
npm run test

# E2E tests
npm run test:e2e
```

### Load Testing
```bash
# Install k6
brew install k6  # macOS
# atau
sudo apt install k6  # Ubuntu

# Run load tests
k6 run tests/load/scenarios.js
```

## 📈 Monitoring & Observability

### Health Checks
Setiap service menyediakan endpoint `/health` untuk health monitoring:

```bash
# Check all services health
curl http://localhost:8080/health
curl http://localhost:8001/health
curl http://localhost:8002/health
# ... dst
```

### Logs
```bash
# View all logs
docker compose logs -f

# View specific service logs
docker compose logs -f user-service
docker compose logs -f api-gateway
```

### Metrics
- Prometheus metrics tersedia di port 9545 (Besu)
- Custom metrics dapat diakses via API Gateway

## 🔧 Development

### Local Development Setup

#### Backend (Rust)
```bash
cd backend_rust

# Install dependencies
cargo build

# Run migrations
cd migrations
./run_migrations.sh

# Run a specific service
cargo run --bin user_service
cargo run --bin api_gateway
```

#### Frontend (Nuxt)
```bash
cd frontend_nuxt

# Install dependencies
npm install

# Run development server
npm run dev

# Build for production
npm run build
```

### Database Migrations
```bash
# PostgreSQL migrations
cd backend_rust/migrations
./run_migrations.sh

# MongoDB indexes
cd docker/mongodb_service
# Indexes dibuat otomatis saat inisialisasi
```

## 🐛 Troubleshooting

### Docker Issues

**Build taking too long?**
Rust compilation membutuhkan waktu 5-15 menit per service. Ini normal untuk first-time build.

**Container won't start?**
```bash
# Check logs
docker compose logs <service-name>

# Restart specific service
docker compose restart <service-name>

# Rebuild service
docker compose up -d --build <service-name>
```

**Database connection errors?**
```bash
# Check if database is running
docker compose ps

# Check database logs
docker compose logs postgresql
docker compose logs mongodb

# Restart databases
docker compose restart postgresql mongodb
```

### Performance Issues

**Editor lagging?**
- Check browser console untuk errors
- Clear browser cache
- Check network latency
- Monitor CPU usage

**API slow response?**
- Check Redis cache hit ratio
- Monitor database query performance
- Check service logs untuk errors
- Verify rate limiting configuration

## 📚 Documentation

- [API Documentation](./docs/API.md)
- [Deployment Guide](./docs/DEPLOYMENT.md)
- [Architecture Guide](./docs/ARCHITECTURE.md)
- [Contributing Guide](./CONTRIBUTING.md)

## 🤝 Contributing

Kami menyambut kontribusi dari komunitas! Silakan baca [Contributing Guide](./CONTRIBUTING.md) untuk panduan detail.

### Development Workflow
1. Fork repository
2. Buat feature branch (`git checkout -b feature/amazing-feature`)
3. Commit changes (`git commit -m 'Add amazing feature'`)
4. Push ke branch (`git push origin feature/amazing-feature`)
5. Buat Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Hyperledger Besu untuk blockchain infrastructure
- Rust community untuk performant tools
- Vue.js/Nuxt.js team untuk excellent frontend framework
- Stripe untuk payment processing

## 📞 Support

- **Documentation**: [docs.rinova.io](https://docs.rinova.io)
- **Issues**: [GitHub Issues](https://github.com/rinova/issues)
- **Email**: support@rinova.io
- **Discord**: [Rinova Community](https://discord.gg/rinova)

---

**Built with ❤️ by the Rinova Team**
