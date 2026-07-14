/**
 * Auth token storage.
 *
 * The backend issues a 7-day HS256 JWT. It also sets a `Secure` HttpOnly
 * cookie, but that cookie is unusable over plain http (local dev) and is
 * not readable by JS. So we persist the token ourselves in a cookie that
 * is sent during SSR and readable on the client.
 */
export function useAuthToken() {
  return useCookie<string | null>('auth_token', {
    maxAge: 60 * 60 * 24 * 7, // 7 days, matches backend JWT expiry
    sameSite: 'lax',
    path: '/',
    // `Secure` is intentionally omitted so it works over http during dev.
  })
}
