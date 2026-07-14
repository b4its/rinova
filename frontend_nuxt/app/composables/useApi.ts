/**
 * Central HTTP client for the Rust backend.
 *
 * All backend calls go through the API gateway at `runtimeConfig.public.apiUrl`
 * (default http://localhost:8080), base prefix `/api/v1`.
 *
 * Responsibilities:
 *  - inject `Authorization: Bearer <token>` from the auth cookie
 *  - normalise the two backend error shapes into a single message
 *  - clear the session and redirect to /login on 401
 */
type HttpMethod = 'GET' | 'POST' | 'PUT' | 'PATCH' | 'DELETE'

interface RequestOptions {
  method?: HttpMethod
  body?: unknown
  query?: Record<string, unknown>
  headers?: Record<string, string>
}

/** Extract a human-readable message from either backend error envelope. */
function extractErrorMessage(payload: unknown, fallback: string): string {
  if (payload && typeof payload === 'object') {
    const p = payload as Record<string, unknown>
    // shared::errors::ApiErrorResponse -> { code, message, request_id, timestamp }
    if (typeof p.message === 'string') return p.message
    // ad-hoc services -> { error, code }
    if (typeof p.error === 'string') return p.error
  }
  return fallback
}

export function useApi() {
  const config = useRuntimeConfig()
  const token = useAuthToken()
  const baseURL = `${config.public.apiUrl}/api/v1`

  async function request<T>(path: string, options: RequestOptions = {}): Promise<T> {
    const headers: Record<string, string> = { ...options.headers }
    if (token.value) {
      headers.Authorization = `Bearer ${token.value}`
    }

    try {
      return await $fetch<T>(path, {
        baseURL,
        method: options.method,
        body: options.body as Record<string, unknown> | undefined,
        query: options.query,
        headers,
      }) as T
    } catch (err: unknown) {
      const e = err as { status?: number; statusCode?: number; data?: unknown }
      const status = e.status ?? e.statusCode

      // Session expired / invalid -> clear and bounce to login (client only).
      if (status === 401 && import.meta.client) {
        token.value = null
        const userStore = useUserStore()
        userStore.clearUser()
        const route = useRoute()
        if (!route.path.startsWith('/login')) {
          await navigateTo(`/login?redirect=${encodeURIComponent(route.fullPath)}`)
        }
      }

      throw new Error(extractErrorMessage(e.data, 'Request failed. Please try again.'))
    }
  }

  return {
    get: <T>(path: string, options?: RequestOptions) =>
      request<T>(path, { ...options, method: 'GET' }),
    post: <T>(path: string, body?: unknown, options?: RequestOptions) =>
      request<T>(path, { ...options, method: 'POST', body }),
    put: <T>(path: string, body?: unknown, options?: RequestOptions) =>
      request<T>(path, { ...options, method: 'PUT', body }),
    patch: <T>(path: string, body?: unknown, options?: RequestOptions) =>
      request<T>(path, { ...options, method: 'PATCH', body }),
    del: <T>(path: string, options?: RequestOptions) =>
      request<T>(path, { ...options, method: 'DELETE' }),
  }
}
