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
    animationEditor: false,
    customCSS: false,
    seoTools: false,
    assetMarketplace: false,
    oneClickPublish: false,
    analyticsDashboard: false,
    customDomain: false,
    responsiveDesign: false
  },
  enterprise: {
    animationEditor: true,
    customCSS: true,
    seoTools: true,
    assetMarketplace: true,
    oneClickPublish: false,
    analyticsDashboard: false,
    customDomain: false,
    responsiveDesign: true
  },
  exclusive: {
    animationEditor: true,
    customCSS: true,
    seoTools: true,
    assetMarketplace: true,
    oneClickPublish: true,
    analyticsDashboard: true,
    customDomain: true,
    responsiveDesign: true
  }
}

const PLAN_LIMITS: Record<PlanType, PlanLimits> = {
  freemium: {
    maxProjects: 2,
    maxTeamMembers: 1,
    maxStorageMB: 100,
    rateLimitPerMinute: 100
  },
  enterprise: {
    maxProjects: 10,
    maxTeamMembers: 5,
    maxStorageMB: 1000,
    rateLimitPerMinute: 1000
  },
  exclusive: {
    maxProjects: Infinity,
    maxTeamMembers: Infinity,
    maxStorageMB: 10000,
    rateLimitPerMinute: 5000
  }
}

export const useSubscriptionStore = defineStore('subscription', () => {
  // State
  const subscription = ref<Subscription | null>(null)
  const isLoading = ref(false)

  // Getters
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
    isLoading.value = true
    try {
      // TODO: Implement API call
      // const response = await $fetch('/api/v1/subscriptions/me')
      // setSubscription(response.subscription)
    } finally {
      isLoading.value = false
    }
  }

  async function upgradePlan(_plan: PlanType): Promise<void> {
    isLoading.value = true
    try {
      // TODO: Implement upgrade API call
      // const response = await $fetch('/api/v1/subscriptions/upgrade', {
      //   method: 'POST',
      //   body: { plan }
      // })
      // setSubscription(response.subscription)
    } finally {
      isLoading.value = false
    }
  }

  return {
    // State
    subscription,
    isLoading,
    // Getters
    currentPlan,
    isActive,
    features,
    limits,
    // Actions
    isFeatureEnabled,
    checkLimit,
    setSubscription,
    clearSubscription,
    fetchSubscription,
    upgradePlan
  }
})
