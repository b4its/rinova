# Rinova Website Builder

Platform SaaS berbasis microservice untuk membuat website profesional dengan editor drag & drop (200 FPS++), dilengkapi multi-tenant workspace, blockchain proof-of-ownership, marketplace komponen, dan publishing satu-klik.

---

## Daftar Isi

- [Arsitektur](#arsitektur)
- [Fitur Utama](#fitur-utama)
- [Technology Stack](#technology-stack)
- [Microservices](#microservices)
- [Docker Compose](#docker-compose)
- [Quick Start](#quick-start)
- [Environment Variables](#environment-variables)
- [Database Migrations](#database-migrations)
- [Subscription & Pricing](#subscription--pricing)
- [Admin User Management](#admin-user-management)
- [Marketplace](#marketplace)
- [Development Guide](#development-guide)
- [Troubleshooting](#troubleshooting)
- [Reset & Cleanup](#reset--cleanup)

---

## Arsitektur

```
Client (Browser)
    │
    ▼
┌─────────────────────────────────────────────────────┐
│                  API Gateway (:8080)                  │
│  (JWT Auth, Rate Limiting, Request Routing)           │
└──────┬──────┬──────┬──────┬──────┬──────┬──────┬────┘
       │      │      │      │      │      │      │
       ▼      ▼      ▼      ▼      ▼      ▼      ▼
   User   Worksp  Subs.  Proj.  Publ.  Block. Notif.
  (:8001) (:8007) (:8002)(:8003)(:8004)(:8005)(:8006)
       │      │      │      │      │      │      │
       └──────┴──────┴──────┼──────┼──────┼──────┘
                            │      │      │
                    ┌───────▼──────▼──────▼───────┐
                    │     ZeroMQ Message Bus       │
                    │  (PUB :5555 / SUB :5556)     │
                    └─────────────────────────────┘

Data Layer:
  ┌──────────┐  ┌──────────┐  ┌──────────┐
  │PostgreSQL│  │ MongoDB  │  │  Redis   │
  │  :5432   │  │ :27017   │  │  :6379   │
  └──────────┘  └──────────┘  └──────────┘

Blockchain Layer:
  ┌──────────────────────────────┐
  │  Hyperledger Besu (:8545)     │
  │  (IBFT 2.0, Chain ID 20182024)│
  └──────────────────────────────┘
```

---

## Fitur Utama

| Fitur | Deskripsi |
|-------|-----------|
| **Drag & Drop Editor** | Editor visual 200 FPS++ dengan komponen-grid dan real-time preview |
| **Multi-Tenant Workspace** | Personal & Company workspace dengan isolasi data lengkap |
| **Role-Based Access** | Superuser (admin panel) dan User (regular panel) |
| **Blockchain Proof-of-Ownership** | Setiap website terverifikasi di Hyperledger Besu |
| **Asset Marketplace** | Jual-beli template, komponen, dan asset premium |
| **Subscription Management** | 3 tier: Freemium, Enterprise ($29-99/bln), Exclusive ($79-199/bln) |
| **One-Click Publishing** | Deploy ke hosting S3-compatible atau IPFS |
| **Analytics Dashboard** | Real-time traffic, engagement, dan performa website |
| **Notification System** | Email (SMTP), WebSocket, dan in-app notification |

---

## Technology Stack

### Backend
| Komponen | Teknologi |
|----------|-----------|
| **Runtime** | Rust (actix-web 4.4) |
| **Database Utama** | PostgreSQL 15 (sqlx 0.7) |
| **Database Sekunder** | MongoDB 6.0 |
| **Cache** | Redis 7 (ioredis) |
| **Message Bus** | ZeroMQ (PUB/SUB) |
| **Blockchain** | Hyperledger Besu (IBFT 2.0) |
| **gRPC** | Tonic + Prost |
| **JWT** | jsonwebtoken (HS256, 7 hari) |
| **Payment** | Stripe (async-stripe) |
| **Email** | Lettre (SMTP) |

### Frontend
| Komponen | Teknologi |
|----------|-----------|
| **Framework** | Nuxt 4 + Vue 3.5 |
| **Styling** | Tailwind CSS (Nuxt module) |
| **State Management** | Pinia 3 |
| **UI Library** | reka-ui (Radix Vue) |
| **SSR** | Full SSR dengan Redis cache |

---

## Microservices

Ada **10 microservices** + **1 API Gateway**:

| Service | Port | Fungsi | Dependencies |
|---------|------|--------|-------------|
| **api-gateway** | `:8080` | JWT auth, rate limiting, routing ke semua service | postgresql, redis |
| **user-service** | `:8001` | Register/login, manajemen user, admin user CRUD, role management | postgresql, zmq |
| **workspace-service** | `:8007` | CRUD workspace, manajemen anggota, invitation workflow | postgresql, zmq |
| **subscription-service** | `:8002` | Subscription plans, workspace subscription, Stripe integration | postgresql, zmq |
| **project-service** | `:8003` | CRUD project, drag & drop data, autosave | postgresql, mongodb, redis, zmq |
| **publishing-service** | `:8004` | Build & deploy website ke hosting (S3/IPFS) | postgresql, mongodb, zmq |
| **blockchain-service** | `:8005` | Proof-of-ownership, audit trail via Besu smart contract | besu, zmq |
| **notification-service** | `:8006` | Email, WebSocket, in-app notification | postgresql, redis, zmq |
| **analytics-service** | — | Dashboard analytics (standalone Cargo.toml) | postgresql |
| **marketplace-service** | — | Marketplace API (standalone Cargo.toml) | postgresql, mongodb, stripe |

### Port Mapping Lengkap

| Service | Container Port | Host Port |
|---------|---------------|-----------|
| postgresql | 5432 | 5432 |
| mongodb | 27017 | 27017 |
| redis | 6379 | 6379 |
| besu (JSON-RPC) | 8545 | 8545 |
| besu (WebSocket) | 8546 | 8546 |
| besu (P2P) | 30303 | 30303 |
| besu (Metrics) | 9545 | 9545 |
| zmq-publisher (PUB) | 5555 | 5555 |
| zmq-publisher (SUB) | 5556 | 5556 |
| api-gateway | 8080 | 8080 |
| user-service | 8001 | 8001 |
| workspace-service | 8007 | 8007 |
| subscription-service | 8002 | 8002 |
| project-service | 8003 | 8003 |
| publishing-service | 8004 | 8004 |
| blockchain-service (HTTP) | 8005 | 8005 |
| blockchain-service (gRPC) | 9005 | 9005 |
| notification-service | 8006 | 8006 |
| frontend | 3000 | 3000 |

---

## Docker Compose

### Profiles

| Profile | Services |
|---------|----------|
| `full` | Semua service termasuk blockchain |
| `database` | postgresql, mongodb, redis |
| `backend` | database + zmq + api-gateway + semua Rust service (kecuali blockchain) |
| `frontend` | Nuxt frontend |
| `blockchain` | besu + blockchain-service |

### Service Dependencies Graph

```
frontend (3000)
  └── api-gateway (8080) ──┬── redis (6379)
                            └── postgresql (5432)

user-service (8001)        ──┬── postgresql
                              └── zmq (5555)

workspace-service (8007)   ──┬── postgresql
                              └── zmq

subscription-service (8002)──┬── postgresql
                              └── zmq

project-service (8003)      ──┬── postgresql
                              ├── mongodb (27017)
                              ├── redis
                              └── zmq

publishing-service (8004)   ──┬── postgresql
                              ├── mongodb
                              └── zmq

blockchain-service (8005)   ──┬── besu (8545)
                              └── zmq

notification-service (8006) ──┬── postgresql
                              ├── redis
                              └── zmq
```

### Build & Run

```bash
# Build dan start SEMUA service
docker-compose --profile full up -d --build

# Build dan start service tertentu
docker-compose up -d --build user-service api-gateway

# Start hanya database
docker-compose --profile database up -d

# Start backend (tanpa blockchain)
docker-compose --profile backend up -d

# Start frontend saja
docker-compose --profile frontend up -d

# Run di background
docker-compose up -d

# Lihat logs
docker-compose logs -f user-service
docker-compose logs -f api-gateway

# Stop service tertentu
docker-compose stop user-service

# Restart service
docker-compose restart user-service
```

### Build Individual Services

Setiap Rust service di-build dengan Dockerfile yang sama (`Dockerfile.service`) menggunakan arg `SERVICE_NAME`:

```bash
# Build satu service spesifik (rekomendasi untuk development)
docker-compose build user-service
docker-compose build api-gateway
docker-compose build subscription-service

# Build ulang + start ulang
docker-compose up -d --build user-service

# Build frontend
docker-compose build frontend
```

**Build context:** Seluruh folder `backend_rust/` (karena workspace Cargo). Hal ini menyebabkan build Rust memakan waktu **30+ menit** saat pertama kali karena harus mengunduh dan mengkompilasi semua dependencies.

> **Tip:** Rust Docker build bisa timeout. Jika terjadi, naikkan `COMPOSE_HTTP_TIMEOUT`:
> ```bash
> export COMPOSE_HTTP_TIMEOUT=600
> docker-compose build user-service
> ```

---

## Quick Start

### Persiapan Awal

```bash
# 1. Clone repository
git clone <repo-url>
cd rinova

# 2. Copy environment file
cp .env.example .env

# 3. Build dan start database services
docker-compose --profile database up -d

# 4. Tunggu hingga database siap (~30 detik)
docker-compose logs -f postgresql
# (Tunggu sampai muncul: "database system is ready to accept connections")

# 5. Build dan start backend services
docker-compose --profile backend up -d --build
```

### Verifikasi

```bash
# Cek status semua service
docker-compose ps

# Cek health API Gateway
curl http://localhost:8080/health

# Cek health user-service (via Gateway)
curl http://localhost:8001/health
```

### Akses Aplikasi

| Service | URL |
|---------|-----|
| **Frontend (Website)** | http://localhost:3000 |
| **Frontend (Panel)** | http://localhost:3000/panel |
| **API Gateway** | http://localhost:8080 |
| **API Docs** | http://localhost:8080/health |

### Akun Default (Seed Data)

Password semua akun: `Password123!`

| Email | Role | Plan |
|-------|------|------|
| `admin1@rinova.test` - `admin5@rinova.test` | Superuser | Enterprise |
| `exclusive1@rinova.test` - `exclusive5@rinova.test` | User | Exclusive |
| `enterprise1@rinova.test` - `enterprise5@rinova.test` | User | Enterprise |

---

## Environment Variables

### Root `.env`

| Variable | Default | Deskripsi |
|----------|---------|-----------|
| `POSTGRES_USER` | `rinova` | User PostgreSQL |
| `POSTGRES_PASSWORD` | `rinova_secret` | Password PostgreSQL |
| `DATABASE_URL` | `postgresql://...` | Connection string PostgreSQL |
| `MONGO_USER` | `rinova` | User MongoDB |
| `MONGO_PASSWORD` | `rinova_secret` | Password MongoDB |
| `MONGODB_URI` | `mongodb://...` | Connection string MongoDB |
| `REDIS_PASSWORD` | `rinova_redis_secret` | Password Redis |
| `REDIS_URL` | `redis://...` | Connection string Redis |
| `JWT_SECRET` | `rinova_jwt_secret_key_...` | Secret key untuk JWT token |
| `SMTP_HOST` | `smtp.hostinger.com` | SMTP server untuk email |
| `SMTP_PORT` | `465` | Port SMTP |
| `SMTP_USER` | `noreply@b4its.cloud` | Username SMTP |
| `SMTP_PASSWORD` | `...` | Password SMTP |
| `FRONTEND_URL` | `http://localhost:3000` | URL frontend |
| `API_URL` | `http://localhost:8080` | URL API Gateway |
| `WS_URL` | `ws://localhost:8080` | URL WebSocket |
| `STRIPE_SECRET_KEY` | `sk_test_...` | Stripe secret key |
| `STRIPE_WEBHOOK_SECRET` | `whsec_...` | Stripe webhook secret |
| `BESU_PRIVATE_KEY` | `0x8f2a...` | Private key Besu validator |
| `AWS_ACCESS_KEY_ID` | — | AWS S3 access key |
| `AWS_SECRET_ACCESS_KEY` | — | AWS S3 secret key |
| `HOSTING_S3_BUCKET` | — | S3 bucket untuk hosting |
| `IPFS_API_URL` | `http://ipfs:5001` | IPFS API endpoint |
| `SENTRY_DSN` | — | Sentry DSN untuk monitoring |
| `CDN_URL` | — | CDN URL |

### `backend_rust/.env` (Development Lokal)

```env
DATABASE_URL=postgresql://rinova:rinova_secret@localhost:5432/rinova
TEST_DATABASE_URL=postgresql://rinova:rinova_secret@localhost:5432/rinova_test
REDIS_URL=redis://:rinova_redis_secret@localhost:6379
JWT_SECRET=your-super-secret-jwt-key-change-in-production
ZMQ_PUBLISHER_URL=tcp://localhost:5555
SERVICE_PORT=8001
```

---

## Database Migrations

### Lokasi Migration

| Sumber | Lokasi | Format |
|--------|--------|--------|
| **sqlx (Rust)** | `backend_rust/libs/database/migrations/` | `YYYYMMDDHHMMSS_name.sql` |
| **sqlx (alternate)** | `backend_rust/migrations/` | `YYYYMMDD_name.sql` |
| **Docker init** | `docker/postgresql_service/migrations/` | `NNN_name.sql` |
| **Docker init utama** | `docker/postgresql_service/init.sql` | File tunggal 461 baris |

### Migration Files (sqlx)

| File | Isi |
|------|-----|
| `20240115000001_create_users_workspaces.sql` | Tables: `users`, `workspaces`, `workspace_members` (dengan UUID, RLS, triggers) |
| `20240115000002_create_subscriptions.sql` | Table: `subscriptions` (plan_type: freemium/enterprise/exclusive, status, stripe_subscription_id) |
| `20240115000003_create_projects.sql` | Tables: `projects`, `published_sites`, `audit_logs` (dengan RLS) |
| `20240115000004_add_workspace_to_subscriptions.sql` | Add: `workspace_id`, `stripe_customer_id` ke subscriptions |

### Menjalankan Migration

Migration otomatis dijalankan oleh `docker-entrypoint-initdb.d` saat container PostgreSQL pertama kali start.

Untuk migrasi manual setelah perubahan:

```bash
# Masuk ke container PostgreSQL
docker exec -it rinova-postgresql-1 psql -U rinova -d rinova

# Jalankan SQL migrasi
\i /docker-entrypoint-initdb.d/migrations/004_add_workspace_to_subscriptions.sql
```

### Migration Baru

```bash
# Buat file migrasi baru
touch backend_rust/libs/database/migrations/$(date -u +"%Y%m%d%H%M%S")_description.sql
```

---

## Subscription & Pricing

### Plan Tiers

| Plan | Personal (User) | Workspace (Company) |
|------|----------------|---------------------|
| **Freemium** | Gratis | Gratis |
| **Enterprise** | $29/bulan | $99/bulan |
| **Exclusive** | $79/bulan | $199/bulan |

### Plan Features

| Feature | Freemium | Enterprise | Exclusive |
|---------|----------|------------|-----------|
| Max Projects | 2 | 10 | Unlimited |
| Rate Limit (/min) | 100 | 1,000 | 5,000 |
| Custom Domain | — | ✅ | ✅ |
| Team Members | — | Up to 10 | Unlimited |
| Blockchain Proof | — | — | ✅ |
| Priority Support | — | — | ✅ |

### Effective Plan Resolution

Saat login, sistem menghitung **effective plan** dengan mengambil plan tertinggi antara:
1. **Personal subscription** (workspace_id IS NULL)
2. **Workspace subscriptions** (dari semua workspace tempat user menjadi member)

Prioritas: `exclusive > enterprise > freemium`

### API Endpoints Subscription

| Endpoint | Method | Deskripsi |
|----------|--------|-----------|
| `/api/v1/subscriptions/plans/personal` | GET | List personal plans |
| `/api/v1/subscriptions/plans/workspace` | GET | List workspace plans |
| `/api/v1/subscriptions/freemium` | POST | Buat personal freemium |
| `/api/v1/subscriptions/workspace/freemium` | POST | Buat workspace freemium |
| `/api/v1/subscriptions/{user_id}/effective-plan` | GET | Dapatkan effective plan user |

---

## Admin User Management

### Role System

Setiap user memiliki role: `'user'` (default) atau `'superuser'`.

- **Superuser** — Melihat admin sidebar dengan akses ke: Users, Products, Components, Kategori, Templates, Marketplace
- **User** — Hanya melihat panel regular (Dashboard, Projects, Analytics, Settings)

### Admin API Endpoints

| Endpoint | Method | Auth | Deskripsi |
|----------|--------|------|-----------|
| `/api/v1/admin/users` | GET | Superuser | List semua users |
| `/api/v1/admin/users/{id}/role` | PUT | Superuser | Update role user |
| `/api/v1/admin/users/{id}` | DELETE | Superuser | Hapus user |

### Frontend Role Resolution

Role ditentukan dengan prioritas:
1. **Backend response** — field `role` dari API login (`/auth/login`) atau `/users/me`
2. **localStorage fallback** — key `admin_users`, cocokkan email user
3. **Default** — `'user'`

> **Pertama login:** Jika localStorage `admin_users` kosong, user pertama otomatis mendapat role `'superuser'` untuk memudahkan setup awal. Role bisa diubah dari halaman Users di admin panel.

---

## Marketplace

### Data Flow

Marketplace admin menggunakan **localStorage** sebagai penyimpanan (bukan database) dengan key:
- `admin_marketplace_kategori` — Array kategori
- `admin_marketplace_produk` — Array produk

### Halaman Marketplace

| Halaman | Path | Fungsi |
|---------|------|--------|
| **Kategori** | `/panel/admin/marketplace/kategori` | CRUD + pagination untuk kategori marketplace |
| **Produk List** | `/panel/admin/marketplace/produk` | List produk + CRUD dialog + pagination |
| **Add Produk** | `/panel/admin/marketplace/produk/add` | Add produk dengan HTML template preview (iframe) |

### Template Preview

Untuk produk dengan kategori "Templates", form Add Produk menampilkan:
- **Textarea HTML** — dengan fitur Ctrl+F (find in text)
- **Live Preview** — iframe dengan sandbox, 3 viewport mode (desktop/tablet/mobile)
- **Full Preview** — overlay mode layar penuh

---

## Development Guide

### Prasyarat Development Lokal

```bash
# Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable

# Node.js 22+ (via nvm)
nvm install 22
nvm use 22

# pnpm
corepack enable
corepack prepare pnpm@latest --activate

# Database services (via Docker)
docker-compose --profile database up -d
```

### Setup Development

```bash
# 1. Setup backend env
cp backend_rust/.env.example backend_rust/.env

# 2. Setup frontend env
# (gunakan default dari .env root)

# 3. Jalankan database
docker-compose --profile database up -d

# 4. Jalankan ZeroMQ (untuk development lokal)
docker-compose up -d zmq-publisher

# 5. Jalankan frontend (HMR)
cd frontend_nuxt
pnpm install
pnpm dev
# → http://localhost:3000

# 6. Build & jalankan service tertentu (Docker)
docker-compose up -d --build user-service api-gateway
```

### Struktur Frontend

```
frontend_nuxt/
├── app/
│   ├── app.vue                 # Root component
│   ├── components/             # Vue components
│   ├── composables/            # Vue composables (useAuth, useApi, dll)
│   ├── layouts/                # Layouts (default, panel)
│   ├── lib/                    # Library code
│   ├── middleware/              # Route middleware (auth guard)
│   ├── pages/                  # Pages
│   │   ├── index.vue           # Landing page
│   │   ├── login.vue           # Login
│   │   ├── register.vue        # Register
│   │   └── panel/              # Panel (authenticated)
│   │       ├── admin/          # Admin section (superuser only)
│   │       │   ├── index.vue
│   │       │   ├── users.vue
│   │       │   ├── marketplace/
│   │       │   └── templates/
│   │       └── ...
│   ├── plugins/                # Client plugins
│   └── stores/                 # Pinia stores (user, workspace, subscription)
├── server/
│   ├── api/                    # Server API routes
│   ├── plugins/                # Server plugins
│   └── utils/                  # Server utilities (Redis)
└── nuxt.config.ts
```

### Struktur Backend

```
backend_rust/
├── Cargo.toml                  # Workspace root (12 members)
├── Dockerfile.service           # Multi-stage build untuk Rust services
├── Dockerfile.gateway           # Multi-stage build untuk API Gateway
├── libs/
│   ├── shared/                  # Shared types (User, Claims, Subscription, dll)
│   ├── database/                # Database connection pools, migrations, Redis client
│   ├── messaging/               # ZeroMQ publisher/subscriber
│   └── proto/                   # gRPC protobuf definitions
├── services/
│   ├── api_gateway/             # JWT auth, rate limiting, proxy routing
│   ├── user_service/            # User auth, CRUD, admin user management
│   ├── workspace_service/       # Workspace CRUD, member invitation
│   ├── subscription_service/    # Subscription plans, Stripe, workspace subscription
│   ├── project_service/         # Project CRUD, drag & drop data
│   ├── publishing_service/      # Build & deploy website
│   ├── blockchain_service/      # Besu integration, proof-of-ownership
│   ├── notification_service/    # Email, WebSocket, in-app notification
│   ├── analytics_service/       # Dashboard analytics (standalone)
│   └── marketplace_service/     # Marketplace API (standalone)
├── migrations/                  # Additional SQL migrations
└── tests/                       # Integration tests
```

---

## Troubleshooting

### Docker Build Issues

#### Rust build timeout (30+ menit)

```bash
# Naikkan timeout
export COMPOSE_HTTP_TIMEOUT=600
docker compose build user-service

# Atau build langsung dengan Docker
cd backend_rust
docker build -f Dockerfile.service --build-arg SERVICE_NAME=user_service -t rinova-user-service .
```

#### Docker memory insufficient

Pastikan Docker memiliki resource cukup:
- **CPU:** Minimal 4 cores
- **RAM:** Minimal 8GB (recommended 16GB)
- **Swap:** 4GB

Di Docker Desktop → Settings → Resources → naikkan RAM & Swap.

#### Build gagal di alpine karena `libzmq`

```bash
# Pastikan alpine package tersedia
apk add zeromq-dev
```

Atau gunakan `Dockerfile.service` yang sudah include `libzmq`.

### Database Issues

#### PostgreSQL connection refused

```bash
# Cek status container
docker-compose ps postgresql

# Cek logs
docker-compose logs postgresql

# Pastikan DATABASE_URL benar
docker exec rinova-postgresql-1 psql -U rinova -d rinova -c "SELECT 1"
```

#### Migration gagal

```bash
# Jalankan migration manual
docker exec -it rinova-postgresql-1 psql -U rinova -d rinova

# Cek status migrasi
SELECT * FROM _sqlx_migrations;
```

### Service Not Starting

#### Health check gagal

```bash
# Cek langsung health endpoint dari dalam container
docker exec rinova-user-service-1 wget -qO- http://localhost:8001/health

# Cek logs
docker-compose logs user-service
```

#### Container restart loop

```bash
# Stop semua
docker-compose down

# Hapus volume database (HATI-HATI: data hilang!)
docker-compose down -v

# Start ulang dari awal
docker-compose --profile backend up -d --build
```

### Frontend Issues

#### Nuxt dev server error

```bash
cd frontend_nuxt
rm -rf .nuxt node_modules
pnpm install
pnpm dev
```

#### API connection refused (frontend can't reach backend)

Pastikan `NUXT_PUBLIC_API_URL` di `.env` mengarah ke `http://localhost:8080` (API Gateway).

```bash
# Test dari browser
curl http://localhost:8080/health
```

### Redis Issues

Redis di project ini hanya digunakan untuk:
1. **Landing page cache** (frontend Nuxt) — cache konten halaman landing
2. **Scaffolding rate limiter** — belum diimplementasikan

**Redis TIDAK menyimpan data user, session, atau role.**

#### Clear Redis cache

```bash
# Via Docker
docker exec rinova-redis-1 redis-cli -a rinova_redis_secret FLUSHALL

# Atau restart container
docker-compose restart redis
```

#### Verifikasi Redis

```bash
docker exec rinova-redis-1 redis-cli -a rinova_redis_secret PING
# → PONG
```

### Admin Panel Not Visible

Jika admin sidebar tidak muncul setelah login:

1. **Cek role user** — Buka console browser (F12):
   ```javascript
   // Cek role dari store
   JSON.parse(localStorage.getItem('admin_users'))
   ```

2. **Pertama login** — Seharusnya otomatis superuser. Jika tidak, hapus localStorage:
   ```javascript
   localStorage.removeItem('admin_users')
   // Kemudian refresh halaman
   ```

3. **Role dari backend** — Pastikan Docker container sudah di-rebuild:
   ```bash
   docker-compose up -d --build user-service api-gateway
   ```

4. **Fallback localStorage** — Edit langsung di console:
   ```javascript
   const users = JSON.parse(localStorage.getItem('admin_users'))
   const myUser = users.find(u => u.email === 'email_anda')
   myUser.role = 'superuser'
   localStorage.setItem('admin_users', JSON.stringify(users))
   ```

### Seed Data

```bash
# Jalankan seed data dari dalam container
docker exec -it rinova-postgresql-1 psql -U rinova -d rinova

# Cek apakah user sudah ada
SELECT email, role FROM users;
```

Jika seed data belum ada, jalankan migration yang berisi seed.

---

## Reset & Cleanup

```bash
# Stop semua container
docker-compose down

# Stop + hapus volume (semua data hilang!)
docker-compose down -v

# Hapus semua container, network, image (hati-hati!)
docker system prune -a --volumes

# Reset spesifik service
docker-compose stop user-service
docker-compose rm -f user-service
docker-compose up -d --build user-service
```

### Reset Database

```bash
# Hapus volume PostgreSQL saja
docker-compose stop postgresql
docker-compose rm -f postgresql
docker volume rm rinova_postgres_data
docker-compose up -d postgresql

# Atau via psql (DROP + CREATE)
docker exec -it rinova-postgresql-1 psql -U rinova -d postgres
DROP DATABASE rinova;
CREATE DATABASE rinova;
\q

# Migration akan otomatis jalan saat container restart
docker-compose restart postgresql
```

---

## License

MIT License — Lihat [LICENSE](LICENSE) untuk detail.

---



*Rinova — Professional Website Builder. Dibangun dengan Rust + Nuxt 4*
