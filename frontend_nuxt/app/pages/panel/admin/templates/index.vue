<script setup lang="ts">
definePageMeta({ layout: 'panel', sidebarSection: 'admin-templates' })

interface Template {
  id: number
  name: string
  category: string
  code: string
}

const kategories = ['Landing Page', 'Company Profile', 'Portfolio']

const defaultTemplates: Template[] = [
  { id: 1, name: 'Hero Startup', category: 'Landing Page', code: '<section class="py-20 text-center bg-gradient-to-br from-primary-500 to-purple-600 text-white"><h1 class="text-4xl font-bold">Welcome</h1><p class="mt-4 text-lg opacity-90">Build something amazing</p></section>' },
  { id: 2, name: 'Footer Simple', category: 'Company Profile', code: '<footer class="bg-gray-900 text-gray-300 py-8 text-center"><p>&copy; 2024 Your Company. All rights reserved.</p></footer>' },
]

const items = ref<Template[]>([])

function loadTemplates() {
  const saved = localStorage.getItem('admin_templates')
  if (saved) {
    items.value = JSON.parse(saved)
  } else {
    items.value = [...defaultTemplates]
    localStorage.setItem('admin_templates', JSON.stringify(defaultTemplates))
  }
}

loadTemplates()

let nextId = computed(() => items.value.length ? Math.max(...items.value.map(t => t.id)) + 1 : 1)

const dialogOpen = ref(false)
const editingId = ref<number | null>(null)
const formName = ref('')
const formCategory = ref('')
const formCode = ref('')
const search = ref('')

const filtered = computed(() => {
  const q = search.value.toLowerCase()
  return items.value.filter(t => t.name.toLowerCase().includes(q) || t.category.toLowerCase().includes(q))
})

function openEdit(t: Template) {
  editingId.value = t.id
  formName.value = t.name
  formCategory.value = t.category
  formCode.value = t.code
  dialogOpen.value = true
}

function save() {
  if (!formName.value.trim() || !formCode.value.trim()) return
  if (editingId.value) {
    const t = items.value.find(t => t.id === editingId.value)
    if (t) { t.name = formName.value.trim(); t.category = formCategory.value; t.code = formCode.value }
  }
  localStorage.setItem('admin_templates', JSON.stringify(items.value))
  dialogOpen.value = false
}

function remove(id: number) {
  items.value = items.value.filter(t => t.id !== id)
  localStorage.setItem('admin_templates', JSON.stringify(items.value))
}
</script>

<template>
  <div class="space-y-6">
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-2xl font-bold tracking-tight">Templates</h1>
        <p class="text-sm text-muted-foreground mt-1">Manage HTML templates</p>
      </div>
      <NuxtLink to="/panel/admin/templates/add" class="inline-flex items-center justify-center rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground shadow hover:bg-primary/90 transition-colors">+ Add Template</NuxtLink>
    </div>

    <div class="relative max-w-sm">
      <svg class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" /></svg>
      <input v-model="search" type="text" placeholder="Search templates..." class="flex h-9 w-full rounded-md border border-input bg-background px-3 py-1 pl-9 text-sm shadow-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring" />
    </div>

    <div class="rounded-xl border bg-card text-card-foreground shadow-sm">
      <table class="w-full">
        <thead>
          <tr class="border-b border-border">
            <th class="h-10 px-4 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">ID</th>
            <th class="h-10 px-4 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">Name</th>
            <th class="h-10 px-4 text-left text-xs font-medium text-muted-foreground uppercase tracking-wider">Category</th>
            <th class="h-10 px-4 text-right text-xs font-medium text-muted-foreground uppercase tracking-wider">Actions</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="t in filtered" :key="t.id" class="border-b border-border last:border-0 hover:bg-muted/50 transition-colors">
            <td class="p-4 text-sm font-mono text-muted-foreground">{{ t.id }}</td>
            <td class="p-4 text-sm font-medium">{{ t.name }}</td>
            <td class="p-4 text-sm text-muted-foreground">{{ t.category }}</td>
            <td class="p-4 text-right space-x-1">
              <NuxtLink :to="`/panel/admin/templates/preview/${t.id}`" class="inline-flex items-center justify-center rounded-md px-3 py-1.5 text-xs font-medium transition-colors hover:bg-accent hover:text-accent-foreground">Preview</NuxtLink>
              <button class="inline-flex items-center justify-center rounded-md px-3 py-1.5 text-xs font-medium transition-colors hover:bg-accent hover:text-accent-foreground" @click="openEdit(t)">Edit</button>
              <button class="inline-flex items-center justify-center rounded-md px-3 py-1.5 text-xs font-medium text-destructive hover:bg-destructive/10 transition-colors" @click="remove(t.id)">Delete</button>
            </td>
          </tr>
          <tr v-if="!filtered.length">
            <td colspan="4" class="p-8 text-center text-sm text-muted-foreground">No templates found.</td>
          </tr>
        </tbody>
      </table>
    </div>

    <!-- Add/Edit Dialog -->
    <div v-if="dialogOpen" class="fixed inset-0 z-50 flex items-center justify-center">
      <div class="fixed inset-0 bg-black/80" @click="dialogOpen = false" />
      <div class="relative bg-background text-foreground rounded-xl shadow-lg border w-full max-w-3xl mx-4 p-6 space-y-4 max-h-[85vh] overflow-y-auto">
        <h2 class="text-lg font-semibold">{{ editingId ? 'Edit Template' : 'Add Template' }}</h2>
        <div class="grid grid-cols-2 gap-4">
          <div>
            <label class="text-sm font-medium mb-1.5 block">Template Name</label>
            <input v-model="formName" type="text" placeholder="e.g. Hero Startup" class="flex h-9 w-full rounded-md border border-input bg-background px-3 py-1 text-sm shadow-sm transition-colors placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring" />
          </div>
          <div>
            <label class="text-sm font-medium mb-1.5 block">Category</label>
            <select v-model="formCategory" class="flex h-9 w-full rounded-md border border-input bg-background px-3 py-1 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring">
              <option v-for="k in kategories" :key="k" :value="k">{{ k }}</option>
            </select>
          </div>
        </div>
        <div>
          <label class="text-sm font-medium mb-1.5 block">HTML Code</label>
          <textarea v-model="formCode" rows="10" placeholder="<section>...</section>" class="flex w-full rounded-md border border-input bg-background px-3 py-2 text-sm font-mono shadow-sm transition-colors placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring resize-y" />
        </div>

        <!-- Live preview -->
        <div v-if="formCode.trim()" class="rounded-lg border overflow-hidden">
          <div class="px-3 py-1.5 text-[10px] font-semibold uppercase tracking-wider text-muted-foreground bg-muted/50 border-b">Live Preview</div>
          <div class="p-4 bg-card"><iframe :srcdoc="formCode" class="w-full border-0" style="min-height:120px" @load="($event.target as HTMLIFrameElement).style.height = ($event.target as HTMLIFrameElement).contentWindow?.document.body.scrollHeight + 'px'"></iframe></div>
        </div>

        <div class="flex justify-end gap-2 pt-2">
          <button class="inline-flex items-center justify-center rounded-md border border-input bg-background px-4 py-2 text-sm font-medium shadow-sm hover:bg-accent hover:text-accent-foreground transition-colors" @click="dialogOpen = false">Cancel</button>
          <button class="inline-flex items-center justify-center rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground shadow hover:bg-primary/90 transition-colors" @click="save">Save</button>
        </div>
      </div>
    </div>
  </div>
</template>