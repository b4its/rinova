<script setup lang="ts">
import type { ComponentType } from '~/types'

const subscriptionStore = useSubscriptionStore()
const searchQuery = ref('')
const expandedCategory = ref<string | null>('basic')

// Component categories
const categories = [
  {
    id: 'basic',
    name: 'Basic',
    components: [
      { type: 'Text' as ComponentType, name: 'Text', icon: 'type', premium: false },
      { type: 'Image' as ComponentType, name: 'Image', icon: 'image', premium: false },
      { type: 'Button' as ComponentType, name: 'Button', icon: 'button', premium: false },
      { type: 'Divider' as ComponentType, name: 'Divider', icon: 'divider', premium: false }
    ]
  },
  {
    id: 'layout',
    name: 'Layout',
    components: [
      { type: 'Container' as ComponentType, name: 'Container', icon: 'container', premium: false },
      { type: 'Header' as ComponentType, name: 'Header', icon: 'header', premium: false },
      { type: 'Footer' as ComponentType, name: 'Footer', icon: 'footer', premium: false }
    ]
  },
  {
    id: 'sections',
    name: 'Sections',
    components: [
      { type: 'Hero' as ComponentType, name: 'Hero', icon: 'hero', premium: false },
      { type: 'Gallery' as ComponentType, name: 'Gallery', icon: 'gallery', premium: true },
      { type: 'Form' as ComponentType, name: 'Form', icon: 'form', premium: false }
    ]
  },
  {
    id: 'media',
    name: 'Media',
    components: [
      { type: 'Video' as ComponentType, name: 'Video', icon: 'video', premium: true }
    ]
  }
]

// Filter components by search
const filteredCategories = computed(() => {
  if (!searchQuery.value) return categories
  
  return categories
    .map(cat => ({
      ...cat,
      components: cat.components.filter(comp =>
        comp.name.toLowerCase().includes(searchQuery.value.toLowerCase())
      )
    }))
    .filter(cat => cat.components.length > 0)
})

// Check if user can access premium components
const canAccessPremium = computed(() => 
  subscriptionStore.isFeatureEnabled('assetMarketplace')
)

// Drag start handler
function handleDragStart(event: DragEvent, componentType: ComponentType) {
  if (event.dataTransfer) {
    event.dataTransfer.setData('componentType', componentType)
    event.dataTransfer.effectAllowed = 'copy'
  }
}
</script>

<template>
  <div class="h-full flex flex-col">
    <!-- Header -->
    <div class="p-4 border-b border-gray-200">
      <h3 class="font-semibold text-gray-900 mb-3">Components</h3>
      
      <!-- Search -->
      <div class="relative">
        <svg class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
        </svg>
        <input
          v-model="searchQuery"
          type="search"
          placeholder="Search components..."
          class="input pl-9 py-2 text-sm"
        >
      </div>
    </div>
    
    <!-- Component List -->
    <div class="flex-1 overflow-y-auto p-2">
      <div
        v-for="category in filteredCategories"
        :key="category.id"
        class="mb-2"
      >
        <!-- Category Header -->
        <button
          class="w-full flex items-center justify-between px-3 py-2 text-sm font-medium text-gray-700 hover:bg-gray-100 rounded-lg"
          @click="expandedCategory = expandedCategory === category.id ? null : category.id"
        >
          <span>{{ category.name }}</span>
          <svg
            class="w-4 h-4 text-gray-400 transition-transform"
            :class="expandedCategory === category.id ? 'rotate-180' : ''"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
          </svg>
        </button>
        
        <!-- Components -->
        <div
          v-show="expandedCategory === category.id"
          class="grid grid-cols-2 gap-2 mt-1 px-1"
        >
          <div
            v-for="component in category.components"
            :key="component.type"
            class="relative p-3 bg-gray-50 rounded-lg border border-gray-200 cursor-grab hover:border-primary-300 hover:bg-primary-50 transition-colors group"
            :class="{ 'opacity-50': component.premium && !canAccessPremium }"
            draggable="true"
            @dragstart="handleDragStart($event, component.type)"
          >
            <!-- Premium Badge -->
            <span
              v-if="component.premium"
              class="absolute -top-1 -right-1 badge text-xs"
              :class="canAccessPremium ? 'badge-premium' : 'badge-locked'"
            >
              {{ canAccessPremium ? '✓' : '🔒' }}
            </span>
            
            <!-- Icon -->
            <div class="w-8 h-8 mx-auto mb-2 flex items-center justify-center text-gray-600">
              <!-- Text Icon -->
              <svg v-if="component.icon === 'type'" class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16m-7 6h7" />
              </svg>
              
              <!-- Image Icon -->
              <svg v-else-if="component.icon === 'image'" class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
              </svg>
              
              <!-- Button Icon -->
              <svg v-else-if="component.icon === 'button'" class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 15l-2 5L9 9l11 4-5 2zm0 0l5 5M7.188 2.239l.777 2.897M5.136 7.965l-2.898-.777M13.95 4.05l-2.122 2.122m-5.657 5.656l-2.12 2.122" />
              </svg>
              
              <!-- Divider Icon -->
              <svg v-else-if="component.icon === 'divider'" class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 12H4" />
              </svg>
              
              <!-- Container Icon -->
              <svg v-else-if="component.icon === 'container'" class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 5a1 1 0 011-1h14a1 1 0 011 1v2a1 1 0 01-1 1H5a1 1 0 01-1-1V5zM4 13a1 1 0 011-1h6a1 1 0 011 1v6a1 1 0 01-1 1H5a1 1 0 01-1-1v-6zM16 13a1 1 0 011-1h2a1 1 0 011 1v6a1 1 0 01-1 1h-2a1 1 0 01-1-1v-6z" />
              </svg>
              
              <!-- Header Icon -->
              <svg v-else-if="component.icon === 'header'" class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 10h16M4 14h8m-8 4h16" />
              </svg>
              
              <!-- Footer Icon -->
              <svg v-else-if="component.icon === 'footer'" class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M12 10h8M12 14h8M4 18h16" />
              </svg>
              
              <!-- Hero Icon -->
              <svg v-else-if="component.icon === 'hero'" class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 3v4M3 5h4M6 17v4m-2-2h4m5-16l2.286 6.857L21 12l-5.714 2.143L13 21l-2.286-6.857L5 12l5.714-2.143L13 3z" />
              </svg>
              
              <!-- Gallery Icon -->
              <svg v-else-if="component.icon === 'gallery'" class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
              </svg>
              
              <!-- Form Icon -->
              <svg v-else-if="component.icon === 'form'" class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
              </svg>
              
              <!-- Video Icon -->
              <svg v-else-if="component.icon === 'video'" class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z" />
              </svg>
              
              <!-- Default Icon -->
              <svg v-else class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 5a1 1 0 011-1h14a1 1 0 011 1v2a1 1 0 01-1 1H5a1 1 0 01-1-1V5z" />
              </svg>
            </div>
            
            <!-- Name -->
            <p class="text-xs text-center text-gray-700 font-medium">
              {{ component.name }}
            </p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
