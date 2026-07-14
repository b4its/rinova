<script setup lang="ts">
definePageMeta({ layout: 'panel', sidebarSection: 'admin-products' })

interface Product {
  id: string
  name: string
  description: string
  price: number
  category: string
  status: 'active' | 'inactive'
}

const products = ref<Product[]>([
  { id: '1', name: 'Website Builder Pro', description: 'Professional website builder with full features', price: 29.99, category: 'Software', status: 'active' },
  { id: '2', name: 'SEO Optimization Pack', description: 'Boost your site ranking with advanced SEO tools', price: 19.99, category: 'Service', status: 'active' },
  { id: '3', name: 'Analytics Dashboard', description: 'Detailed analytics and insights for your website', price: 9.99, category: 'Add-on', status: 'active' },
  { id: '4', name: 'Custom Domain Setup', description: 'Professional custom domain configuration', price: 14.99, category: 'Service', status: 'inactive' },
])

const dialogOpen = ref(false)
const editingProduct = ref<Product | null>(null)
const formName = ref('')
const formDescription = ref('')
const formPrice = ref(0)
const formCategory = ref('Software')
const formStatus = ref<'active' | 'inactive'>('active')

function openAdd() {
  editingProduct.value = null
  formName.value = ''
  formDescription.value = ''
  formPrice.value = 0
  formCategory.value = 'Software'
  formStatus.value = 'active'
  dialogOpen.value = true
}

function openEdit(p: Product) {
  editingProduct.value = p
  formName.value = p.name
  formDescription.value = p.description
  formPrice.value = p.price
  formCategory.value = p.category
  formStatus.value = p.status
  dialogOpen.value = true
}

function saveProduct() {
  if (editingProduct.value) {
    const idx = products.value.findIndex(p => p.id === editingProduct.value!.id)
    if (idx !== -1) {
      products.value[idx] = { ...products.value[idx], name: formName.value, description: formDescription.value, price: formPrice.value, category: formCategory.value, status: formStatus.value }
    }
  } else {
    products.value.push({
      id: String(Date.now()),
      name: formName.value,
      description: formDescription.value,
      price: formPrice.value,
      category: formCategory.value,
      status: formStatus.value,
    })
  }
  dialogOpen.value = false
}

function removeProduct(id: string) {
  products.value = products.value.filter(p => p.id !== id)
}

const search = ref('')
const filtered = computed(() => {
  if (!search.value) return products.value
  const q = search.value.toLowerCase()
  return products.value.filter(p => p.name.toLowerCase().includes(q) || p.category.toLowerCase().includes(q))
})
</script>

<template>
  <div class="space-y-6">
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-2xl font-bold tracking-tight">Products</h1>
        <p class="text-muted-foreground">Manage products and services offered on the platform.</p>
      </div>
      <button class="inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50 bg-primary text-primary-foreground shadow hover:bg-primary/90 h-9 px-4 py-2" @click="openAdd">
        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
        </svg>
        Add Product
      </button>
    </div>

    <div class="flex w-full max-w-sm items-center gap-2 rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm">
      <svg class="w-4 h-4 text-muted-foreground shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
      </svg>
      <input v-model="search" type="text" placeholder="Search products..." class="flex-1 bg-transparent outline-none placeholder:text-muted-foreground text-sm">
    </div>

    <div class="rounded-xl border bg-card text-card-foreground shadow">
      <div class="relative w-full overflow-auto">
        <table class="w-full caption-bottom text-sm">
          <thead class="[&_tr]:border-b">
            <tr class="border-b border-border transition-colors hover:bg-muted/50">
              <th class="h-10 px-2 text-left align-middle font-medium text-muted-foreground">Name</th>
              <th class="h-10 px-2 text-left align-middle font-medium text-muted-foreground">Description</th>
              <th class="h-10 px-2 text-left align-middle font-medium text-muted-foreground">Price</th>
              <th class="h-10 px-2 text-left align-middle font-medium text-muted-foreground">Category</th>
              <th class="h-10 px-2 text-left align-middle font-medium text-muted-foreground">Status</th>
              <th class="h-10 px-2 text-left align-middle font-medium text-muted-foreground">Actions</th>
            </tr>
          </thead>
          <tbody class="[&_tr:last-child]:border-0">
            <tr v-for="p in filtered" :key="p.id" class="border-b border-border transition-colors hover:bg-muted/50">
              <td class="p-2 align-middle font-medium">{{ p.name }}</td>
              <td class="p-2 align-middle text-muted-foreground max-w-[200px] truncate">{{ p.description }}</td>
              <td class="p-2 align-middle">${{ p.price.toFixed(2) }}</td>
              <td class="p-2 align-middle">
                <span class="inline-flex items-center rounded-md border px-2.5 py-0.5 text-xs font-semibold border-transparent bg-secondary text-secondary-foreground">{{ p.category }}</span>
              </td>
              <td class="p-2 align-middle">
                <span class="inline-flex items-center rounded-md border px-2.5 py-0.5 text-xs font-semibold"
                  :class="p.status === 'active' ? 'border-transparent bg-emerald-100 text-emerald-700 dark:bg-emerald-900/40 dark:text-emerald-400' : 'border-transparent bg-destructive/10 text-destructive'"
                >{{ p.status }}</span>
              </td>
              <td class="p-2 align-middle">
                <div class="flex items-center gap-1">
                  <button class="inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-xs font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50 hover:bg-accent hover:text-accent-foreground h-8 px-2" @click="openEdit(p)">Edit</button>
                  <button class="inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-xs font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50 hover:bg-destructive/10 hover:text-destructive h-8 px-2" @click="removeProduct(p.id)">Delete</button>
                </div>
              </td>
            </tr>
            <tr v-if="filtered.length === 0">
              <td colspan="6" class="p-2 align-middle text-center text-muted-foreground py-8">No products found.</td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <!-- Dialog -->
    <div v-if="dialogOpen" class="fixed inset-0 z-50 flex items-center justify-center">
      <div class="fixed inset-0 bg-black/80" @click="dialogOpen = false" />
      <div class="fixed left-[50%] top-[50%] z-50 grid w-full max-w-lg translate-x-[-50%] translate-y-[-50%] gap-4 border bg-background text-foreground p-6 shadow-lg duration-200 sm:rounded-lg">
        <div class="flex flex-col space-y-1.5">
          <h2 class="text-lg font-semibold leading-none tracking-tight">{{ editingProduct ? 'Edit Product' : 'Add Product' }}</h2>
          <p class="text-sm text-muted-foreground">{{ editingProduct ? 'Update product details.' : 'Create a new product or service.' }}</p>
        </div>
        <div class="space-y-4">
          <div class="space-y-1">
            <label class="text-sm font-medium leading-none">Name</label>
            <input v-model="formName" type="text" class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring" placeholder="Product name">
          </div>
          <div class="space-y-1">
            <label class="text-sm font-medium leading-none">Description</label>
            <textarea v-model="formDescription" class="flex min-h-[60px] w-full rounded-md border border-input bg-transparent px-3 py-2 text-sm shadow-sm placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring" placeholder="Description" />
          </div>
          <div class="grid grid-cols-2 gap-4">
            <div class="space-y-1">
              <label class="text-sm font-medium leading-none">Price ($)</label>
              <input v-model.number="formPrice" type="number" step="0.01" min="0" class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring">
            </div>
            <div class="space-y-1">
              <label class="text-sm font-medium leading-none">Category</label>
              <select v-model="formCategory" class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring">
                <option value="Software">Software</option>
                <option value="Service">Service</option>
                <option value="Add-on">Add-on</option>
                <option value="Template">Template</option>
              </select>
            </div>
          </div>
          <div class="space-y-1">
            <label class="text-sm font-medium leading-none">Status</label>
            <select v-model="formStatus" class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring">
              <option value="active">Active</option>
              <option value="inactive">Inactive</option>
            </select>
          </div>
        </div>
        <div class="flex flex-col-reverse sm:flex-row sm:justify-end sm:space-x-2">
          <button class="inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50 hover:bg-accent hover:text-accent-foreground h-9 px-4 py-2" @click="dialogOpen = false">Cancel</button>
          <button class="inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50 bg-primary text-primary-foreground shadow hover:bg-primary/90 h-9 px-4 py-2" @click="saveProduct">{{ editingProduct ? 'Save' : 'Create' }}</button>
        </div>
      </div>
    </div>
  </div>
</template>
