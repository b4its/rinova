# API Documentation - Rinova Website Builder

## Overview

Dokumentasi ini menjelaskan semua API endpoints yang tersedia di Rinova Website Builder. Semua request memerlukan authentication kecuali endpoints yang secara eksplisit ditandai sebagai public.

### Base URL

```
Production: https://api.rinova.io
Development: http://localhost:8080
```

### Authentication

Semua request (kecuali public endpoints) memerlukan JWT token yang dikirim via HTTP-only cookie atau Authorization header:

```
Authorization: Bearer <jwt_token>
```

JWT token diperoleh melalui login endpoint dan memiliki masa berlaku 7 hari.

### Rate Limiting

- **Default**: 1000 requests per minute per user
- **Headers**: 
  - `X-RateLimit-Limit`: Maximum requests per window
  - `X-RateLimit-Remaining`: Remaining requests in current window
  - `X-RateLimit-Reset`: Timestamp when rate limit resets

### Response Format

Semua response menggunakan format JSON:

```json
{
  "success": true|false,
  "data": { ... } | null,
  "error": {
    "code": "ERROR_CODE",
    "message": "Human readable message",
    "details": { ... }
  } | null
}
```

---

## 1. Authentication Endpoints

### 1.1 Register User

Mendaftarkan user baru.

**Endpoint**: `POST /api/v1/auth/register`

**Access**: Public

**Request Body**:
```json
{
  "email": "user@example.com",
  "password": "SecurePass123",
  "account_type": "personal" | "company",
  "company_name": "Company Name (optional, required if account_type is company)"
}
```

**Validation Rules**:
- `email`: Valid email format (RFC 5322)
- `password`: Minimum 8 characters, must contain uppercase, lowercase, and numbers
- `account_type`: Must be "personal" or "company"

**Response (201 Created)**:
```json
{
  "success": true,
  "data": {
    "user_id": "uuid",
    "email": "user@example.com",
    "account_type": "personal",
    "verification_required": true,
    "message": "Verification email sent"
  }
}
```

**Error Responses**:
- `400 Bad Request`: Invalid input data
- `409 Conflict`: Email already registered (generic message to prevent enumeration)

---

### 1.2 Verify Email

Memverifikasi email user setelah registrasi.

**Endpoint**: `POST /api/v1/auth/verify-email`

**Access**: Public

**Request Body**:
```json
{
  "token": "verification_token_from_email"
}
```

**Response (200 OK)**:
```json
{
  "success": true,
  "data": {
    "user_id": "uuid",
    "email": "user@example.com",
    "account_verified": true,
    "jwt_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "expires_at": "2024-01-20T00:00:00Z"
  }
}
```

**Error Responses**:
- `400 Bad Request`: Invalid or expired token
- `404 Not Found`: Token not found

---

### 1.3 Login

Login user dan mendapatkan JWT token.

**Endpoint**: `POST /api/v1/auth/login`

**Access**: Public

**Request Body**:
```json
{
  "email": "user@example.com",
  "password": "SecurePass123"
}
```

**Response (200 OK)**:
```json
{
  "success": true,
  "data": {
    "user_id": "uuid",
    "email": "user@example.com",
    "account_type": "personal",
    "jwt_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "expires_at": "2024-01-20T00:00:00Z",
    "workspaces": [
      {
        "workspace_id": "uuid",
        "name": "Personal Workspace",
        "role": "owner",
        "type": "personal"
      }
    ]
  }
}
```

**Error Responses**:
- `401 Unauthorized`: Invalid credentials
- `403 Forbidden`: Account not verified
- `423 Locked`: Account locked (too many failed attempts)

---

### 1.4 Logout

Logout user dan invalidate JWT token.

**Endpoint**: `POST /api/v1/auth/logout`

**Access**: Authenticated

**Response (200 OK)**:
```json
{
  "success": true,
  "data": {
    "message": "Logged out successfully"
  }
}
```

---

### 1.5 Forgot Password

Request password reset email.

**Endpoint**: `POST /api/v1/auth/forgot-password`

**Access**: Public

**Request Body**:
```json
{
  "email": "user@example.com"
}
```

**Response (200 OK)**:
```json
{
  "success": true,
  "data": {
    "message": "Password reset email sent if account exists"
  }
}
```

---

### 1.6 Reset Password

Reset password using token from email.

**Endpoint**: `POST /api/v1/auth/reset-password`

**Access**: Public

**Request Body**:
```json
{
  "token": "reset_token_from_email",
  "new_password": "NewSecurePass123"
}
```

**Response (200 OK)**:
```json
{
  "success": true,
  "data": {
    "message": "Password reset successfully"
  }
}
```

---

## 2. User Endpoints

### 2.1 Get Current User

Mendapatkan informasi user yang sedang login.

**Endpoint**: `GET /api/v1/users/me`

**Access**: Authenticated

**Response (200 OK)**:
```json
{
  "success": true,
  "data": {
    "user_id": "uuid",
    "email": "user@example.com",
    "full_name": "John Doe",
    "avatar_url": "https://...",
    "account_type": "personal",
    "email_verified": true,
    "created_at": "2024-01-01T00:00:00Z",
    "updated_at": "2024-01-01T00:00:00Z"
  }
}
```

---

### 2.2 Update User Profile

Update profil user.

**Endpoint**: `PATCH /api/v1/users/me`

**Access**: Authenticated

**Request Body**:
```json
{
  "full_name": "John Doe",
  "avatar_url": "https://..."
}
```

**Response (200 OK)**:
```json
{
  "success": true,
  "data": {
    "user_id": "uuid",
    "full_name": "John Doe",
    "avatar_url": "https://...",
    "updated_at": "2024-01-01T00:00:00Z"
  }
}
```

---

### 2.3 Change Password

Mengubah password user.

**Endpoint**: `POST /api/v1/users/me/change-password`

**Access**: Authenticated

**Request Body**:
```json
{
  "current_password": "CurrentPass123",
  "new_password": "NewSecurePass123"
}
```

**Response (200 OK)**:
```json
{
  "success": true,
  "data": {
    "message": "Password changed successfully"
  }
}
```

**Error Responses**:
- `400 Bad Request`: Invalid current password
- `400 Bad Request`: New password doesn't meet requirements

---

## 3. Workspace Endpoints

### 3.1 List Workspaces

Mendapatkan semua workspace yang dapat diakses user.

**Endpoint**: `GET /api/v1/workspaces`

**Access**: Authenticated

**Response (200 OK)**:
```json
{
  "success": true,
  "data": {
    "workspaces": [
      {
        "workspace_id": "uuid",
        "name": "Personal Workspace",
        "type": "personal",
        "role": "owner",
        "members_count": 1,
        "projects_count": 5,
        "created_at": "2024-01-01T00:00:00Z"
      },
      {
        "workspace_id": "uuid",
        "name": "Company Workspace",
        "type": "company",
        "role": "admin",
        "members_count": 10,
        "projects_count": 20,
        "created_at": "2024-01-01T00:00:00Z"
      }
    ],
    "total": 2
  }
}
```

---

### 3.2 Create Workspace

Membuat workspace baru (Company/Organization).

**Endpoint**: `POST /api/v1/workspaces`

**Access**: Authenticated

**Request Body**:
```json
{
  "name": "My Company",
  "type": "company"
}
```

**Response (201 Created)**:
```json
{
  "success": true,
  "data": {
    "workspace_id": "uuid",
    "name": "My Company",
    "type": "company",
    "role": "owner",
    "created_at": "2024-01-01T00:00:00Z"
  }
}
```

---

### 3.3 Get Workspace

Mendapatkan detail workspace.

**Endpoint**: `GET /api/v1/workspaces/{workspace_id}`

**Access**: Authenticated (workspace member)

**Response (200 OK)**:
```json
{
  "success": true,
  "data": {
    "workspace_id": "uuid",
    "name": "Company Workspace",
    "type": "company",
    "role": "admin",
    "members": [
      {
        "user_id": "uuid",
        "email": "user@example.com",
        "full_name": "John Doe",
        "role": "owner",
        "joined_at": "2024-01-01T00:00:00Z"
      }
    ],
    "created_at": "2024-01-01T00:00:00Z"
  }
}
```

---

### 3.4 Update Workspace

Update workspace information.

**Endpoint**: `PATCH /api/v1/workspaces/{workspace_id}`

**Access**: Authenticated (workspace owner or admin)

**Request Body**:
```json
{
  "name": "Updated Company Name"
}
```

**Response (200 OK)**:
```json
{
  "success": true,
  "data": {
    "workspace_id": "uuid",
    "name": "Updated Company Name",
    "updated_at": "2024-01-01T00:00:00Z"
  }
}
```

---

### 3.5 Invite User to Workspace

Mengundang user ke workspace.

**Endpoint**: `POST /api/v1/workspaces/{workspace_id}/invitations`

**Access**: Authenticated (workspace owner or admin)

**Request Body**:
```json
{
  "email": "newuser@example.com",
  "role": "member"
}
```

**Role Options**:
- `member`: Can edit projects
- `admin`: Can manage members and projects
- `owner`: Full access (cannot be assigned via invitation)

**Response (201 Created)**:
```json
{
  "success": true,
  "data": {
    "invitation_id": "uuid",
    "email": "newuser@example.com",
    "role": "member",
    "expires_at": "2024-01-08T00:00:00Z",
    "message": "Invitation sent successfully"
  }
}
```

---

### 3.6 Accept Workspace Invitation

Menerima undangan workspace.

**Endpoint**: `POST /api/v1/workspaces/invitations/{invitation_id}/accept`

**Access**: Authenticated

**Response (200 OK)**:
```json
{
  "success": true,
  "data": {
    "workspace_id": "uuid",
    "workspace_name": "Company Workspace",
    "role": "member",
    "message": "Successfully joined workspace"
  }
}
```

---

### 3.7 Decline Workspace Invitation

Menolak undangan workspace.

**Endpoint**: `POST /api/v1/workspaces/invitations/{invitation_id}/decline`

**Access**: Authenticated

**Response (200 OK)**:
```json
{
  "success": true,
  "data": {
    "message": "Invitation declined"
  }
}
```

---

## 4. Subscription Endpoints

### 4.1 List Subscription Plans

Mendapatkan daftar semua subscription plans.

**Endpoint**: `GET /api/v1/subscriptions/plans`

**Access**: Public

**Response (200 OK)**:
```json
{
  "success": true,
  "data": {
    "plans": [
      {
        "plan_id": "freemium",
        "name": "Freemium",
        "price": 0,
        "currency": "USD",
        "billing_period": "monthly",
        "features": {
          "max_projects": 2,
          "custom_domain": false,
          "analytics": false,
          "animation_editor": false,
          "custom_css": false,
          "seo_tools": false,
          "asset_marketplace_access": false,
          "one_click_publish": false,
          "priority_support": false
        },
        "description": "Perfect for trying out the platform"
      },
      {
        "plan_id": "enterprise",
        "name": "Enterprise",
        "price": 29,
        "currency": "USD",
        "billing_period": "monthly",
        "features": {
          "max_projects": 10,
          "custom_domain": false,
          "analytics": false,
          "animation_editor": true,
          "custom_css": true,
          "seo_tools": true,
          "asset_marketplace_access": true,
          "one_click_publish": false,
          "priority_support": true
        },
        "description": "For professional designers and small teams"
      },
      {
        "plan_id": "exclusive",
        "name": "Exclusive",
        "price": 99,
        "currency": "USD",
        "billing_period": "monthly",
        "features": {
          "max_projects": -1,
          "custom_domain": true,
          "analytics": true,
          "animation_editor": true,
          "custom_css": true,
          "seo_tools": true,
          "asset_marketplace_access": true,
          "one_click_publish": true,
          "priority_support": true
        },
        "description": "For agencies and power users"
      }
    ]
  }
}
```

---

### 4.2 Get Current Subscription

Mendapatkan subscription saat ini.

**Endpoint**: `GET /api/v1/subscriptions/current`

**Access**: Authenticated

**Response (200 OK)**:
```json
{
  "success": true,
  "data": {
    "subscription_id": "uuid",
    "plan_id": "enterprise",
    "plan_name": "Enterprise",
    "status": "active",
    "current_period_start": "2024-01-01T00:00:00Z",
    "current_period_end": "2024-02-01T00:00:00Z",
    "cancel_at_period_end": false,
    "features": {
      "max_projects": 10,
      "animation_editor": true,
      "custom_css": true,
      "seo_tools": true,
      "asset_marketplace_access": true,
      "priority_support": true
    }
  }
}
```

---

### 4.3 Create Checkout Session

Membuat Stripe checkout session untuk subscription.

**Endpoint**: `POST /api/v1/subscriptions/checkout`

**Access**: Authenticated

**Request Body**:
```json
{
  "plan_id": "enterprise",
  "success_url": "https://app.rinova.io/success",
  "cancel_url": "https://app.rinova.io/cancel"
}
```

**Response (200 OK)**:
```json
{
  "success": true,
  "data": {
    "checkout_url": "https://checkout.stripe.com/...",
    "session_id": "cs_test_..."
  }
}
```

---

### 4.4 Cancel Subscription

Membatalkan subscription.

**Endpoint**: `POST /api/v1/subscriptions/cancel`

**Access**: Authenticated

**Response (200 OK)**:
```json
{
  "success": true,
  "data": {
    "subscription_id": "uuid",
    "status": "active",
    "cancel_at_period_end": true,
    "current_period_end": "2024-02-01T00:00:00Z",
    "message": "Subscription will be cancelled at the end of the current period"
  }
}
```

---

### 4.5 Reactivate Subscription

Mengaktifkan kembali subscription yang dibatalkan.

**Endpoint**: `POST /api/v1/subscriptions/reactivate`

**Access**: Authenticated

**Response (200 OK)**:
```json
{
  "success": true,
  "data": {
    "subscription_id": "uuid",
    "status": "active",
    "cancel_at_period_end": false,
    "message": "Subscription reactivated successfully"
  }
}
```

---

### 4.6 Check Feature Access

Mengecek apakah user memiliki akses ke fitur tertentu.

**Endpoint**: `GET /api/v1/subscriptions/features/{feature_name}`

**Access**: Authenticated

**Feature Names**:
- `animation_editor`
- `custom_css`
- `seo_tools`
- `asset_marketplace`
- `one_click_publish`
- `analytics`
- `custom_domain`

**Response (200 OK)**:
```json
{
  "success": true,
  "data": {
    "feature": "animation_editor",
    "has_access": true,
    "plan_required": "enterprise",
    "current_plan": "enterprise"
  }
}
```

---

## 5. Project Endpoints

### 5.1 List Projects

Mendapatkan semua project dalam workspace.

**Endpoint**: `GET /api/v1/workspaces/{workspace_id}/projects`

**Access**: Authenticated (workspace member)

**Query Parameters**:
- `page` (optional): Page number (default: 1)
- `limit` (optional): Items per page (default: 20, max: 100)
- `status` (optional): Filter by status (draft, published, archived)

**Response (200 OK)**:
```json
{
  "success": true,
  "data": {
    "projects": [
      {
        "project_id": "uuid",
        "name": "My Website",
        "description": "Portfolio website",
        "status": "draft",
        "created_at": "2024-01-01T00:00:00Z",
        "updated_at": "2024-01-01T00:00:00Z",
        "published_url": null
      }
    ],
    "pagination": {
      "page": 1,
      "limit": 20,
      "total": 5,
      "total_pages": 1
    }
  }
}
```

---

### 5.2 Create Project

Membuat project baru.

**Endpoint**: `POST /api/v1/workspaces/{workspace_id}/projects`

**Access**: Authenticated (workspace member)

**Request Body**:
```json
{
  "name": "My New Website",
  "description": "E-commerce website"
}
```

**Response (201 Created)**:
```json
{
  "success": true,
  "data": {
    "project_id": "uuid",
    "name": "My New Website",
    "description": "E-commerce website",
    "status": "draft",
    "created_at": "2024-01-01T00:00:00Z"
  }
}
```

**Error Responses**:
- `403 Forbidden`: Project limit reached for current plan
- `403 Forbidden`: Feature not available in current plan

---

### 5.3 Get Project

Mendapatkan detail project.

**Endpoint**: `GET /api/v1/projects/{project_id}`

**Access**: Authenticated (project owner or workspace member)

**Response (200 OK)**:
```json
{
  "success": true,
  "data": {
    "project_id": "uuid",
    "name": "My Website",
    "description": "Portfolio website",
    "status": "draft",
    "workspace_id": "uuid",
    "created_at": "2024-01-01T00:00:00Z",
    "updated_at": "2024-01-01T00:00:00Z",
    "canvas_state": {
      "components": [...],
      "version": "1.0"
    },
    "published_url": null
  }
}
```

---

### 5.4 Update Project

Update project metadata.

**Endpoint**: `PATCH /api/v1/projects/{project_id}`

**Access**: Authenticated (project owner or workspace member with edit access)

**Request Body**:
```json
{
  "name": "Updated Website Name",
  "description": "Updated description"
}
```

**Response (200 OK)**:
```json
{
  "success": true,
  "data": {
    "project_id": "uuid",
    "name": "Updated Website Name",
    "description": "Updated description",
    "updated_at": "2024-01-01T00:00:00Z"
  }
}
```

---

### 5.5 Delete Project

Menghapus project.

**Endpoint**: `DELETE /api/v1/projects/{project_id}`

**Access**: Authenticated (workspace owner or admin)

**Response (200 OK)**:
```json
{
  "success": true,
  "data": {
    "message": "Project deleted successfully"
  }
}
```

---

### 5.6 Save Canvas State

Menyimpan state canvas project.

**Endpoint**: `PUT /api/v1/projects/{project_id}/canvas`

**Access**: Authenticated (project owner or workspace member with edit access)

**Request Body**:
```json
{
  "components": [
    {
      "id": "component-uuid",
      "type": "button",
      "props": {
        "text": "Click Me",
        "color": "#007bff"
      },
      "position": {
        "x": 100,
        "y": 200
      }
    }
  ],
  "version": "1.0"
}
```

**Response (200 OK)**:
```json
{
  "success": true,
  "data": {
    "project_id": "uuid",
    "version": "1.0",
    "saved_at": "2024-01-01T00:00:00Z",
    "components_count": 1
  }
}
```

---

## 6. Publishing Endpoints

### 6.1 Publish Project

Mempublikasikan project ke hosting.

**Endpoint**: `POST /api/v1/projects/{project_id}/publish`

**Access**: Authenticated (Exclusive plan required)

**Request Body**:
```json
{
  "domain": "mywebsite.rinova.app",
  "custom_domain": "mywebsite.com" // optional
}
```

**Response (202 Accepted)**:
```json
{
  "success": true,
  "data": {
    "publish_id": "uuid",
    "project_id": "uuid",
    "status": "building",
    "domain": "mywebsite.rinova.app",
    "estimated_completion": "2024-01-01T00:05:00Z"
  }
}
```

---

### 6.2 Get Publish Status

Mendapatkan status publishing.

**Endpoint**: `GET /api/v1/projects/{project_id}/publish/status`

**Access**: Authenticated

**Response (200 OK)**:
```json
{
  "success": true,
  "data": {
    "publish_id": "uuid",
    "status": "completed",
    "domain": "mywebsite.rinova.app",
    "url": "https://mywebsite.rinova.app",
    "completed_at": "2024-01-01T00:05:00Z",
    "audit_hash": "0x..."
  }
}
```

**Status Values**:
- `building`: Generating static files
- `deploying`: Uploading to hosting
- `configuring_dns`: Setting up DNS
- `completed`: Successfully published
- `failed`: Publishing failed

---

### 6.3 Unpublish Project

Menghapus website dari hosting.

**Endpoint**: `DELETE /api/v1/projects/{project_id}/publish`

**Access**: Authenticated (workspace owner or admin)

**Response (200 OK)**:
```json
{
  "success": true,
  "data": {
    "message": "Website unpublished successfully"
  }
}
```

---

## 7. Blockchain Endpoints

### 7.1 Record Ownership

Mencatat kepemilikan asset di blockchain.

**Endpoint**: `POST /api/v1/blockchain/ownership`

**Access**: Authenticated (internal service call)

**Request Body**:
```json
{
  "asset_id": "uuid",
  "asset_type": "component" | "project",
  "owner_address": "0x...",
  "metadata": {
    "name": "Premium Component",
    "description": "...",
    "file_hash": "Qm..."
  }
}
```

**Response (201 Created)**:
```json
{
  "success": true,
  "data": {
    "transaction_hash": "0x...",
    "token_id": "123",
    "contract_address": "0x...",
    "owner_address": "0x...",
    "recorded_at": "2024-01-01T00:00:00Z"
  }
}
```

---

### 7.2 Verify Ownership

Memverifikasi kepemilikan asset.

**Endpoint**: `GET /api/v1/blockchain/ownership/{asset_id}`

**Access**: Authenticated

**Response (200 OK)**:
```json
{
  "success": true,
  "data": {
    "asset_id": "uuid",
    "owner_address": "0x...",
    "token_id": "123",
    "verified": true,
    "verified_at": "2024-01-01T00:00:00Z"
  }
}
```

---

### 7.3 Get Audit Trail

Mendapatkan audit trail untuk project.

**Endpoint**: `GET /api/v1/blockchain/audit/{project_id}`

**Access**: Authenticated

**Response (200 OK)**:
```json
{
  "success": true,
  "data": {
    "project_id": "uuid",
    "audit_records": [
      {
        "version": 1,
        "hash": "0x...",
        "published_at": "2024-01-01T00:00:00Z",
        "published_by": "user@example.com",
        "transaction_hash": "0x..."
      }
    ]
  }
}
```

---

## 8. Notification Endpoints

### 8.1 List Notifications

Mendapatkan notifikasi user.

**Endpoint**: `GET /api/v1/notifications`

**Access**: Authenticated

**Query Parameters**:
- `page` (optional): Page number (default: 1)
- `limit` (optional): Items per page (default: 20)
- `unread_only` (optional): Filter unread only (default: false)

**Response (200 OK)**:
```json
{
  "success": true,
  "data": {
    "notifications": [
      {
        "notification_id": "uuid",
        "type": "workspace_invitation",
        "title": "Workspace Invitation",
        "message": "You've been invited to join Company Workspace",
        "read": false,
        "created_at": "2024-01-01T00:00:00Z",
        "data": {
          "workspace_id": "uuid",
          "invitation_id": "uuid"
        }
      }
    ],
    "pagination": {
      "page": 1,
      "limit": 20,
      "total": 50,
      "unread_count": 5
    }
  }
}
```

---

### 8.2 Mark Notification as Read

Menandai notifikasi sebagai sudah dibaca.

**Endpoint**: `PATCH /api/v1/notifications/{notification_id}/read`

**Access**: Authenticated

**Response (200 OK)**:
```json
{
  "success": true,
  "data": {
    "notification_id": "uuid",
    "read": true,
    "read_at": "2024-01-01T00:00:00Z"
  }
}
```

---

### 8.3 Mark All Notifications as Read

Menandai semua notifikasi sebagai sudah dibaca.

**Endpoint**: `POST /api/v1/notifications/read-all`

**Access**: Authenticated

**Response (200 OK)**:
```json
{
  "success": true,
  "data": {
    "marked_count": 5,
    "message": "All notifications marked as read"
  }
}
```

---

## 9. WebSocket Events

Connect to WebSocket for real-time updates:

```
ws://localhost:8080/ws
```

### Authentication

Send authentication message after connection:

```json
{
  "type": "auth",
  "token": "your_jwt_token"
}
```

### Event Types

#### Project Update Event
```json
{
  "type": "project_update",
  "project_id": "uuid",
  "data": {
    "updated_by": "user_id",
    "changes": { ... }
  },
  "timestamp": "2024-01-01T00:00:00Z"
}
```

#### Notification Event
```json
{
  "type": "notification",
  "notification_id": "uuid",
  "title": "New Notification",
  "message": "You have a new workspace invitation",
  "timestamp": "2024-01-01T00:00:00Z"
}
```

#### Collaboration Event
```json
{
  "type": "collaboration",
  "project_id": "uuid",
  "user_id": "uuid",
  "action": "cursor_move" | "component_edit" | "component_add" | "component_delete",
  "data": { ... },
  "timestamp": "2024-01-01T00:00:00Z"
}
```

---

## 10. Error Codes

### Common Error Codes

| Code | HTTP Status | Description |
|------|-------------|-------------|
| `INVALID_INPUT` | 400 | Invalid request body or parameters |
| `UNAUTHORIZED` | 401 | Missing or invalid authentication token |
| `FORBIDDEN` | 403 | Insufficient permissions |
| `NOT_FOUND` | 404 | Resource not found |
| `CONFLICT` | 409 | Resource conflict (e.g., duplicate email) |
| `RATE_LIMIT_EXCEEDED` | 429 | Too many requests |
| `INTERNAL_ERROR` | 500 | Internal server error |

### Subscription Error Codes

| Code | Description |
|------|-------------|
| `PLAN_LIMIT_REACHED` | Project limit reached for current plan |
| `FEATURE_NOT_AVAILABLE` | Feature not available in current plan |
| `PAYMENT_FAILED` | Payment processing failed |
| `SUBSCRIPTION_INACTIVE` | Subscription is not active |

### Workspace Error Codes

| Code | Description |
|------|-------------|
| `WORKSPACE_NOT_FOUND` | Workspace does not exist |
| `NOT_WORKSPACE_MEMBER` | User is not a member of workspace |
| `INSUFFICIENT_ROLE` | User role insufficient for action |
| `INVITATION_EXPIRED` | Workspace invitation has expired |
| `INVITATION_INVALID` | Invalid invitation token |

---

## 11. Pagination

Most list endpoints support pagination with the following query parameters:

- `page`: Page number (starts at 1)
- `limit`: Items per page (default: 20, max: 100)

### Pagination Response Format

```json
{
  "success": true,
  "data": {
    "items": [...],
    "pagination": {
      "page": 1,
      "limit": 20,
      "total": 150,
      "total_pages": 8
    }
  }
}
```

---

## 12. Webhooks

### Stripe Webhook

Endpoint untuk menerima webhook dari Stripe.

**Endpoint**: `POST /api/v1/webhooks/stripe`

**Headers**:
- `Stripe-Signature`: Stripe webhook signature

**Events**:
- `checkout.session.completed`: Payment successful
- `customer.subscription.updated`: Subscription updated
- `customer.subscription.deleted`: Subscription cancelled

---

## 13. Health Check

### API Health Check

**Endpoint**: `GET /health`

**Access**: Public

**Response (200 OK)**:
```json
{
  "status": "healthy",
  "version": "1.0.0",
  "timestamp": "2024-01-01T00:00:00Z",
  "services": {
    "database": "healthy",
    "redis": "healthy",
    "user_service": "healthy",
    "subscription_service": "healthy",
    "project_service": "healthy",
    "publishing_service": "healthy",
    "blockchain_service": "healthy",
    "notification_service": "healthy"
  }
}
```

---

## 14. API Versioning

API menggunakan versioning di URL path (e.g., `/api/v1/`). Breaking changes akan di-release sebagai versi baru (e.g., `/api/v2/`) dengan backward compatibility maintained selama mungkin.

### Version History

| Version | Release Date | Status | Notes |
|---------|--------------|--------|-------|
| v1 | 2024-01-01 | Current | Initial release |

---

## Support

Untuk pertanyaan atau masalah terkait API:

- **Documentation**: [docs.rinova.io/api](https://docs.rinova.io/api)
- **Support Email**: api-support@rinova.io
- **GitHub Issues**: [github.com/rinova/api-issues](https://github.com/rinova/api-issues)
