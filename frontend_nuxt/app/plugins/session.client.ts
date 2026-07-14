/**
 * Session bootstrap.
 *
 * On app start (client), if an auth token is present but the user store is
 * empty, hydrate the current user, workspaces, and subscription. This keeps
 * the session alive across full page reloads.
 */
export default defineNuxtPlugin(async () => {
  const token = useAuthToken()
  if (!token.value) return

  const userStore = useUserStore()
  if (userStore.user) return

  const { fetchCurrentUser } = useAuth()
  await fetchCurrentUser()

  // Only continue hydration if the token was still valid.
  if (!userStore.user) return

  const workspaceStore = useWorkspaceStore()
  const subscriptionStore = useSubscriptionStore()
  // allSettled so one failing hydration call never crashes app startup.
  await Promise.allSettled([
    workspaceStore.fetchWorkspaces(),
    subscriptionStore.fetchSubscription(),
  ])
})
