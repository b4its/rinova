<script setup lang="ts">
definePageMeta({
  layout: 'default'
})

const router = useRouter()
const userStore = useUserStore()

const billingPeriod = ref<'monthly' | 'yearly'>('monthly')

const plans = [
  {
    id: 'freemium',
    name: 'Free',
    price: { monthly: 0, yearly: 0 },
    description: 'Perfect for getting started with website building',
    features: [
      'Up to 2 projects',
      'Basic drag & drop editor',
      'Free templates',
      'Community support',
      'Export to ZIP'
    ],
    cta: 'Get Started',
    current: false
  },
  {
    id: 'enterprise',
    name: 'Enterprise',
    price: { monthly: 29, yearly: 290 },
    description: 'For professionals and growing businesses',
    features: [
      'Up to 10 projects',
      'Animation Editor',
      'Custom CSS',
      'SEO Tools',
      'Asset Marketplace access',
      'Responsive Design Control',
      '5 team members',
      'Priority support'
    ],
    cta: 'Start Free Trial',
    popular: true
  },
  {
    id: 'exclusive',
    name: 'Exclusive',
    price: { monthly: 99, yearly: 990 },
    description: 'For agencies and large teams',
    features: [
      'Unlimited projects',
      'Everything in Enterprise',
      'One-Click Publish',
      'Custom Domain',
      'Analytics Dashboard',
      'Unlimited team members',
      'Dedicated support',
      'SLA guarantee'
    ],
    cta: 'Start Free Trial'
  }
]

function handleSelectPlan(planId: string) {
  if (userStore.isAuthenticated) {
    router.push(`/panel/subscription?plan=${planId}`)
  } else {
    router.push(`/register?plan=${planId}`)
  }
}

const yearlySavings = computed(() => {
  return Math.round((1 - (290 / (29 * 12))) * 100)
})
</script>

<template>
  <div class="min-h-screen bg-gradient-to-b from-background to-background dark:from-gray-950 dark:to-gray-900">
    <!-- Header -->
    <header class="pt-12 pb-16">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
        <NuxtLink to="/" class="inline-block text-2xl font-bold text-primary-600 mb-8">
          Rinova
        </NuxtLink>
        
        <h1 class="text-4xl md:text-5xl font-bold text-foreground mb-4">
          Choose Your Plan
        </h1>
        <p class="text-xl text-muted-foreground max-w-2xl mx-auto">
          Start building professional websites today. Upgrade anytime as your needs grow.
        </p>
        
        <!-- Billing Toggle -->
        <div class="flex items-center justify-center gap-4 mt-8">
          <span
            class="text-sm font-medium"
            :class="billingPeriod === 'monthly' ? 'text-foreground' : 'text-muted-foreground'"
          >
            Monthly
          </span>
          <button
            class="relative w-14 h-8 bg-primary-600 rounded-full p-1 transition-colors"
            @click="billingPeriod = billingPeriod === 'monthly' ? 'yearly' : 'monthly'"
          >
            <div
              class="w-6 h-6 bg-white rounded-full shadow transition-transform"
              :class="billingPeriod === 'yearly' ? 'translate-x-6' : ''"
            />
          </button>
          <span
            class="text-sm font-medium"
            :class="billingPeriod === 'yearly' ? 'text-foreground' : 'text-muted-foreground'"
          >
            Yearly
            <span class="ml-1 badge badge-success">Save {{ yearlySavings }}%</span>
          </span>
        </div>
      </div>
    </header>
    
    <!-- Pricing Cards -->
    <section class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 pb-16">
      <div class="grid md:grid-cols-3 gap-8">
        <div
          v-for="plan in plans"
          :key="plan.id"
          class="relative rounded-2xl bg-card border-2 p-8"
          :class="plan.popular ? 'border-primary-500 shadow-xl' : 'border-border'"
        >
          <!-- Popular Badge -->
          <span
            v-if="plan.popular"
            class="absolute -top-4 left-1/2 -translate-x-1/2 badge badge-premium px-4 py-1"
          >
            Most Popular
          </span>
          
          <!-- Plan Name -->
          <h3 class="text-xl font-bold text-foreground">{{ plan.name }}</h3>
          
          <!-- Price -->
          <div class="mt-4 mb-6">
            <span class="text-4xl font-bold text-foreground">
              ${{ plan.price[billingPeriod] }}
            </span>
            <span v-if="plan.price[billingPeriod] > 0" class="text-muted-foreground">
              /{{ billingPeriod === 'monthly' ? 'mo' : 'yr' }}
            </span>
            <span v-else class="text-muted-foreground">/ forever</span>
          </div>
          
          <!-- Description -->
          <p class="text-muted-foreground mb-6">{{ plan.description }}</p>
          
          <!-- CTA -->
          <button
            class="btn w-full mb-8"
            :class="plan.popular ? 'btn-primary' : 'btn-secondary'"
            @click="handleSelectPlan(plan.id)"
          >
            {{ plan.cta }}
          </button>
          
          <!-- Features -->
          <ul class="space-y-3">
            <li
              v-for="(feature, index) in plan.features"
              :key="index"
              class="flex items-start text-sm text-muted-foreground"
            >
              <svg class="w-5 h-5 text-green-500 mr-2 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
              </svg>
              {{ feature }}
            </li>
          </ul>
        </div>
      </div>
    </section>
    
    <!-- Feature Comparison -->
    <section class="bg-muted py-16">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <h2 class="text-3xl font-bold text-foreground text-center mb-12">
          Compare Features
        </h2>
        
        <div class="overflow-x-auto">
          <table class="w-full">
            <thead>
              <tr class="border-b border-border">
                <th class="py-4 px-6 text-left text-sm font-medium text-muted-foreground">Feature</th>
                <th class="py-4 px-6 text-center text-sm font-medium text-foreground">Free</th>
                <th class="py-4 px-6 text-center text-sm font-medium text-primary-600">Enterprise</th>
                <th class="py-4 px-6 text-center text-sm font-medium text-foreground">Exclusive</th>
              </tr>
            </thead>
            <tbody class="divide-y divide-border">
              <tr>
                <td class="py-4 px-6 text-sm text-muted-foreground">Projects</td>
                <td class="py-4 px-6 text-center text-sm text-foreground">2</td>
                <td class="py-4 px-6 text-center text-sm text-foreground">10</td>
                <td class="py-4 px-6 text-center text-sm text-foreground">Unlimited</td>
              </tr>
              <tr>
                <td class="py-4 px-6 text-sm text-muted-foreground">Animation Editor</td>
                <td class="py-4 px-6 text-center">
                  <svg class="w-5 h-5 text-muted-foreground mx-auto" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                  </svg>
                </td>
                <td class="py-4 px-6 text-center">
                  <svg class="w-5 h-5 text-green-500 mx-auto" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
                  </svg>
                </td>
                <td class="py-4 px-6 text-center">
                  <svg class="w-5 h-5 text-green-500 mx-auto" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
                  </svg>
                </td>
              </tr>
              <tr>
                <td class="py-4 px-6 text-sm text-muted-foreground">One-Click Publish</td>
                <td class="py-4 px-6 text-center">
                  <svg class="w-5 h-5 text-muted-foreground mx-auto" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                  </svg>
                </td>
                <td class="py-4 px-6 text-center">
                  <svg class="w-5 h-5 text-muted-foreground mx-auto" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                  </svg>
                </td>
                <td class="py-4 px-6 text-center">
                  <svg class="w-5 h-5 text-green-500 mx-auto" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
                  </svg>
                </td>
              </tr>
              <tr>
                <td class="py-4 px-6 text-sm text-muted-foreground">Custom Domain</td>
                <td class="py-4 px-6 text-center">
                  <svg class="w-5 h-5 text-gray-300 mx-auto" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                  </svg>
                </td>
                <td class="py-4 px-6 text-center">
                  <svg class="w-5 h-5 text-gray-300 mx-auto" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                  </svg>
                </td>
                <td class="py-4 px-6 text-center">
                  <svg class="w-5 h-5 text-green-500 mx-auto" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
                  </svg>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </section>
    
    <!-- FAQ Section -->
    <section class="py-16">
      <div class="max-w-3xl mx-auto px-4 sm:px-6 lg:px-8">
        <h2 class="text-3xl font-bold text-gray-900 text-center mb-12">
          Frequently Asked Questions
        </h2>
        
        <div class="space-y-6">
          <div class="card p-6">
            <h3 class="font-semibold text-gray-900 mb-2">
              Can I switch plans later?
            </h3>
            <p class="text-gray-600">
              Yes, you can upgrade or downgrade your plan at any time. When upgrading, you'll have immediate access to new features. When downgrading, the change takes effect at the end of your billing period.
            </p>
          </div>
          
          <div class="card p-6">
            <h3 class="font-semibold text-gray-900 mb-2">
              Is there a free trial?
            </h3>
            <p class="text-gray-600">
              Enterprise and Exclusive plans come with a 14-day free trial. No credit card required to start.
            </p>
          </div>
          
          <div class="card p-6">
            <h3 class="font-semibold text-gray-900 mb-2">
              What happens to my projects if I downgrade?
            </h3>
            <p class="text-gray-600">
              Your existing projects will remain accessible, but you won't be able to create new ones beyond your plan's limit. You'll also lose access to premium features on existing projects.
            </p>
          </div>
          
          <div class="card p-6">
            <h3 class="font-semibold text-gray-900 mb-2">
              Do you offer refunds?
            </h3>
            <p class="text-gray-600">
              Yes, we offer a 30-day money-back guarantee on all paid plans. If you're not satisfied, contact our support team for a full refund.
            </p>
          </div>
        </div>
      </div>
    </section>
    
    <!-- CTA Section -->
    <section class="py-16 bg-primary-600 dark:bg-primary-800">
      <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
        <h2 class="text-3xl font-bold text-white mb-4">
          Ready to start building?
        </h2>
        <p class="text-primary-100 mb-8">
          Join thousands of users creating amazing websites with Rinova
        </p>
        <NuxtLink
          to="/register"
          class="btn bg-white text-primary-600 hover:bg-primary-50 btn-lg"
        >
          Get Started for Free
        </NuxtLink>
      </div>
    </section>
  </div>
</template>
