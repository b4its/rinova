import type { User } from '~/stores/user'

/** Raw user shape as returned by the Rust user_service (snake_case). */
interface BackendUser {
  id: string
  email: string
  full_name: string | null
  account_type: 'personal' | 'company'
  email_verified_at: string | null
  created_at: string
  updated_at: string
  role?: 'user' | 'superuser'
}

interface AuthResponse {
  user: BackendUser
  token: string
}

interface LocalUser {
  id: string
  name: string
  email: string
  role: 'user' | 'superuser'
  status: string
  created: string
}

const STORAGE_KEY = 'admin_users'

/**
 * Ensure the logged-in user exists in localStorage admin_users.
 * - If localStorage is empty → seed defaults + add current user as superuser
 * - If email not found → add with 'user' role
 * Returns the effective role for this user.
 */
function ensureLocalUser(email: string, name: string): 'user' | 'superuser' {
  if (import.meta.server) return 'user'

  const raw = localStorage.getItem(STORAGE_KEY)
  let users: LocalUser[] = []

  if (!raw) {
    users = [
      { id: '1', name: 'Admin', email: 'admin@rinova.id', role: 'superuser', status: 'active', created: '2026-01-01' },
      { id: '2', name: 'Alice Johnson', email: 'alice@example.com', role: 'user', status: 'active', created: '2026-01-15' },
      { id: '3', name: 'Bob Smith', email: 'bob@example.com', role: 'user', status: 'active', created: '2026-02-20' },
      { id: '4', name: 'Diana Prince', email: 'diana@example.com', role: 'superuser', status: 'active', created: '2026-01-01' },
    ]
  } else {
    try { users = JSON.parse(raw) } catch { users = [] }
  }

  const existing = users.find(u => u.email === email)
  if (existing) return existing.role

  // First-time login for this email → if localStorage was empty, grant superuser
  const role: 'user' | 'superuser' = !raw ? 'superuser' : 'user'
  users.push({
    id: String(Date.now()),
    name,
    email,
    role,
    status: 'active',
    created: new Date().toISOString().split('T')[0],
  })
  localStorage.setItem(STORAGE_KEY, JSON.stringify(users))

  return role
}

/** Map the backend user payload to the frontend `User` model. */
function mapUser(u: BackendUser): User {
  const name = u.full_name ?? u.email.split('@')[0]
  const localRole = ensureLocalUser(u.email, name)
  return {
    id: u.id,
    email: u.email,
    fullName: u.full_name,
    accountType: u.account_type,
    role: u.role ?? localRole,
    emailVerified: u.email_verified_at != null,
    createdAt: u.created_at,
    updatedAt: u.updated_at,
  }
}

export function useAuth() {
  const userStore = useUserStore()
  const token = useAuthToken()
  const api = useApi()

  const isLoading = ref(false)
  const error = ref<string | null>(null)

  async function run<T>(fn: () => Promise<T>): Promise<T | null> {
    isLoading.value = true
    error.value = null
    try {
      return await fn()
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Something went wrong'
      return null
    } finally {
      isLoading.value = false
    }
  }

  async function login(email: string, password: string): Promise<boolean> {
    const res = await run(() =>
      api.post<AuthResponse>('/auth/login', { email, password })
    )
    if (!res) return false
    token.value = res.token
    userStore.setUser(mapUser(res.user))
    return true
  }

  async function register(
    email: string,
    password: string,
    fullName: string,
    accountType: 'personal' | 'company'
  ): Promise<boolean> {
    // Backend register returns a message only (email verification required),
    // no token yet.
    const res = await run(() =>
      api.post<{ message: string }>('/auth/register', {
        email,
        password,
        full_name: fullName,
        account_type: accountType,
      })
    )
    return res !== null
  }

  async function verifyEmail(token_: string): Promise<boolean> {
    const res = await run(() =>
      api.post<AuthResponse>('/auth/verify-email', { token: token_ })
    )
    if (!res) return false
    // verify-email issues a login token + user.
    if (res.token) token.value = res.token
    if (res.user) userStore.setUser(mapUser(res.user))
    return true
  }

  async function logout(): Promise<void> {
    await run(() => api.post('/auth/logout'))
    token.value = null
    userStore.clearUser()
  }

  async function fetchCurrentUser(): Promise<void> {
    if (!token.value) return
    const res = await run(() => api.get<BackendUser>('/users/me'))
    if (res) userStore.setUser(mapUser(res))
  }

  async function resendVerification(email: string): Promise<boolean> {
    const res = await run(() =>
      api.post<{ message: string }>('/auth/resend-verification', { email })
    )
    return res !== null
  }

  return {
    isLoading: readonly(isLoading),
    error: readonly(error),
    login,
    register,
    verifyEmail,
    logout,
    fetchCurrentUser,
    resendVerification,
  }
}
