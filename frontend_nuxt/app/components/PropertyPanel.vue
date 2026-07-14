<script setup lang="ts">
import type { ComponentNode, ViewportMode } from '~/types'

const props = withDefaults(defineProps<{
  selectedComponent?: ComponentNode | null
}>(), { selectedComponent: null })

const emit = defineEmits<{
  'update:props': [componentId: string, props: Record<string, unknown>]
  'update:styles': [componentId: string, styles: Record<string, string | number>, breakpoint: ViewportMode]
  'update:animations': [componentId: string, animations: any[]]
}>()

const activeTab = ref<'content' | 'style'>('content')

const comp = computed(() => props.selectedComponent)

const localProps = ref<Record<string, unknown>>({})
const localStyles = ref<Record<string, string | number>>({})
const localAnimations = ref<any[]>([])
const selectedItemIndex = ref<number | null>(null)

watch(comp, (c) => {
  selectedItemIndex.value = null
  if (c) {
    localProps.value = JSON.parse(JSON.stringify(c.props))
    localStyles.value = JSON.parse(JSON.stringify(c.styles.desktop || {}))
    localAnimations.value = c.animations ? JSON.parse(JSON.stringify(c.animations)) : []
  } else {
    localProps.value = {}
    localStyles.value = {}
    localAnimations.value = []
  }
}, { immediate: true, deep: true })

const isEditingItem = computed(() => comp.value?.type === 'Container' && selectedItemIndex.value !== null)
const editingItemData = computed(() => {
  if (!isEditingItem.value) return null
  const items = (localProps.value.items as any[]) || []
  return items[selectedItemIndex.value!] || null
})

function onPropChange() {
  if (!comp.value) return
  emit('update:props', comp.value.id, { ...localProps.value })
}

function onStyleChange() {
  if (!comp.value) return
  if (isEditingItem.value) {
    const items = [...((localProps.value.items as any[]) || [])]
    items[selectedItemIndex.value!] = { ...items[selectedItemIndex.value!], styles: { ...localStyles.value } }
    localProps.value = { ...localProps.value, items }
    emit('update:props', comp.value.id, { ...localProps.value })
  } else {
    emit('update:styles', comp.value.id, { ...localStyles.value }, 'desktop')
  }
}

function selectContainerItem(index: number | null) {
  if (index === selectedItemIndex.value) {
    // Deselect — restore container styles
    saveCurrentItemStyles()
    selectedItemIndex.value = null
    if (comp.value) {
      localStyles.value = JSON.parse(JSON.stringify(comp.value.styles.desktop || {}))
    }
    return
  }
  // Save current item styles before switching
  saveCurrentItemStyles()
  selectedItemIndex.value = index
  // Load selected item's styles
  const items = (localProps.value.items as any[]) || []
  const item = items[index!]
  localStyles.value = item?.styles ? JSON.parse(JSON.stringify(item.styles)) : {}
}

function saveCurrentItemStyles() {
  if (selectedItemIndex.value === null) return
  const items = [...((localProps.value.items as any[]) || [])]
  if (items[selectedItemIndex.value]) {
    items[selectedItemIndex.value] = { ...items[selectedItemIndex.value], styles: { ...localStyles.value } }
    localProps.value = { ...localProps.value, items }
  }
}

function updateProp(key: string, value: unknown) {
  localProps.value[key] = value
  onPropChange()
}

function setStyle(e: Event, key: string) {
  localStyles.value[key] = (e.target as HTMLInputElement).value
  onStyleChange()
}

function setStyleVal(key: string, value: string | number) {
  localStyles.value[key] = value
  onStyleChange()
}

const animationEnabled = computed(() => localAnimations.value.length > 0)

function setAnimation(val: any) {
  if (!comp.value) return
  if (val) {
    localAnimations.value = [{
      type: 'fade',
      duration: 800,
      delay: 0,
      easing: 'ease-in-out',
      iterationCount: 1,
      trigger: 'scroll'
    }]
  } else {
    localAnimations.value = []
  }
  emit('update:animations', comp.value.id, localAnimations.value)
}

function updateAnimationField(field: string, value: any) {
  if (!comp.value || localAnimations.value.length === 0) return
  localAnimations.value[0] = { ...localAnimations.value[0], [field]: value }
  emit('update:animations', comp.value.id, localAnimations.value)
}

function addItem(key: string, template: any) {
  const arr = (localProps.value[key] as any[]) || []
  arr.push(JSON.parse(JSON.stringify(template)))
  localProps.value = { ...localProps.value, [key]: arr }
  onPropChange()
}

function removeItem(key: string, idx: number) {
  const arr = (localProps.value[key] as any[]) || []
  arr.splice(idx, 1)
  localProps.value = { ...localProps.value, [key]: arr }
  onPropChange()
}

function updateObjItem(key: string, idx: number, sub: string, val: any) {
  const arr = (localProps.value[key] as any[]) || []
  if (!arr[idx]) arr[idx] = {}
  arr[idx][sub] = val
  localProps.value = { ...localProps.value, [key]: arr }
  onPropChange()
}

function updateStrItem(key: string, idx: number, e: Event) {
  const arr = (localProps.value[key] as string[]) || []
  arr[idx] = (e.target as HTMLInputElement).value
  localProps.value = { ...localProps.value, [key]: arr }
  onPropChange()
}

// ── Style enable/disable system ──
const iconOptions = [
  { class: 'fa-solid fa-star', label: 'Star' },
  { class: 'fa-solid fa-heart', label: 'Heart' },
  { class: 'fa-solid fa-check', label: 'Check' },
  { class: 'fa-solid fa-xmark', label: 'Close' },
  { class: 'fa-solid fa-home', label: 'Home' },
  { class: 'fa-solid fa-user', label: 'User' },
  { class: 'fa-solid fa-cog', label: 'Settings' },
  { class: 'fa-solid fa-envelope', label: 'Envelope' },
  { class: 'fa-solid fa-phone', label: 'Phone' },
  { class: 'fa-solid fa-globe', label: 'Globe' },
  { class: 'fa-solid fa-bolt', label: 'Bolt' },
  { class: 'fa-solid fa-rocket', label: 'Rocket' },
  { class: 'fa-solid fa-shield-halved', label: 'Shield' },
  { class: 'fa-solid fa-chart-simple', label: 'Chart' },
  { class: 'fa-solid fa-circle-info', label: 'Info' },
  { class: 'fa-solid fa-bell', label: 'Bell' },
  { class: 'fa-solid fa-cart-shopping', label: 'Cart' },
  { class: 'fa-solid fa-download', label: 'Download' },
  { class: 'fa-solid fa-arrow-right', label: 'Arrow Right' },
  { class: 'fa-solid fa-arrow-up', label: 'Arrow Up' },
  { class: 'fa-solid fa-check-circle', label: 'Check Circle' },
  { class: 'fa-solid fa-play', label: 'Play' },
  { class: 'fa-solid fa-lock', label: 'Lock' },
  { class: 'fa-solid fa-eye', label: 'Eye' },
  { class: 'fa-brands fa-github', label: 'GitHub' },
  { class: 'fa-brands fa-twitter', label: 'Twitter' },
  { class: 'fa-brands fa-facebook', label: 'Facebook' },
  { class: 'fa-brands fa-instagram', label: 'Instagram' },
  { class: 'fa-brands fa-linkedin', label: 'LinkedIn' },
  { class: 'fa-brands fa-youtube', label: 'YouTube' },
]

const styleSections = [
  {
    name: 'Background',
    props: [
      { key: 'backgroundColor', label: 'Background Color', type: 'color', def: '#ffffff' },
      { key: 'backgroundImage', label: 'Background Image URL', type: 'text', def: '' },
      { key: 'backgroundRepeat', label: 'Repeat', type: 'select', def: 'no-repeat', opts: ['no-repeat', 'repeat', 'repeat-x', 'repeat-y'] },
      { key: 'backgroundSize', label: 'Size', type: 'select', def: 'cover', opts: ['cover', 'contain', 'auto', '100% 100%'] },
      { key: 'backgroundPosition', label: 'Position', type: 'select', def: 'center', opts: ['center', 'top', 'bottom', 'left', 'right', 'top left', 'top right', 'bottom left', 'bottom right'] },
    ]
  },
  {
    name: 'Typography',
    props: [
      { key: 'fontFamily', label: 'Font Family', type: 'select', def: 'Inter', opts: ['Inter', 'system-ui', 'serif', 'monospace', 'Arial', 'Georgia', 'Times New Roman', 'Courier New'] },
      { key: 'fontSize', label: 'Font Size', type: 'text', def: '16px' },
      { key: 'fontWeight', label: 'Font Weight', type: 'select', def: '400', opts: ['100','200','300','400','500','600','700','800','900'] },
      { key: 'lineHeight', label: 'Line Height', type: 'text', def: '1.5' },
      { key: 'letterSpacing', label: 'Letter Spacing', type: 'text', def: '0px' },
      { key: 'wordSpacing', label: 'Word Spacing', type: 'text', def: '0px' },
      { key: 'textAlign', label: 'Text Align', type: 'select', def: 'left', opts: ['left', 'center', 'right', 'justify'] },
      { key: 'textTransform', label: 'Text Transform', type: 'select', def: 'none', opts: ['none', 'uppercase', 'lowercase', 'capitalize'] },
      { key: 'textDecoration', label: 'Text Decoration', type: 'select', def: 'none', opts: ['none', 'underline', 'line-through', 'overline'] },
      { key: 'fontStyle', label: 'Font Style', type: 'select', def: 'normal', opts: ['normal', 'italic', 'oblique'] },
      { key: 'color', label: 'Text Color', type: 'color', def: '#000000' },
    ]
  },
  {
    name: 'Spacing',
    props: [
      { key: 'paddingTop', label: 'Padding Top', type: 'text', def: '0px' },
      { key: 'paddingRight', label: 'Padding Right', type: 'text', def: '0px' },
      { key: 'paddingBottom', label: 'Padding Bottom', type: 'text', def: '0px' },
      { key: 'paddingLeft', label: 'Padding Left', type: 'text', def: '0px' },
      { key: 'marginTop', label: 'Margin Top', type: 'text', def: '0px' },
      { key: 'marginRight', label: 'Margin Right', type: 'text', def: '0px' },
      { key: 'marginBottom', label: 'Margin Bottom', type: 'text', def: '0px' },
      { key: 'marginLeft', label: 'Margin Left', type: 'text', def: '0px' },
    ]
  },
  {
    name: 'Border',
    props: [
      { key: 'borderWidth', label: 'Border Width', type: 'text', def: '1px' },
      { key: 'borderStyle', label: 'Border Style', type: 'select', def: 'solid', opts: ['none', 'solid', 'dashed', 'dotted', 'double', 'groove', 'ridge'] },
      { key: 'borderColor', label: 'Border Color', type: 'color', def: '#e5e7eb' },
      { key: 'borderRadius', label: 'Border Radius', type: 'text', def: '8px' },
      { key: 'borderTopLeftRadius', label: 'Radius Top-Left', type: 'text', def: '' },
      { key: 'borderTopRightRadius', label: 'Radius Top-Right', type: 'text', def: '' },
      { key: 'borderBottomRightRadius', label: 'Radius Bottom-Right', type: 'text', def: '' },
      { key: 'borderBottomLeftRadius', label: 'Radius Bottom-Left', type: 'text', def: '' },
    ]
  },
  {
    name: 'Shadow',
    props: [
      { key: 'boxShadow', label: 'Box Shadow', type: 'select', def: 'none', opts: [
        { label: 'None', value: 'none' },
        { label: 'Small', value: '0 1px 2px 0 rgb(0 0 0 / 0.05)' },
        { label: 'Medium', value: '0 4px 6px -1px rgb(0 0 0 / 0.1)' },
        { label: 'Large', value: '0 10px 15px -3px rgb(0 0 0 / 0.1)' },
        { label: 'X-Large', value: '0 20px 25px -5px rgb(0 0 0 / 0.1)' },
        { label: 'Custom', value: '' },
      ] },
      { key: 'textShadow', label: 'Text Shadow', type: 'text', def: 'none' },
    ]
  },
  {
    name: 'Layout & Sizing',
    props: [
      { key: 'width', label: 'Width', type: 'text', def: '100%' },
      { key: 'minWidth', label: 'Min Width', type: 'text', def: '' },
      { key: 'maxWidth', label: 'Max Width', type: 'text', def: '1200px' },
      { key: 'height', label: 'Height', type: 'text', def: 'auto' },
      { key: 'minHeight', label: 'Min Height', type: 'text', def: '' },
      { key: 'maxHeight', label: 'Max Height', type: 'text', def: '' },
      { key: 'display', label: 'Display', type: 'select', def: 'block', opts: ['block', 'flex', 'grid', 'inline-block', 'inline', 'none'] },
      { key: 'flexDirection', label: 'Flex Direction', type: 'select', def: 'row', opts: ['row', 'column', 'row-reverse', 'column-reverse'] },
      { key: 'alignItems', label: 'Align Items', type: 'select', def: 'stretch', opts: ['stretch', 'flex-start', 'flex-end', 'center', 'baseline'] },
      { key: 'justifyContent', label: 'Justify Content', type: 'select', def: 'flex-start', opts: ['flex-start', 'flex-end', 'center', 'space-between', 'space-around', 'space-evenly'] },
      { key: 'gap', label: 'Gap', type: 'text', def: '0px' },
    ]
  },
  {
    name: 'Position',
    props: [
      { key: 'position', label: 'Position', type: 'select', def: 'static', opts: ['static', 'relative', 'absolute', 'fixed', 'sticky'] },
      { key: 'top', label: 'Top', type: 'text', def: 'auto' },
      { key: 'right', label: 'Right', type: 'text', def: 'auto' },
      { key: 'bottom', label: 'Bottom', type: 'text', def: 'auto' },
      { key: 'left', label: 'Left', type: 'text', def: 'auto' },
      { key: 'zIndex', label: 'Z-Index', type: 'number', def: 1 },
    ]
  },
  {
    name: 'Effects',
    props: [
      { key: 'opacity', label: 'Opacity', type: 'range', def: 1, min: 0, max: 1, step: 0.05 },
      { key: 'transform', label: 'Transform', type: 'text', def: 'none' },
      { key: 'transition', label: 'Transition', type: 'text', def: 'all 0.3s ease' },
      { key: 'cursor', label: 'Cursor', type: 'select', def: 'auto', opts: ['auto', 'pointer', 'default', 'text', 'move', 'not-allowed', 'wait', 'crosshair', 'zoom-in', 'grab'] },
      { key: 'overflow', label: 'Overflow', type: 'select', def: 'visible', opts: ['visible', 'hidden', 'scroll', 'auto'] },
      { key: 'overflowX', label: 'Overflow-X', type: 'select', def: 'visible', opts: ['visible', 'hidden', 'scroll', 'auto'] },
      { key: 'overflowY', label: 'Overflow-Y', type: 'select', def: 'visible', opts: ['visible', 'hidden', 'scroll', 'auto'] },
    ]
  },
  {
    name: 'Filter',
    props: [
      { key: 'filter', label: 'CSS Filter', type: 'text', def: 'none', placeholder: 'blur(2px) brightness(0.8)' },
      { key: 'backdropFilter', label: 'Backdrop Filter', type: 'text', def: 'none', placeholder: 'blur(4px)' },
    ]
  },
]

function hasStyle(key: string) {
  return key in localStyles.value
}

function toggleStyle(key: string, def: any) {
  if (hasStyle(key)) {
    delete localStyles.value[key]
    localStyles.value = { ...localStyles.value }
  } else {
    localStyles.value[key] = def
  }
  onStyleChange()
}

function styleExists(key: string) {
  return key in localStyles.value && localStyles.value[key] !== '' && localStyles.value[key] !== undefined
}

const openSections = ref<Set<string>>(new Set(['Colors', 'Typography', 'Spacing']))

function toggleSection(name: string) {
  if (openSections.value.has(name)) openSections.value.delete(name)
  else openSections.value.add(name)
  openSections.value = new Set(openSections.value)
}

function sectionOpen(name: string) {
  return openSections.value.has(name)
}
</script>

<template>
  <!-- Empty state -->
  <div v-if="!comp" class="h-full flex items-center justify-center p-6">
    <div class="text-center">
      <svg class="mx-auto w-12 h-12 text-gray-300 dark:text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.066 2.573c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.573 1.066c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.066-2.573c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
      </svg>
      <p class="mt-3 text-sm text-muted-foreground">Select a component on the canvas to edit all its properties</p>
    </div>
  </div>

  <!-- Panel -->
  <div v-else class="h-full flex flex-col">
    <!-- Header -->
    <div class="px-4 py-3 border-b border-border shrink-0">
      <div class="flex items-center gap-2">
        <span class="w-6 h-6 rounded bg-muted flex items-center justify-center text-[10px] font-bold text-muted-foreground uppercase">{{ comp.type.slice(0, 2) }}</span>
        <div>
          <h3 class="text-sm font-semibold text-foreground">{{ comp.type }}</h3>
          <p class="text-xs text-muted-foreground font-mono">{{ comp.id.slice(0, 16) }}..</p>
        </div>
      </div>
    </div>

    <!-- Tabs -->
    <div class="flex border-b border-border shrink-0">
      <button v-for="tab in ['content', 'style']" :key="tab"
        class="flex-1 py-2.5 text-xs font-semibold uppercase tracking-wider transition-colors"
        :class="activeTab === tab ? 'text-primary-600 dark:text-primary-400 border-b-2 border-primary-500' : 'text-gray-400 dark:text-gray-500 hover:text-gray-600 dark:hover:text-gray-300'"
        @click="activeTab = tab">
        {{ tab === 'content' ? 'Content' : 'Style' }}
      </button>
    </div>

    <!-- Body -->
    <div class="flex-1 overflow-y-auto">

      <!-- ===== CONTENT TAB ===== -->
      <div v-if="activeTab === 'content'" class="p-4 space-y-3">

        <!-- Text -->
        <template v-if="comp.type === 'Text'">
          <div><label class="text-xs font-medium text-muted-foreground mb-1.5 block">Content</label><textarea :value="localProps.content as string" rows="4" class="input" @input="updateProp('content', ($event.target as HTMLTextAreaElement).value)" /></div>
          <div><label class="text-xs font-medium text-muted-foreground mb-1.5 block">HTML Tag</label>
            <select :value="localProps.tag as string" class="input" @change="updateProp('tag', ($event.target as HTMLSelectElement).value)">
              <option v-for="t in ['p','h1','h2','h3','h4','h5','h6','span','div']" :key="t" :value="t">{{ t }}</option>
            </select>
          </div>
        </template>

        <!-- Button -->
        <template v-if="comp.type === 'Button'">
          <div><label class="text-xs font-medium text-muted-foreground mb-1.5 block">Button Text</label><input :value="localProps.label as string" type="text" class="input" @input="updateProp('label', ($event.target as HTMLInputElement).value)"></div>
          <div><label class="text-xs font-medium text-muted-foreground mb-1.5 block">Link URL</label><input :value="localProps.link as string" type="text" class="input" placeholder="https://" @input="updateProp('link', ($event.target as HTMLInputElement).value)"></div>
        </template>

        <!-- Image -->
        <template v-if="comp.type === 'Image'">
          <div><label class="text-xs font-medium text-muted-foreground mb-1.5 block">Image URL</label><input :value="localProps.src as string" type="url" class="input" placeholder="https://..." @input="updateProp('src', ($event.target as HTMLInputElement).value)"></div>
          <div><label class="text-xs font-medium text-muted-foreground mb-1.5 block">Alt Text</label><input :value="localProps.alt as string" type="text" class="input" @input="updateProp('alt', ($event.target as HTMLInputElement).value)"></div>
        </template>

        <!-- Hero -->
        <template v-if="comp.type === 'Hero'">
          <div><label class="text-xs font-medium text-muted-foreground mb-1.5 block">Heading</label><input :value="localProps.heading as string" type="text" class="input" @input="updateProp('heading', ($event.target as HTMLInputElement).value)"></div>
          <div><label class="text-xs font-medium text-muted-foreground mb-1.5 block">Subheading</label><textarea :value="localProps.subheading as string" rows="2" class="input" @input="updateProp('subheading', ($event.target as HTMLTextAreaElement).value)" /></div>
          <div><label class="text-xs font-medium text-muted-foreground mb-1.5 block">CTA Text</label><input :value="localProps.ctaText as string" type="text" class="input" @input="updateProp('ctaText', ($event.target as HTMLInputElement).value)"></div>
        </template>

        <!-- Header -->
        <template v-if="comp.type === 'Header'">
          <div><label class="text-xs font-medium text-muted-foreground mb-1.5 block">Logo Text</label><input :value="localProps.logo as string" type="text" class="input" @input="updateProp('logo', ($event.target as HTMLInputElement).value)"></div>
          <div>
            <div class="flex items-center justify-between mb-1.5">
              <label class="text-xs font-medium text-muted-foreground">Menu Items</label>
              <button class="text-[10px] text-primary-600 hover:text-primary-700 font-medium" @click="addItem('menuItems', '')">+ Add</button>
            </div>
            <div v-for="(item, i) in (localProps.menuItems as string[]) || []" :key="i" class="flex items-center gap-1.5 mb-1">
              <input :value="item" type="text" class="input flex-1 text-xs" @input="updateStrItem('menuItems', i, $event)">
              <button class="text-muted-foreground hover:text-red-500 shrink-0 p-0.5" @click="removeItem('menuItems', i)"><svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" /></svg></button>
            </div>
          </div>
        </template>

        <!-- Features -->
        <template v-if="comp.type === 'Features'">
          <div><label class="text-xs font-medium text-muted-foreground mb-1.5 block">Title</label><input :value="localProps.title as string" type="text" class="input" @input="updateProp('title', ($event.target as HTMLInputElement).value)"></div>
          <div><label class="text-xs font-medium text-muted-foreground mb-1.5 block">Subtitle</label><input :value="localProps.subtitle as string" type="text" class="input" @input="updateProp('subtitle', ($event.target as HTMLInputElement).value)"></div>
          <div>
            <div class="flex items-center justify-between mb-1.5">
              <label class="text-xs font-medium text-muted-foreground">Columns</label>
              <button class="text-[10px] text-primary-600 hover:text-primary-700 font-medium" @click="addItem('columns', { title: '', description: '' })">+ Add</button>
            </div>
            <div v-for="(col, i) in (localProps.columns as any[]) || []" :key="i" class="p-3 rounded-lg bg-muted mb-2">
              <div class="flex items-center justify-between mb-1.5">
                <span class="text-[10px] font-semibold text-muted-foreground uppercase">Column {{ i+1 }}</span>
                <button class="text-muted-foreground hover:text-red-500 p-0.5" @click="removeItem('columns', i)"><svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" /></svg></button>
              </div>
              <input :value="col.title" type="text" class="input mb-1.5 text-xs" placeholder="Title" @input="updateObjItem('columns', i, 'title', ($event.target as HTMLInputElement).value)">
              <textarea :value="col.description" rows="2" class="input text-xs" placeholder="Description" @input="updateObjItem('columns', i, 'description', ($event.target as HTMLTextAreaElement).value)" />
            </div>
          </div>
        </template>

        <!-- Testimonials -->
        <template v-if="comp.type === 'Testimonials'">
          <div><label class="text-xs font-medium text-muted-foreground mb-1.5 block">Title</label><input :value="localProps.title as string" type="text" class="input" @input="updateProp('title', ($event.target as HTMLInputElement).value)"></div>
          <div><label class="text-xs font-medium text-muted-foreground mb-1.5 block">Subtitle</label><input :value="localProps.subtitle as string" type="text" class="input" @input="updateProp('subtitle', ($event.target as HTMLInputElement).value)"></div>
          <div>
            <div class="flex items-center justify-between mb-1.5">
              <label class="text-xs font-medium text-muted-foreground">Testimonials</label>
              <button class="text-[10px] text-primary-600 hover:text-primary-700 font-medium" @click="addItem('testimonials', { name: '', role: '', content: '' })">+ Add</button>
            </div>
            <div v-for="(t, i) in (localProps.testimonials as any[]) || []" :key="i" class="p-3 rounded-lg bg-muted mb-2">
              <div class="flex items-center justify-between mb-1.5">
                <span class="text-[10px] font-semibold text-muted-foreground uppercase">#{{ i+1 }}</span>
                <button class="text-muted-foreground hover:text-red-500 p-0.5" @click="removeItem('testimonials', i)"><svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" /></svg></button>
              </div>
              <input :value="t.name" type="text" class="input mb-1.5 text-xs" placeholder="Name" @input="updateObjItem('testimonials', i, 'name', ($event.target as HTMLInputElement).value)">
              <input :value="t.role" type="text" class="input mb-1.5 text-xs" placeholder="Role" @input="updateObjItem('testimonials', i, 'role', ($event.target as HTMLInputElement).value)">
              <textarea :value="t.content" rows="2" class="input text-xs" placeholder="Testimonial" @input="updateObjItem('testimonials', i, 'content', ($event.target as HTMLTextAreaElement).value)" />
            </div>
          </div>
        </template>

        <!-- Pricing -->
        <template v-if="comp.type === 'Pricing'">
          <div><label class="text-xs font-medium text-muted-foreground mb-1.5 block">Title</label><input :value="localProps.title as string" type="text" class="input" @input="updateProp('title', ($event.target as HTMLInputElement).value)"></div>
          <div><label class="text-xs font-medium text-muted-foreground mb-1.5 block">Subtitle</label><input :value="localProps.subtitle as string" type="text" class="input" @input="updateProp('subtitle', ($event.target as HTMLInputElement).value)"></div>
          <div>
            <div class="flex items-center justify-between mb-1.5">
              <label class="text-xs font-medium text-muted-foreground">Plans</label>
              <button class="text-[10px] text-primary-600 hover:text-primary-700 font-medium" @click="addItem('plans', { name: '', price: '$0', features: [], cta: 'Choose', highlighted: false })">+ Add</button>
            </div>
            <div v-for="(plan, i) in (localProps.plans as any[]) || []" :key="i" class="p-3 rounded-lg bg-muted mb-2">
              <div class="flex items-center justify-between mb-1.5">
                <span class="text-[10px] font-semibold text-muted-foreground uppercase">Plan {{ i+1 }}</span>
                <button class="text-muted-foreground hover:text-red-500 p-0.5" @click="removeItem('plans', i)"><svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" /></svg></button>
              </div>
              <input :value="plan.name" type="text" class="input mb-1.5 text-xs" placeholder="Plan Name" @input="updateObjItem('plans', i, 'name', ($event.target as HTMLInputElement).value)">
              <input :value="plan.price" type="text" class="input mb-1.5 text-xs" placeholder="$29" @input="updateObjItem('plans', i, 'price', ($event.target as HTMLInputElement).value)">
              <textarea :value="(plan.features || []).join('\n')" rows="3" class="input mb-1.5 text-xs" placeholder="One feature per line" @input="updateObjItem('plans', i, 'features', ($event.target as HTMLTextAreaElement).value.split('\n'))" />
              <label class="flex items-center gap-2 mt-1 text-xs text-muted-foreground">
                <input type="checkbox" :checked="plan.highlighted" @change="updateObjItem('plans', i, 'highlighted', ($event.target as HTMLInputElement).checked)">
                Featured plan
              </label>
            </div>
          </div>
        </template>

        <!-- FAQ -->
        <template v-if="comp.type === 'FAQ'">
          <div><label class="text-xs font-medium text-muted-foreground mb-1.5 block">Title</label><input :value="localProps.title as string" type="text" class="input" @input="updateProp('title', ($event.target as HTMLInputElement).value)"></div>
          <div><label class="text-xs font-medium text-muted-foreground mb-1.5 block">Subtitle</label><input :value="localProps.subtitle as string" type="text" class="input" @input="updateProp('subtitle', ($event.target as HTMLInputElement).value)"></div>
          <div>
            <div class="flex items-center justify-between mb-1.5">
              <label class="text-xs font-medium text-muted-foreground">FAQ Items</label>
              <button class="text-[10px] text-primary-600 hover:text-primary-700 font-medium" @click="addItem('items', { q: '', a: '' })">+ Add</button>
            </div>
            <div v-for="(faq, i) in (localProps.items as any[]) || []" :key="i" class="p-3 rounded-lg bg-muted mb-2">
              <div class="flex items-center justify-between mb-1.5">
                <span class="text-[10px] font-semibold text-muted-foreground uppercase">Q{{ i+1 }}</span>
                <button class="text-muted-foreground hover:text-red-500 p-0.5" @click="removeItem('items', i)"><svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" /></svg></button>
              </div>
              <input :value="faq.q" type="text" class="input mb-1.5 text-xs" placeholder="Question" @input="updateObjItem('items', i, 'q', ($event.target as HTMLInputElement).value)">
              <textarea :value="faq.a" rows="2" class="input text-xs" placeholder="Answer" @input="updateObjItem('items', i, 'a', ($event.target as HTMLTextAreaElement).value)" />
            </div>
          </div>
        </template>

        <!-- Contact -->
        <template v-if="comp.type === 'Contact'">
          <div><label class="text-xs font-medium text-muted-foreground mb-1.5 block">Title</label><input :value="localProps.title as string" type="text" class="input" @input="updateProp('title', ($event.target as HTMLInputElement).value)"></div>
          <div><label class="text-xs font-medium text-muted-foreground mb-1.5 block">Subtitle</label><textarea :value="localProps.subtitle as string" rows="2" class="input" @input="updateProp('subtitle', ($event.target as HTMLTextAreaElement).value)" /></div>
          <div><label class="text-xs font-medium text-muted-foreground mb-1.5 block">Email</label><input :value="localProps.email as string" type="email" class="input" @input="updateProp('email', ($event.target as HTMLInputElement).value)"></div>
          <div><label class="text-xs font-medium text-muted-foreground mb-1.5 block">Phone</label><input :value="localProps.phone as string" type="text" class="input" @input="updateProp('phone', ($event.target as HTMLInputElement).value)"></div>
          <div><label class="text-xs font-medium text-muted-foreground mb-1.5 block">Address</label><textarea :value="localProps.address as string" rows="2" class="input" @input="updateProp('address', ($event.target as HTMLTextAreaElement).value)" /></div>
        </template>

        <!-- Stats -->
        <template v-if="comp.type === 'Stats'">
          <div><label class="text-xs font-medium text-muted-foreground mb-1.5 block">Title</label><input :value="localProps.title as string" type="text" class="input" @input="updateProp('title', ($event.target as HTMLInputElement).value)"></div>
          <div>
            <div class="flex items-center justify-between mb-1.5">
              <label class="text-xs font-medium text-muted-foreground">Stats</label>
              <button class="text-[10px] text-primary-600 hover:text-primary-700 font-medium" @click="addItem('stats', { value: '', label: '' })">+ Add</button>
            </div>
            <div v-for="(s, i) in (localProps.stats as any[]) || []" :key="i" class="p-3 rounded-lg bg-muted mb-2">
              <div class="flex items-center justify-between mb-1.5">
                <span class="text-[10px] font-semibold text-muted-foreground uppercase">Stat {{ i+1 }}</span>
                <button class="text-muted-foreground hover:text-red-500 p-0.5" @click="removeItem('stats', i)"><svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" /></svg></button>
              </div>
              <input :value="s.value" type="text" class="input mb-1.5 text-xs" placeholder="10K+" @input="updateObjItem('stats', i, 'value', ($event.target as HTMLInputElement).value)">
              <input :value="s.label" type="text" class="input text-xs" placeholder="Users" @input="updateObjItem('stats', i, 'label', ($event.target as HTMLInputElement).value)">
            </div>
          </div>
        </template>

        <!-- Footer -->
        <template v-if="comp.type === 'Footer'">
          <div><label class="text-xs font-medium text-muted-foreground mb-1.5 block">Copyright</label><input :value="localProps.copyright as string" type="text" class="input" @input="updateProp('copyright', ($event.target as HTMLInputElement).value)"></div>
          <div>
            <div class="flex items-center justify-between mb-1.5">
              <label class="text-xs font-medium text-muted-foreground">Links</label>
              <button class="text-[10px] text-primary-600 hover:text-primary-700 font-medium" @click="addItem('links', { label: '', url: '#' })">+ Add</button>
            </div>
            <div v-for="(link, i) in (localProps.links as any[]) || []" :key="i" class="p-3 rounded-lg bg-muted mb-2">
              <div class="flex items-center justify-between mb-1.5">
                <span class="text-[10px] font-semibold text-muted-foreground uppercase">Link {{ i+1 }}</span>
                <button class="text-muted-foreground hover:text-red-500 p-0.5" @click="removeItem('links', i)"><svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" /></svg></button>
              </div>
              <input :value="link.label" type="text" class="input mb-1.5 text-xs" placeholder="Label" @input="updateObjItem('links', i, 'label', ($event.target as HTMLInputElement).value)">
              <input :value="link.url" type="text" class="input text-xs" placeholder="URL" @input="updateObjItem('links', i, 'url', ($event.target as HTMLInputElement).value)">
            </div>
          </div>
        </template>

        <!-- Divider -->
        <template v-if="comp.type === 'Divider'">
          <div><label class="text-xs font-medium text-muted-foreground mb-1.5 block">Style</label>
            <select :value="localProps.style as string" class="input" @change="updateProp('style', ($event.target as HTMLSelectElement).value)">
              <option value="solid">Solid</option>
              <option value="dashed">Dashed</option>
              <option value="dotted">Dotted</option>
            </select>
          </div>
        </template>

        <!-- Form -->
        <template v-if="comp.type === 'Form'">
          <div><label class="text-xs font-medium text-muted-foreground mb-1.5 block">Title</label><input :value="localProps.title as string" type="text" class="input" @input="updateProp('title', ($event.target as HTMLInputElement).value)"></div>
          <div><label class="text-xs font-medium text-muted-foreground mb-1.5 block">Submit Button</label><input :value="localProps.submitText as string" type="text" class="input" @input="updateProp('submitText', ($event.target as HTMLInputElement).value)"></div>
        </template>

        <!-- Form Inputs -->
        <template v-if="comp.type === 'Input'">
          <div><label class="text-xs font-medium text-muted-foreground mb-1.5 block">Label</label><input :value="localProps.label as string" type="text" class="input" @input="updateProp('label', ($event.target as HTMLInputElement).value)"></div>
          <div><label class="text-xs font-medium text-muted-foreground mb-1.5 block">Placeholder</label><input :value="localProps.placeholder as string" type="text" class="input" @input="updateProp('placeholder', ($event.target as HTMLInputElement).value)"></div>
          <div><label class="text-xs font-medium text-muted-foreground mb-1.5 block">Type</label>
            <select :value="localProps.type as string" class="input" @change="updateProp('type', ($event.target as HTMLSelectElement).value)">
              <option v-for="t in ['text','email','password','url','tel','search']" :key="t" :value="t">{{ t }}</option>
            </select>
          </div>
          <div><label class="flex items-center gap-2 text-sm text-muted-foreground"><input type="checkbox" :checked="!!localProps.required" @change="updateProp('required', ($event.target as HTMLInputElement).checked)"> Required</label></div>
        </template>

        <template v-if="comp.type === 'Textarea'">
          <div><label class="text-xs font-medium text-muted-foreground mb-1.5 block">Label</label><input :value="localProps.label as string" type="text" class="input" @input="updateProp('label', ($event.target as HTMLInputElement).value)"></div>
          <div><label class="text-xs font-medium text-muted-foreground mb-1.5 block">Placeholder</label><input :value="localProps.placeholder as string" type="text" class="input" @input="updateProp('placeholder', ($event.target as HTMLInputElement).value)"></div>
          <div><label class="text-xs font-medium text-muted-foreground mb-1.5 block">Rows</label><input :value="localProps.rows as number" type="number" min="1" max="20" class="input" @input="updateProp('rows', parseInt(($event.target as HTMLInputElement).value))"></div>
          <div><label class="flex items-center gap-2 text-sm text-muted-foreground"><input type="checkbox" :checked="!!localProps.required" @change="updateProp('required', ($event.target as HTMLInputElement).checked)"> Required</label></div>
        </template>

        <template v-if="comp.type === 'Number'">
          <div><label class="text-xs font-medium text-muted-foreground mb-1.5 block">Label</label><input :value="localProps.label as string" type="text" class="input" @input="updateProp('label', ($event.target as HTMLInputElement).value)"></div>
          <div><label class="text-xs font-medium text-muted-foreground mb-1.5 block">Placeholder</label><input :value="localProps.placeholder as string" type="text" class="input" @input="updateProp('placeholder', ($event.target as HTMLInputElement).value)"></div>
          <div class="grid grid-cols-3 gap-2">
            <div><label class="text-[10px] text-muted-foreground block mb-1">Min</label><input :value="localProps.min as number" type="number" class="input text-xs" @input="updateProp('min', parseInt(($event.target as HTMLInputElement).value))"></div>
            <div><label class="text-[10px] text-muted-foreground block mb-1">Max</label><input :value="localProps.max as number" type="number" class="input text-xs" @input="updateProp('max', parseInt(($event.target as HTMLInputElement).value))"></div>
            <div><label class="text-[10px] text-muted-foreground block mb-1">Step</label><input :value="localProps.step as number" type="number" class="input text-xs" @input="updateProp('step', parseInt(($event.target as HTMLInputElement).value))"></div>
          </div>
          <div><label class="flex items-center gap-2 text-sm text-muted-foreground"><input type="checkbox" :checked="!!localProps.required" @change="updateProp('required', ($event.target as HTMLInputElement).checked)"> Required</label></div>
        </template>

        <template v-if="comp.type === 'Select'">
          <div><label class="text-xs font-medium text-muted-foreground mb-1.5 block">Label</label><input :value="localProps.label as string" type="text" class="input" @input="updateProp('label', ($event.target as HTMLInputElement).value)"></div>
          <div>
            <div class="flex items-center justify-between mb-1.5">
              <label class="text-xs font-medium text-muted-foreground">Options</label>
              <button class="text-[10px] text-primary-600 hover:text-primary-700 font-medium" @click="addItem('options', '')">+ Add</button>
            </div>
            <div v-for="(opt, i) in (localProps.options as string[]) || []" :key="i" class="flex items-center gap-1.5 mb-1">
              <input :value="opt" type="text" class="input flex-1 text-xs" @input="updateStrItem('options', i, $event)">
              <button class="text-muted-foreground hover:text-red-500 p-0.5" @click="removeItem('options', i)"><svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" /></svg></button>
            </div>
          </div>
          <div><label class="flex items-center gap-2 text-sm text-muted-foreground"><input type="checkbox" :checked="!!localProps.required" @change="updateProp('required', ($event.target as HTMLInputElement).checked)"> Required</label></div>
        </template>

        <template v-if="comp.type === 'Checkbox'">
          <div><label class="text-xs font-medium text-muted-foreground mb-1.5 block">Label</label><textarea :value="localProps.label as string" rows="2" class="input" @input="updateProp('label', ($event.target as HTMLTextAreaElement).value)" /></div>
          <div><label class="flex items-center gap-2 text-sm text-muted-foreground"><input type="checkbox" :checked="!!localProps.checked" @change="updateProp('checked', ($event.target as HTMLInputElement).checked)"> Checked by default</label></div>
        </template>

        <template v-if="comp.type === 'Radio'">
          <div><label class="text-xs font-medium text-muted-foreground mb-1.5 block">Label</label><input :value="localProps.label as string" type="text" class="input" @input="updateProp('label', ($event.target as HTMLInputElement).value)"></div>
          <div>
            <div class="flex items-center justify-between mb-1.5">
              <label class="text-xs font-medium text-muted-foreground">Options</label>
              <button class="text-[10px] text-primary-600 hover:text-primary-700 font-medium" @click="addItem('options', '')">+ Add</button>
            </div>
            <div v-for="(opt, i) in (localProps.options as string[]) || []" :key="i" class="flex items-center gap-1.5 mb-1">
              <input :value="opt" type="text" class="input flex-1 text-xs" @input="updateStrItem('options', i, $event)">
              <button class="text-muted-foreground hover:text-red-500 p-0.5" @click="removeItem('options', i)"><svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" /></svg></button>
            </div>
          </div>
          <div><label class="text-xs font-medium text-muted-foreground mb-1.5 block">Default Selection</label>
            <select :value="localProps.selected as string" class="input" @change="updateProp('selected', ($event.target as HTMLSelectElement).value)">
              <option v-for="opt in (localProps.options as string[]) || []" :key="opt" :value="opt">{{ opt }}</option>
            </select>
          </div>
        </template>

        <template v-if="comp.type === 'Range'">
          <div><label class="text-xs font-medium text-muted-foreground mb-1.5 block">Label</label><input :value="localProps.label as string" type="text" class="input" @input="updateProp('label', ($event.target as HTMLInputElement).value)"></div>
          <div class="grid grid-cols-3 gap-2">
            <div><label class="text-[10px] text-muted-foreground block mb-1">Min</label><input :value="localProps.min as number" type="number" class="input text-xs" @input="updateProp('min', parseInt(($event.target as HTMLInputElement).value))"></div>
            <div><label class="text-[10px] text-muted-foreground block mb-1">Max</label><input :value="localProps.max as number" type="number" class="input text-xs" @input="updateProp('max', parseInt(($event.target as HTMLInputElement).value))"></div>
            <div><label class="text-[10px] text-muted-foreground block mb-1">Step</label><input :value="localProps.step as number" type="number" class="input text-xs" @input="updateProp('step', parseInt(($event.target as HTMLInputElement).value))"></div>
          </div>
          <div><label class="text-xs font-medium text-muted-foreground mb-1.5 block">Default Value</label><input :value="localProps.value as number" type="number" class="input" @input="updateProp('value', parseInt(($event.target as HTMLInputElement).value))"></div>
        </template>

        <template v-if="comp.type === 'Date'">
          <div><label class="text-xs font-medium text-muted-foreground mb-1.5 block">Label</label><input :value="localProps.label as string" type="text" class="input" @input="updateProp('label', ($event.target as HTMLInputElement).value)"></div>
          <div><label class="flex items-center gap-2 text-sm text-muted-foreground"><input type="checkbox" :checked="!!localProps.required" @change="updateProp('required', ($event.target as HTMLInputElement).checked)"> Required</label></div>
        </template>

        <!-- Link -->
        <template v-if="comp.type === 'Link'">
          <div><label class="text-xs font-medium text-muted-foreground mb-1.5 block">Text</label><input :value="localProps.text as string" type="text" class="input" @input="updateProp('text', ($event.target as HTMLInputElement).value)"></div>
          <div><label class="text-xs font-medium text-muted-foreground mb-1.5 block">URL</label><input :value="localProps.url as string" type="text" class="input" placeholder="https://" @input="updateProp('url', ($event.target as HTMLInputElement).value)"></div>
        </template>

        <!-- Icon -->
        <template v-if="comp.type === 'Icon'">
          <div><label class="text-xs font-medium text-muted-foreground mb-2 block">Icon</label>
            <div class="grid grid-cols-6 gap-1 mb-3">
              <button v-for="ico in iconOptions" :key="ico.class" :title="ico.label" class="w-full aspect-square rounded-md border flex items-center justify-center text-sm transition-all hover:border-primary-500 hover:bg-primary-50 dark:hover:bg-primary-900/20" :class="(localProps.className as string) === ico.class ? 'border-primary-500 bg-primary-50 dark:bg-primary-900/20 ring-1 ring-primary-500' : 'border-gray-200 dark:border-gray-700'" @click="updateProp('className', ico.class)">
                <i :class="ico.class" :style="{ fontSize: '14px', color: (localProps.className as string) === ico.class ? (localProps.color as string) || 'currentColor' : 'currentColor' }"></i>
              </button>
            </div>
          </div>
          <div class="flex gap-2">
            <div class="flex-1"><label class="text-xs font-medium text-muted-foreground mb-1.5 block">Size (px)</label><input :value="localProps.size as number" type="number" class="input" @input="updateProp('size', parseInt(($event.target as HTMLInputElement).value))"></div>
            <div class="flex-1"><label class="text-xs font-medium text-muted-foreground mb-1.5 block">Color</label><input :value="localProps.color as string" type="color" class="w-full h-9 rounded-md border border-input cursor-pointer" @input="updateProp('color', ($event.target as HTMLInputElement).value)"></div>
          </div>
        </template>

        <!-- Container -->
        <template v-if="comp.type === 'Container'">
          <div>
            <!-- Alignment -->
            <div class="mb-3">
              <label class="text-xs font-semibold text-muted-foreground uppercase mb-1.5 block">Alignment</label>
              <div class="flex rounded-lg border border-input overflow-hidden">
                <button
                  v-for="a in ['left', 'center', 'right']" :key="a"
                  class="flex-1 py-1.5 text-xs font-medium transition-colors"
                  :class="(localProps.align as string) === a ? 'bg-primary text-white' : 'bg-transparent text-gray-500 hover:bg-gray-100 dark:hover:bg-gray-800'"
                  @click="updateProp('align', a)"
                >
                  <svg v-if="a === 'left'" class="w-3.5 h-3.5 mx-auto" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 10h10M4 14h16M4 18h10" /></svg>
                  <svg v-else-if="a === 'center'" class="w-3.5 h-3.5 mx-auto" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M8 10h8M4 14h16M8 18h8" /></svg>
                  <svg v-else class="w-3.5 h-3.5 mx-auto" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M10 10h10M4 14h16M10 18h10" /></svg>
                </button>
              </div>
            </div>

            <!-- Children info -->
            <div v-if="comp.children?.length" class="mb-3 px-3 py-2 rounded-lg bg-primary/10 border border-primary/20 text-xs text-primary text-center">
              {{ comp.children.length }} component(s) inside — click on them in the canvas to edit
            </div>

            <div class="separator my-3" />

            <!-- Header -->
            <div class="flex items-center justify-between mb-2">
              <label class="text-xs font-semibold text-muted-foreground uppercase">
                <template v-if="isEditingItem">Item #{{ selectedItemIndex! + 1 }}</template>
                <template v-else>Items</template>
              </label>
              <div class="flex items-center gap-1">
                <button v-if="isEditingItem" class="text-[10px] text-muted-foreground hover:text-foreground font-medium" @click="selectContainerItem(null)">← Container</button>
                <button v-else class="text-[10px] text-primary-600 hover:text-primary-700 font-medium" @click="addItem('items', { type: 'text', content: '' })">+ Add Item</button>
              </div>
            </div>

            <!-- Item editor (single item selected) -->
            <template v-if="isEditingItem && editingItemData">
              <div class="space-y-2">
                <select :value="editingItemData.type" class="input text-xs" @change="updateObjItem('items', selectedItemIndex!, 'type', ($event.target as HTMLSelectElement).value)">
                  <option value="text">Text</option>
                  <option value="link">Link</option>
                  <option value="icon">Icon</option>
                  <option value="button">Button</option>
                  <option value="divider">Divider</option>
                </select>
                <input v-if="['text','link','button'].includes(editingItemData.type)" :value="editingItemData.content || editingItemData.text" type="text" class="input text-xs" placeholder="Content" @input="updateObjItem('items', selectedItemIndex!, 'content', ($event.target as HTMLInputElement).value)">
                <input v-if="editingItemData.type === 'link'" :value="editingItemData.url" type="text" class="input text-xs" placeholder="URL" @input="updateObjItem('items', selectedItemIndex!, 'url', ($event.target as HTMLInputElement).value)">
                <div v-if="editingItemData.type === 'icon'" class="space-y-2">
                  <label class="text-xs font-medium text-muted-foreground block">Icon</label>
                  <div class="grid grid-cols-6 gap-1">
                    <button v-for="ico in iconOptions" :key="ico.class" :title="ico.label" class="w-full aspect-square rounded-md border flex items-center justify-center text-sm transition-all hover:border-primary-500 hover:bg-primary-50 dark:hover:bg-primary-900/20" :class="(editingItemData.className as string) === ico.class ? 'border-primary-500 bg-primary-50 dark:bg-primary-900/20 ring-1 ring-primary-500' : 'border-gray-200 dark:border-gray-700'" @click="updateObjItem('items', selectedItemIndex!, 'className', ico.class)">
                      <i :class="ico.class" :style="{ fontSize: '14px', color: (editingItemData.className as string) === ico.class ? (editingItemData.color as string) || 'currentColor' : 'currentColor' }"></i>
                    </button>
                  </div>
                  <div class="flex gap-2">
                    <div class="flex-1"><label class="text-xs font-medium text-muted-foreground mb-1 block">Size (px)</label><input :value="editingItemData.size" type="number" class="input text-xs" placeholder="24" @input="updateObjItem('items', selectedItemIndex!, 'size', parseInt(($event.target as HTMLInputElement).value))"></div>
                    <div class="flex-1"><label class="text-xs font-medium text-muted-foreground mb-1 block">Color</label><input :value="editingItemData.color" type="color" class="w-full h-9 rounded-md border border-input cursor-pointer" @input="updateObjItem('items', selectedItemIndex!, 'color', ($event.target as HTMLInputElement).value)"></div>
                  </div>
                </div>
              </div>
            </template>

            <!-- Items list (no item selected) -->
            <template v-if="!isEditingItem">
              <div v-for="(item, i) in (localProps.items as any[]) || []" :key="i" class="p-3 rounded-lg bg-muted mb-2 cursor-pointer hover:ring-2 hover:ring-primary-300 transition-all" @click="selectContainerItem(i)">
                <div class="flex items-center justify-between">
                  <div class="flex items-center gap-2 min-w-0 flex-1">
                    <svg class="w-3.5 h-3.5 text-gray-400 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" /></svg>
                    <span class="text-xs text-foreground truncate">
                      {{ item.type === 'text' ? (item.content || 'Text') : item.type === 'link' ? (item.content || item.text || 'Link') : item.type === 'icon' ? (item.className || 'Icon') : item.type === 'button' ? (item.content || 'Button') : 'Divider' }}
                    </span>
                  </div>
                  <button class="text-muted-foreground hover:text-red-500 p-0.5 shrink-0" @click.stop="removeItem('items', i)">
                    <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" /></svg>
                  </button>
                </div>
              </div>
              <div v-if="!((localProps.items as any[])?.length)" class="text-center py-4 text-xs text-muted-foreground">No items yet.</div>
            </template>
          </div>
        </template>

        <!-- Fallback -->
        <div v-if="!['Text','Button','Image','Hero','Header','Features','Testimonials','Pricing','FAQ','Contact','Stats','Footer','Divider','Form','Input','Textarea','Number','Select','Checkbox','Radio','Range','Date','Link','Icon','Container'].includes(comp.type)" class="text-center py-8 text-xs text-muted-foreground">
          No editable properties for this component
        </div>
      </div>

      <!-- ===== STYLE TAB (enable/disable system) ===== -->
      <div v-if="activeTab === 'style'" class="p-3 space-y-1">
        <p class="text-[10px] text-muted-foreground mb-3 px-1">Toggle each property ON to enable it with a default value</p>

          <div v-for="section in styleSections" :key="section.name" class="mb-1">
            <div class="flex items-center gap-2 px-1 py-2 rounded-lg hover:bg-accent cursor-pointer" @click="toggleSection(section.name)">
              <svg class="w-3 h-3 text-gray-400 transition-transform" :class="sectionOpen(section.name) ? 'rotate-90' : ''" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" /></svg>
              <span class="text-xs font-semibold text-muted-foreground uppercase tracking-wider">{{ section.name }}</span>
            </div>

            <div v-if="sectionOpen(section.name)" class="ml-1 pl-3 border-l-2 border-border space-y-2 py-2">
              <div v-for="prop in section.props" :key="prop.key" class="flex items-start gap-2">
                <input type="checkbox" :checked="styleExists(prop.key)" class="mt-2 shrink-0 rounded border-input accent-primary-500" @change="toggleStyle(prop.key, prop.def)">
                <div class="flex-1 min-w-0" v-if="styleExists(prop.key)">
                  <label class="text-[10px] text-muted-foreground block mb-0.5">{{ prop.label }}</label>
                  <div v-if="prop.type === 'color'" class="flex gap-1">
                    <input type="color" :value="localStyles[prop.key] as string" class="w-8 h-8 rounded border border-input cursor-pointer shrink-0" @input="setStyleVal(prop.key, ($event.target as HTMLInputElement).value)">
                    <input :value="localStyles[prop.key] as string" type="text" class="input text-xs flex-1" @input="setStyleVal(prop.key, ($event.target as HTMLInputElement).value)">
                  </div>
                  <select v-else-if="prop.type === 'select'" :value="localStyles[prop.key] as string" class="input text-xs" @change="setStyleVal(prop.key, ($event.target as HTMLSelectElement).value)">
                    <option v-for="opt in prop.opts" :key="typeof opt === 'string' ? opt : opt.value" :value="typeof opt === 'string' ? opt : opt.value">
                      {{ typeof opt === 'string' ? opt : opt.label }}
                    </option>
                  </select>
                  <div v-else-if="prop.type === 'range'" class="flex items-center gap-2">
                    <input type="range" :min="prop.min ?? 0" :max="prop.max ?? 1" :step="prop.step ?? 0.1" :value="localStyles[prop.key] as number" class="flex-1 accent-primary-500 h-1" @input="setStyleVal(prop.key, parseFloat(($event.target as HTMLInputElement).value))">
                    <span class="text-xs text-muted-foreground w-8 text-right font-mono">{{ localStyles[prop.key] }}</span>
                  </div>
                  <input v-else-if="prop.type === 'number'" :value="localStyles[prop.key] as number" type="number" class="input text-xs" @input="setStyleVal(prop.key, parseInt(($event.target as HTMLInputElement).value))">
                  <input v-else :value="localStyles[prop.key] as string" type="text" class="input text-xs" :placeholder="(prop as any).placeholder || ''" @input="setStyleVal(prop.key, ($event.target as HTMLInputElement).value)">
                </div>
                <div v-else class="flex-1 py-1.5">
                  <span class="text-xs text-muted-foreground">{{ prop.label }}</span>
                </div>
              </div>
            </div>
          </div>

          <!-- Animation Section -->
          <div class="mb-1 mt-4">
            <div class="flex items-center gap-2 px-1 py-2 rounded-lg hover:bg-accent cursor-pointer" @click="toggleSection('ANIMATIONS')">
              <svg class="w-3 h-3 text-gray-400 transition-transform" :class="sectionOpen('ANIMATIONS') ? 'rotate-90' : ''" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" /></svg>
              <span class="text-xs font-semibold text-muted-foreground uppercase tracking-wider">Animations</span>
            </div>
            <div v-if="sectionOpen('ANIMATIONS')" class="ml-1 pl-3 border-l-2 border-border space-y-3 py-2">
              <div class="flex items-start gap-2">
                <input type="checkbox" :checked="animationEnabled" class="mt-2 shrink-0 rounded border-input accent-primary-500" @change="setAnimation(($event.target as HTMLInputElement).checked)">
                <div class="flex-1 min-w-0" v-if="animationEnabled">
                  <label class="text-[10px] text-muted-foreground block mb-0.5">Type</label>
                  <select :value="localAnimations[0]?.type" class="input text-xs" @change="updateAnimationField('type', ($event.target as HTMLSelectElement).value)">
                    <option value="fade">Fade</option>
                    <option value="slide">Slide</option>
                    <option value="bounce">Bounce</option>
                    <option value="rotate">Rotate</option>
                    <option value="scale">Scale</option>
                  </select>
                </div>
                <div v-else class="flex-1 py-1.5">
                  <span class="text-xs text-muted-foreground">Animate on scroll</span>
                </div>
              </div>
              <template v-if="animationEnabled">
                <div>
                  <label class="text-[10px] text-muted-foreground block mb-0.5">Duration (ms)</label>
                  <div class="flex items-center gap-2">
                    <input type="range" min="300" max="3000" step="100" :value="localAnimations[0]?.duration ?? 800" class="flex-1 accent-primary-500 h-1" @input="updateAnimationField('duration', parseInt(($event.target as HTMLInputElement).value))">
                    <span class="text-xs text-muted-foreground w-9 text-right font-mono">{{ localAnimations[0]?.duration }}ms</span>
                  </div>
                </div>
                <div>
                  <label class="text-[10px] text-muted-foreground block mb-0.5">Delay (ms)</label>
                  <div class="flex items-center gap-2">
                    <input type="range" min="0" max="2000" step="50" :value="localAnimations[0]?.delay ?? 0" class="flex-1 accent-primary-500 h-1" @input="updateAnimationField('delay', parseInt(($event.target as HTMLInputElement).value))">
                    <span class="text-xs text-muted-foreground w-9 text-right font-mono">{{ localAnimations[0]?.delay }}ms</span>
                  </div>
                </div>
                <div>
                  <label class="text-[10px] text-muted-foreground block mb-0.5">Easing</label>
                  <select :value="localAnimations[0]?.easing" class="input text-xs" @change="updateAnimationField('easing', ($event.target as HTMLSelectElement).value)">
                    <option value="linear">Linear</option>
                    <option value="ease">Ease</option>
                    <option value="ease-in">Ease In</option>
                    <option value="ease-out">Ease Out</option>
                    <option value="ease-in-out">Ease In-Out</option>
                  </select>
                </div>
                <div>
                  <label class="text-[10px] text-muted-foreground block mb-0.5">Trigger</label>
                  <select :value="localAnimations[0]?.trigger" class="input text-xs" @change="updateAnimationField('trigger', ($event.target as HTMLSelectElement).value)">
                    <option value="scroll">Scroll</option>
                    <option value="load">Load</option>
                    <option value="hover">Hover</option>
                    <option value="click">Click</option>
                  </select>
                </div>
              </template>
            </div>
          </div>

          <button class="w-full mt-3 py-2 text-xs text-red-500 hover:text-red-600 font-medium transition-colors rounded-lg hover:bg-red-50 dark:hover:bg-red-900/20" @click="localStyles = {}; onStyleChange()">Reset All Styles</button>
      </div>
    </div>
  </div>
</template>
