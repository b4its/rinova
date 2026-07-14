<script setup lang="ts">
import type { ViewportMode, ComponentNode } from '~/types'

const props = defineProps<{
  componentTree: Map<string, ComponentNode>
  rootComponentIds: string[]
  selectedComponentId: string | null
  viewportMode: ViewportMode
  previewDark?: boolean
  preview?: boolean
}>()

const emit = defineEmits<{
  'select': [id: string | null]
  'delete': [id: string]
}>()

const hoveredId = ref<string | null>(null)
const mobileMenuOpen = ref(false)

function aosAttrs(comp: ComponentNode | undefined): Record<string, string | number> {
  const anims = comp?.animations
  if (!anims || anims.length === 0) return {}
  const a = anims[0]
  const map: Record<string, string> = {
    fade: 'fade-up', slide: 'slide-up', bounce: 'zoom-in',
    rotate: 'flip-up', scale: 'zoom-in',
  }
  return {
    'data-aos': map[a.type] || 'fade-up',
    'data-aos-duration': a.duration || 800,
    'data-aos-delay': a.delay || 0,
    'data-aos-easing': a.easing || 'ease-in-out',
  }
}

function previewStyle(comp: ComponentNode | undefined): Record<string, string> {
  if (!comp?.styles?.desktop) return {}
  const result: Record<string, string> = {}
  for (const [key, val] of Object.entries(comp.styles.desktop)) {
    result[key.replace(/([A-Z])/g, '-$1').toLowerCase()] = String(val)
  }
  return result
}

const headerIds = computed(() =>
  props.rootComponentIds.filter(id => props.componentTree.get(id)?.type === 'Header')
)
const sectionIds = computed(() =>
  props.rootComponentIds.filter(id => {
    const t = props.componentTree.get(id)?.type
    return t !== 'Header' && t !== 'Footer'
  })
)
const footerIds = computed(() =>
  props.rootComponentIds.filter(id => props.componentTree.get(id)?.type === 'Footer')
)

const isEmpty = computed(() => props.rootComponentIds.length === 0)

function comp(id: string) {
  return props.componentTree.get(id)
}

function pc(id: string) {
  return (comp(id)?.props || {}) as any
}
</script>

<template>
  <div class="w-full h-full overflow-y-auto bg-muted relative" :class="['vp-' + viewportMode, { 'dark preview-dark': previewDark }]" style="container-type: inline-size;">
    <!-- Empty state -->
    <div v-if="isEmpty" class="h-full flex items-center justify-center p-6">
      <div class="text-center max-w-sm">
        <div class="w-20 h-20 mx-auto rounded-2xl bg-muted flex items-center justify-center mb-4">
          <svg class="w-8 h-8 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
          </svg>
        </div>
        <h3 class="text-lg font-semibold text-gray-700 dark:text-gray-300 mb-2">Start Building Your Page</h3>
        <p class="text-sm text-muted-foreground">Click any component from the sidebar to add it here</p>
      </div>
    </div>

    <!-- Page Preview -->
    <div v-else class="min-h-full flex flex-col">

      <!-- HEADER ZONE -->
      <div v-if="headerIds.length" class="sticky top-0 z-10">
        <div v-for="rootId in headerIds" :key="rootId">
          <div class="relative transition-all" :class="[!preview ? 'cursor-pointer' : '', !preview && selectedComponentId === rootId ? 'ring-2 ring-primary-500' : !preview ? 'hover:ring-2 hover:ring-primary-300' : '']" @click="!preview && emit('select', rootId)" @mouseenter="!preview && (hoveredId = rootId)" @mouseleave="!preview && (hoveredId = null)" v-bind="aosAttrs(comp(rootId))">
            <div v-if="!preview && hoveredId === rootId" class="absolute top-2 right-2 z-20 flex gap-1">
              <button class="w-7 h-7 bg-red-500 text-white rounded-lg flex items-center justify-center hover:bg-red-600 shadow-lg transition-colors" @click.stop="emit('delete', rootId)" title="Delete"><svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" /></svg></button>
            </div>
            <div :style="previewStyle(comp(rootId))" class="bg-card shadow-sm">
              <div class="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="flex items-center justify-between h-14 sm:h-16 lg:h-20">
                  <div class="text-base sm:text-lg lg:text-xl font-bold text-gray-800 dark:text-white">{{ pc(rootId)?.logo || 'Rinova' }}</div>
                  <div class="hidden lg:flex items-center gap-6 text-sm font-medium text-gray-600 dark:text-gray-300">
                    <a v-for="(item, i) in pc(rootId)?.menuItems || ['Home','Features','Pricing','Contact']" :key="i" class="hover:text-primary-600 cursor-pointer transition-colors">{{ item }}</a>
                    <span class="px-4 py-2 bg-primary-600 text-white rounded-lg text-sm font-semibold cursor-pointer hover:bg-primary-700 transition-colors">Get Started</span>
                  </div>
                  <button class="lg:hidden p-2 rounded-lg text-muted-foreground hover:bg-accent transition-colors" @click.stop="mobileMenuOpen = !mobileMenuOpen">
                    <svg v-if="!mobileMenuOpen" class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" /></svg>
                    <svg v-else class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" /></svg>
                  </button>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- MAIN CONTENT SECTIONS -->
      <div class="flex-1">
        <div v-for="(rootId, idx) in sectionIds" :key="rootId">
          <div class="relative transition-all" :class="[!preview && selectedComponentId === rootId ? 'ring-2 ring-primary-500' : !preview ? 'hover:ring-2 hover:ring-primary-200' : '']" @click="!preview && emit('select', rootId)" @mouseenter="!preview && (hoveredId = rootId)" @mouseleave="!preview && (hoveredId = null)" v-bind="aosAttrs(comp(rootId))">
            <div v-if="!preview && hoveredId === rootId" class="absolute top-2 right-2 z-20 flex gap-1">
              <button class="w-7 h-7 bg-red-500 text-white rounded-lg flex items-center justify-center hover:bg-red-600 shadow-lg transition-colors" @click.stop="emit('delete', rootId)" title="Delete"><svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" /></svg></button>
            </div>
            <div :style="previewStyle(comp(rootId))" class="bg-card">
              <div class="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8">

                <!-- ─── Text ─── -->
                <div v-if="comp(rootId)?.type === 'Text'" class="py-6 sm:py-8 lg:py-10 text-sm sm:text-base leading-relaxed text-gray-600 dark:text-gray-300">{{ pc(rootId)?.content }}</div>

                <!-- ─── Link ─── -->
                <div v-else-if="comp(rootId)?.type === 'Link'" class="py-4">
                  <a :href="pc(rootId)?.url || '#'" class="text-primary-600 hover:text-primary-700 underline text-sm sm:text-base cursor-pointer">{{ pc(rootId)?.text || 'Link' }}</a>
                </div>

                <!-- ─── Icon ─── -->
                <div v-else-if="comp(rootId)?.type === 'Icon'" class="py-6 text-center">
                  <i :class="pc(rootId)?.className || 'fa-solid fa-star'" :style="{ fontSize: (pc(rootId)?.size || 24) + 'px', color: pc(rootId)?.color || 'currentColor' }"></i>
                </div>

                <!-- ─── Container ─── -->
                <div v-else-if="comp(rootId)?.type === 'Container'" class="py-6 sm:py-8">
                  <div class="space-y-3" :class="{ 'text-left': pc(rootId)?.align === 'left', 'text-center': pc(rootId)?.align === 'center', 'text-right': pc(rootId)?.align === 'right' }">
                    <!-- Items -->
                    <div v-for="(item, i) in pc(rootId)?.items || []" :key="'item-'+i" class="mb-2 last:mb-0">
                      <span v-if="item.type === 'text'">{{ item.content }}</span>
                      <a v-else-if="item.type === 'link'" :href="item.url || '#'" class="text-primary-600 hover:underline cursor-pointer text-sm">{{ item.text || item.content }}</a>
                      <i v-else-if="item.type === 'icon'" :class="item.className" :style="{ fontSize: (item.size || 24) + 'px', color: item.color || 'currentColor' }"></i>
                      <span v-else-if="item.type === 'button'" class="inline-flex px-5 py-2.5 bg-primary-600 text-white rounded-lg text-sm font-semibold cursor-pointer hover:bg-primary-700 transition-colors">{{ item.text || item.content }}</span>
                      <hr v-else-if="item.type === 'divider'" class="border-input" />
                      <span v-else class="text-sm text-muted-foreground">{{ item.content }}</span>
                    </div>

                    <!-- Child components -->
                    <div v-for="childId in comp(rootId)?.children || []" :key="childId" class="relative transition-all" :class="[!preview ? 'cursor-pointer' : '', !preview && selectedComponentId === childId ? 'ring-2 ring-primary-500' : !preview ? 'hover:ring-2 hover:ring-primary-200' : '']" @click.stop="!preview && emit('select', childId)" @mouseenter="!preview && (hoveredId = childId)" @mouseleave="!preview && (hoveredId = null)">
                      <div v-if="!preview && hoveredId === childId" class="absolute top-1 right-1 z-20">
                        <button class="w-6 h-6 bg-red-500 text-white rounded flex items-center justify-center hover:bg-red-600 shadow text-[10px]" @click.stop="emit('delete', childId)" title="Delete">
                          <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" /></svg>
                        </button>
                      </div>

                      <!-- Text -->
                      <div v-if="comp(childId)?.type === 'Text'" class="text-sm sm:text-base leading-relaxed text-gray-600 dark:text-gray-300">{{ pc(childId)?.content }}</div>

                      <!-- Link -->
                      <div v-else-if="comp(childId)?.type === 'Link'">
                        <a :href="pc(childId)?.url || '#'" class="text-primary-600 hover:text-primary-700 underline text-sm sm:text-base cursor-pointer">{{ pc(childId)?.text || 'Link' }}</a>
                      </div>

                      <!-- Icon -->
                      <div v-else-if="comp(childId)?.type === 'Icon'" class="text-center">
                        <i :class="pc(childId)?.className || 'fa-solid fa-star'" :style="{ fontSize: (pc(childId)?.size || 24) + 'px', color: pc(childId)?.color || 'currentColor' }"></i>
                      </div>

                      <!-- Button -->
                      <div v-else-if="comp(childId)?.type === 'Button'" class="text-center">
                        <span class="inline-flex px-5 py-2.5 bg-primary-600 text-white rounded-lg text-sm font-semibold cursor-pointer hover:bg-primary-700 transition-colors">{{ pc(childId)?.label }}</span>
                      </div>

                      <!-- Image -->
                      <div v-else-if="comp(childId)?.type === 'Image'">
                        <img v-if="pc(childId)?.src" :src="pc(childId)?.src" :alt="pc(childId)?.alt || ''" class="w-full rounded-xl object-cover" style="max-height:200px" />
                        <div v-else class="bg-muted rounded-xl h-32 flex items-center justify-center">
                          <svg class="w-8 h-8 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" /></svg>
                        </div>
                      </div>

                      <!-- Divider -->
                      <div v-else-if="comp(childId)?.type === 'Divider'">
                        <hr class="border-input" />
                      </div>

                      <!-- Fallback -->
                      <div v-else class="py-3 text-center">
                        <p class="text-xs text-muted-foreground">{{ comp(childId)?.type }} component</p>
                      </div>
                    </div>

                    <!-- Empty state -->
                    <div v-if="!pc(rootId)?.items?.length && !comp(rootId)?.children?.length" class="text-center py-6 text-xs text-muted-foreground border-2 border-dashed border-border rounded-lg">
                      Container is empty — add items in Properties panel or drop components inside while Container is selected
                    </div>
                  </div>
                </div>

                <!-- ─── Hero ─── -->
                <div v-else-if="comp(rootId)?.type === 'Hero'" class="py-12 sm:py-16 lg:py-20 text-center lg:text-left">
                  <h1 class="text-3xl sm:text-4xl lg:text-5xl xl:text-6xl font-extrabold text-foreground leading-tight">{{ pc(rootId)?.heading }}</h1>
                  <p class="mt-4 sm:mt-6 text-base sm:text-lg lg:text-xl text-muted-foreground max-w-2xl mx-auto lg:mx-0">{{ pc(rootId)?.subheading }}</p>
                  <div class="mt-6 sm:mt-8 flex flex-col sm:flex-row items-center gap-4 justify-center lg:justify-start">
                    <span class="w-full sm:w-auto inline-flex justify-center px-6 sm:px-8 py-3 sm:py-3.5 bg-primary-600 text-white font-semibold rounded-xl text-sm sm:text-base hover:bg-primary-700 cursor-pointer transition-colors">{{ pc(rootId)?.ctaText || 'Get Started' }}</span>
                  </div>
                </div>

                <!-- ─── Button ─── -->
                <div v-else-if="comp(rootId)?.type === 'Button'" class="py-6 sm:py-8 text-center">
                  <span class="inline-flex px-6 sm:px-8 py-3 sm:py-3.5 bg-primary-600 text-white font-semibold rounded-xl text-sm sm:text-base cursor-pointer hover:bg-primary-700 transition-colors">{{ pc(rootId)?.label }}</span>
                </div>

                <!-- ─── Image ─── -->
                <div v-else-if="comp(rootId)?.type === 'Image'" class="py-6 sm:py-8">
                  <img v-if="pc(rootId)?.src" :src="pc(rootId)?.src" :alt="pc(rootId)?.alt || ''" class="w-full rounded-xl object-cover" style="max-height:400px" />
                  <div v-else class="bg-muted rounded-xl h-40 sm:h-48 lg:h-56 flex items-center justify-center">
                    <svg class="w-10 sm:w-12 h-10 sm:h-12 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" /></svg>
                  </div>
                </div>

                <!-- ─── Features ─── -->
                <div v-else-if="comp(rootId)?.type === 'Features'" class="py-12 sm:py-16 lg:py-20">
                  <div class="text-center max-w-2xl mx-auto mb-10 sm:mb-12">
                    <h2 class="text-2xl sm:text-3xl lg:text-4xl font-bold text-foreground">{{ pc(rootId)?.title }}</h2>
                    <p class="mt-3 text-sm sm:text-base text-muted-foreground">{{ pc(rootId)?.subtitle }}</p>
                  </div>
                  <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 sm:gap-6">
                    <div v-for="(col, i) in pc(rootId)?.columns || []" :key="i" class="p-5 sm:p-6 rounded-xl bg-muted text-center hover:shadow-lg transition-shadow">
                      <p class="text-base sm:text-lg font-semibold text-foreground">{{ col.title }}</p>
                      <p class="mt-2 text-sm text-muted-foreground">{{ col.description }}</p>
                    </div>
                  </div>
                </div>

                <!-- ─── Testimonials ─── -->
                <div v-else-if="comp(rootId)?.type === 'Testimonials'" class="py-12 sm:py-16 lg:py-20">
                  <div class="text-center max-w-2xl mx-auto mb-10 sm:mb-12">
                    <h2 class="text-2xl sm:text-3xl lg:text-4xl font-bold text-foreground">{{ pc(rootId)?.title }}</h2>
                    <p class="mt-3 text-sm sm:text-base text-muted-foreground">{{ pc(rootId)?.subtitle }}</p>
                  </div>
                  <div class="grid grid-cols-1 sm:grid-cols-2 gap-4 sm:gap-6">
                    <div v-for="(t, i) in pc(rootId)?.testimonials || []" :key="i" class="p-5 sm:p-6 rounded-xl bg-muted hover:shadow-lg transition-shadow">
                      <p class="text-gray-600 dark:text-gray-300 italic text-sm sm:text-base">"{{ t.content }}"</p>
                      <div class="mt-4 flex items-center gap-3">
                        <div class="w-9 h-9 sm:w-10 sm:h-10 rounded-full bg-primary-100 dark:bg-primary-900 flex items-center justify-center text-primary-600 font-semibold text-sm shrink-0">{{ t.name?.charAt(0) }}</div>
                        <div><p class="text-sm font-semibold text-foreground">{{ t.name }}</p><p class="text-xs text-gray-500">{{ t.role }}</p></div>
                      </div>
                    </div>
                  </div>
                </div>

                <!-- ─── Pricing ─── -->
                <div v-else-if="comp(rootId)?.type === 'Pricing'" class="py-12 sm:py-16 lg:py-20">
                  <div class="text-center max-w-2xl mx-auto mb-10 sm:mb-12">
                    <h2 class="text-2xl sm:text-3xl lg:text-4xl font-bold text-foreground">{{ pc(rootId)?.title }}</h2>
                    <p class="mt-3 text-sm sm:text-base text-muted-foreground">{{ pc(rootId)?.subtitle }}</p>
                  </div>
                  <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 sm:gap-6 lg:gap-8">
                    <div v-for="(plan, i) in pc(rootId)?.plans || []" :key="i" class="p-6 sm:p-8 rounded-xl border" :class="plan.highlighted ? 'border-primary-500 bg-primary-50 dark:bg-primary-900/20 ring-2 ring-primary-500 scale-[1.02]' : 'border-gray-200 dark:border-gray-700'">
                      <p class="text-lg font-semibold text-foreground">{{ plan.name }}</p>
                      <p class="mt-2 text-3xl font-bold text-foreground">{{ plan.price }}<span class="text-sm font-normal text-gray-500">/mo</span></p>
                      <ul class="mt-6 space-y-3 text-sm text-gray-600 dark:text-gray-300"><li v-for="(f, fi) in plan.features" :key="fi" class="flex items-center gap-2"><svg class="w-4 h-4 text-primary-500 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" /></svg>{{ f }}</li></ul>
                      <div class="mt-8"><span class="block w-full text-center py-3 rounded-xl text-sm font-semibold cursor-pointer transition-colors" :class="plan.highlighted ? 'bg-primary-600 text-white hover:bg-primary-700' : 'bg-gray-100 dark:bg-gray-800 text-gray-900 dark:text-white hover:bg-gray-200 dark:hover:bg-gray-700'">{{ plan.cta }}</span></div>
                    </div>
                  </div>
                </div>

                <!-- ─── FAQ ─── -->
                <div v-else-if="comp(rootId)?.type === 'FAQ'" class="py-12 sm:py-16 lg:py-20 max-w-3xl mx-auto">
                  <div class="text-center mb-10 sm:mb-12">
                    <h2 class="text-2xl sm:text-3xl lg:text-4xl font-bold text-foreground">{{ pc(rootId)?.title }}</h2>
                    <p class="mt-3 text-sm sm:text-base text-muted-foreground">{{ pc(rootId)?.subtitle }}</p>
                  </div>
                  <div class="space-y-3 sm:space-y-4">
                    <details v-for="(item, i) in pc(rootId)?.items || []" :key="i" class="group rounded-xl border border-border overflow-hidden">
                      <summary class="px-4 sm:px-6 py-3 sm:py-4 text-sm font-semibold text-foreground cursor-pointer hover:bg-accent flex items-center justify-between">{{ item.q }}<svg class="w-4 h-4 text-gray-500 group-open:rotate-180 transition-transform shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" /></svg></summary>
                      <p class="px-4 sm:px-6 py-3 sm:py-4 text-sm text-muted-foreground border-t border-border">{{ item.a }}</p>
                    </details>
                  </div>
                </div>

                <!-- ─── Contact ─── -->
                <div v-else-if="comp(rootId)?.type === 'Contact'" class="py-12 sm:py-16 lg:py-20">
                  <div class="text-center max-w-2xl mx-auto mb-10 sm:mb-12">
                    <h2 class="text-2xl sm:text-3xl lg:text-4xl font-bold text-foreground">{{ pc(rootId)?.title }}</h2>
                    <p class="mt-3 text-sm sm:text-base text-muted-foreground">{{ pc(rootId)?.subtitle }}</p>
                  </div>
                  <div class="grid grid-cols-1 lg:grid-cols-2 gap-8 sm:gap-10">
                    <div class="space-y-4 sm:space-y-5">
                      <div class="flex items-center gap-3 text-sm sm:text-base text-gray-600 dark:text-gray-300"><svg class="w-5 h-5 text-primary-500 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" /></svg><span>{{ pc(rootId)?.email }}</span></div>
                      <div class="flex items-center gap-3 text-sm sm:text-base text-gray-600 dark:text-gray-300"><svg class="w-5 h-5 text-primary-500 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 5a2 2 0 012-2h3.28a1 1 0 01.948.684l1.498 4.493a1 1 0 01-.502 1.21l-2.257 1.13a11.042 11.042 0 005.516 5.516l1.13-2.257a1 1 0 011.21-.502l4.493 1.498a1 1 0 01.684.949V19a2 2 0 01-2 2h-1C9.716 21 3 14.284 3 6V5z" /></svg><span>{{ pc(rootId)?.phone }}</span></div>
                    </div>
                    <div class="bg-muted rounded-xl p-5 sm:p-6">
                      <input placeholder="Your Name" class="w-full px-3 sm:px-4 py-2.5 rounded-lg border border-border bg-card text-sm mb-3" />
                      <input placeholder="Your Email" class="w-full px-3 sm:px-4 py-2.5 rounded-lg border border-border bg-card text-sm mb-3" />
                      <textarea placeholder="Your Message" rows="3" class="w-full px-3 sm:px-4 py-2.5 rounded-lg border border-border bg-card text-sm mb-3"></textarea>
                      <span class="block w-full text-center py-2.5 bg-primary-600 text-white rounded-lg text-sm font-semibold cursor-pointer hover:bg-primary-700 transition-colors">Send Message</span>
                    </div>
                  </div>
                </div>

                <!-- ─── Stats ─── -->
                <div v-else-if="comp(rootId)?.type === 'Stats'" class="py-12 sm:py-16 lg:py-20 text-center">
                  <h2 class="text-2xl sm:text-3xl lg:text-4xl font-bold text-foreground mb-10 sm:mb-12">{{ pc(rootId)?.title }}</h2>
                  <div class="grid grid-cols-2 sm:grid-cols-4 gap-6 sm:gap-8">
                    <div v-for="(s, i) in pc(rootId)?.stats || []" :key="i"><p class="text-3xl sm:text-4xl font-bold text-foreground">{{ s.value }}</p><p class="mt-1 text-sm sm:text-base opacity-80 text-gray-300">{{ s.label }}</p></div>
                  </div>
                </div>

                <!-- ─── Divider ─── -->
                <div v-else-if="comp(rootId)?.type === 'Divider'" class="py-6 sm:py-8">
                  <hr class="border-input" />
                </div>

                <!-- ─── Form ─── -->
                <div v-else-if="comp(rootId)?.type === 'Form'" class="py-12 sm:py-16 max-w-xl mx-auto">
                  <h2 class="text-xl sm:text-2xl font-bold text-center text-foreground mb-6">{{ pc(rootId)?.title || 'Contact Form' }}</h2>
                  <div class="bg-muted rounded-xl p-5 sm:p-6 space-y-4">
                    <input placeholder="Your Name" class="w-full px-3 sm:px-4 py-2.5 rounded-lg border bg-card text-sm" />
                    <input placeholder="Your Email" class="w-full px-3 sm:px-4 py-2.5 rounded-lg border bg-card text-sm" />
                    <textarea placeholder="Your Message" rows="3" class="w-full px-3 sm:px-4 py-2.5 rounded-lg border bg-card text-sm"></textarea>
                    <span class="block w-full text-center py-3 bg-primary-600 text-white rounded-lg text-sm font-semibold cursor-pointer hover:bg-primary-700 transition-colors">{{ pc(rootId)?.submitText || 'Submit' }}</span>
                  </div>
                </div>

                <!-- ─── Form Inputs ─── -->
                <div v-else-if="comp(rootId)?.type === 'Input'" class="py-4 sm:py-5 max-w-md">
                  <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1.5">{{ pc(rootId)?.label }}</label>
                  <input type="text" :placeholder="pc(rootId)?.placeholder" class="w-full px-3 py-2 rounded-lg border border-input bg-card text-sm" disabled>
                </div>
                <div v-else-if="comp(rootId)?.type === 'Textarea'" class="py-4 sm:py-5 max-w-md">
                  <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1.5">{{ pc(rootId)?.label }}</label>
                  <textarea disabled :placeholder="pc(rootId)?.placeholder" :rows="pc(rootId)?.rows || 4" class="w-full px-3 py-2 rounded-lg border border-input bg-card text-sm" />
                </div>
                <div v-else-if="comp(rootId)?.type === 'Number'" class="py-4 sm:py-5 max-w-md">
                  <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1.5">{{ pc(rootId)?.label }}</label>
                  <input type="number" disabled :placeholder="pc(rootId)?.placeholder" class="w-full px-3 py-2 rounded-lg border border-input bg-card text-sm">
                </div>
                <div v-else-if="comp(rootId)?.type === 'Select'" class="py-4 sm:py-5 max-w-md">
                  <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1.5">{{ pc(rootId)?.label }}</label>
                  <select disabled class="w-full px-3 py-2 rounded-lg border border-input bg-card text-sm">
                    <option v-for="opt in pc(rootId)?.options || []" :key="opt">{{ opt }}</option>
                  </select>
                </div>
                <div v-else-if="comp(rootId)?.type === 'Checkbox'" class="py-3 sm:py-4">
                  <label class="flex items-center gap-2.5 text-sm text-gray-700 dark:text-gray-300">
                    <input type="checkbox" disabled :checked="pc(rootId)?.checked" class="rounded border-gray-300">{{ pc(rootId)?.label }}
                  </label>
                </div>
                <div v-else-if="comp(rootId)?.type === 'Radio'" class="py-4 sm:py-5">
                  <p class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">{{ pc(rootId)?.label }}</p>
                  <div v-for="opt in pc(rootId)?.options || []" :key="opt" class="flex items-center gap-2.5 mb-1.5 text-sm text-muted-foreground"><input type="radio" disabled :checked="opt === pc(rootId)?.selected" class="border-gray-300">{{ opt }}</div>
                </div>
                <div v-else-if="comp(rootId)?.type === 'Range'" class="py-4 sm:py-5 max-w-md">
                  <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1.5">{{ pc(rootId)?.label }}: <span class="text-primary-600 font-semibold">{{ pc(rootId)?.value }}</span></label>
                  <input type="range" disabled :min="pc(rootId)?.min || 0" :max="pc(rootId)?.max || 100" :value="pc(rootId)?.value || 50" class="w-full accent-primary-500">
                </div>
                <div v-else-if="comp(rootId)?.type === 'Date'" class="py-4 sm:py-5 max-w-md">
                  <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1.5">{{ pc(rootId)?.label }}</label>
                  <input type="date" disabled class="w-full px-3 py-2 rounded-lg border border-input bg-card text-sm">
                </div>

                <!-- ─── Fallback ─── -->
                <div v-else class="py-10 text-center">
                  <p class="text-sm text-muted-foreground">{{ comp(rootId)?.type }} section</p>
                </div>

              </div>
            </div>
          </div>
          <div v-if="idx < sectionIds.length - 1" class="h-px bg-muted" />
        </div>
      </div>

      <!-- FOOTER ZONE -->
      <div v-if="footerIds.length">
        <div v-for="rootId in footerIds" :key="rootId">
          <div class="relative transition-all" :class="[!preview ? 'cursor-pointer' : '', !preview && selectedComponentId === rootId ? 'ring-2 ring-primary-500' : !preview ? 'hover:ring-2 hover:ring-primary-300' : '']" @click="!preview && emit('select', rootId)" @mouseenter="!preview && (hoveredId = rootId)" @mouseleave="!preview && (hoveredId = null)" v-bind="aosAttrs(comp(rootId))">
            <div v-if="!preview && hoveredId === rootId" class="absolute top-2 right-2 z-20 flex gap-1">
              <button class="w-7 h-7 bg-red-500 text-white rounded-lg flex items-center justify-center hover:bg-red-600 shadow-lg transition-colors" @click.stop="emit('delete', rootId)" title="Delete"><svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" /></svg></button>
            </div>
            <div :style="previewStyle(comp(rootId))" class="bg-gray-900 dark:bg-black text-gray-300">
              <div class="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8 py-10 sm:py-12 lg:py-16">
                <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-8 sm:gap-10">
                  <div class="sm:col-span-2 lg:col-span-1">
                    <p class="text-base sm:text-lg font-bold text-white mb-3">{{ pc(rootId)?.logo || 'Rinova' }}</p>
                    <p class="text-sm text-muted-foreground leading-relaxed">Build beautiful websites with our drag-and-drop builder. No coding required.</p>
                  </div>
                  <div>
                    <p class="text-sm font-semibold text-white mb-4">Links</p>
                    <div class="space-y-2 text-sm">
                      <a v-for="(link, i) in pc(rootId)?.links || [{label:'Privacy',url:'#'},{label:'Terms',url:'#'},{label:'Contact',url:'#'}]" :key="i" class="block text-muted-foreground hover:text-white cursor-pointer transition-colors">{{ link.label }}</a>
                    </div>
                  </div>
                  <div>
                    <p class="text-sm font-semibold text-white mb-4">Follow Us</p>
                    <div class="flex gap-3">
                      <span class="w-8 h-8 rounded-full bg-gray-800 flex items-center justify-center hover:bg-gray-700 cursor-pointer transition-colors text-xs font-bold text-muted-foreground">T</span>
                      <span class="w-8 h-8 rounded-full bg-gray-800 flex items-center justify-center hover:bg-gray-700 cursor-pointer transition-colors text-xs font-bold text-muted-foreground">F</span>
                      <span class="w-8 h-8 rounded-full bg-gray-800 flex items-center justify-center hover:bg-gray-700 cursor-pointer transition-colors text-xs font-bold text-muted-foreground">I</span>
                    </div>
                  </div>
                </div>
                <div class="mt-8 sm:mt-10 pt-6 sm:pt-8 border-t border-gray-800 text-center text-xs sm:text-sm text-gray-500">{{ pc(rootId)?.copyright || '© 2024 Rinova. All rights reserved.' }}</div>
              </div>
            </div>
          </div>
        </div>
      </div>
      <!-- Mobile menu overlay (canvas-level) -->
      <Transition name="mobile">
        <div v-if="mobileMenuOpen" key="mobile-menu" class="absolute inset-0 z-50 lg:hidden" @click.self="mobileMenuOpen = false">
          <div class="absolute inset-0 bg-black/20 mobile-backdrop" />
          <div class="relative z-10 bg-card shadow-lg border-b border-border px-4 py-4 space-y-3 text-sm mobile-panel">
            <template v-if="headerIds.length">
              <a v-for="(item, i) in pc(headerIds[0])?.menuItems || ['Home','Features','Pricing','Contact']" :key="i" class="block py-2 text-gray-600 dark:text-gray-300 hover:text-primary-600 transition-colors cursor-pointer" @click="mobileMenuOpen = false">{{ item }}</a>
              <span class="block w-full text-center px-4 py-2.5 bg-primary-600 text-white rounded-lg text-sm font-semibold cursor-pointer hover:bg-primary-700 transition-colors" @click="mobileMenuOpen = false">Get Started</span>
            </template>
          </div>
        </div>
      </Transition>

    </div>
  </div>
</template>

<style>
.vp-mobile .sm\:flex-row { flex-direction: column !important; }
.vp-mobile .sm\:grid-cols-2 { grid-template-columns: repeat(1, minmax(0, 1fr)) !important; }
.vp-mobile .sm\:grid-cols-4 { grid-template-columns: repeat(2, minmax(0, 1fr)) !important; }
.vp-mobile .sm\:col-span-2 { grid-column: span 1 / span 1 !important; }
.vp-mobile .sm\:text-4xl { font-size: 1.875rem !important; line-height: 2.25rem !important; }
.vp-mobile .sm\:text-3xl { font-size: 1.5rem !important; line-height: 2rem !important; }
.vp-mobile .sm\:text-lg { font-size: 1rem !important; line-height: 1.5rem !important; }
.vp-mobile .sm\:text-base { font-size: 0.875rem !important; line-height: 1.25rem !important; }
.vp-mobile .sm\:h-16 { height: 3.5rem !important; }
.vp-mobile .sm\:h-48 { height: 10rem !important; }
.vp-mobile .sm\:px-6 { padding-left: 1rem !important; padding-right: 1rem !important; }
.vp-mobile .sm\:py-16 { padding-top: 3rem !important; padding-bottom: 3rem !important; }
.vp-mobile .sm\:py-8 { padding-top: 1.5rem !important; padding-bottom: 1.5rem !important; }
.vp-mobile .sm\:py-5 { padding-top: 1rem !important; padding-bottom: 1rem !important; }
.vp-mobile .sm\:p-6 { padding: 1rem !important; }
.vp-mobile .sm\:gap-6 { gap: 1rem !important; }
.vp-mobile .sm\:gap-10 { gap: 1.5rem !important; }
.vp-mobile .sm\:mx-0 { margin-left: auto !important; margin-right: auto !important; }
.vp-mobile .sm\:mt-6 { margin-top: 1rem !important; }
.vp-mobile .sm\:mb-12 { margin-bottom: 2rem !important; }
.vp-mobile .sm\:w-auto { width: 100% !important; }

.vp-mobile .lg\:flex,
.vp-tablet .lg\:flex { display: none !important; }
.vp-mobile .lg\:hidden,
.vp-tablet .lg\:hidden { display: initial !important; }
.vp-mobile .lg\:grid-cols-3 { grid-template-columns: repeat(1, minmax(0, 1fr)) !important; }
.vp-mobile .lg\:grid-cols-2 { grid-template-columns: repeat(1, minmax(0, 1fr)) !important; }
.vp-mobile .lg\:text-left,
.vp-tablet .lg\:text-left { text-align: center !important; }
.vp-mobile .lg\:justify-start,
.vp-tablet .lg\:justify-start { justify-content: center !important; }
.vp-mobile .lg\:mx-0,
.vp-tablet .lg\:mx-0 { margin-left: auto !important; margin-right: auto !important; }
.vp-mobile .lg\:h-20 { height: 3.5rem !important; }

.preview-dark {
  --background: 240 10% 3.9%;
  --foreground: 0 0% 98%;
  --card: 240 10% 3.9%;
  --card-foreground: 0 0% 98%;
  --primary: 246 80% 60%;
  --primary-foreground: 0 0% 100%;
  --secondary: 240 4% 16%;
  --secondary-foreground: 0 0% 98%;
  --muted: 240 4% 16%;
  --muted-foreground: 240 5% 65%;
  --accent: 240 4% 16%;
  --accent-foreground: 0 0% 98%;
  --border: 240 4% 16%;
  --input: 240 4% 16%;
  --ring: 246 80% 60%;
  color-scheme: dark;
}

.mobile-enter-active {
  transition: opacity 0.2s ease;
}
.mobile-leave-active {
  transition: opacity 0.15s ease;
}
.mobile-enter-active .mobile-panel {
  transition: transform 0.25s ease;
}
.mobile-leave-active .mobile-panel {
  transition: transform 0.15s ease;
}
.mobile-enter-from,
.mobile-leave-to {
  opacity: 0;
}
.mobile-enter-from .mobile-panel {
  transform: translateY(-100%);
}
.mobile-leave-to .mobile-panel {
  transform: translateY(-100%);
}
</style>
