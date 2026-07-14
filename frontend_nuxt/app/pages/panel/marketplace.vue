<script setup lang="ts">
definePageMeta({ layout: 'panel', sidebarSection: 'marketplace' })

const assets = ref([
  { id: '1', name: 'Modern Dashboard', type: 'Template', downloads: 1234, price: 'Free', popular: true },
  { id: '2', name: 'Hero Animations Pack', type: 'Animation', downloads: 856, price: '$9.99', popular: false },
  { id: '3', name: 'Business Icons Set', type: 'Icon', downloads: 2100, price: 'Free', popular: true },
  { id: '4', name: 'Portfolio Layout Kit', type: 'Template', downloads: 654, price: '$14.99', popular: false },
  { id: '5', name: 'Contact Form Builder', type: 'Component', downloads: 1567, price: 'Free', popular: true },
  { id: '6', name: 'Premium UI Components', type: 'Component', downloads: 423, price: '$19.99', popular: false },
])

const categories = ['All', 'Templates', 'Components', 'Animations', 'Icons']
const activeCategory = ref('All')
</script>

<template>
  <div class="space-y-6">
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-2xl font-bold tracking-tight">Marketplace</h1>
        <p class="text-muted-foreground text-sm mt-1">Discover assets to enhance your websites</p>
      </div>
      <NuxtLink to="/panel/subscription" class="btn btn-outline btn-sm">
        <svg class="w-4 h-4 mr-1.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 10h18M7 15h1m4 0h1m-7 4h12a3 3 0 003-3V8a3 3 0 00-3-3H6a3 3 0 00-3 3v8a3 3 0 003 3z" />
        </svg>
        Upgrade Plan
      </NuxtLink>
    </div>

    <div class="flex items-center gap-2 overflow-x-auto pb-2">
      <button
        v-for="cat in categories"
        :key="cat"
        class="btn btn-sm"
        :class="activeCategory === cat ? 'btn-primary' : 'btn-secondary'"
        @click="activeCategory = cat"
      >
        {{ cat }}
      </button>
    </div>

    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      <div v-for="asset in assets" :key="asset.id" class="card overflow-hidden group hover:shadow-md transition-all">
        <div class="h-36 bg-gradient-to-br from-primary/10 to-primary/5 flex items-center justify-center relative">
          <svg class="w-10 h-10 text-primary/40" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M19.5 14.25v-2.625a3.375 3.375 0 00-3.375-3.375h-1.5A1.125 1.125 0 0113.5 7.125v-1.5a3.375 3.375 0 00-3.375-3.375H8.25m0 12.75h7.5m-7.5 3H12M10.5 2.25H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 00-9-9z" />
          </svg>
          <div v-if="asset.popular" class="absolute top-2 right-2 badge bg-primary/10 text-primary border-primary/20 text-[10px]">
            Popular
          </div>
        </div>
        <div class="p-4">
          <h3 class="font-semibold text-sm">{{ asset.name }}</h3>
          <div class="flex items-center gap-2 mt-2 text-xs text-muted-foreground">
            <span class="badge bg-secondary text-secondary-foreground border-0 text-[10px]">{{ asset.type }}</span>
            <span>{{ asset.downloads.toLocaleString() }} downloads</span>
          </div>
          <div class="flex items-center justify-between mt-4">
            <span class="font-bold text-foreground">{{ asset.price }}</span>
            <button class="btn btn-primary btn-sm">Get</button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
