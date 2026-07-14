/**
 * Route guard for the authenticated panel area.
 *
 * Applied via `definePageMeta({ middleware: 'auth' })` or globally to /panel.
 * If there's no auth token, redirect to /login preserving the intended path.
 */
export default defineNuxtRouteMiddleware((to) => {
  const token = useAuthToken()
  if (!token.value) {
    return navigateTo(`/login?redirect=${encodeURIComponent(to.fullPath)}`)
  }
})
