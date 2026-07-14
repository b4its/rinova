import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

export type PlanType = 'freemium' | 'enterprise' | 'exclusive'

export interface FeatureFlags {
  animationEditor: boolean
  customCSS: boolean
  seoTools: boolean
  assetMarketplace: boolean
  oneClickPublish: boolean
  analyticsDashboard: boolean
  customDomain: boolean
  responsiveDesign: boolean
}

export interface PlanLimits {
  maxProjects: number
  maxTeamMembers: number
  maxStorageMB: number
  rateLimitPerMinute: number
}

export interface Subscription {
  id: string
  planType: PlanType
  status: 'active' | 'canceled' | 'expired' | 'past_due'
  currentPeriodStart: string | null
  currentPeriodEnd: string | null
  features: FeatureFlags
  limits: PlanLimits
}

const PLAN_FEATURES: Record<PlanType, FeatureFlags> = {
  freemium: {
    animationEditor: false, customCSS: false, seoTools: false,
    assetMarketplace: false, oneClickPublish: false,
    analyticsDashboard: false, customDomain: false, responsiveDesign: false
  },
  enterprise: {
    animationEditor: true, customCSS: true, seoTools: true,
    assetMarketplace: true, oneClickPublish: false,
    analyticsDashboard: false, customDomain: false, responsiveDesign: true
  },
  exclusive: {
    animationEditor: true, customCSS: true, seoTools: true,
    assetMarketplace: true, oneClickPublish: true,
    analyticsDashboard: true, customDomain: true, responsiveDesign: true
  }
}

const PLAN_LIMITS: Record<PlanType, PlanLimits> = {
  freemium: { maxProjects: 2, maxTeamMembers: 1, maxStorageMB: 100, rateLimitPerMinute: 100 },
  enterprise: { maxProjects: 10, maxTeamMembers: 5, maxStorageMB: 1000, rateLimitPerMinute: 1000 },
  exclusive: { maxProjects: Infinity, maxTeamMembers: Infinity, maxStorageMB: 10000, rateLimitPerMinute: 5000 }
}

/** Raw subscription shape returned by subscription_service (snake_case). */
interface BackendSubscription {
  id: string
  plan_type: PlanType
  status: 'active' | 'canceled' | 'expired' | 'past_due'
  current_period_start: string | null
  current_period_end: string | null
}

interface SubscriptionResponse {
  subscription: BackendSubscription
}

function mapSubscription(s: BackendSubscription): Subscription {
  return {
    id: s.id,
    planType: s.plan_type,
    status: s.status,
    currentPeriodStart: s.current_period_start,
    currentPeriodEnd: s.current_period_end,
    features: PLAN_FEATURES[s.plan_type],
    limits: PLAN_LIMITS[s.plan_type]
  }
}

export const useSubscriptionStore = defineStore('subscription', () => {
  const subscription = ref<Subscription | null>(null)
  const isLoading = ref(false)

  // Default to the most restrictive plan until real data loads.
  const currentPlan = computed(() => subscription.value?.planType ?? 'freemium')
  const isActive = computed(() => subscription.value?.status === 'active')
  const features = computed(() => PLAN_FEATURES[currentPlan.value])
  const limits = computed(() => PLAN_LIMITS[currentPlan.value])

  function isFeatureEnabled(feature: keyof FeatureFlags): boolean {
    return features.value[feature] ?? false
  }

  function checkLimit(limitType: keyof PlanLimits, current: number): boolean {
    return current < limits.value[limitType]
  }

  function setSubscription(subscriptionData: Subscription): void {
    subscription.value = subscriptionData
  }

  function clearSubscription(): void {
    subscription.value = null
  }

  async function fetchSubscription(): Promise<void> {
    const userStore = useUserStore()
    if (!userStore.user) return
    const api = useApi()
    isLoading.value = true
    try {
      const res = await api.get<SubscriptionResponse>(
        `/subscriptions/${userStore.user.id}`
      )
      subscription.value = mapSubscription(res.subscription)
    } catch {
      subscription.value = null
    } finally {
      isLoading.value = false
    }
  }

  async function upgradePlan(plan: PlanType): Promise<void> {
    const userStore = useUserStore()
    if (!userStore.user) return
    const api = useApi()
    isLoading.value = true
    try {
      const res = await api.post<SubscriptionResponse>(
        `/subscriptions/${userStore.user.id}/upgrade`,
        { target_plan: plan }
      )
      subscription.value = mapSubscription(res.subscription)
    } finally {
      isLoading.value = false
    }
  }

  async function cancelSubscription(): Promise<void> {
    if (!subscription.value) return
    const api = useApi()
    isLoading.value = true
    try {
      const res = await api.post<SubscriptionResponse>(
        `/subscriptions/${subscription.value.id}/cancel`
      )
      subscription.value = mapSubscription(res.subscription)
    } finally {
      isLoading.value = false
    }
  }

  return {
    subscription, isLoading,
    currentPlan, isActive, features, limits,
    isFeatureEnabled, checkLimit,
    setSubscription, clearSubscription,
    fetchSubscription, upgradePlan, cancelSubscription
  }
})
