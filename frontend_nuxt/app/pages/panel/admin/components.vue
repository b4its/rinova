<script setup lang="ts">
definePageMeta({ layout: 'panel', sidebarSection: 'admin-components' })

interface BuilderComponent {
  id: string
  name: string
  type: string
  category: string
  enabled: boolean
  description: string
}

const components = ref<BuilderComponent[]>([
  { id: '1', name: 'Hero', type: 'Hero', category: 'Layout', enabled: true, description: 'Full-width hero section with title and CTA' },
  { id: '2', name: 'Header', type: 'Header', category: 'Layout', enabled: true, description: 'Site header with navigation' },
  { id: '3', name: 'Footer', type: 'Footer', category: 'Layout', enabled: true, description: 'Site footer with links' },
  { id: '4', name: 'Features', type: 'Features', category: 'Layout', enabled: true, description: 'Feature grid section' },
  { id: '5', name: 'Testimonials', type: 'Testimonials', category: 'Layout', enabled: true, description: 'Customer testimonials carousel' },
  { id: '6', name: 'Pricing', type: 'Pricing', category: 'Layout', enabled: true, description: 'Pricing plans table' },
  { id: '7', name: 'FAQ', type: 'FAQ', category: 'Layout', enabled: true, description: 'Frequently asked questions accordion' },
  { id: '8', name: 'Contact', type: 'Contact', category: 'Layout', enabled: true, description: 'Contact form section' },
  { id: '9', name: 'Stats', type: 'Stats', category: 'Layout', enabled: true, description: 'Statistics counter section' },
  { id: '10', name: 'Container', type: 'Container', category: 'Content', enabled: true, description: 'Custom container with items' },
  { id: '11', name: 'Text', type: 'Text', category: 'Content', enabled: true, description: 'Rich text block' },
  { id: '12', name: 'Button', type: 'Button', category: 'Content', enabled: true, description: 'Call-to-action button' },
  { id: '13', name: 'Image', type: 'Image', category: 'Content', enabled: true, description: 'Image with alt text' },
  { id: '14', name: 'Divider', type: 'Divider', category: 'Content', enabled: true, description: 'Horizontal divider line' },
  { id: '15', name: 'Link', type: 'Link', category: 'Content', enabled: true, description: 'Hyperlink' },
  { id: '16', name: 'Icon', type: 'Icon', category: 'Content', enabled: true, description: 'Font Awesome icon' },
  { id: '17', name: 'Form', type: 'Form', category: 'Form', enabled: true, description: 'Contact form with fields' },
  { id: '18', name: 'Input', type: 'Input', category: 'Form', enabled: true, description: 'Text input field' },
  { id: '19', name: 'Textarea', type: 'Textarea', category: 'Form', enabled: true, description: 'Multi-line text input' },
  { id: '20', name: 'Select', type: 'Select', category: 'Form', enabled: true, description: 'Dropdown select field' },
  { id: '21', name: 'Checkbox', type: 'Checkbox', category: 'Form', enabled: true, description: 'Checkbox input' },
  { id: '22', name: 'Radio', type: 'Radio', category: 'Form', enabled: true, description: 'Radio button group' },
  { id: '23', name: 'Number', type: 'Number', category: 'Form', enabled: true, description: 'Number input' },
  { id: '24', name: 'Date', type: 'Date', category: 'Form', enabled: true, description: 'Date picker input' },
])

function toggleComponent(id: string) {
  const c = components.value.find(c => c.id === id)
  if (c) c.enabled = !c.enabled
}

const search = ref('')
const categoryFilter = ref<'all' | string>('all')

const categories = computed(() => {
  const set = new Set(components.value.map(c => c.category))
  return ['all', ...Array.from(set)]
})

const filtered = computed(() => {
  let result = components.value
  if (search.value) {
    const q = search.value.toLowerCase()
    result = result.filter(c => c.name.toLowerCase().includes(q) || c.type.toLowerCase().includes(q))
  }
  if (categoryFilter.value !== 'all') {
    result = result.filter(c => c.category === categoryFilter.value)
  }
  return result
})

const enabledCount = computed(() => components.value.filter(c => c.enabled).length)
const disabledCount = computed(() => components.value.filter(c => !c.enabled).length)
</script>

<template>
  <div class="space-y-6">
    <div>
      <h1 class="text-2xl font-bold tracking-tight">Components</h1>
      <p class="text-muted-foreground">Manage builder components that appear in the editor sidebar.</p>
    </div>

    <!-- Stats -->
    <div class="grid gap-4 grid-cols-3">
      <div class="rounded-xl border bg-card text-card-foreground shadow p-4">
        <div class="text-2xl font-bold">{{ components.length }}</div>
        <div class="text-xs text-muted-foreground">Total Components</div>
      </div>
      <div class="rounded-xl border bg-card text-card-foreground shadow p-4">
        <div class="text-2xl font-bold text-emerald-600 dark:text-emerald-400">{{ enabledCount }}</div>
        <div class="text-xs text-muted-foreground">Enabled</div>
      </div>
      <div class="rounded-xl border bg-card text-card-foreground shadow p-4">
        <div class="text-2xl font-bold text-destructive">{{ disabledCount }}</div>
        <div class="text-xs text-muted-foreground">Disabled</div>
      </div>
    </div>

    <!-- Filters -->
    <div class="flex items-center gap-4">
      <div class="flex w-full max-w-sm items-center gap-2 rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm">
        <svg class="w-4 h-4 text-muted-foreground shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
        </svg>
        <input v-model="search" type="text" placeholder="Search components..." class="flex-1 bg-transparent outline-none placeholder:text-muted-foreground text-sm">
      </div>
      <select v-model="categoryFilter" class="flex h-9 w-40 rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring">
        <option value="all">All Categories</option>
        <option v-for="cat in categories.filter(c => c !== 'all')" :key="cat" :value="cat">{{ cat }}</option>
      </select>
    </div>

    <!-- Component list -->
    <div class="rounded-xl border bg-card text-card-foreground shadow">
      <div class="relative w-full overflow-auto">
        <table class="w-full caption-bottom text-sm">
          <thead class="[&_tr]:border-b">
            <tr class="border-b border-border transition-colors hover:bg-muted/50">
              <th class="h-10 px-2 text-left align-middle font-medium text-muted-foreground">Name</th>
              <th class="h-10 px-2 text-left align-middle font-medium text-muted-foreground">Type</th>
              <th class="h-10 px-2 text-left align-middle font-medium text-muted-foreground">Category</th>
              <th class="h-10 px-2 text-left align-middle font-medium text-muted-foreground">Description</th>
              <th class="h-10 px-2 text-left align-middle font-medium text-muted-foreground">Status</th>
            </tr>
          </thead>
          <tbody class="[&_tr:last-child]:border-0">
            <tr v-for="c in filtered" :key="c.id" class="border-b border-border transition-colors hover:bg-muted/50">
              <td class="p-2 align-middle font-medium">{{ c.name }}</td>
              <td class="p-2 align-middle">
                <code class="rounded bg-muted px-1.5 py-0.5 text-xs font-mono">{{ c.type }}</code>
              </td>
              <td class="p-2 align-middle">
                <span class="inline-flex items-center rounded-md border px-2.5 py-0.5 text-xs font-semibold border-transparent bg-secondary text-secondary-foreground">{{ c.category }}</span>
              </td>
              <td class="p-2 align-middle text-muted-foreground max-w-[250px] truncate">{{ c.description }}</td>
              <td class="p-2 align-middle">
                <button
                  class="relative inline-flex h-5 w-9 shrink-0 cursor-pointer items-center rounded-full border-2 border-transparent transition-colors duration-200 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:ring-offset-background"
                  :class="c.enabled ? 'bg-primary' : 'bg-input'"
                  @click="toggleComponent(c.id)"
                  role="switch"
                  :aria-checked="c.enabled"
                >
                  <span class="pointer-events-none block h-4 w-4 rounded-full bg-background shadow-sm ring-0 transition-transform duration-200"
                    :class="c.enabled ? 'translate-x-4' : 'translate-x-0'"
                  />
                </button>
              </td>
            </tr>
            <tr v-if="filtered.length === 0">
              <td colspan="5" class="p-2 align-middle text-center text-muted-foreground py-8">No components found.</td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>
