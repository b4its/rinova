/**
 * Global route guard for authenticated areas (/panel, /editor).
 *
 * Runs on every navigation. If the visitor hits a protected route without an
 * auth token, bounce them to /login preserving the intended destination.
 */
export default defineNuxtRouteMiddleware((to) => {
  const protectedPrefixes = ['/panel', '/editor']
  const needsAuth = protectedPrefixes.some(p => to.path.startsWith(p))
  if (!needsAuth) return

  const token = useAuthToken()
  if (!token.value) {
    return navigateTo(`/login?redirect=${encodeURIComponent(to.fullPath)}`)
  }
})
