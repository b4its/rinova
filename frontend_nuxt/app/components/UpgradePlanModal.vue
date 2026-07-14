<script setup lang="ts">
import type { PlanType } from '~/stores/subscription'

const emit = defineEmits<{ close: [] }>()

const subscriptionStore = useSubscriptionStore()
const selectedPlan = ref<PlanType>('enterprise')
const isLoading = ref(false)

const plans = [
  {
    id: 'enterprise' as PlanType,
    name: 'Enterprise',
    price: 29,
    features: [
      'Up to 10 projects', 'Animation Editor', 'Custom CSS',
      'SEO Tools', 'Asset Marketplace access', 'Responsive Design Control', '5 team members'
    ]
  },
  {
    id: 'exclusive' as PlanType,
    name: 'Exclusive',
    price: 99,
    features: [
      'Unlimited projects', 'Everything in Enterprise', 'One-Click Publish',
      'Custom Domain', 'Analytics Dashboard', 'Priority Support', 'Unlimited team members'
    ],
    popular: true
  }
]

async function handleUpgrade() {
  isLoading.value = true
  try {
    await subscriptionStore.upgradePlan(selectedPlan.value)
    emit('close')
  } finally {
    isLoading.value = false
  }
}
</script>

<template>
  <div class="modal-overlay" @click.self="emit('close')">
    <div class="modal max-w-2xl">
      <div class="modal-header">
        <h3 class="text-lg font-semibold text-gray-900">Upgrade Your Plan</h3>
        <button class="absolute top-4 right-4 text-gray-400 hover:text-gray-600" @click="emit('close')">
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>

      <div class="modal-body">
        <div class="grid md:grid-cols-2 gap-4">
          <button
            v-for="plan in plans" :key="plan.id"
            class="relative p-4 rounded-xl border-2 text-left transition-all"
            :class="selectedPlan === plan.id ? 'border-primary-500 bg-primary-50' : 'border-gray-200 hover:border-gray-300'"
            @click="selectedPlan = plan.id"
          >
            <span v-if="plan.popular" class="absolute -top-3 left-1/2 -translate-x-1/2 badge bg-primary-100 text-primary-700">
              Most Popular
            </span>
            <h4 class="text-lg font-semibold text-gray-900">{{ plan.name }}</h4>
            <div class="mt-2 mb-4">
              <span class="text-3xl font-bold text-gray-900">${{ plan.price }}</span>
              <span class="text-gray-500">/month</span>
            </div>
            <ul class="space-y-2">
              <li v-for="(feature, index) in plan.features" :key="index" class="flex items-start text-sm text-gray-600">
                <svg class="w-5 h-5 text-green-500 mr-2 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
                </svg>
                {{ feature }}
              </li>
            </ul>
            <div v-if="selectedPlan === plan.id" class="absolute top-4 right-4 w-5 h-5 bg-primary-500 rounded-full flex items-center justify-center">
              <svg class="w-3 h-3 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M5 13l4 4L19 7" />
              </svg>
            </div>
          </button>
        </div>
      </div>

      <div class="modal-footer">
        <button class="btn btn-secondary" @click="emit('close')">Cancel</button>
        <button class="btn btn-primary" :disabled="isLoading" @click="handleUpgrade">
          <svg v-if="isLoading" class="animate-spin -ml-1 mr-2 h-4 w-4 text-white" fill="none" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
          </svg>
          {{ isLoading ? 'Processing...' : 'Proceed to Payment' }}
        </button>
      </div>
    </div>
  </div>
</template>
