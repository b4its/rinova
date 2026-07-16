<script setup lang="ts">
definePageMeta({ layout: 'panel', sidebarSection: 'admin-mk-produk' })

const api = useApi()

interface Produk {
  id: string
  name: string
  category_id: string
  descriptions: string | null
  visual: string | null
  price: number
  html_code: string | null
  created_at: string
}

interface Kategori {
  id: string
  name: string
}

interface PaginatedResponse {
  data: Produk[]
  total: number
  page: number
  page_size: number
  total_pages: number
}

interface KategoriResponse {
  data: Kategori[]
}

const items = ref<Produk[]>([])
const kategories = ref<Kategori[]>([])
const search = ref('')
const dialogOpen = ref(false)
const editingId = ref<string | null>(null)
const formName = ref('')
const formKategoriId = ref('')
const formDescriptions = ref('')
const formVisual = ref('')
const formPrice = ref(0)
const formHtmlCode = ref('')
const isLoading = ref(false)

const currentPage = ref(1)
const perPage = ref(10)
const totalPages = ref(1)

const kategoryMap = computed(() => {
  const map: Record<string, string> = {}
  for (const k of kategories.value) map[k.id] = k.name
  return map
})

async function load() {
  isLoading.value = true
  try {
    const res = await api.get<PaginatedResponse>('/marketplace/products', {
      query: { page: currentPage.value, page_size: perPage.value, search: search.value || undefined }
    })
    items.value = res.data
    totalPages.value = res.total_pages
  } catch (e) {
    items.value = []
  } finally {
    isLoading.value = false
  }
}

async function loadKategori() {
  try {
    const res = await api.get<KategoriResponse>('/marketplace/categories', {
      query: { page_size: 100 }
    })
    kategories.value = res.data
  } catch {
    kategories.value = []
  }
}

load()
loadKategori()

watch([search, currentPage], load)

function formatPrice(cents: number) {
  return '$' + (cents / 100).toFixed(2)
}

const isTemplates = computed(() => {
  const k = kategories.value.find(k => k.id === formKategoriId.value)
  return k?.name === 'Templates'
})

function openAdd() {
  editingId.value = null
  formName.value = ''
  formKategoriId.value = kategories.value[0]?.id || ''
  formDescriptions.value = ''
  formVisual.value = ''
  formPrice.value = 0
  formHtmlCode.value = ''
  dialogOpen.value = true
}

function openEdit(p: Produk) {
  editingId.value = p.id
  formName.value = p.name
  formKategoriId.value = p.category_id
  formDescriptions.value = p.descriptions || ''
  formVisual.value = p.visual || ''
  formPrice.value = p.price
  formHtmlCode.value = p.html_code || ''
  dialogOpen.value = true
}

async function save() {
  if (!formName.value.trim() || !formKategoriId.value) return
  try {
    const body: Record<string, unknown> = {
      name: formName.value.trim(),
      category_id: formKategoriId.value,
      descriptions: formDescriptions.value,
      visual: formVisual.value,
      price: formPrice.value,
    }
    if (isTemplates.value) {
      body.html_code = formHtmlCode.value
    }
    if (editingId.value) {
      await api.put(`/marketplace/products/${editingId.value}`, body)
    } else {
      await api.post('/marketplace/products', body)
    }
    dialogOpen.value = false
    await load()
  } catch (e) {
    // handled by useApi
  }
}

async function remove(id: string) {
  try {
    await api.del(`/marketplace/products/${id}`)
    await load()
  } catch (e) {
    // handled by useApi
  }
}
</script>

<template>
  <div class="mx-auto max-w-6xl space-y-6">
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-2xl font-bold tracking-tight">Produk Marketplace</h1>
        <p class="text-sm text-muted-foreground mt-1">Manage marketplace products</p>
      </div>
      <NuxtLink to="/panel/admin/marketplace/produk/add" class="inline-flex items-center justify-center rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground shadow hover:bg-primary/90 transition-colors">+ Add Produk</NuxtLink>
    </div>

    <div class="relative max-w-sm">
      <svg class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" /></svg>
      <input v-model="search" type="text" placeholder="Search products..." class="flex h-9 w-full rounded-md border border-input bg-background px-3 py-1 pl-9 text-sm shadow-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring" />
    </div>

    <div v-if="isLoading" class="flex items-center justify-center py-12">
      <svg class="animate-spin h-6 w-6 text-primary" fill="none" viewBox="0 0 24 24"><circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" /><path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" /></svg>
    </div>

    <div v-else class="rounded-xl border bg-card text-card-foreground shadow-sm overflow-x-auto">
      <table class="w-full">
        <thead>
          <tr class="border-b border-border">
            <th class="h-10 px-4 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">Name</th>
            <th class="h-10 px-4 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">Kategori</th>
            <th class="h-10 px-4 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">Price</th>
            <th class="h-10 px-4 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">Created</th>
            <th class="h-10 px-4 text-right text-xs font-medium text-muted-foreground uppercase tracking-wider">Actions</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="p in items" :key="p.id" class="border-b border-border last:border-0 hover:bg-muted/50 transition-colors">
            <td class="p-4 text-sm font-medium">{{ p.name }}</td>
            <td class="p-4 text-sm text-muted-foreground">{{ kategoryMap[p.category_id] || p.category_id }}</td>
            <td class="p-4 text-sm font-mono">{{ formatPrice(p.price) }}</td>
            <td class="p-4 text-sm text-muted-foreground">{{ new Date(p.created_at).toLocaleDateString() }}</td>
            <td class="p-4 text-right space-x-1 whitespace-nowrap">
              <button class="inline-flex items-center justify-center rounded-md px-3 py-1.5 text-xs font-medium transition-colors hover:bg-accent hover:text-accent-foreground" @click="openEdit(p)">Edit</button>
              <button class="inline-flex items-center justify-center rounded-md px-3 py-1.5 text-xs font-medium text-destructive hover:bg-destructive/10 transition-colors" @click="remove(p.id)">Delete</button>
            </td>
          </tr>
          <tr v-if="!items.length && !isLoading">
            <td colspan="5" class="p-8 text-center text-sm text-muted-foreground">No products found.</td>
          </tr>
        </tbody>
      </table>
    </div>

    <div v-if="totalPages > 1" class="flex items-center justify-center gap-2">
      <button class="inline-flex items-center justify-center rounded-md border border-input bg-background px-3 py-1.5 text-sm font-medium shadow-sm hover:bg-accent hover:text-accent-foreground transition-colors disabled:opacity-50" :disabled="currentPage <= 1" @click="currentPage--">Previous</button>
      <span class="text-sm text-muted-foreground">{{ currentPage }} / {{ totalPages }}</span>
      <button class="inline-flex items-center justify-center rounded-md border border-input bg-background px-3 py-1.5 text-sm font-medium shadow-sm hover:bg-accent hover:text-accent-foreground transition-colors disabled:opacity-50" :disabled="currentPage >= totalPages" @click="currentPage++">Next</button>
    </div>

    <div v-if="dialogOpen" class="fixed inset-0 z-50 flex items-center justify-center">
      <div class="fixed inset-0 bg-black/80" @click="dialogOpen = false" />
      <div class="relative bg-background text-foreground rounded-xl shadow-lg border w-full max-w-3xl mx-4 p-6 space-y-4 max-h-[85vh] overflow-y-auto">
        <h2 class="text-lg font-semibold">{{ editingId ? 'Edit Produk' : 'Add Produk' }}</h2>

        <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
          <div>
            <label class="text-sm font-medium mb-1.5 block">Name</label>
            <input v-model="formName" type="text" placeholder="Product name" class="flex h-9 w-full rounded-md border border-input bg-background px-3 py-1 text-sm shadow-sm transition-colors placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring" />
          </div>
          <div>
            <label class="text-sm font-medium mb-1.5 block">Kategori</label>
            <select v-model="formKategoriId" class="flex h-9 w-full rounded-md border border-input bg-background px-3 py-1 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring">
              <option v-for="k in kategories" :key="k.id" :value="k.id">{{ k.name }}</option>
            </select>
          </div>
          <div class="sm:col-span-2">
            <label class="text-sm font-medium mb-1.5 block">Descriptions</label>
            <textarea v-model="formDescriptions" rows="3" placeholder="Product description..." class="flex w-full rounded-md border border-input bg-background px-3 py-2 text-sm shadow-sm transition-colors placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring resize-y" />
          </div>
          <div>
            <label class="text-sm font-medium mb-1.5 block">Visual (URL)</label>
            <input v-model="formVisual" type="text" placeholder="https://example.com/image.png" class="flex h-9 w-full rounded-md border border-input bg-background px-3 py-1 text-sm shadow-sm transition-colors placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring" />
          </div>
          <div>
            <label class="text-sm font-medium mb-1.5 block">Price (cents)</label>
            <input v-model.number="formPrice" type="number" min="0" placeholder="2900" class="flex h-9 w-full rounded-md border border-input bg-background px-3 py-1 text-sm shadow-sm transition-colors placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring" />
          </div>
        </div>

        <div v-if="isTemplates">
          <label class="text-sm font-medium mb-1.5 block">HTML Code</label>
          <textarea v-model="formHtmlCode" rows="8" placeholder="<section>...</section>" class="flex w-full rounded-md border border-input bg-background px-3 py-2 text-sm font-mono shadow-sm transition-colors placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring resize-y" />
        </div>

        <div v-if="isTemplates && formHtmlCode.trim()" class="rounded-lg border overflow-hidden">
          <div class="px-3 py-1.5 text-[10px] font-semibold uppercase tracking-wider text-muted-foreground bg-muted/50 border-b">Live Preview</div>
          <div class="p-4 bg-card">
            <iframe :srcdoc="formHtmlCode" class="w-full border-0" style="min-height:120px" @load="($event.target as HTMLIFrameElement).style.height = ($event.target as HTMLIFrameElement).contentWindow?.document.body.scrollHeight + 'px'"></iframe>
          </div>
        </div>

        <div class="flex justify-end gap-2 pt-2">
          <button class="inline-flex items-center justify-center rounded-md border border-input bg-background px-4 py-2 text-sm font-medium shadow-sm hover:bg-accent hover:text-accent-foreground transition-colors" @click="dialogOpen = false">Cancel</button>
          <button class="inline-flex items-center justify-center rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground shadow hover:bg-primary/90 transition-colors" @click="save">Save</button>
        </div>
      </div>
    </div>
  </div>
</template>
