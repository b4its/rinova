<script setup lang="ts">
definePageMeta({
  layout: 'default'
})

const mobileMenuOpen = ref(false)
const billingPeriod = ref<'monthly' | 'yearly'>('monthly')
const themeStore = useThemeStore()

const navigation = [
  { name: 'Features', href: '#features' },
  { name: 'How It Works', href: '#how-it-works' },
  { name: 'Pricing', href: '#pricing' },
  { name: 'FAQ', href: '#faq' },
]

// Landing content is served from a Redis-cached Nitro route (/api/landing).
// Static marketing data only; flushed + reseeded on each server start.
interface LandingContent {
  features: { title: string; description: string; icon: string }[]
  steps: { step: string; title: string; description: string; color: string }[]
  plans: {
    id: string; name: string; price: { monthly: number; yearly: number }
    period: string; description: string; features: string[]; cta: string; highlight: boolean
  }[]
  faqs: { question: string; answer: string }[]
  stats: { label: string; value: number; suffix: string }[]
}

const { data: landing } = await useFetch<LandingContent>('/api/landing')

const features = computed(() => landing.value?.features ?? [])
const steps = computed(() => landing.value?.steps ?? [])
const plans = computed(() => landing.value?.plans ?? [])
const faqs = computed(() => landing.value?.faqs ?? [])
const stats = computed(() => landing.value?.stats ?? [])

// Map icon names (from the cached content) to Heroicon SVG paths.
const iconPaths: Record<string, string> = {
  cursor: 'M15 15l-2 5L9 9l11 4-5 2zm0 0l5 5M7.188 2.239l.777 2.897M5.136 7.965l-2.898-.777M13.95 4.05l-2.122 2.122m-5.657 5.656l-2.12 2.122',
  sparkles: 'M5 3v4M3 5h4M6 17v4m-2-2h4m5-16l2.286 6.857L21 12l-5.714 2.143L13 21l-2.286-6.857L5 12l5.714-2.143L13 3z',
  devices: 'M9 17.25v1.007a3 3 0 01-.879 2.122L7.5 21h9l-.621-.621A3 3 0 0115 18.257V17.25m6-12V15a2.25 2.25 0 01-2.25 2.25H5.25A2.25 2.25 0 013 15V5.25A2.25 2.25 0 015.25 3h13.5A2.25 2.25 0 0121 5.25z',
  code: 'M17.25 6.75L22.5 12l-5.25 5.25m-10.5 0L1.5 12l5.25-5.25m7.5-3l-4.5 16.5',
  globe: 'M12 21a9.004 9.004 0 008.716-6.747M12 21a9.004 9.004 0 01-8.716-6.747M12 21c2.485 0 4.5-4.03 4.5-9S14.485 3 12 3m0 18c-2.485 0-4.5-4.03-4.5-9S9.515 3 12 3m0 0a8.997 8.997 0 017.843 4.582M12 3a8.997 8.997 0 00-7.843 4.582m15.686 0A11.953 11.953 0 0112 10.5c-2.998 0-5.74-1.1-7.843-2.918m15.686 0A8.959 8.959 0 0121 12c0 .778-.099 1.533-.284 2.253',
  rocket: 'M15.59 14.37a6 6 0 01-5.84 7.38v-4.8m5.84-2.58a14.98 14.98 0 006.16-12.12A14.98 14.98 0 009.631 8.41m5.96 5.96a14.926 14.926 0 01-5.841 2.58m-.119-8.54a6 6 0 00-7.381 5.84h4.8m2.581-5.84a14.927 14.927 0 00-2.58 5.84m2.699 2.7c-.103.021-.207.041-.311.06a15.09 15.09 0 01-2.448-2.448 14.9 14.9 0 01.06-.312m-2.24 2.39a4.493 4.493 0 00-1.757 4.306 4.493 4.493 0 004.306-1.758M16.5 9a1.5 1.5 0 11-3 0 1.5 1.5 0 013 0z',
}

const openFaq = ref<number | null>(null)

// Animated count-up for the stats bar (interactive polish).
const displayStats = ref<number[]>([])
watch(stats, (s) => { displayStats.value = s.map(() => 0) }, { immediate: true })

function animateStats() {
  const targets = stats.value
  const duration = 1400
  const start = performance.now()
  const tick = (now: number) => {
    const p = Math.min((now - start) / duration, 1)
    const eased = 1 - Math.pow(1 - p, 3)
    displayStats.value = targets.map(t => Math.round(t.value * eased))
    if (p < 1) requestAnimationFrame(tick)
  }
  requestAnimationFrame(tick)
}

onMounted(async () => {
  await nextTick()
  const tryInit = () => {
    if (import.meta.client && typeof (window as any)?.AOS?.init === 'function') {
      (window as any).AOS.init({
        duration: 800,
        easing: 'ease-in-out',
        once: true,
        offset: 100,
      })
      return true
    }
    return false
  }
  if (!tryInit()) {
    const timer = setInterval(() => {
      if (tryInit()) clearInterval(timer)
    }, 100)
    setTimeout(() => clearInterval(timer), 5000)
  }

  // Trigger the count-up when the stats bar scrolls into view.
  if (import.meta.client && statsSection.value) {
    const obs = new IntersectionObserver((entries) => {
      if (entries.some(e => e.isIntersecting)) {
        animateStats()
        obs.disconnect()
      }
    }, { threshold: 0.3 })
    obs.observe(statsSection.value)
  }
})

const statsSection = ref<HTMLElement | null>(null)

function toggleFaq(index: number) {
  openFaq.value = openFaq.value === index ? null : index
}

const yearlySavings = computed(() => Math.round((1 - (290 / (29 * 12))) * 100))
</script>

<template>
  <div class="min-h-screen bg-background text-foreground transition-colors scroll-smooth">
    <!-- Navigation -->
    <nav class="fixed top-0 left-0 right-0 z-50 bg-background/80 backdrop-blur-xl border-b border-border">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="flex items-center justify-between h-16 lg:h-20">
          <NuxtLink to="/" class="flex items-center gap-2">
            <div class="w-8 h-8 bg-gradient-to-br from-blue-500 to-emerald-500 rounded-lg flex items-center justify-center">
              <svg class="w-5 h-5 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
              </svg>
            </div>
            <span class="text-xl font-bold text-foreground">Rinova</span>
          </NuxtLink>

          <div class="hidden lg:flex items-center gap-8">
            <a v-for="item in navigation" :key="item.name" :href="item.href" class="text-sm font-medium text-muted-foreground hover:text-foreground transition-colors">
              {{ item.name }}
            </a>
          </div>

          <div class="hidden lg:flex items-center gap-4">
            <NuxtLink to="/login" class="btn btn-secondary btn-sm">Sign In</NuxtLink>
            <NuxtLink to="/register" class="btn btn-primary btn-sm">Get Started</NuxtLink>
          </div>

          <button class="lg:hidden p-2 rounded-lg hover:bg-accent text-muted-foreground" @click="mobileMenuOpen = !mobileMenuOpen">
            <svg v-if="!mobileMenuOpen" class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
            </svg>
            <svg v-else class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>
      </div>

      <div v-if="mobileMenuOpen" class="lg:hidden border-t border-border bg-background">
        <div class="px-4 py-4 space-y-3">
          <a v-for="item in navigation" :key="item.name" :href="item.href" class="block px-3 py-2 text-sm font-medium text-muted-foreground hover:text-foreground rounded-lg hover:bg-accent" @click="mobileMenuOpen = false">
            {{ item.name }}
          </a>
          <hr class="border-border">
          <NuxtLink to="/login" class="block w-full text-center btn btn-secondary btn-sm" @click="mobileMenuOpen = false">Sign In</NuxtLink>
          <NuxtLink to="/register" class="block w-full text-center btn btn-primary btn-sm" @click="mobileMenuOpen = false">Get Started</NuxtLink>
        </div>
      </div>
    </nav>

    <!-- Hero -->
    <section data-aos="fade-up" class="relative pt-32 pb-20 lg:pt-40 lg:pb-28 overflow-hidden">
      <div class="absolute inset-0 -z-10">
        <div class="absolute top-0 right-0 w-[600px] h-[600px] bg-gradient-to-bl from-blue-100/50 via-emerald-50/30 to-transparent dark:from-blue-900/20 dark:via-emerald-900/10 rounded-full blur-3xl" />
        <div class="absolute bottom-0 left-0 w-[500px] h-[500px] bg-gradient-to-tr from-emerald-100/40 via-blue-50/20 to-transparent dark:from-emerald-900/20 dark:via-blue-900/10 rounded-full blur-3xl" />
      </div>
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="grid lg:grid-cols-2 gap-12 lg:gap-16 items-center">
          <div class="text-center lg:text-left">
            <div class="inline-flex items-center gap-2 px-4 py-2 bg-primary-50 dark:bg-primary-900/40 border border-primary-100 dark:border-primary-800 rounded-full text-sm font-medium text-primary-700 dark:text-primary-300 mb-6">
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
              </svg>
              The Future of Website Building
            </div>
            <h1 class="text-4xl sm:text-5xl lg:text-6xl font-bold text-foreground leading-tight mb-6">
              Build Beautiful
              <span class="text-transparent bg-clip-text bg-gradient-to-r from-blue-600 to-emerald-500 dark:from-blue-400 dark:to-emerald-400">Websites</span>
              <br>Without Code
            </h1>
            <p class="text-lg sm:text-xl text-muted-foreground max-w-xl mx-auto lg:mx-0 mb-8 leading-relaxed">
              Rinova is the professional website builder that empowers you to create stunning, responsive websites with an intuitive drag-and-drop interface. No coding skills required.
            </p>
            <div class="flex flex-col sm:flex-row gap-4 justify-center lg:justify-start">
              <NuxtLink to="/register" class="btn btn-primary btn-lg group">
                Start Building Free
                <svg class="w-5 h-5 ml-2 group-hover:translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7l5 5m0 0l-5 5m5-5H6" />
                </svg>
              </NuxtLink>
              <a href="#how-it-works" class="btn btn-secondary btn-lg">
                <svg class="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14.752 11.168l-3.197-2.132A1 1 0 0010 9.87v4.263a1 1 0 001.555.832l3.197-2.132a1 1 0 000-1.664z" />
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                </svg>
                See How It Works
              </a>
            </div>
            <div class="flex items-center gap-6 mt-8 justify-center lg:justify-start text-sm text-muted-foreground">
              <span class="flex items-center gap-1">
                <svg class="w-4 h-4 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" /></svg>
                No credit card
              </span>
              <span class="flex items-center gap-1">
                <svg class="w-4 h-4 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" /></svg>
                14-day free trial
              </span>
              <span class="flex items-center gap-1">
                <svg class="w-4 h-4 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" /></svg>
                Cancel anytime
              </span>
            </div>
          </div>
          <div class="relative lg:block">
            <div class="relative w-full aspect-[4/3] rounded-2xl bg-gradient-to-br from-primary-100 via-purple-50 to-blue-50 dark:from-primary-900/30 dark:via-purple-900/20 dark:to-blue-900/20 border border-border/50 shadow-2xl overflow-hidden">
              <div class="absolute inset-0 p-6">
                <div class="flex items-center gap-2 mb-4 p-3 bg-card/90 rounded-xl shadow-sm border border-border">
                  <div class="w-3 h-3 rounded-full bg-red-400" />
                  <div class="w-3 h-3 rounded-full bg-yellow-400" />
                  <div class="w-3 h-3 rounded-full bg-green-400" />
                  <div class="flex-1" />
                  <div class="px-3 py-1 bg-primary-500 text-white text-xs rounded-md font-medium">Publish</div>
                </div>
                <div class="flex gap-3 h-[calc(100%-3.5rem)]">
                  <div class="w-20 bg-card/80 rounded-xl p-2 space-y-2 shadow-sm border border-border">
                    <div v-for="i in 4" :key="i" class="aspect-square bg-muted rounded-lg flex items-center justify-center">
                      <svg class="w-5 h-5 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zm10 0a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zm10 0a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z" />
                      </svg>
                    </div>
                  </div>
                  <div class="flex-1 bg-card/90 rounded-xl p-4 shadow-sm border border-border space-y-3 overflow-hidden">
                    <div class="h-3 w-2/3 bg-muted rounded-full" />
                    <div class="h-3 w-1/2 bg-muted rounded-full" />
                    <div class="grid grid-cols-3 gap-2 mt-4">
                      <div v-for="i in 3" :key="i" class="aspect-[4/3] bg-gradient-to-br from-blue-100 to-emerald-50 dark:from-blue-900/40 dark:to-emerald-900/40 rounded-lg flex items-center justify-center">
                        <svg class="w-8 h-8 text-blue-300 dark:text-blue-500" fill="currentColor" viewBox="0 0 24 24">
                          <path d="M19.5 6c-1.31 0-2.37 1.01-2.48 2.3-1.71.32-3.22 1.27-4.25 2.6-.72-.69-1.68-1.15-2.77-1.15-1.31 0-2.37 1.01-2.48 2.3-1.71.32-3.22 1.27-4.25 2.6V6H3v12h1.5v-1.17c.01-1.66 1.35-3 3.01-3 1.66 0 3 1.34 3 3v1.17h1.5v-1.17c0-1.66 1.35-3 3.01-3 1.66 0 3 1.34 3 3v1.17H21V6h-1.5z" />
                        </svg>
                      </div>
                    </div>
                    <div class="h-3 w-3/4 bg-muted rounded-full mt-4" />
                    <div class="h-3 w-1/3 bg-muted rounded-full" />
                  </div>
                </div>
              </div>
              <div class="absolute -top-4 -right-4 w-16 h-16 bg-gradient-to-br from-emerald-400 to-green-500 rounded-2xl shadow-lg animate-float flex items-center justify-center">
                <svg class="w-5 h-5 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" /></svg>
              </div>
              <div class="absolute -bottom-4 -left-4 w-16 h-16 bg-gradient-to-tr from-blue-400 to-cyan-400 rounded-xl shadow-lg animate-float-delay flex items-center justify-center">
                <svg class="w-5 h-5 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" /></svg>
              </div>
            </div>

            
          </div>
        </div>
      </div>
    </section>

    <!-- Stats Bar -->
    <section data-aos="fade-up" class="py-12 bg-muted/50 border-y border-border">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="grid grid-cols-2 md:grid-cols-4 gap-8 text-center">
          <div v-for="(stat, si) in stats" :key="stat.label" data-aos="fade-up" :data-aos-delay="si * 100">
            <div class="text-3xl lg:text-4xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-blue-600 to-emerald-500 dark:from-blue-400 dark:to-emerald-400">{{ displayStats[si] }}{{ stat.suffix }}</div>
            <div class="text-sm text-muted-foreground mt-1">{{ stat.label }}</div>
          </div>
        </div>
      </div>
    </section>

    <!-- Features -->
    <section id="features" data-aos="fade-up" class="py-20 lg:py-28">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="text-center mb-16">
          <h2 class="section-title mb-4 dark:text-white">Everything You Need to Build</h2>
          <p class="section-subtitle text-muted-foreground">Powerful features that make website building effortless and enjoyable.</p>
        </div>
        <div class="grid sm:grid-cols-2 lg:grid-cols-3 gap-6">
          <div v-for="(feature, fi) in features" :key="feature.title" data-aos="fade-up" :data-aos-delay="fi * 100" class="group p-6 rounded-2xl border border-border hover:border-primary-200 dark:hover:border-primary-800 hover:shadow-lg hover:shadow-primary-500/5 dark:bg-gray-900/50 transition-all duration-300">
            <div class="w-12 h-12 bg-primary-50 dark:bg-primary-900/40 rounded-xl flex items-center justify-center mb-4 group-hover:scale-110 transition-transform duration-300">
              <svg class="w-6 h-6 text-primary-600 dark:text-primary-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" :d="iconPaths[feature.icon] ?? iconPaths.cursor" />
              </svg>
            </div>
            <h3 class="text-lg font-semibold text-foreground mb-2">{{ feature.title }}</h3>
            <p class="text-sm text-muted-foreground leading-relaxed">{{ feature.description }}</p>
          </div>
        </div>
      </div>
    </section>

    <!-- How It Works -->
    <section id="how-it-works" data-aos="fade-up" class="py-20 lg:py-28 bg-muted/50">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="text-center mb-16">
          <h2 class="section-title mb-4 dark:text-white">How It Works</h2>
          <p class="section-subtitle text-muted-foreground">Get your website online in four simple steps.</p>
        </div>
        <div class="grid md:grid-cols-4 gap-6">
          <div v-for="(step, index) in steps" :key="step.step" data-aos="fade-up" :data-aos-delay="index * 150" class="relative">
            <div v-if="index < steps.length - 1" class="hidden md:block absolute top-8 left-[60%] w-[80%] h-0.5 border-t-2 border-dashed border-border" />
            <div class="text-center">
              <div :class="`w-16 h-16 bg-blue-500 rounded-2xl flex items-center justify-center mx-auto mb-4 shadow-lg`">
                <span class="text-2xl font-bold text-white">{{ step.step }}</span>
              </div>
              <h3 class="text-lg font-semibold text-foreground mb-2">{{ step.title }}</h3>
              <p class="text-sm text-muted-foreground leading-relaxed">{{ step.description }}</p>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- Pricing -->
    <section id="pricing" data-aos="fade-up" class="py-20 lg:py-28">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="text-center mb-16">
          <h2 class="section-title mb-4 dark:text-white">Choose Your Plan</h2>
          <p class="section-subtitle text-muted-foreground">Start free, upgrade when you need more. All plans include a 14-day free trial.</p>
          <div class="flex items-center justify-center gap-4 mt-8">
            <span class="text-sm font-medium" :class="billingPeriod === 'monthly' ? 'text-foreground' : 'text-muted-foreground'">Monthly</span>
            <button class="relative w-14 h-8 bg-primary-600 rounded-full p-1 transition-colors cursor-pointer" @click="billingPeriod = billingPeriod === 'monthly' ? 'yearly' : 'monthly'">
              <div class="w-6 h-6 bg-white rounded-full shadow transition-transform duration-300" :class="billingPeriod === 'yearly' ? 'translate-x-6' : ''" />
            </button>
            <span class="text-sm font-medium" :class="billingPeriod === 'yearly' ? 'text-foreground' : 'text-muted-foreground'">
              Yearly
              <span class="ml-1.5 inline-flex items-center rounded-full bg-green-100 dark:bg-green-900/50 px-2.5 py-0.5 text-xs font-medium text-green-700 dark:text-green-300">
                Save up to {{ yearlySavings }}%
              </span>
            </span>
          </div>
        </div>
        <div class="grid md:grid-cols-3 gap-6 lg:gap-8 max-w-5xl mx-auto">
          <div v-for="(plan, pi) in plans" :key="plan.id" data-aos="fade-up" :data-aos-delay="pi * 150"
            class="relative flex flex-col rounded-2xl border-2 p-6 sm:p-8 transition-all duration-300 bg-card"
            :class="plan.highlight ? 'border-primary-500 shadow-xl shadow-primary-500/10 scale-105' : 'border-border hover:border-primary-200'">
            <div v-if="plan.highlight" class="absolute -top-4 left-1/2 -translate-x-1/2">
              <span class="inline-flex items-center rounded-full bg-primary-600 px-4 py-1 text-xs font-semibold text-white shadow-lg">Most Popular</span>
            </div>
            <div class="mb-6">
              <h3 class="text-xl font-bold text-foreground">{{ plan.name }}</h3>
              <p class="mt-2 text-sm text-muted-foreground">{{ plan.description }}</p>
            </div>
            <div class="mb-6">
              <div class="flex items-baseline gap-1">
                <span class="text-4xl font-bold text-foreground">${{ plan.price[billingPeriod] }}</span>
                <span v-if="plan.price[billingPeriod] > 0" class="text-muted-foreground text-sm">/month</span>
                <span v-else class="text-muted-foreground text-sm">/forever</span>
              </div>
              <p v-if="plan.price[billingPeriod] > 0 && billingPeriod === 'yearly'" class="text-xs text-muted-foreground mt-1">${{ plan.price.yearly }} billed annually</p>
            </div>
            <NuxtLink :to="plan.price.monthly === 0 ? '/register' : `/register?plan=${plan.id}`" class="btn w-full mb-6" :class="plan.highlight ? 'btn-primary' : 'btn-secondary'">{{ plan.cta }}</NuxtLink>
            <ul class="space-y-3 flex-1">
              <li v-for="feature in plan.features" :key="feature" class="flex items-start text-sm text-muted-foreground">
                <svg class="w-5 h-5 text-primary-500 dark:text-primary-400 mr-3 flex-shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" /></svg>
                {{ feature }}
              </li>
            </ul>
          </div>
        </div>
        <p class="text-center text-sm text-muted-foreground mt-8">All paid plans include a 14-day free trial. No credit card required.</p>
      </div>
    </section>

    <!-- FAQ -->
    <section id="faq" data-aos="fade-up" class="py-20 lg:py-28 bg-muted/50">
      <div class="max-w-3xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="text-center mb-16">
          <h2 class="section-title mb-4 dark:text-white">Frequently Asked Questions</h2>
          <p class="section-subtitle dark:text-gray-400">Everything you need to know about Rinova.</p>
        </div>
        <div class="space-y-3">
          <div v-for="(faq, index) in faqs" :key="index"
            class="bg-card rounded-2xl border border-border overflow-hidden transition-all duration-300"
            :class="{ 'shadow-md dark:shadow-gray-900/50': openFaq === index }">
            <button class="w-full flex items-center justify-between px-4 sm:px-6 py-4 sm:py-5 text-left" @click="toggleFaq(index)">
              <span class="text-sm sm:text-base font-semibold text-foreground pr-4">{{ faq.question }}</span>
              <svg class="w-5 h-5 text-muted-foreground flex-shrink-0 transition-transform duration-300" :class="{ 'rotate-180': openFaq === index }" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
              </svg>
            </button>
            <div v-if="openFaq === index" class="px-4 sm:px-6 pb-4 sm:pb-5">
              <p class="text-sm text-muted-foreground leading-relaxed">{{ faq.answer }}</p>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- CTA -->
    <section data-aos="zoom-in" class="py-20 lg:py-28 bg-gradient-to-br from-blue-600 via-blue-700 to-emerald-600 relative overflow-hidden">
      <div class="absolute inset-0">
        <div class="absolute top-10 left-10 w-64 h-64 bg-white/5 rounded-full blur-3xl" />
        <div class="absolute bottom-10 right-10 w-96 h-96 bg-emerald-400/15 rounded-full blur-3xl" />
      </div>
      <div class="relative max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
        <h2 class="text-3xl md:text-4xl font-bold text-white mb-4">Ready to Build Your Website?</h2>
        <p class="text-lg text-primary-100 mb-8 max-w-2xl mx-auto">Join thousands of users who have already created stunning websites with Rinova. Start building today, no credit card required.</p>
        <div class="flex flex-col sm:flex-row gap-4 justify-center">
          <NuxtLink to="/register" class="inline-flex items-center justify-center rounded-xl px-8 py-4 text-base font-semibold bg-white text-primary-700 hover:bg-primary-50 transition-all duration-200 shadow-lg shadow-black/10 hover:shadow-xl">
            Start Building Free
            <svg class="w-5 h-5 ml-2" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7l5 5m0 0l-5 5m5-5H6" /></svg>
          </NuxtLink>
          <a href="#pricing" class="inline-flex items-center justify-center rounded-xl px-8 py-4 text-base font-semibold bg-primary-500 text-white hover:bg-primary-400 transition-all duration-200 border-2 border-white/20">View Pricing</a>
        </div>
      </div>
    </section>

    <!-- Footer -->
    <footer data-aos="fade-up" class="bg-gray-900 dark:bg-black text-gray-400">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
        <div class="grid sm:grid-cols-2 lg:grid-cols-4 gap-8">
          <div class="sm:col-span-2 lg:col-span-1">
            <NuxtLink to="/" class="flex items-center gap-2 mb-4">
              <div class="w-8 h-8 bg-gradient-to-br from-blue-400 to-emerald-500 rounded-lg flex items-center justify-center">
                <svg class="w-5 h-5 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" /></svg>
              </div>
              <span class="text-xl font-bold text-white">Rinova</span>
            </NuxtLink>
            <p class="text-sm leading-relaxed text-gray-400">Build beautiful websites without code. Professional drag-and-drop website builder for everyone.</p>
          </div>
          <div>
            <h4 class="text-sm font-semibold text-white mb-4">Product</h4>
            <ul class="space-y-2">
              <li><a href="#features" class="text-sm text-gray-400 hover:text-white transition-colors">Features</a></li>
              <li><a href="#pricing" class="text-sm text-gray-400 hover:text-white transition-colors">Pricing</a></li>
              <li><a href="#how-it-works" class="text-sm text-gray-400 hover:text-white transition-colors">How It Works</a></li>
              <li><NuxtLink to="/login" class="text-sm text-gray-400 hover:text-white transition-colors">Sign In</NuxtLink></li>
            </ul>
          </div>
          <div>
            <h4 class="text-sm font-semibold text-white mb-4">Company</h4>
            <ul class="space-y-2">
              <li><a href="#" class="text-sm text-gray-400 hover:text-white transition-colors">About</a></li>
              <li><a href="#" class="text-sm text-gray-400 hover:text-white transition-colors">Blog</a></li>
              <li><a href="#" class="text-sm text-gray-400 hover:text-white transition-colors">Careers</a></li>
              <li><a href="#" class="text-sm text-gray-400 hover:text-white transition-colors">Contact</a></li>
            </ul>
          </div>
          <div>
            <h4 class="text-sm font-semibold text-white mb-4">Legal</h4>
            <ul class="space-y-2">
              <li><a href="#" class="text-sm text-gray-400 hover:text-white transition-colors">Privacy Policy</a></li>
              <li><a href="#" class="text-sm text-gray-400 hover:text-white transition-colors">Terms of Service</a></li>
              <li><a href="#" class="text-sm text-gray-400 hover:text-white transition-colors">Cookie Policy</a></li>
              <li><a href="#" class="text-sm text-gray-400 hover:text-white transition-colors">SLA</a></li>
            </ul>
          </div>
        </div>
        <div class="border-t border-gray-800 mt-10 pt-8 flex flex-col sm:flex-row items-center justify-between gap-4">
          <p class="text-sm text-gray-500">&copy; {{ new Date().getFullYear() }} Rinova. All rights reserved.</p>
          <div class="flex items-center gap-4">
            <a href="#" class="text-gray-400 hover:text-white transition-colors">
              <span class="sr-only">Twitter</span>
              <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24"><path d="M8.29 20.251c7.547 0 11.675-6.253 11.675-11.675 0-.178 0-.355-.012-.53A8.348 8.348 0 0022 5.92a8.19 8.19 0 01-2.357.646 4.118 4.118 0 001.804-2.27 8.224 8.224 0 01-2.605.996 4.107 4.107 0 00-6.993 3.743 11.65 11.65 0 01-8.457-4.287 4.106 4.106 0 001.27 5.477A4.072 4.072 0 012.8 9.713v.052a4.105 4.105 0 003.292 4.022 4.095 4.095 0 01-1.853.07 4.108 4.108 0 003.834 2.85A8.233 8.233 0 012 18.407a11.616 11.616 0 006.29 1.84" /></svg>
            </a>
            <a href="#" class="text-gray-400 hover:text-white transition-colors">
              <span class="sr-only">GitHub</span>
              <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24"><path fill-rule="evenodd" d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z" clip-rule="evenodd" /></svg>
            </a>
          </div>
        </div>
      </div>
    </footer>
  </div>
</template>
