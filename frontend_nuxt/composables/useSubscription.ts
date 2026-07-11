import type { FeatureFlags, PlanLimits } from '~/stores/subscription'

/**
 * Composable for subscription and feature flag management
 * Validates: Requirements 4 (Feature Flagging and UI Locking)
 */
export function useSubscription() {
  const subscriptionStore = useSubscriptionStore()
  const userStore = useUserStore()

  const isFeatureEnabled = (feature: keyof FeatureFlags): boolean => {
    if (!userStore.isAuthenticated) {
      return false
    }
    return subscriptionStore.isFeatureEnabled(feature)
  }

  const checkLimit = (limitType: keyof PlanLimits, current: number): boolean => {
    return subscriptionStore.checkLimit(limitType, current)
  }

  const canAccessAnimationEditor = computed(() => 
    isFeatureEnabled('animationEditor')
  )

  const canAccessCustomCSS = computed(() => 
    isFeatureEnabled('customCSS')
  )

  const canAccessSEOTools = computed(() => 
    isFeatureEnabled('seoTools')
  )

  const canAccessAssetMarketplace = computed(() => 
    isFeatureEnabled('assetMarketplace')
  )

  const canOneClickPublish = computed(() => 
    isFeatureEnabled('oneClickPublish')
  )

  const canAccessAnalytics = computed(() => 
    isFeatureEnabled('analyticsDashboard')
  )

  const canUseCustomDomain = computed(() => 
    isFeatureEnabled('customDomain')
  )

  const canUseResponsiveDesign = computed(() => 
    isFeatureEnabled('responsiveDesign')
  )

  return {
    // State
    subscription: toRef(subscriptionStore, 'subscription'),
    isLoading: toRef(subscriptionStore, 'isLoading'),
    currentPlan: toRef(subscriptionStore, 'currentPlan'),
    
    // Feature flags
    canAccessAnimationEditor,
    canAccessCustomCSS,
    canAccessSEOTools,
    canAccessAssetMarketplace,
    canOneClickPublish,
    canAccessAnalytics,
    canUseCustomDomain,
    canUseResponsiveDesign,
    
    // Methods
    isFeatureEnabled,
    checkLimit,
    upgrade: subscriptionStore.upgradePlan,
    fetchSubscription: subscriptionStore.fetchSubscription
  }
}
