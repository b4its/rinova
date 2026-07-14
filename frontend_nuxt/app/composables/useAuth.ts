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

/** Map the backend user payload to the frontend `User` model. */
function mapUser(u: BackendUser): User {
  return {
    id: u.id,
    email: u.email,
    fullName: u.full_name,
    accountType: u.account_type,
    // Backend has no role column yet; default to 'user' unless it sends one.
    role: u.role ?? 'user',
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
