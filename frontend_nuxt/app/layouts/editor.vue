<script setup lang="ts">
import type { ComponentNode, ComponentType } from '~/types'

const route = useRoute()
const projectId = computed(() => route.params.id as string)
const themeStore = useThemeStore()

const viewportMode = ref<'desktop' | 'tablet' | 'mobile'>('desktop')
const propertyPanelOpen = ref(true)
const previewDark = ref(false)
const viewportStyle = computed(() => {
  if (viewportMode.value === 'desktop') {
    return { width: '100%', height: '100%' }
  }
  const w = viewportMode.value === 'tablet' ? '768px' : '375px'
  return { width: w, minWidth: w, height: '100%' }
})

const sidebarMode = ref<'docked' | 'floating' | 'collapsed'>('docked')
const sidebarLayout = ref<'list' | 'grid'>('list')
const sidebarSize = ref<'sm' | 'md' | 'lg'>('md')
const sidebarWidth = ref(260)
const sidebarMinWidth = 180
const sidebarMaxWidth = 480
const sizePresets = { sm: 180, md: 260, lg: 360 }
const sizeOrder = ['sm', 'md', 'lg'] as const
const modeOrder = ['docked', 'floating', 'collapsed'] as const
let sidebarResizeStart = 0
let sidebarResizeWidth = 0

function cycleSize() {
  const idx = sizeOrder.indexOf(sidebarSize.value)
  sidebarSize.value = sizeOrder[(idx + 1) % 3]
  sidebarWidth.value = sizePresets[sidebarSize.value]
}

function cycleMode() {
  const idx = modeOrder.indexOf(sidebarMode.value)
  sidebarMode.value = modeOrder[(idx + 1) % 3]
}

function startSidebarResize(e: MouseEvent) {
  sidebarResizeStart = e.clientX
  sidebarResizeWidth = sidebarWidth.value
  document.addEventListener('mousemove', onSidebarResize)
  document.addEventListener('mouseup', stopSidebarResize)
  document.body.style.cursor = 'col-resize'
  document.body.style.userSelect = 'none'
}
function onSidebarResize(e: MouseEvent) {
  const diff = e.clientX - sidebarResizeStart
  sidebarWidth.value = Math.min(sidebarMaxWidth, Math.max(sidebarMinWidth, sidebarResizeWidth + diff))
}
function stopSidebarResize() {
  document.removeEventListener('mousemove', onSidebarResize)
  document.removeEventListener('mouseup', stopSidebarResize)
  document.body.style.cursor = ''
  document.body.style.userSelect = ''
}

const {
  componentTree, rootComponentIds, selectedComponentId, selectedComponent,
  addComponent, deleteComponent, selectComponent, updateComponent, updateComponentStyles,
  loadFromBackend, startAutoSave, saveToBackend, isDirty, lastSaved
} = useCanvas(projectId.value)

onMounted(() => {
  loadFromBackend()
  startAutoSave()
})

watch(componentTree, () => {
  if (import.meta.client && typeof (window as any).AOS?.refresh === 'function') {
    setTimeout(() => (window as any).AOS.refresh(), 150)
  }
}, { deep: true })

function handleAddComponent(type: ComponentType) {
  const targetParent = selectedComponent.value?.type === 'Container' ? selectedComponent.value : null

  if (!targetParent && (type === 'Header' || type === 'Footer')) {
    const existing = rootComponentIds.value.find(id => componentTree.value.get(id)?.type === type)
    if (existing) deleteComponent(existing)
  }
  const id = `comp-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`
  const comp: ComponentNode = {
    id, type,
    props: getDefaultProps(type),
    styles: { desktop: getDefaultStyles(type) },
    children: [], parentId: targetParent ? targetParent.id : null
  }
  addComponent(comp, targetParent?.id || null, targetParent ? targetParent.children.length : rootComponentIds.value.length)
  selectComponent(comp.id)
}

function handleSelect(id: string | null) {
  selectComponent(id)
}

function handleDelete(id: string) {
  deleteComponent(id)
}

function getDefaultProps(type: ComponentType): Record<string, unknown> {
  switch (type) {
    case 'Text': return { content: 'Edit this text in the property panel.', tag: 'p' }
    case 'Button': return { label: 'Get Started', variant: 'primary' }
    case 'Image': return { src: '', alt: 'Image' }
    case 'Hero': return { heading: 'Build Your Dream Website', subheading: 'Create stunning landing pages with our drag-and-drop builder. No coding required.', ctaText: 'Get Started Free' }
    case 'Header': return { logo: 'Rinova', menuItems: ['Home', 'Features', 'Pricing', 'Contact'] }
    case 'Footer': return { copyright: '© 2024 Rinova. All rights reserved.', links: [{ label: 'Privacy', url: '#' }, { label: 'Terms', url: '#' }, { label: 'Contact', url: '#' }] }
    case 'Features': return { title: 'Powerful Features', subtitle: 'Everything you need to build and grow', columns: [{ icon: 'star', title: 'Easy to Use', description: 'Drag and drop interface' }, { icon: 'shield', title: 'Secure', description: 'Enterprise-grade security' }, { icon: 'zap', title: 'Fast', description: 'Optimized for speed' }] }
    case 'Testimonials': return { title: 'What Customers Say', subtitle: 'Join thousands of satisfied customers', testimonials: [{ name: 'Sarah Johnson', role: 'CEO, TechStart', content: 'This platform transformed how we build landing pages.' }, { name: 'Mike Chen', role: 'Founder, DesignLab', content: 'The best website builder I have ever used.' }] }
    case 'Pricing': return { title: 'Simple Pricing', subtitle: 'Choose the plan that fits your needs', plans: [{ name: 'Free', price: '$0', features: ['1 Project', 'Basic Components'], cta: 'Get Started', highlighted: false }, { name: 'Pro', price: '$29', features: ['10 Projects', 'All Components', 'Priority Support'], cta: 'Start Trial', highlighted: true }, { name: 'Enterprise', price: '$99', features: ['Unlimited Projects', 'Dedicated Support'], cta: 'Contact Us', highlighted: false }] }
    case 'FAQ': return { title: 'FAQs', subtitle: 'Got questions? We have answers.', items: [{ q: 'How do I get started?', a: 'Sign up and start building. No credit card required.' }, { q: 'Can I use my own domain?', a: 'Yes, on Pro and Enterprise plans.' }] }
    case 'Contact': return { title: 'Get in Touch', subtitle: 'We would love to hear from you.', email: 'hello@example.com', phone: '+1 (555) 123-4567' }
    case 'Stats': return { title: 'Our Impact', stats: [{ value: '10K+', label: 'Users' }, { value: '50K+', label: 'Websites Built' }, { value: '99.9%', label: 'Uptime' }, { value: '4.9/5', label: 'Rating' }] }
    case 'Input': return { label: 'Name', placeholder: 'Enter your name', type: 'text', required: false }
    case 'Textarea': return { label: 'Message', placeholder: 'Write your message...', rows: 4, required: false }
    case 'Number': return { label: 'Quantity', placeholder: '0', min: 0, max: 100, step: 1, required: false }
    case 'Select': return { label: 'Choose an option', options: ['Option 1', 'Option 2', 'Option 3'], required: false }
    case 'Checkbox': return { label: 'I agree to the terms', checked: false, required: false }
    case 'Radio': return { label: 'Select one', options: ['Yes', 'No', 'Maybe'], selected: 'Yes' }
    case 'Range': return { label: 'Volume', min: 0, max: 100, step: 1, value: 50 }
    case 'Date': return { label: 'Pick a date', required: false }
    case 'Container': return { items: [{ type: 'text', content: 'Start building inside this container' }], align: 'left' }
    case 'Link': return { text: 'Click here', url: '#' }
    case 'Icon': return { className: 'fa-solid fa-star', size: 24, color: '#6366f1' }
    default: return {}
  }
}

function getDefaultStyles(type: ComponentType): Record<string, string | number> {
  switch (type) {
    case 'Hero': return { backgroundColor: '#f9fafb' }
    case 'Features': return { backgroundColor: '#ffffff' }
    case 'Testimonials': return { backgroundColor: '#f9fafb' }
    case 'Pricing': return { backgroundColor: '#ffffff' }
    case 'FAQ': return { backgroundColor: '#f9fafb' }
    case 'Contact': return { backgroundColor: '#ffffff' }
    case 'Stats': return { backgroundColor: '#1e40af', color: '#ffffff' }
    case 'Input': case 'Textarea': case 'Number': case 'Select': case 'Date': return { width: '100%', padding: '10px 14px', fontSize: '14px', border: '1px solid #d1d5db', borderRadius: '8px', backgroundColor: '#ffffff' }
    case 'Checkbox': case 'Radio': return { display: 'flex', alignItems: 'center', gap: '8px', fontSize: '14px' }
    case 'Range': return { width: '100%' }
    case 'Container': return { backgroundColor: '#ffffff', border: '1px dashed #d1d5db', borderRadius: '8px', padding: '16px' }
    default: return {}
  }
}
</script>

<template>
  <div class="h-screen flex flex-col">
    <!-- Toolbar -->
    <header class="h-12 bg-card border-b border-border flex items-center justify-between px-3 z-30 shrink-0">
      <div class="flex items-center gap-3">
        <NuxtLink to="/panel" class="flex items-center gap-2">
          <div class="w-6 h-6 bg-gradient-to-br from-primary-500 to-purple-600 rounded flex items-center justify-center">
            <svg class="w-3 h-3 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" /></svg>
          </div>
          <span class="text-sm font-bold text-primary-600 dark:text-primary-400">Rinova</span>
        </NuxtLink>
        <span class="text-xs text-muted-foreground">Editor</span>
      </div>

      <div class="flex items-center gap-2">
        <div class="flex items-center gap-0.5 bg-muted rounded-md p-0.5">
          <button v-for="mode in ['desktop', 'tablet', 'mobile'] as const" :key="mode"
            class="p-1.5 rounded transition-colors"
            :class="viewportMode === mode ? 'bg-white dark:bg-gray-700 shadow-sm' : 'hover:bg-accent'"
            @click="viewportMode = mode">
            <svg v-if="mode === 'desktop'" class="w-3.5 h-3.5 text-gray-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" /></svg>
            <svg v-else-if="mode === 'tablet'" class="w-3.5 h-3.5 text-gray-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 18h.01M7 21h10a2 2 0 002-2V5a2 2 0 00-2-2H7a2 2 0 00-2 2v14a2 2 0 002 2z" /></svg>
            <svg v-else class="w-3.5 h-3.5 text-gray-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 18h.01M8 21h8a2 2 0 002-2V5a2 2 0 00-2-2H8a2 2 0 00-2 2v14a2 2 0 002 2z" /></svg>
          </button>
        </div>

        <NuxtLink :to="`/preview/${projectId}`" target="_blank" class="flex items-center gap-1 px-2.5 py-1.5 rounded-md bg-primary-600 text-white text-xs font-medium hover:bg-primary-700 transition-colors" title="Preview">
          <svg class="w-3.5 h-3.5" fill="currentColor" viewBox="0 0 24 24"><path d="M8 5v14l11-7z" /></svg>
          <span class="hidden sm:inline">Preview</span>
        </NuxtLink>

        <span class="text-[10px] text-muted-foreground font-mono">
          {{ isDirty ? 'unsaved' : lastSaved ? 'saved' : '' }}
        </span>
        <button class="relative w-9 h-4 rounded-full transition-colors" :class="themeStore.mode === 'dark' ? 'bg-primary-600' : 'bg-muted'" @click="themeStore.toggle()" title="Toggle theme">
          <span class="absolute top-0.5 left-0.5 w-3 h-3 rounded-full bg-white shadow-sm transition-transform" :class="themeStore.mode === 'dark' ? 'translate-x-5' : 'translate-x-0'" />
        </button>
      </div>
    </header>

    <!-- Editor body: 2-column layout -->
    <div class="flex-1 flex overflow-hidden">
      <!-- Left sidebar: component picker (docked/floating/collapsed) -->
      <!-- Floating/collapsed sidebar toggle button (shown when not docked) -->
      <button
        v-if="sidebarMode !== 'docked'"
        class="w-8 bg-card border-r border-border flex items-center justify-center shrink-0 hover:bg-accent relative z-10"
        @click="sidebarMode = 'floating'"
        title="Toggle floating panel"
      >
        <svg class="w-4 h-4 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" /></svg>
      </button>

      <!-- Docked sidebar -->
      <aside
        v-if="sidebarMode === 'docked'"
        :style="{ width: sidebarWidth + 'px' }"
        class="bg-card border-r border-border flex flex-col shrink-0 relative overflow-hidden"
        :class="{ 'sidebar-grid': sidebarLayout === 'grid' }"
      >
        <!-- Toolbar -->
        <div class="flex items-center justify-between px-3 h-10 border-b border-border shrink-0">
          <span class="text-xs font-semibold text-muted-foreground uppercase tracking-wider">Components</span>
          <div class="flex items-center gap-0.5">
            <!-- Layout toggle -->
            <button
              class="p-1 rounded text-muted-foreground hover:text-foreground hover:bg-accent"
              @click="sidebarLayout = sidebarLayout === 'list' ? 'grid' : 'list'"
              title="Toggle layout"
            >
              <svg v-if="sidebarLayout === 'list'" class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" /></svg>
              <svg v-else class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h8M4 12h16M4 18h8" /></svg>
            </button>
            <!-- Size cycle -->
            <button
              class="p-1 rounded text-muted-foreground hover:text-foreground hover:bg-accent text-xs font-mono font-semibold"
              @click="cycleSize"
              title="Cycle size"
            >{{ sidebarSize.toUpperCase() }}</button>
            <!-- Mode cycle -->
            <button
              class="p-1 rounded text-muted-foreground hover:text-foreground hover:bg-accent"
              @click="cycleMode"
              title="Cycle mode"
            >
              <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 8V4m0 0h4M4 4l5 5m11-1V4m0 0h-4m4 0l-5 5M4 16v4m0 0h4m-4 0l5-5m11 5l-5-5m5 5v-4m0 4h-4" /></svg>
            </button>
            <!-- Collapse -->
            <button
              class="p-1 rounded text-muted-foreground hover:text-foreground hover:bg-accent"
              @click="sidebarMode = 'collapsed'"
              title="Collapse sidebar"
            >
              <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 19l-7-7 7-7m8 14l-7-7 7-7" /></svg>
            </button>
          </div>
        </div>
        <div class="flex-1 overflow-y-auto">
          <SidebarComponents @add:component="handleAddComponent" />
        </div>
        <!-- Helper Panel -->
        <HelperPanel v-model:preview-dark="previewDark" @close="sidebarMode = 'collapsed'" />
        <!-- Resize handle -->
        <div class="absolute top-0 right-0 w-1.5 h-full cursor-col-resize hover:bg-primary-400/30 group z-10" @mousedown="startSidebarResize">
          <div class="absolute inset-y-0 right-0 w-0.5 group-hover:w-0.5 group-hover:bg-primary-400 transition-colors" />
        </div>
      </aside>

      <!-- Collapsed thin bar -->
      <button
        v-if="sidebarMode === 'collapsed'"
        class="w-2 bg-card border-r border-border shrink-0 hover:bg-accent cursor-pointer"
        @click="sidebarMode = 'docked'"
        title="Open sidebar"
      />

      <!-- Floating panel (teleported to body) -->
      <Teleport to="body">
        <div
          v-if="sidebarMode === 'floating'"
          class="fixed inset-0 z-40 bg-black/20 backdrop-blur-sm"
          @click="sidebarMode = 'docked'"
        >
          <div
            class="fixed left-0 top-12 bottom-0 bg-card shadow-2xl border-r border-border flex flex-col z-50"
            :style="{ width: sidebarWidth + 'px' }"
            @click.stop
          >
            <div class="flex items-center justify-between px-3 h-10 border-b border-border shrink-0">
              <span class="text-xs font-semibold text-muted-foreground uppercase tracking-wider">Components</span>
              <div class="flex items-center gap-0.5">
                <button class="p-1 rounded text-muted-foreground hover:text-foreground hover:bg-accent" @click="sidebarLayout = sidebarLayout === 'list' ? 'grid' : 'list'" title="Toggle layout">
                  <svg v-if="sidebarLayout === 'list'" class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" /></svg>
                  <svg v-else class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h8M4 12h16M4 18h8" /></svg>
                </button>
                <button class="p-1 rounded text-muted-foreground hover:text-foreground hover:bg-accent text-xs font-mono font-semibold" @click="cycleSize" title="Cycle size">{{ sidebarSize.toUpperCase() }}</button>
                <button class="p-1 rounded text-muted-foreground hover:text-foreground hover:bg-accent" @click="sidebarMode = 'collapsed'" title="Dock to collapsed">
                  <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 19l-7-7 7-7m8 14l-7-7 7-7" /></svg>
                </button>
              </div>
            </div>
            <div class="flex-1 overflow-y-auto" :class="sidebarLayout === 'grid' ? 'p-2' : ''">
              <SidebarComponents @add:component="handleAddComponent" />
            </div>
            <HelperPanel v-model:preview-dark="previewDark" @close="sidebarMode = 'collapsed'" />
          </div>
        </div>
      </Teleport>

      <!-- Right: canvas preview + property panel -->
      <main class="flex-1 flex flex-col bg-background">
        <div class="flex-1 flex overflow-hidden">
          <!-- Canvas preview -->
          <div class="flex-1 overflow-auto p-3">
            <div class="min-h-full flex items-start justify-center">
              <div class="bg-card shadow-lg rounded-lg overflow-hidden transition-all duration-200"
                :class="viewportMode === 'desktop' ? 'h-full w-full' : 'shadow-xl'"
                :style="viewportStyle">
                <BuilderCanvas
                  :component-tree="componentTree"
                  :root-component-ids="rootComponentIds"
                  :selected-component-id="selectedComponentId"
                  :viewport-mode="viewportMode"
                  :preview-dark="previewDark"
                  @select="handleSelect"
                  @delete="handleDelete"
                />
              </div>
            </div>
          </div>

          <!-- Right: property panel toggle (when closed) -->
          <button
            v-if="!propertyPanelOpen && selectedComponent"
            class="w-7 bg-card border-l border-border flex items-center justify-center shrink-0 hover:bg-accent"
            @click="propertyPanelOpen = true"
            title="Open properties"
          >
            <svg class="w-3.5 h-3.5 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" /></svg>
          </button>

          <!-- Right: property panel (collapsible) -->
          <aside v-if="propertyPanelOpen && selectedComponent" class="w-72 bg-card border-l border-border shrink-0 flex flex-col">
            <div class="flex items-center justify-between px-3 h-9 border-b border-border shrink-0">
              <span class="text-[10px] font-semibold text-muted-foreground uppercase tracking-wider">Properties</span>
              <button
                class="p-1 rounded text-muted-foreground hover:text-foreground hover:bg-accent"
                @click="propertyPanelOpen = false"
                title="Close properties"
              >
                <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" /></svg>
              </button>
            </div>
            <div class="flex-1 overflow-y-auto">
              <PropertyPanel
                :selected-component="selectedComponent"
                @update:props="(id, props) => updateComponent(id, { props })"
                @update:styles="(id, styles, bp) => updateComponentStyles(id, { [bp]: styles }, bp)"
                @update:animations="(id, animations) => updateComponent(id, { animations })"
              />
            </div>
          </aside>
        </div>
      </main>
    </div>

    <slot />
  </div>
</template>

<style scoped>
.sidebar-grid :deep(.sidebar-btn) {
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
  padding: 0.5rem 0.25rem;
  gap: 0.25rem;
}
.sidebar-grid :deep(.sidebar-btn .min-w-0) {
  text-align: center;
}
.sidebar-grid :deep(.sidebar-btn .min-w-0 p:first-child) {
  font-size: 0.75rem;
}
.sidebar-grid :deep(.sidebar-btn .min-w-0 p:last-child) {
  display: none;
}
.sidebar-grid :deep(.space-y-0\.5) {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 0.25rem;
}
.sidebar-grid :deep(.space-y-0\.5 > *) {
  margin-top: 0;
}
</style>
