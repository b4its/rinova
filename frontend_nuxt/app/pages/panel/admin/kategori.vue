<script setup lang="ts">
definePageMeta({ layout: 'panel', sidebarSection: 'admin-kategori' })

const api = useApi()

interface Kategori {
  id: string
  name: string
  created_at: string
}

interface PaginatedResponse {
  data: Kategori[]
  total: number
  page: number
  page_size: number
  total_pages: number
}

const items = ref<Kategori[]>([])
const search = ref('')
const dialogOpen = ref(false)
const editingId = ref<string | null>(null)
const formName = ref('')
const isLoading = ref(false)

const currentPage = ref(1)
const perPage = ref(10)
const totalPages = ref(1)

async function load() {
  isLoading.value = true
  try {
    const res = await api.get<PaginatedResponse>('/marketplace/project-categories', {
      query: { page: currentPage.value, page_size: perPage.value, search: search.value || undefined }
    })
    items.value = res.data
    totalPages.value = res.total_pages
  } catch {
    items.value = []
  } finally {
    isLoading.value = false
  }
}

load()

watch([search, currentPage], load)

function openAdd() {
  editingId.value = null
  formName.value = ''
  dialogOpen.value = true
}

function openEdit(k: Kategori) {
  editingId.value = k.id
  formName.value = k.name
  dialogOpen.value = true
}

async function save() {
  if (!formName.value.trim()) return
  try {
    if (editingId.value) {
      await api.put(`/marketplace/project-categories/${editingId.value}`, { name: formName.value.trim() })
    } else {
      await api.post('/marketplace/project-categories', { name: formName.value.trim() })
    }
    dialogOpen.value = false
    await load()
  } catch {}
}

async function remove(id: string) {
  try {
    await api.del(`/marketplace/project-categories/${id}`)
    await load()
  } catch {}
}
</script>

<template>
  <div class="space-y-6">
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-2xl font-bold tracking-tight">Kategori</h1>
        <p class="text-sm text-muted-foreground mt-1">Manage template categories</p>
      </div>
      <button class="inline-flex items-center justify-center rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground shadow hover:bg-primary/90 transition-colors" @click="openAdd">+ Add Kategori</button>
    </div>

    <div class="relative max-w-sm">
      <svg class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" /></svg>
      <input v-model="search" type="text" placeholder="Search categories..." class="flex h-9 w-full rounded-md border border-input bg-background px-3 py-1 pl-9 text-sm shadow-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring" />
    </div>

    <div v-if="isLoading" class="flex items-center justify-center py-12">
      <svg class="animate-spin h-6 w-6 text-primary" fill="none" viewBox="0 0 24 24"><circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" /><path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" /></svg>
    </div>

    <div v-else class="rounded-xl border bg-card text-card-foreground shadow-sm">
      <table class="w-full">
        <thead>
          <tr class="border-b border-border">
            <th class="h-10 px-4 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">Name</th>
            <th class="h-10 px-4 text-right text-xs font-medium text-muted-foreground uppercase tracking-wider">Actions</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="k in items" :key="k.id" class="border-b border-border last:border-0 hover:bg-muted/50 transition-colors">
            <td class="p-4 text-sm font-medium">{{ k.name }}</td>
            <td class="p-4 text-right">
              <button class="inline-flex items-center justify-center rounded-md px-3 py-1.5 text-xs font-medium transition-colors hover:bg-accent hover:text-accent-foreground mr-1" @click="openEdit(k)">Edit</button>
              <button class="inline-flex items-center justify-center rounded-md px-3 py-1.5 text-xs font-medium text-destructive hover:bg-destructive/10 transition-colors" @click="remove(k.id)">Delete</button>
            </td>
          </tr>
          <tr v-if="!items.length && !isLoading">
            <td colspan="2" class="p-8 text-center text-sm text-muted-foreground">No categories found.</td>
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
      <div class="relative bg-background text-foreground rounded-xl shadow-lg border w-full max-w-md mx-4 p-6 space-y-4">
        <h2 class="text-lg font-semibold">{{ editingId ? 'Edit Kategori' : 'Add Kategori' }}</h2>
        <div>
          <label class="text-sm font-medium mb-1.5 block">Name</label>
          <input v-model="formName" type="text" placeholder="Category name" class="flex h-9 w-full rounded-md border border-input bg-background px-3 py-1 text-sm shadow-sm transition-colors placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring" @keyup.enter="save" />
        </div>
        <div class="flex justify-end gap-2 pt-2">
          <button class="inline-flex items-center justify-center rounded-md border border-input bg-background px-4 py-2 text-sm font-medium shadow-sm hover:bg-accent hover:text-accent-foreground transition-colors" @click="dialogOpen = false">Cancel</button>
          <button class="inline-flex items-center justify-center rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground shadow hover:bg-primary/90 transition-colors" @click="save">Save</button>
        </div>
      </div>
    </div>
  </div>
</template>
