<script setup lang="ts">
definePageMeta({ layout: 'panel', sidebarSection: 'analytics' })

type DateFilter = 'today' | '7d' | '30d' | '90d' | 'custom'

const dateFilter = ref<DateFilter>('7d')
const customStart = ref('')
const customEnd = ref('')

const today = new Date().toISOString().split('T')[0]

const effectiveDays = computed(() => {
  if (dateFilter.value === 'today') return 1
  if (dateFilter.value === 'custom') {
    if (customStart.value && customEnd.value) {
      const s = new Date(customStart.value)
      const e = new Date(customEnd.value)
      return Math.max(1, Math.ceil((e.getTime() - s.getTime()) / (1000 * 60 * 60 * 24)) + 1)
    }
    return 7
  }
  return dateFilter.value === '7d' ? 7 : dateFilter.value === '30d' ? 30 : 90
})

const stats = computed(() => {
  const d = effectiveDays.value
  return [
    { label: 'Total Visitors', value: d <= 1 ? '1,842' : d <= 7 ? '12,432' : d <= 30 ? '48,291' : '142,839', change: '+12.5%', trend: 'up' as const },
    { label: 'Page Views', value: d <= 1 ? '5,291' : d <= 7 ? '38,291' : d <= 30 ? '152,847' : '489,201', change: '+8.2%', trend: 'up' as const },
    { label: 'Conversion Rate', value: '3.24%', change: '-0.4%', trend: 'down' as const },
    { label: 'Bounce Rate', value: '42.1%', change: '-2.1%', trend: 'down' as const },
    { label: 'Avg. Session', value: '4m 32s', change: '+5.3%', trend: 'up' as const },
    { label: 'Total Projects', value: '5', change: '+2 this month', trend: 'up' as const },
  ]
})

const chartData = computed(() => {
  const days = effectiveDays.value
  const now = new Date()
  const rand = (min: number, max: number) => Math.floor(min + Math.random() * (max - min))

  if (days <= 1) {
    return Array.from({ length: 24 }, (_, i) => ({
      label: `${i}:00`,
      visitors: rand(50, 400),
      pageViews: rand(200, 1200),
    }))
  }

  if (days <= 7) {
    return Array.from({ length: days }, (_, i) => {
      const d = new Date(now)
      d.setDate(d.getDate() - (days - 1 - i))
      return {
        label: d.toLocaleDateString('en-US', { weekday: 'short' }),
        visitors: rand(800, 2000),
        pageViews: rand(2000, 5000),
      }
    })
  }

  if (days <= 45) {
    const weeks = Math.ceil(days / 7)
    return Array.from({ length: weeks }, (_, i) => ({
      label: `W${weeks - i}`,
      visitors: rand(4000, 10000),
      pageViews: rand(12000, 30000),
    })).reverse()
  }

  const months = Math.min(12, Math.ceil(days / 30))
  return Array.from({ length: months }, (_, i) => {
    const d = new Date(now)
    d.setMonth(d.getMonth() - (months - 1 - i))
    return {
      label: d.toLocaleDateString('en-US', { month: 'short' }),
      visitors: rand(12000, 30000),
      pageViews: rand(40000, 100000),
    }
  })
})

const maxChartValue = computed(() => Math.max(...chartData.value.map(d => d.pageViews), 1))

const pages = computed(() => {
  const mult = effectiveDays.value
  const maxV = Math.max(1, ...([8421, 3291, 2847, 2134, 1892].map(v => Math.round(v * mult / 7))))
  return [
    { path: '/', title: 'Home', views: Math.round(8421 * mult / 7), bounceRate: '38%', pct: Math.round(8421 / 8421 * 100) },
    { path: '/pricing', title: 'Pricing', views: Math.round(3291 * mult / 7), bounceRate: '52%', pct: Math.round(3291 / 8421 * 100) },
    { path: '/login', title: 'Login', views: Math.round(2847 * mult / 7), bounceRate: '28%', pct: Math.round(2847 / 8421 * 100) },
    { path: '/register', title: 'Register', views: Math.round(2134 * mult / 7), bounceRate: '45%', pct: Math.round(2134 / 8421 * 100) },
    { path: '/features', title: 'Features', views: Math.round(1892 * mult / 7), bounceRate: '41%', pct: Math.round(1892 / 8421 * 100) },
  ].map(p => ({ ...p, pct: Math.round(p.views / maxV * 100) }))
})

const devices = [
  { name: 'Desktop', percentage: 52, color: 'hsl(var(--primary))', label: 'hsl(var(--primary))' },
  { name: 'Mobile', percentage: 35, color: '#3b82f6', label: '#3b82f6' },
  { name: 'Tablet', percentage: 13, color: '#22c55e', label: '#22c55e' },
]

const trafficSources = [
  { name: 'Organic Search', percentage: 38, color: 'hsl(var(--primary))' },
  { name: 'Direct', percentage: 27, color: '#8b5cf6' },
  { name: 'Social Media', percentage: 18, color: '#3b82f6' },
  { name: 'Referral', percentage: 12, color: '#f59e0b' },
  { name: 'Email', percentage: 5, color: '#22c55e' },
]

const filters: { label: string; value: DateFilter }[] = [
  { label: 'Today', value: 'today' },
  { label: '7 Days', value: '7d' },
  { label: '30 Days', value: '30d' },
  { label: '90 Days', value: '90d' },
  { label: 'Custom', value: 'custom' },
]

function polarToCartesian(cx: number, cy: number, r: number, deg: number) {
  const rad = ((deg - 90) * Math.PI) / 180
  return { x: cx + r * Math.cos(rad), y: cy + r * Math.sin(rad) }
}

function describeArc(cx: number, cy: number, r: number, startDeg: number, endDeg: number) {
  const start = polarToCartesian(cx, cy, r, endDeg)
  const end = polarToCartesian(cx, cy, r, startDeg)
  const large = endDeg - startDeg > 180 ? 1 : 0
  return `M ${cx} ${cy} L ${start.x} ${start.y} A ${r} ${r} 0 ${large} 0 ${end.x} ${end.y} Z`
}

const hoveredDay = ref<{ label: string; visitors: number; pageViews: number } | null>(null)
const hoverPos = ref({ x: 0, y: 0 })
const selectedDay = ref<{ label: string; visitors: number; pageViews: number } | null>(null)

function onBarHover(e: MouseEvent, day: { label: string; visitors: number; pageViews: number }) {
  const rect = (e.currentTarget as HTMLElement).getBoundingClientRect()
  hoverPos.value = { x: rect.left + rect.width / 2, y: rect.top }
  hoveredDay.value = day
}

function onBarLeave() {
  hoveredDay.value = null
}

function selectDay(day: { label: string; visitors: number; pageViews: number }) {
  selectedDay.value = day
}
</script>

<template>
  <div class="space-y-6">
    <div class="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4">
      <div>
        <h1 class="text-2xl font-bold tracking-tight">Analytics</h1>
        <p class="text-muted-foreground text-sm mt-1">Track your website performance and visitor insights</p>
      </div>
    </div>

    <div class="flex flex-wrap items-center gap-3">
      <div class="flex items-center rounded-lg border bg-card overflow-hidden">
        <button
          v-for="f in filters"
          :key="f.value"
          class="px-3 py-1.5 text-sm font-medium transition-colors"
          :class="dateFilter === f.value ? 'bg-primary text-primary-foreground' : 'text-muted-foreground hover:bg-accent hover:text-accent-foreground'"
          @click="dateFilter = f.value"
        >
          {{ f.label }}
        </button>
      </div>

      <div v-if="dateFilter === 'custom'" class="flex items-center gap-2">
        <input v-model="customStart" type="date" :max="customEnd || today" class="input h-9 w-40 text-sm" />
        <span class="text-muted-foreground text-sm">to</span>
        <input v-model="customEnd" type="date" :min="customStart" :max="today" class="input h-9 w-40 text-sm" />
      </div>

      <div v-if="dateFilter !== 'custom'" class="text-xs text-muted-foreground">{{ effectiveDays === 1 ? 'Last 24 hours' : `Last ${effectiveDays} days` }}</div>
    </div>

    <!-- Stat Cards -->
    <div class="grid grid-cols-2 lg:grid-cols-3 gap-4">
      <div v-for="stat in stats" :key="stat.label" class="card group hover:shadow-md transition-all">
        <div class="card-body">
          <div class="flex items-center justify-between">
            <p class="text-sm text-muted-foreground">{{ stat.label }}</p>
            <span class="text-xs font-medium px-1.5 py-0.5 rounded" :class="stat.trend === 'up' ? 'bg-green-100 dark:bg-green-900/30 text-green-700 dark:text-green-400' : 'bg-red-100 dark:bg-red-900/30 text-red-700 dark:text-red-400'">
              {{ stat.change }}
            </span>
          </div>
          <p class="text-2xl font-bold tracking-tight mt-2">{{ stat.value }}</p>
        </div>
      </div>
    </div>

    <!-- Visitor Trends — Bar Chart (aggregated) -->
    <div class="card">
      <div class="card-body">
        <div class="flex items-center justify-between mb-4">
          <h3 class="font-semibold">Visitor Trends</h3>
          <div class="flex items-center gap-3 text-xs text-muted-foreground">
            <div class="flex items-center gap-1.5">
              <div class="w-3 h-3 rounded bg-primary/60" />
              <span>Visitors</span>
            </div>
            <div class="flex items-center gap-1.5">
              <div class="w-3 h-3 rounded bg-blue-400/60" />
              <span>Page Views</span>
            </div>
          </div>
        </div>
        <div class="relative" style="height: 200px;">
          <!-- Tooltip -->
          <div
            v-if="hoveredDay"
            class="fixed z-50 -translate-x-1/2 -translate-y-full pointer-events-none"
            :style="{ left: hoverPos.x + 'px', top: hoverPos.y - 8 + 'px' }"
          >
            <div class="bg-popover text-popover-foreground shadow-lg rounded-lg border p-2.5 text-xs space-y-1 min-w-[140px]">
              <p class="font-semibold text-[11px] text-muted-foreground mb-1">{{ hoveredDay.label }}</p>
              <div class="flex items-center justify-between gap-4">
                <span class="flex items-center gap-1.5"><span class="w-2 h-2 rounded-sm bg-primary/60" />Visitors</span>
                <span class="font-medium tabular-nums">{{ hoveredDay.visitors.toLocaleString() }}</span>
              </div>
              <div class="flex items-center justify-between gap-4">
                <span class="flex items-center gap-1.5"><span class="w-2 h-2 rounded-sm bg-blue-400/60" />Page Views</span>
                <span class="font-medium tabular-nums">{{ hoveredDay.pageViews.toLocaleString() }}</span>
              </div>
            </div>
          </div>
          <div class="absolute inset-0 flex items-end gap-[3px] px-1">
            <div v-for="day in chartData" :key="day.label"
              class="flex-1 flex flex-col items-center justify-end h-full relative cursor-pointer group"
              @mouseenter="onBarHover($event, day)"
              @mouseleave="onBarLeave"
              @click="selectDay(day)"
            >
              <div class="w-full flex gap-[2px] items-end flex-1" style="min-height: 4px;">
                <div
                  class="flex-1 rounded-t transition-all duration-300"
                  :class="selectedDay?.label === day.label ? 'bg-primary' : 'bg-primary/60 group-hover:bg-primary'"
                  :style="{ height: `${(day.visitors / maxChartValue) * 100}%` }"
                />
                <div
                  class="flex-1 rounded-t transition-all duration-300"
                  :class="selectedDay?.label === day.label ? 'bg-blue-400' : 'bg-blue-400/60 group-hover:bg-blue-400'"
                  :style="{ height: `${(day.pageViews / maxChartValue) * 100}%` }"
                />
              </div>
              <span class="text-[10px] text-muted-foreground truncate w-full text-center leading-tight mt-1">{{ day.label }}</span>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Row: Top Pages (horiz bar) + Devices (donut) -->
    <div class="grid lg:grid-cols-2 gap-6">
      <!-- Top Pages — Horizontal Bar Chart -->
      <div class="card">
        <div class="card-body">
          <h3 class="font-semibold mb-4">Top Pages</h3>
          <div class="space-y-3">
            <div v-for="(page, i) in pages" :key="page.path">
              <div class="flex items-center justify-between text-sm mb-1">
                <div class="flex items-center gap-2 min-w-0">
                  <span class="text-xs font-medium text-muted-foreground w-4 shrink-0">{{ i + 1 }}</span>
                  <div class="truncate">
                    <span class="font-medium truncate">{{ page.title }}</span>
                    <span class="text-xs text-muted-foreground ml-1">({{ page.bounceRate }})</span>
                  </div>
                </div>
                <span class="text-sm font-medium shrink-0 ml-2">{{ page.views.toLocaleString() }}</span>
              </div>
              <div class="h-2 rounded-full bg-muted overflow-hidden">
                <div
                  class="h-full rounded-full transition-all bg-primary"
                  :style="{ width: `${page.pct}%` }"
                />
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Devices — Donut Chart -->
      <div class="card">
        <div class="card-body">
          <h3 class="font-semibold mb-4">Devices</h3>
          <div class="flex items-center gap-6">
            <svg viewBox="0 0 36 36" class="w-28 h-28 shrink-0">
              <circle cx="18" cy="18" r="15.9" fill="none" stroke="hsl(var(--muted))" stroke-width="2.5" />
              <circle
                v-for="(d, i) in devices"
                :key="d.name"
                cx="18" cy="18" r="15.9"
                fill="none"
                :stroke="d.color"
                stroke-width="2.5"
                stroke-linecap="round"
                :stroke-dasharray="`${d.percentage * 1.0} ${100 - d.percentage * 1.0}`"
                :stroke-dashoffset="devices.slice(0, i).reduce((sum, dd) => sum + dd.percentage * 1.0, 0) * -1"
                transform="rotate(-90 18 18)"
                class="transition-all duration-500"
              />
            </svg>
            <div class="space-y-2 flex-1">
              <div v-for="d in devices" :key="d.name" class="flex items-center justify-between text-sm">
                <div class="flex items-center gap-2">
                  <span class="w-2.5 h-2.5 rounded-full shrink-0" :style="{ background: d.color }" />
                  <span>{{ d.name }}</span>
                </div>
                <span class="font-medium">{{ d.percentage }}%</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Row: Traffic Sources (pie) + extra card -->
    <div class="grid lg:grid-cols-2 gap-6">
      <!-- Traffic Sources — Pie Chart -->
      <div class="card">
        <div class="card-body">
          <h3 class="font-semibold mb-4">Traffic Sources</h3>
          <div class="flex items-center gap-6">
            <svg viewBox="0 0 36 36" class="w-28 h-28 shrink-0">
              <circle v-for="(s, i) in trafficSources" :key="s.name"
                cx="18" cy="18" r="15.9"
                fill="none"
                :stroke="s.color"
                stroke-width="2.5"
                stroke-linecap="round"
                :stroke-dasharray="`${s.percentage * 1.0} ${100 - s.percentage * 1.0}`"
                :stroke-dashoffset="trafficSources.slice(0, i).reduce((sum, ss) => sum + ss.percentage * 1.0, 0) * -1"
                transform="rotate(-90 18 18)"
                class="transition-all duration-500"
              />
            </svg>
            <div class="space-y-2 flex-1">
              <div v-for="s in trafficSources" :key="s.name" class="flex items-center justify-between text-sm">
                <div class="flex items-center gap-2">
                  <span class="w-2.5 h-2.5 rounded-full shrink-0" :style="{ background: s.color }" />
                  <span>{{ s.name }}</span>
                </div>
                <span class="font-medium">{{ s.percentage }}%</span>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Bounce Rate by Page — simple bar -->
      <div class="card">
        <div class="card-body">
          <h3 class="font-semibold mb-4">Bounce Rate by Page</h3>
          <div class="space-y-3">
            <div v-for="(page, i) in [...pages].sort((a, b) => parseInt(b.bounceRate) - parseInt(a.bounceRate))" :key="page.path">
              <div class="flex items-center justify-between text-sm mb-1">
                <div class="flex items-center gap-2">
                  <span class="w-2 h-2 rounded-full shrink-0" :class="parseInt(page.bounceRate) > 45 ? 'bg-red-400' : parseInt(page.bounceRate) > 35 ? 'bg-yellow-400' : 'bg-green-400'" />
                  <span>{{ page.title }}</span>
                </div>
                <span class="font-medium">{{ page.bounceRate }}</span>
              </div>
              <div class="h-1.5 rounded-full bg-muted overflow-hidden">
                <div
                  class="h-full rounded-full transition-all"
                  :class="parseInt(page.bounceRate) > 45 ? 'bg-red-400' : parseInt(page.bounceRate) > 35 ? 'bg-yellow-400' : 'bg-green-400'"
                  :style="{ width: `${page.bounceRate}%` }"
                />
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
    <!-- Detail Modal -->
    <div v-if="selectedDay" class="fixed inset-0 z-50 flex items-center justify-center">
      <div class="fixed inset-0 bg-black/80" @click="selectedDay = null" />
      <div class="relative bg-background text-foreground rounded-xl shadow-lg border w-full max-w-lg mx-4 p-6 space-y-4">
        <div class="flex items-center justify-between">
          <h3 class="text-lg font-semibold">Detail: {{ selectedDay.label }}</h3>
          <button class="p-1 rounded-md text-muted-foreground hover:bg-accent hover:text-accent-foreground transition-colors" @click="selectedDay = null">
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" /></svg>
          </button>
        </div>
        <div class="grid grid-cols-2 gap-4">
          <div class="rounded-lg border bg-card p-4 space-y-1">
            <p class="text-xs text-muted-foreground flex items-center gap-1.5">
              <span class="w-2 h-2 rounded-sm bg-primary/60" /> Visitors
            </p>
            <p class="text-2xl font-bold tabular-nums">{{ selectedDay.visitors.toLocaleString() }}</p>
          </div>
          <div class="rounded-lg border bg-card p-4 space-y-1">
            <p class="text-xs text-muted-foreground flex items-center gap-1.5">
              <span class="w-2 h-2 rounded-sm bg-blue-400/60" /> Page Views
            </p>
            <p class="text-2xl font-bold tabular-nums">{{ selectedDay.pageViews.toLocaleString() }}</p>
          </div>
        </div>
        <div class="rounded-lg border bg-card p-4 space-y-2">
          <p class="text-xs font-semibold text-muted-foreground uppercase tracking-wider">Period</p>
          <p class="text-sm">{{ selectedDay.label }}</p>
          <p class="text-xs text-muted-foreground">Click outside or press ESC to close</p>
        </div>
      </div>
    </div>
  </div>
</template>
