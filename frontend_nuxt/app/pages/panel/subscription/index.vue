<script setup lang="ts">
definePageMeta({ layout: 'panel', sidebarSection: 'settings' })

const subscriptionStore = useSubscriptionStore()

onMounted(() => subscriptionStore.fetchSubscription())

const plans = [
  { id: 'freemium' as const, name: 'Free', price: 0, desc: 'Get started for free', features: ['1 Project', 'Basic Templates', 'Community Support'] },
  { id: 'enterprise' as const, name: 'Enterprise', price: 29, desc: 'For professionals', features: ['Unlimited Projects', 'All Templates', 'Animation Editor', 'Custom CSS', 'Priority Support'] },
  { id: 'exclusive' as const, name: 'Exclusive', price: 99, desc: 'For agencies & teams', features: ['Everything in Enterprise', 'Team Collaboration', 'White Label', 'Dedicated Manager', 'API Access'] }
]

const isLoading = ref(false)
const showCancelModal = ref(false)

const currentPlanIndex = computed(() => plans.findIndex(p => p.id === subscriptionStore.currentPlan))

async function handleUpgrade(planId: 'freemium' | 'enterprise' | 'exclusive') {
  if (planId === subscriptionStore.currentPlan) return
  isLoading.value = true
  try { await subscriptionStore.upgradePlan(planId) } finally { isLoading.value = false }
}

async function handleCancelSubscription() {
  isLoading.value = true
  try {
    await subscriptionStore.cancelSubscription()
    showCancelModal.value = false
  } finally {
    isLoading.value = false
  }
}
</script>

<template>
  <div class="space-y-6 max-w-3xl mx-auto">
    <div>
      <h1 class="text-2xl font-bold tracking-tight">Subscription</h1>
      <p class="text-muted-foreground text-sm mt-1">Manage your subscription and billing</p>
    </div>

    <div class="card">
      <div class="card-header">
        <h2 class="card-title">Current Plan</h2>
        <p class="card-description">You are currently on the {{ subscriptionStore.currentPlan }} plan</p>
      </div>
      <div class="card-body">
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-3">
            <div class="text-xl font-bold capitalize text-foreground">{{ subscriptionStore.currentPlan }}</div>
            <span
              class="badge"
              :class="subscriptionStore.currentPlan === 'enterprise' ? 'bg-primary/10 text-primary border-primary/20' : subscriptionStore.currentPlan === 'exclusive' ? 'bg-amber-100 text-amber-700 dark:bg-amber-900/40 dark:text-amber-400 border-amber-200 dark:border-amber-800' : 'bg-muted text-muted-foreground border'"
            >
              {{ subscriptionStore.isActive ? 'Active' : 'Inactive' }}
            </span>
          </div>
          <div v-if="subscriptionStore.currentPlan !== 'freemium'">
            <button class="btn btn-outline btn-sm" @click="showCancelModal = true">Cancel Subscription</button>
          </div>
        </div>
      </div>
    </div>

    <div class="grid md:grid-cols-3 gap-4">
      <div
        v-for="plan in plans"
        :key="plan.id"
        class="rounded-xl border-2 p-6 space-y-4 transition-all hover:shadow-md relative"
        :class="plan.id === subscriptionStore.currentPlan ? 'border-primary shadow-primary/10' : 'border-border hover:border-muted-foreground/30'"
      >
        <div v-if="plan.id === 'enterprise'" class="absolute -top-3 left-1/2 -translate-x-1/2 bg-primary text-primary-foreground text-[10px] font-semibold px-3 py-0.5 rounded-full">
          Most Popular
        </div>
        <div>
          <h3 class="text-lg font-semibold text-foreground">{{ plan.name }}</h3>
          <p class="text-sm text-muted-foreground mt-1">{{ plan.desc }}</p>
        </div>
        <div>
          <span class="text-3xl font-bold text-foreground">${{ plan.price }}</span>
          <span class="text-muted-foreground text-sm">/mo</span>
        </div>
        <ul class="space-y-2 text-sm">
          <li v-for="feature in plan.features" :key="feature" class="flex items-center gap-2">
            <svg class="w-4 h-4 text-green-500 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
            </svg>
            <span class="text-muted-foreground">{{ feature }}</span>
          </li>
        </ul>
        <button
          v-if="plan.id === subscriptionStore.currentPlan"
          class="btn btn-secondary w-full btn-sm" disabled
        >
          Current Plan
        </button>
        <button
          v-else-if="plans.findIndex(p => p.id === plan.id) > currentPlanIndex"
          class="btn btn-primary w-full btn-sm"
          :disabled="isLoading"
          @click="handleUpgrade(plan.id)"
        >
          Upgrade
        </button>
        <button
          v-else
          class="btn btn-outline w-full btn-sm"
          :disabled="isLoading"
          @click="handleUpgrade(plan.id)"
        >
          Downgrade
        </button>
      </div>
    </div>

    <div v-if="subscriptionStore.currentPlan !== 'freemium'" class="card">
      <div class="card-header">
        <h2 class="card-title">Billing History</h2>
        <p class="card-description">View your past invoices</p>
      </div>
      <div class="card-body">
        <p class="text-sm text-muted-foreground text-center py-4">No billing history yet</p>
      </div>
    </div>

    <div v-if="showCancelModal" class="modal-overlay" @click.self="showCancelModal = false">
      <div class="modal">
        <div class="modal-header">
          <h3 class="text-lg font-semibold">Cancel Subscription</h3>
        </div>
        <div class="modal-body">
          <p class="text-sm text-muted-foreground">
            Are you sure you want to cancel? You will lose access to premium features at the end of your billing period.
          </p>
        </div>
        <div class="modal-footer">
          <button class="btn btn-secondary btn-sm" @click="showCancelModal = false">Keep Subscription</button>
          <button class="btn bg-destructive text-destructive-foreground hover:bg-destructive/90 btn-sm" @click="handleCancelSubscription">Cancel Subscription</button>
        </div>
      </div>
    </div>
  </div>
</template>
