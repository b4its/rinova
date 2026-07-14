<script setup lang="ts">
definePageMeta({ layout: 'panel', sidebarSection: 'admin-kategori' })

interface Kategori {
  id: number
  name: string
}

const kategories = ref<Kategori[]>([
  { id: 1, name: 'Landing Page' },
  { id: 2, name: 'Company Profile' },
  { id: 3, name: 'Portfolio' },
])

let nextId = 4

const dialogOpen = ref(false)
const editingId = ref<number | null>(null)
const formName = ref('')
const search = ref('')

const filtered = computed(() => {
  const q = search.value.toLowerCase()
  return kategories.value.filter(k => k.name.toLowerCase().includes(q))
})

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

function save() {
  if (!formName.value.trim()) return
  if (editingId.value) {
    const k = kategories.value.find(k => k.id === editingId.value)
    if (k) k.name = formName.value.trim()
  } else {
    kategories.value.push({ id: nextId++, name: formName.value.trim() })
  }
  dialogOpen.value = false
}

function remove(id: number) {
  kategories.value = kategories.value.filter(k => k.id !== id)
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

    <div class="rounded-xl border bg-card text-card-foreground shadow-sm">
      <table class="w-full">
        <thead>
          <tr class="border-b border-border">
            <th class="h-10 px-4 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">ID</th>
            <th class="h-10 px-4 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">Name</th>
            <th class="h-10 px-4 text-right text-xs font-medium text-muted-foreground uppercase tracking-wider">Actions</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="k in filtered" :key="k.id" class="border-b border-border last:border-0 hover:bg-muted/50 transition-colors">
            <td class="p-4 text-sm font-mono text-muted-foreground">{{ k.id }}</td>
            <td class="p-4 text-sm font-medium">{{ k.name }}</td>
            <td class="p-4 text-right">
              <button class="inline-flex items-center justify-center rounded-md px-3 py-1.5 text-xs font-medium transition-colors hover:bg-accent hover:text-accent-foreground mr-1" @click="openEdit(k)">Edit</button>
              <button class="inline-flex items-center justify-center rounded-md px-3 py-1.5 text-xs font-medium text-destructive hover:bg-destructive/10 transition-colors" @click="remove(k.id)">Delete</button>
            </td>
          </tr>
          <tr v-if="!filtered.length">
            <td colspan="3" class="p-8 text-center text-sm text-muted-foreground">No categories found.</td>
          </tr>
        </tbody>
      </table>
    </div>

    <!-- Dialog -->
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