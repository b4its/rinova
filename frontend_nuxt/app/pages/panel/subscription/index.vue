<script setup lang="ts">
definePageMeta({ layout: 'panel', sidebarSection: 'settings' })

const subscriptionStore = useSubscriptionStore()

onMounted(() => subscriptionStore.fetchSubscription())

const plans = [
  { id: 'freemium' as const, name: 'Free', price: 0, desc: 'Get started for free', features: ['1 Project', 'Basic Templates', 'Community Support'] },
  { id: 'enterprise' as const, name: 'Enterprise', price: 29, desc: 'For professionals', features: ['Unlimited Projects', 'All Templates', 'Animation Editor', 'Custom CSS', 'Priority Support'] },
  { id: 'exclusive' as const, name: 'Exclusive', price: 99, desc: 'For agencies & teams', features: ['Everything in Enterprise', 'Team Collaboration', 'White Label', 'Dedicated Manager', 'API Access'] }
]

type PlanId = 'freemium' | 'enterprise' | 'exclusive'

const isLoading = ref(false)
const showCancelModal = ref(false)

// --- Payment simulation state ---
const showPaymentModal = ref(false)
const pendingPlan = ref<{ id: PlanId; name: string; price: number } | null>(null)
const paymentProcessing = ref(false)
const paymentSuccess = ref(false)
const paymentError = ref<string | null>(null)
const card = ref({ name: '', number: '', expiry: '', cvc: '' })

const currentPlanIndex = computed(() => plans.findIndex(p => p.id === subscriptionStore.currentPlan))

// Downgrade to a cheaper/free plan doesn't require payment; upgrades do.
function handleUpgrade(planId: PlanId) {
  if (planId === subscriptionStore.currentPlan) return
  const plan = plans.find(p => p.id === planId)!
  const isUpgrade = plans.findIndex(p => p.id === planId) > currentPlanIndex.value

  if (!isUpgrade || plan.price === 0) {
    // Free plan / downgrade — apply immediately, no payment needed.
    applyPlanChange(planId)
    return
  }

  // Paid upgrade — open the simulated payment modal.
  pendingPlan.value = { id: plan.id, name: plan.name, price: plan.price }
  paymentSuccess.value = false
  paymentError.value = null
  card.value = { name: '', number: '', expiry: '', cvc: '' }
  showPaymentModal.value = true
}

async function applyPlanChange(planId: PlanId) {
  isLoading.value = true
  try {
    await subscriptionStore.upgradePlan(planId)
  } finally {
    isLoading.value = false
  }
}

const canPay = computed(() => {
  const digits = card.value.number.replace(/\s/g, '')
  return (
    card.value.name.trim().length > 0 &&
    digits.length >= 12 &&
    /^\d{2}\/\d{2}$/.test(card.value.expiry) &&
    /^\d{3,4}$/.test(card.value.cvc)
  )
})

// Format the card number as the user types: groups of 4 digits.
function onCardNumberInput(e: Event) {
  const raw = (e.target as HTMLInputElement).value.replace(/\D/g, '').slice(0, 16)
  card.value.number = raw.replace(/(.{4})/g, '$1 ').trim()
}

function onExpiryInput(e: Event) {
  const raw = (e.target as HTMLInputElement).value.replace(/\D/g, '').slice(0, 4)
  card.value.expiry = raw.length > 2 ? `${raw.slice(0, 2)}/${raw.slice(2)}` : raw
}

async function confirmPayment() {
  if (!pendingPlan.value || !canPay.value) return
  paymentProcessing.value = true
  paymentError.value = null
  try {
    // Simulate a payment gateway round-trip.
    await new Promise(r => setTimeout(r, 1600))
    // Apply the real plan change on the backend after "payment succeeds".
    await subscriptionStore.upgradePlan(pendingPlan.value.id)
    paymentSuccess.value = true
    // Auto-close shortly after showing the success state.
    setTimeout(() => {
      showPaymentModal.value = false
      pendingPlan.value = null
    }, 1500)
  } catch (e) {
    paymentError.value = e instanceof Error ? e.message : 'Payment failed. Please try again.'
  } finally {
    paymentProcessing.value = false
  }
}

function closePaymentModal() {
  if (paymentProcessing.value) return
  showPaymentModal.value = false
  pendingPlan.value = null
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

    <!-- Simulated Payment Modal -->
    <div v-if="showPaymentModal && pendingPlan" class="modal-overlay" @click.self="closePaymentModal">
      <div class="modal">
        <div class="modal-header">
          <h3 class="text-lg font-semibold">Complete Payment</h3>
          <button v-if="!paymentProcessing" class="text-muted-foreground hover:text-foreground" @click="closePaymentModal">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>

        <!-- Success state -->
        <div v-if="paymentSuccess" class="modal-body text-center py-8">
          <div class="mx-auto w-16 h-16 rounded-full bg-green-100 dark:bg-green-900/40 flex items-center justify-center mb-4">
            <svg class="w-8 h-8 text-green-600 dark:text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
            </svg>
          </div>
          <h4 class="text-lg font-semibold">Payment Successful</h4>
          <p class="text-sm text-muted-foreground mt-1">
            You're now on the {{ pendingPlan.name }} plan.
          </p>
        </div>

        <!-- Payment form -->
        <div v-else class="modal-body space-y-4">
          <div class="rounded-lg bg-muted/60 p-3 flex items-center justify-between">
            <span class="text-sm text-muted-foreground">{{ pendingPlan.name }} plan</span>
            <span class="text-lg font-bold">${{ pendingPlan.price }}<span class="text-xs font-normal text-muted-foreground">/mo</span></span>
          </div>

          <div class="rounded-md border border-amber-200 bg-amber-50 dark:border-amber-900 dark:bg-amber-950/40 px-3 py-2 text-xs text-amber-700 dark:text-amber-400">
            Simulation mode — no real charge. Use any test card details (e.g. 4242 4242 4242 4242).
          </div>

          <div v-if="paymentError" class="rounded-md border border-red-200 bg-red-50 dark:border-red-900 dark:bg-red-950/40 px-3 py-2 text-sm text-red-700 dark:text-red-400">
            {{ paymentError }}
          </div>

          <div class="space-y-2">
            <label class="label">Cardholder Name</label>
            <input v-model="card.name" type="text" class="input" placeholder="John Doe" :disabled="paymentProcessing" />
          </div>
          <div class="space-y-2">
            <label class="label">Card Number</label>
            <input :value="card.number" @input="onCardNumberInput" type="text" inputmode="numeric" class="input" placeholder="4242 4242 4242 4242" :disabled="paymentProcessing" />
          </div>
          <div class="grid grid-cols-2 gap-3">
            <div class="space-y-2">
              <label class="label">Expiry</label>
              <input :value="card.expiry" @input="onExpiryInput" type="text" inputmode="numeric" class="input" placeholder="MM/YY" :disabled="paymentProcessing" />
            </div>
            <div class="space-y-2">
              <label class="label">CVC</label>
              <input v-model="card.cvc" type="text" inputmode="numeric" maxlength="4" class="input" placeholder="123" :disabled="paymentProcessing" />
            </div>
          </div>
        </div>

        <div v-if="!paymentSuccess" class="modal-footer">
          <button class="btn btn-secondary btn-sm" :disabled="paymentProcessing" @click="closePaymentModal">Cancel</button>
          <button class="btn btn-primary btn-sm" :disabled="!canPay || paymentProcessing" @click="confirmPayment">
            <svg v-if="paymentProcessing" class="animate-spin -ml-1 mr-2 h-4 w-4" fill="none" viewBox="0 0 24 24">
              <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
              <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
            </svg>
            {{ paymentProcessing ? 'Processing...' : `Pay $${pendingPlan.price}` }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
