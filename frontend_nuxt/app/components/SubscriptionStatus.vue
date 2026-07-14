<script setup lang="ts">
import type { PlanType } from '~/stores/subscription'

const subscriptionStore = useSubscriptionStore()

const planColors: Record<PlanType, string> = {
  freemium: 'bg-muted text-muted-foreground',
  enterprise: 'bg-primary/10 text-primary',
  exclusive: 'bg-gradient-to-r from-amber-400 to-orange-500 text-white'
}

const planNames: Record<PlanType, string> = {
  freemium: 'Free',
  enterprise: 'Enterprise',
  exclusive: 'Exclusive'
}
</script>

<template>
  <div class="card p-4">
    <div class="flex items-center justify-between mb-3">
      <span class="text-sm font-medium text-foreground">Current Plan</span>
      <span class="badge capitalize" :class="planColors[subscriptionStore.currentPlan]">
        {{ planNames[subscriptionStore.currentPlan] }}
      </span>
    </div>

    <div class="mb-3">
      <div class="flex justify-between text-sm text-muted-foreground mb-1">
        <span>Projects</span>
        <span>{{ subscriptionStore.currentPlan === 'exclusive' ? 'Unlimited' : `0 / ${subscriptionStore.limits.maxProjects}` }}</span>
      </div>
      <div v-if="subscriptionStore.currentPlan !== 'exclusive'" class="h-2 bg-muted rounded-full overflow-hidden">
        <div class="h-full bg-primary rounded-full" style="width: 0%" />
      </div>
    </div>

    <NuxtLink v-if="subscriptionStore.currentPlan !== 'exclusive'" to="/panel/subscription" class="btn btn-primary btn-sm w-full">
      Upgrade Plan
    </NuxtLink>
  </div>
</template>
