## Objective
- Complete the superuser/admin role system in the Rinova backend so admin accounts can access the admin panel, and add a Marketplace admin feature with CRUD + pagination for categories and products, including HTML code preview for template-type products.

## Important Details
- Admin feature is exclusively driven by the `role` field returned from the backend — no other `superuser`/admin mechanism exists.
- Subscription workspace subscriptions use different pricing than personal (Enterprise $99 vs $29, Exclusive $199 vs $79 per month).
- Effective plan resolution at login: max of personal subscription + workspace subscriptions (via workspace_membership join).
- Frontend stores/user.ts already expects `role: 'user' | 'superuser'`; useAuth.ts maps backend `role` to the store.
- Marketplace data is stored in `localStorage` keys `admin_marketplace_kategori` and `admin_marketplace_produk` (same pattern as existing admin templates).
- All backend services compile successfully with zero errors. Docker containers still need rebuilding.
- Content in admin pages uses `mx-auto max-w-6xl` for centering.

## Work State
### Completed
- Added `role VARCHAR(50) NOT NULL DEFAULT 'user'` column to `users` table (migration + running DB).
- Created `UserRole` enum (`User` / `Superuser`) in `shared::types`.
- Added `role` field to `shared::types::User`, `Claims`, and `UserResponse`.
- Added `plan` field to `shared::types::Claims` with default `"freemium"`.
- Added `workspace_id: Option<Uuid>` to `Subscription` in both shared and subscription-service models.
- Updated all user_service SQL queries to include `role` column.
- Updated user_service `generate_jwt_token` to accept and embed `plan` and `role`.
- Updated user_service `login_user` to resolve effective plan (personal + workspace subs) and return `(User, String)`.
- Updated user_service `register_user` to create company workspace + workspace subscription for company accounts.
- Added `create_company_workspace` and `create_personal_workspace` with auto-insert of owner as `workspace_members` with `'owner'` role.
- Added workspace subscription methods in subscription_service (repository, service, handlers).
- Added personal vs workspace pricing via `price_cents_workspace()` on `PlanType`.
- Added `SubscriptionType` enum and `all_plans_for(sub_type)` for PlanDetails.
- Added API endpoints: `/plans/personal`, `/plans/workspace`, `/workspace/freemium`, `/workspace/{workspace_id}`, `/{user_id}/effective-plan` in subscription_service.
- Updated API gateway middleware: `role` and `plan` in `Claims`/`AuthenticatedUser`, `X-User-Role` header proxying.
- Created 15 seed accounts (5 admin, 5 exclusive, 5 enterprise) with subscription and workspace owner membership — password for all: `Password123!`.
- Added `Clone` derive to `SubscriptionRepository` and registered it in `app_data` in subscription_service.
- Inserted missing `workspace_members` entries for all workspace owners.
- Created frontend marketplace admin pages:
  - `/panel/admin/marketplace/kategori.vue` — CRUD with pagination for marketplace categories.
  - `/panel/admin/marketplace/produk/index.vue` — product list with CRUD + pagination.
  - `/panel/admin/marketplace/produk/add.vue` — add product with fields: uuid, name, kategori, descriptions, visual, price, html_code (shown only when kategori is "Templates"), with live iframe preview in desktop/tablet/mobile viewports + full-screen preview mode with Ctrl+F search in the HTML textarea.
- Updated sidebar in `panel.vue`: added "Marketplace" dropdown under admin navigation with links to Kategori and Produk.

### Active
- (none)

### Blocked
- All backend Docker containers need rebuilding to pick up code changes — Rust Docker build takes ~30+ minutes and may time out.

## Next Move
1. Run `docker-compose build subscription-service user-service api-gateway` to deploy all backend changes (long-running task, ~30+ minutes).
2. Run the new migration `20240115000004_add_workspace_to_subscriptions.sql` on the database to add `workspace_id` and `stripe_customer_id` columns.
3. Verify admin login with seed accounts works and displays the admin panel with new Marketplace section.

## Relevant Files
- `backend_rust/libs/shared/src/types.rs`: `User`, `Subscription`, `Claims` structs, `UserRole`, `PlanType` enums.
- `backend_rust/libs/database/migrations/20240115000001_create_users_workspaces.sql`: user `role` column.
- `backend_rust/libs/database/migrations/20240115000004_add_workspace_to_subscriptions.sql`: workspace_id column.
- `backend_rust/services/user_service/src/`: all handlers, services, repository — role + plan + workspace subscription flows.
- `backend_rust/services/subscription_service/src/`: workspace subscription logic, pricing, all CRUD.
- `backend_rust/services/api_gateway/src/middleware/auth.rs`: `Claims`, `AuthenticatedUser`, `X-User-Role` forwarding.
- `backend_rust/services/api_gateway/src/proxy/proxy_service.rs`: header forwarding.
- `frontend_nuxt/app/layouts/panel.vue`: sidebar navigation with Marketplace dropdown.
- `frontend_nuxt/app/pages/panel/admin/marketplace/kategori.vue`: marketplace category CRUD.
- `frontend_nuxt/app/pages/panel/admin/marketplace/produk/index.vue`: marketplace product list/dialog CRUD.
- `frontend_nuxt/app/pages/panel/admin/marketplace/produk/add.vue`: marketplace product add with template preview.
- `frontend_nuxt/app/stores/user.ts`: `User` interface expects `role: 'user' | 'superuser'`.
- `frontend_nuxt/app/composables/useAuth.ts`: maps `BackendUser.role` to `User.role`.
