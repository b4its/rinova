<script setup lang="ts">
definePageMeta({
  layout: 'dashboard'
})

const subscriptionStore = useSubscriptionStore()

const plans = [
  { id: 'freemium' as const, name: 'Free', price: 0 },
  { id: 'enterprise' as const, name: 'Enterprise', price: 29 },
  { id: 'exclusive' as const, name: 'Exclusive', price: 99 }
]

const isLoading = ref(false)
const showCancelModal = ref(false)

const currentPlanIndex = computed(() => 
  plans.findIndex(p => p.id === subscriptionStore.currentPlan)
)

async function handleUpgrade(planId: 'freemium' | 'enterprise' | 'exclusive') {
  if (planId === subscriptionStore.currentPlan) return
  
  isLoading.value = true
  try {
    await subscriptionStore.upgradePlan(planId)
  } finally {
    isLoading.value = false
  }
}

async function handleCancelSubscription() {
  isLoading.value = true
  try {
    // TODO: Implement cancel subscription
    showCancelModal.value = false
  } finally {
    isLoading.value = false
  }
}
</script>

<template>
  <div>
    <div class="mb-8">
      <h1 class="text-2xl font-bold text-gray-900">Subscription</h1>
      <p class="mt-1 text-sm text-gray-600">
        Manage your subscription and billing
      </p>
    </div>
    
    <!-- Current Plan -->
    <div class="card mb-8">
      <div class="card-header">
        <h2 class="text-lg font-semibold text-gray-900">Current Plan</h2>
      </div>
      
      <div class="card-body">
        <div class="flex items-center justify-between">
          <div>
            <div class="flex items-center gap-3">
              <h3 class="text-xl font-bold text-gray-900 capitalize">
                {{ subscriptionStore.currentPlan }}
              </h3>
              <span
                class="badge"
                :class="{
                  'badge-primary': subscriptionStore.currentPlan === 'enterprise',
                  'bg-gradient-to-r from-amber-400 to-orange-500 text-white': subscriptionStore.currentPlan === 'exclusive',
                  'bg-gray-100 text-gray-800': subscriptionStore.currentPlan === 'freemium'
                }"
              >
                {{ subscriptionStore.isActive ? 'Active' : 'Inactive' }}
              </span>
            </div>
            <p class="text-gray-600 mt-1">
              ${{ plans.find(p => p.id === subscriptionStore.currentPlan)?.price || 0 }}/month
            </p>
          </div>
          
          <div v-if="subscriptionStore.currentPlan !== 'freemium'">
            <button
              class="btn btn-secondary"
              @click="showCancelModal = true"
            >
              Cancel Subscription
            </button>
          </div>
        </div>
      </div>
    </div>
    
    <!-- Plan Selection -->
    <div class="card">
      <div class="card-header">
        <h2 class="text-lg font-semibold text-gray-900">Available Plans</h2>
      </div>
      
      <div class="card-body">
        <div class="grid md:grid-cols-3 gap-6">
          <div
            v-for="plan in plans"
            :key="plan.id"
            class="p-6 rounded-xl border-2 text-center"
            :class="plan.id === subscriptionStore.currentPlan 
              ? 'border-primary-500 bg-primary-50' 
              : 'border-gray-200'"
          >
            <h3 class="text-lg font-semibold text-gray-900">{{ plan.name }}</h3>
            <div class="mt-2 mb-4">
              <span class="text-3xl font-bold text-gray-900">${{ plan.price }}</span>
              <span class="text-gray-500">/mo</span>
            </div>
            
            <button
              v-if="plan.id === subscriptionStore.currentPlan"
              class="btn btn-secondary w-full"
              disabled
            >
              Current Plan
            </button>
            <button
              v-else-if="plans.findIndex(p => p.id === plan.id) > currentPlanIndex"
              class="btn btn-primary w-full"
              :disabled="isLoading"
              @click="handleUpgrade(plan.id)"
            >
              Upgrade
            </button>
            <button
              v-else
              class="btn btn-secondary w-full"
              :disabled="isLoading"
              @click="handleUpgrade(plan.id)"
            >
              Downgrade
            </button>
          </div>
        </div>
      </div>
    </div>
    
    <!-- Billing History (for paid plans) -->
    <div v-if="subscriptionStore.currentPlan !== 'freemium'" class="card mt-8">
      <div class="card-header">
        <h2 class="text-lg font-semibold text-gray-900">Billing History</h2>
      </div>
      
      <div class="card-body">
        <p class="text-sm text-gray-500 text-center py-4">
          No billing history yet
        </p>
      </div>
    </div>
    
    <!-- Cancel Subscription Modal -->
    <div v-if="showCancelModal" class="modal-overlay" @click.self="showCancelModal = false">
      <div class="modal">
        <div class="modal-header">
          <h3 class="text-lg font-semibold text-gray-900">Cancel Subscription</h3>
        </div>
        
        <div class="modal-body">
          <p class="text-gray-600">
            Are you sure you want to cancel your subscription? You will lose access to premium features at the end of your billing period.
          </p>
        </div>
        
        <div class="modal-footer">
          <button class="btn btn-secondary" @click="showCancelModal = false">
            Keep Subscription
          </button>
          <button class="btn btn-danger" @click="handleCancelSubscription">
            Cancel Subscription
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
