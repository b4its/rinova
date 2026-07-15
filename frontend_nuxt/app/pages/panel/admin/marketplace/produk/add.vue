<script setup lang="ts">
definePageMeta({ layout: 'panel', sidebarSection: 'admin-mk-produk' })

interface Kategori { id: number; name: string }

const kategories = ref<Kategori[]>([])
const formName = ref('')
const formKategori = ref('')
const formDescriptions = ref('')
const formVisual = ref('')
const formPrice = ref(0)
const formHtmlCode = ref('')
const viewportMode = ref<'desktop' | 'tablet' | 'mobile'>('desktop')
const showFullPreview = ref(false)
const windowWidth = ref(1200)
const showSearch = ref(false)
const searchQuery = ref('')
const matches = ref<{ start: number; end: number }[]>([])
const currentMatch = ref(0)
const textareaRef = ref<HTMLTextAreaElement | null>(null)

function loadKategori() {
  const saved = localStorage.getItem('admin_marketplace_kategori')
  if (saved) {
    kategories.value = JSON.parse(saved)
  } else {
    kategories.value = [
      { id: 1, name: 'Templates' },
      { id: 2, name: 'Components' },
      { id: 3, name: 'Assets' },
    ]
  }
}
loadKategori()

onMounted(() => {
  if (!formKategori.value && kategories.value.length) {
    formKategori.value = kategories.value[0].name
  }
  windowWidth.value = window.innerWidth
  window.addEventListener('resize', () => { windowWidth.value = window.innerWidth })
  window.addEventListener('keydown', (e: KeyboardEvent) => {
    if ((e.ctrlKey || e.metaKey) && e.key === 'f' && textareaRef.value === document.activeElement) {
      e.preventDefault()
      showSearch.value = true
      nextTick(() => document.getElementById('search-input')?.focus())
    }
  })
})

const isTemplates = computed(() => formKategori.value === 'Templates')

function findInText() {
  if (!searchQuery.value || !formHtmlCode.value) {
    matches.value = []
    return
  }
  const q = searchQuery.value.toLowerCase()
  const text = formHtmlCode.value.toLowerCase()
  const found: { start: number; end: number }[] = []
  let pos = 0
  while (true) {
    const idx = text.indexOf(q, pos)
    if (idx === -1) break
    found.push({ start: idx, end: idx + q.length })
    pos = idx + q.length
  }
  matches.value = found
  currentMatch.value = found.length ? 0 : -1
}

function selectMatch() {
  const ta = textareaRef.value
  if (!ta || currentMatch.value < 0 || !matches.value.length) return
  const m = matches.value[currentMatch.value]
  ta.focus()
  ta.setSelectionRange(m.start, m.end)
  ta.scrollTop = ta.scrollHeight * (m.start / formHtmlCode.value.length)
}

function nextMatch() {
  if (!matches.value.length) return
  currentMatch.value = (currentMatch.value + 1) % matches.value.length
  selectMatch()
}

function prevMatch() {
  if (!matches.value.length) return
  currentMatch.value = (currentMatch.value - 1 + matches.value.length) % matches.value.length
  selectMatch()
}

function handleSearchKeydown(e: KeyboardEvent) {
  if (e.key === 'Enter') { e.preventDefault(); e.shiftKey ? prevMatch() : nextMatch() }
  if (e.key === 'Escape') { showSearch.value = false; searchQuery.value = '' }
}

function handleTextareaKeydown(e: KeyboardEvent) {
  if ((e.ctrlKey || e.metaKey) && e.key === 'f') {
    e.preventDefault(); showSearch.value = true
    nextTick(() => document.getElementById('search-input')?.focus())
  }
}

function toggleSearch() {
  showSearch.value = !showSearch.value
  if (showSearch.value) nextTick(() => document.getElementById('search-input')?.focus())
  else searchQuery.value = ''
}

function resizeIframe(e: Event) {
  const iframe = e.target as HTMLIFrameElement
  iframe.style.height = iframe.contentWindow?.document.body.scrollHeight + 'px'
}

function save() {
  if (!formName.value.trim() || !formKategori.value) return
  const saved = localStorage.getItem('admin_marketplace_produk')
  const items: any[] = saved ? JSON.parse(saved) : []
  items.push({
    uuid: crypto.randomUUID(),
    name: formName.value.trim(),
    kategori: formKategori.value,
    descriptions: formDescriptions.value,
    visual: formVisual.value,
    price: formPrice.value,
    html_code: isTemplates.value ? formHtmlCode.value : undefined,
    created_at: new Date().toISOString(),
  })
  localStorage.setItem('admin_marketplace_produk', JSON.stringify(items))
  navigateTo('/panel/admin/marketplace/produk')
}
</script>

<template>
  <div class="mx-auto max-w-6xl space-y-6">
    <!-- Header -->
    <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
      <div class="flex items-center gap-3">
        <NuxtLink to="/panel/admin/marketplace/produk" class="inline-flex items-center justify-center rounded-md border border-input bg-background px-3 py-1.5 text-sm font-medium shadow-sm hover:bg-accent hover:text-accent-foreground transition-colors">
          <svg class="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" /></svg>
          Back
        </NuxtLink>
        <div>
          <h1 class="text-xl sm:text-2xl font-bold tracking-tight">Add Produk</h1>
          <p class="text-xs sm:text-sm text-muted-foreground mt-1">Create a new marketplace product</p>
        </div>
      </div>
      <div class="flex gap-2">
        <NuxtLink to="/panel/admin/marketplace/produk" class="inline-flex items-center justify-center rounded-md border border-input bg-background px-3 sm:px-4 py-2 text-sm font-medium shadow-sm hover:bg-accent hover:text-accent-foreground transition-colors">Cancel</NuxtLink>
        <button class="inline-flex items-center justify-center rounded-md bg-primary px-3 sm:px-4 py-2 text-sm font-medium text-primary-foreground shadow hover:bg-primary/90 transition-colors" @click="save">Save Produk</button>
      </div>
    </div>

    <!-- Form -->
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-4 sm:gap-6">
      <!-- Left: Form fields -->
      <div class="space-y-4">
        <div class="rounded-xl border bg-card shadow-sm p-4 sm:p-5 space-y-4">
          <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
            <div>
              <label class="text-sm font-medium mb-1.5 block">Name</label>
              <input v-model="formName" type="text" placeholder="Product name" class="flex h-9 w-full rounded-md border border-input bg-background px-3 py-1 text-sm shadow-sm transition-colors placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring" />
            </div>
            <div>
              <label class="text-sm font-medium mb-1.5 block">Kategori</label>
              <select v-model="formKategori" class="flex h-9 w-full rounded-md border border-input bg-background px-3 py-1 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring">
                <option v-for="k in kategories" :key="k.id" :value="k.name">{{ k.name }}</option>
              </select>
            </div>
          </div>

          <div>
            <label class="text-sm font-medium mb-1.5 block">Descriptions</label>
            <textarea v-model="formDescriptions" rows="3" placeholder="Product description..." class="flex w-full rounded-md border border-input bg-background px-3 py-2 text-sm shadow-sm transition-colors placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring resize-y" />
          </div>

          <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
            <div>
              <label class="text-sm font-medium mb-1.5 block">Visual (URL)</label>
              <input v-model="formVisual" type="text" placeholder="https://example.com/image.png" class="flex h-9 w-full rounded-md border border-input bg-background px-3 py-1 text-sm shadow-sm transition-colors placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring" />
            </div>
            <div>
              <label class="text-sm font-medium mb-1.5 block">Price (cents)</label>
              <input v-model.number="formPrice" type="number" min="0" placeholder="2900" class="flex h-9 w-full rounded-md border border-input bg-background px-3 py-1 text-sm shadow-sm transition-colors placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring" />
            </div>
          </div>
        </div>

        <!-- HTML Code (only for Templates) -->
        <div v-if="isTemplates" class="rounded-xl border bg-card shadow-sm p-4 sm:p-5 space-y-4">
          <div>
            <label class="text-sm font-medium mb-1.5 block">HTML Code</label>
            <div class="relative">
              <textarea ref="textareaRef" v-model="formHtmlCode" :rows="windowWidth < 1024 ? 8 : 16" placeholder="<section>...</section>" class="flex w-full rounded-md border border-input bg-background px-3 py-2 text-sm font-mono shadow-sm transition-colors placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring resize-y" spellcheck="false" @keydown="handleTextareaKeydown" />
              <button class="absolute top-1.5 right-1.5 p-1 rounded text-muted-foreground hover:bg-accent hover:text-accent-foreground transition-colors z-20" title="Find (Ctrl+F)" @click="toggleSearch">
                <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" /></svg>
              </button>
            </div>
            <div v-if="showSearch" class="flex items-center gap-2 mt-1.5 p-2 bg-background border border-input rounded-md shadow-sm">
              <svg class="w-3 h-3 text-muted-foreground shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" /></svg>
              <input id="search-input" v-model="searchQuery" placeholder="Find..." class="flex h-7 flex-1 min-w-0 rounded border border-input bg-background px-2 text-xs font-mono outline-none focus-visible:ring-1 focus-visible:ring-ring" @input="findInText" @keydown="handleSearchKeydown" />
              <span class="text-[11px] text-muted-foreground whitespace-nowrap tabular-nums">{{ matches.length ? `${currentMatch + 1}/${matches.length}` : searchQuery ? '0/0' : '' }}</span>
              <button v-if="matches.length" class="p-0.5 rounded hover:bg-accent text-muted-foreground" @click="prevMatch"><svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" /></svg></button>
              <button v-if="matches.length" class="p-0.5 rounded hover:bg-accent text-muted-foreground" @click="nextMatch"><svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" /></svg></button>
              <button class="p-0.5 rounded hover:bg-accent text-muted-foreground" @click="toggleSearch"><svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" /></svg></button>
            </div>
          </div>
        </div>
      </div>

      <!-- Right: Preview (only for Templates) -->
      <div v-if="isTemplates" class="rounded-xl border bg-card shadow-sm overflow-hidden">
        <div class="px-3 sm:px-4 py-2 text-xs font-semibold uppercase tracking-wider text-muted-foreground bg-muted/50 border-b flex flex-col sm:flex-row sm:items-center justify-between gap-2">
          <div class="flex items-center gap-2">
            <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" /><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" /></svg>
            Live Preview
          </div>
          <div class="flex items-center gap-1.5">
            <button class="px-1.5 sm:px-2 py-1 rounded text-[10px] sm:text-[11px] font-semibold uppercase tracking-wider text-muted-foreground hover:bg-accent transition-colors whitespace-nowrap" @click="showFullPreview = true">
              <svg class="w-3 h-3 sm:w-3.5 sm:h-3.5 inline-block mr-1 -mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 8V4m0 0h4M4 4l5 5m11-1V4m0 0h-4m4 0l-5 5M4 16v4m0 0h4m-4 0l5-5m11 5l-5-5m5 5v-4m0 4h-4" /></svg>
              <span class="hidden sm:inline">Full Preview</span>
              <span class="sm:hidden">Full</span>
            </button>
            <div class="flex items-center gap-0.5 bg-muted rounded-md p-0.5">
              <button v-for="mode in ['desktop', 'tablet', 'mobile'] as const" :key="mode"
                class="p-1 sm:p-1.5 rounded transition-colors"
                :class="viewportMode === mode ? 'bg-card shadow-sm' : 'hover:bg-accent'"
                @click="viewportMode = mode">
                <svg class="w-3 h-3 sm:w-3.5 sm:h-3.5 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <template v-if="mode === 'desktop'"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" /></template>
                  <template v-else-if="mode === 'tablet'"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 18h.01M7 21h10a2 2 0 002-2V5a2 2 0 00-2-2H7a2 2 0 00-2 2v14a2 2 0 002 2z" /></template>
                  <template v-else><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 18h.01M8 21h8a2 2 0 002-2V5a2 2 0 00-2-2H8a2 2 0 00-2 2v14a2 2 0 002 2z" /></template>
                </svg>
              </button>
            </div>
          </div>
        </div>

        <div class="p-3 sm:p-4 bg-muted min-h-[200px] sm:min-h-[300px] flex items-start justify-center overflow-auto">
          <div v-if="formHtmlCode.trim()" class="flex justify-center transition-all duration-200 shrink-0"
            :class="viewportMode === 'mobile' ? 'w-[375px]' : viewportMode === 'tablet' ? 'w-[768px]' : 'w-[1280px]'">
            <div v-if="viewportMode === 'desktop'" class="w-full bg-card rounded-lg overflow-hidden shadow-2xl border border-border">
              <div class="bg-muted px-3 py-1.5 flex items-center gap-1.5 border-b border-border">
                <span class="w-2.5 h-2.5 rounded-full bg-red-400" />
                <span class="w-2.5 h-2.5 rounded-full bg-yellow-400" />
                <span class="w-2.5 h-2.5 rounded-full bg-green-400" />
                <span class="ml-2 text-[10px] text-muted-foreground font-mono truncate">localhost</span>
              </div>
              <iframe sandbox="allow-same-origin allow-scripts" :srcdoc="formHtmlCode" class="w-full border-0 min-h-[280px]" @load="resizeIframe($event)" />
            </div>
            <div v-else-if="viewportMode === 'tablet'" class="w-full rounded-[24px] shadow-2xl p-2.5 bg-card">
              <div class="bg-background rounded-[16px] overflow-hidden">
                <iframe sandbox="allow-same-origin allow-scripts" :srcdoc="formHtmlCode" class="w-full border-0 min-h-[360px]" @load="resizeIframe($event)" />
              </div>
            </div>
            <div v-else class="w-full rounded-[32px] shadow-2xl p-2 relative bg-card">
              <div class="absolute top-0 left-1/2 -translate-x-1/2 w-28 h-5 rounded-b-xl z-10 bg-card" />
              <div class="bg-background rounded-[24px] overflow-hidden">
                <iframe sandbox="allow-same-origin allow-scripts" :srcdoc="formHtmlCode" class="w-full border-0 min-h-[400px]" @load="resizeIframe($event)" />
              </div>
              <div class="absolute bottom-1.5 left-1/2 -translate-x-1/2 w-24 h-1 rounded-full bg-muted-foreground/30" />
            </div>
          </div>
          <div v-else class="text-sm text-muted-foreground">Enter HTML code to see a live preview</div>
        </div>
      </div>

      <!-- Right: Visual preview for non-template products -->
      <div v-else class="rounded-xl border bg-card shadow-sm overflow-hidden">
        <div class="px-3 sm:px-4 py-2 text-xs font-semibold uppercase tracking-wider text-muted-foreground bg-muted/50 border-b">Product Preview</div>
        <div class="p-4 sm:p-6 flex flex-col items-center justify-center min-h-[200px] sm:min-h-[300px]">
          <div v-if="formVisual.trim()" class="w-full max-w-sm">
            <img :src="formVisual" alt="Product visual" class="w-full rounded-lg border object-cover max-h-64" @error="($event.target as HTMLImageElement).style.display = 'none'" />
          </div>
          <div v-else class="flex flex-col items-center gap-2 text-muted-foreground">
            <svg class="w-12 h-12" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" /></svg>
            <span class="text-sm">Enter a visual URL to preview</span>
          </div>
        </div>
      </div>
    </div>
  </div>

  <!-- Full-screen preview overlay (for Templates) -->
  <div v-if="showFullPreview && formHtmlCode.trim() && isTemplates" class="fixed inset-0 z-[999] h-screen flex flex-col bg-background">
    <header class="h-12 bg-card border-b border-border flex items-center justify-between px-2 sm:px-4 shrink-0">
      <div class="flex items-center gap-1.5 sm:gap-3 min-w-0">
        <button class="flex items-center gap-1 text-xs sm:text-sm text-muted-foreground hover:text-foreground transition-colors truncate" @click="showFullPreview = false">
          <svg class="w-3.5 h-3.5 sm:w-4 sm:h-4 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" /></svg>
          <span class="hidden sm:inline">Kembali ke Add Produk</span>
          <span class="sm:hidden">Kembali</span>
        </button>
        <div class="w-5 h-5 sm:w-6 sm:h-6 bg-gradient-to-br from-primary-500 to-purple-600 rounded flex items-center justify-center shrink-0">
          <svg class="w-2.5 h-2.5 sm:w-3 sm:h-3 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" /></svg>
        </div>
        <span class="text-xs sm:text-sm font-bold text-primary-600 dark:text-primary-400 truncate">Rinova</span>
      </div>

      <div class="flex items-center gap-1.5 sm:gap-3 shrink-0">
        <span class="hidden sm:flex items-center gap-1 px-2 py-0.5 rounded-full bg-amber-100 dark:bg-amber-900/30 text-amber-700 dark:text-amber-400 text-[10px] font-semibold uppercase tracking-wider">
          <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" /><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" /></svg>
          Preview
        </span>

        <div class="flex items-center gap-0.5 bg-muted rounded-md p-0.5">
          <button v-for="mode in ['desktop', 'tablet', 'mobile'] as const" :key="mode"
            class="p-1 sm:p-1.5 rounded transition-colors"
            :class="viewportMode === mode ? 'bg-card shadow-sm' : 'hover:bg-accent'"
            @click="viewportMode = mode">
            <svg class="w-3 h-3 sm:w-3.5 sm:h-3.5 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <template v-if="mode === 'desktop'"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" /></template>
              <template v-else-if="mode === 'tablet'"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 18h.01M7 21h10a2 2 0 002-2V5a2 2 0 00-2-2H7a2 2 0 00-2 2v14a2 2 0 002 2z" /></template>
              <template v-else><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 18h.01M8 21h8a2 2 0 002-2V5a2 2 0 00-2-2H8a2 2 0 00-2 2v14a2 2 0 002 2z" /></template>
            </svg>
          </button>
        </div>
      </div>
    </header>

    <div class="flex-1 overflow-auto bg-muted p-2 sm:p-4 md:p-6 lg:p-8">
      <div class="flex justify-center transition-all duration-200 shrink-0"
        :class="viewportMode === 'mobile' ? 'w-[375px]' : viewportMode === 'tablet' ? 'w-[768px]' : 'w-[1280px]'">
        <div v-if="viewportMode === 'desktop'" class="w-full bg-card rounded-lg overflow-hidden shadow-2xl border border-border">
          <div class="bg-muted px-3 py-1.5 flex items-center gap-1.5 border-b border-border">
            <span class="w-2.5 h-2.5 rounded-full bg-red-400" />
            <span class="w-2.5 h-2.5 rounded-full bg-yellow-400" />
            <span class="w-2.5 h-2.5 rounded-full bg-green-400" />
            <span class="ml-2 text-[10px] text-muted-foreground font-mono truncate">localhost</span>
          </div>
          <iframe sandbox="allow-same-origin allow-scripts" :srcdoc="formHtmlCode" class="w-full border-0 min-h-[500px]" @load="resizeIframe($event)" />
        </div>
        <div v-else-if="viewportMode === 'tablet'" class="w-full rounded-[24px] shadow-2xl p-2.5 bg-card">
          <div class="bg-background rounded-[16px] overflow-hidden">
            <iframe sandbox="allow-same-origin allow-scripts" :srcdoc="formHtmlCode" class="w-full border-0 min-h-[600px]" @load="resizeIframe($event)" />
          </div>
        </div>
        <div v-else class="w-full rounded-[32px] shadow-2xl p-2 relative bg-card">
          <div class="absolute top-0 left-1/2 -translate-x-1/2 w-28 h-5 rounded-b-xl z-10 bg-card" />
          <div class="bg-background rounded-[24px] overflow-hidden">
            <iframe sandbox="allow-same-origin allow-scripts" :srcdoc="formHtmlCode" class="w-full border-0 min-h-[600px]" @load="resizeIframe($event)" />
          </div>
          <div class="absolute bottom-1.5 left-1/2 -translate-x-1/2 w-24 h-1 rounded-full bg-muted-foreground/30" />
        </div>
      </div>
    </div>
  </div>
</template>
