<script setup lang="ts">
import type { ComponentNode } from '~/types'

const props = defineProps<{
  selectedComponentId: string | null
}>()

const subscriptionStore = useSubscriptionStore()

// Get component from canvas
const selectedComponent = ref<ComponentNode | null>(null)

// Property tabs
const activeTab = ref<'properties' | 'styles' | 'animation'>('properties')

// Animation feature check
const canEditAnimations = computed(() => 
  subscriptionStore.isFeatureEnabled('animationEditor')
)

// Custom CSS feature check
const canEditCustomCSS = computed(() => 
  subscriptionStore.isFeatureEnabled('customCSS')
)

// Responsive design check
const canEditResponsive = computed(() => 
  subscriptionStore.isFeatureEnabled('responsiveDesign')
)

// Current breakpoint for styles
const currentBreakpoint = ref<'desktop' | 'tablet' | 'mobile'>('desktop')

// Form state for properties
const componentProps = ref<Record<string, unknown>>({})

// Watch for component selection changes
watch(() => props.selectedComponentId, (id) => {
  if (id) {
    // Fetch component data
    // For now, use placeholder
    selectedComponent.value = null
    componentProps.value = {}
  } else {
    selectedComponent.value = null
    componentProps.value = {}
  }
}, { immediate: true })

// Update property
function updateProperty(key: string, value: unknown) {
  componentProps.value[key] = value
  // TODO: Update component via canvas store
}

// Update style
function updateStyle(property: string, value: string) {
  // TODO: Update component styles via canvas store
}
</script>

<template>
  <div class="h-full flex flex-col">
    <!-- Header -->
    <div class="p-4 border-b border-gray-200">
      <h3 class="font-semibold text-gray-900">
        {{ selectedComponent ? selectedComponent.type : 'Properties' }}
      </h3>
      <p v-if="selectedComponent" class="text-sm text-gray-500 mt-1">
        ID: {{ selectedComponent.id }}
      </p>
    </div>
    
    <!-- No Selection State -->
    <div v-if="!selectedComponent" class="flex-1 flex items-center justify-center p-4">
      <div class="text-center">
        <svg class="mx-auto w-12 h-12 text-gray-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 15l-2 5L9 9l11 4-5 2zm0 0l5 5M7.188 2.239l.777 2.897M5.136 7.965l-2.898-.777M13.95 4.05l-2.122 2.122m-5.657 5.656l-2.12 2.122" />
        </svg>
        <p class="mt-2 text-sm text-gray-500">
          Select a component to edit its properties
        </p>
      </div>
    </div>
    
    <!-- Property Editor -->
    <div v-else class="flex-1 overflow-y-auto">
      <!-- Tabs -->
      <div class="flex border-b border-gray-200">
        <button
          v-for="tab in ['properties', 'styles', 'animation']"
          :key="tab"
          class="flex-1 px-4 py-3 text-sm font-medium text-center transition-colors"
          :class="activeTab === tab 
            ? 'text-primary-600 border-b-2 border-primary-600' 
            : 'text-gray-500 hover:text-gray-700'"
          @click="activeTab = tab as typeof activeTab"
        >
          {{ tab.charAt(0).toUpperCase() + tab.slice(1) }}
        </button>
      </div>
      
      <!-- Properties Tab -->
      <div v-if="activeTab === 'properties'" class="p-4 space-y-4">
        <!-- Content Property (for Text components) -->
        <div v-if="'content' in componentProps">
          <label class="label">Content</label>
          <textarea
            v-model="componentProps.content as string"
            rows="3"
            class="input"
            @input="updateProperty('content', ($event.target as HTMLTextAreaElement).value)"
          />
        </div>
        
        <!-- Label Property (for Button components) -->
        <div v-if="'label' in componentProps">
          <label class="label">Button Text</label>
          <input
            v-model="componentProps.label as string"
            type="text"
            class="input"
            @input="updateProperty('label', ($event.target as HTMLInputElement).value)"
          />
        </div>
        
        <!-- Image Source -->
        <div v-if="'src' in componentProps">
          <label class="label">Image URL</label>
          <input
            v-model="componentProps.src as string"
            type="url"
            class="input"
            placeholder="https://..."
            @input="updateProperty('src', ($event.target as HTMLInputElement).value)"
          />
        </div>
        
        <!-- Alt Text -->
        <div v-if="'alt' in componentProps">
          <label class="label">Alt Text</label>
          <input
            v-model="componentProps.alt as string"
            type="text"
            class="input"
            @input="updateProperty('alt', ($event.target as HTMLInputElement).value)"
          />
        </div>
        
        <!-- Link URL -->
        <div>
          <label class="label">Link URL</label>
          <input
            type="url"
            class="input"
            placeholder="https://..."
          />
        </div>
      </div>
      
      <!-- Styles Tab -->
      <div v-if="activeTab === 'styles'" class="p-4 space-y-4">
        <!-- Breakpoint Selector (for responsive) -->
        <div v-if="canEditResponsive" class="mb-4">
          <label class="label">Breakpoint</label>
          <div class="flex gap-2">
            <button
              v-for="bp in ['desktop', 'tablet', 'mobile'] as const"
              :key="bp"
              class="flex-1 py-2 text-sm font-medium rounded-lg transition-colors"
              :class="currentBreakpoint === bp 
                ? 'bg-primary-100 text-primary-700' 
                : 'bg-gray-100 text-gray-600 hover:bg-gray-200'"
              @click="currentBreakpoint = bp"
            >
              {{ bp.charAt(0).toUpperCase() + bp.slice(1) }}
            </button>
          </div>
        </div>
        
        <!-- Background Color -->
        <div>
          <label class="label">Background Color</label>
          <div class="flex gap-2">
            <input
              type="color"
              class="w-10 h-10 rounded border border-gray-300 cursor-pointer"
            />
            <input
              type="text"
              class="input flex-1"
              placeholder="#ffffff"
            />
          </div>
        </div>
        
        <!-- Text Color -->
        <div>
          <label class="label">Text Color</label>
          <div class="flex gap-2">
            <input
              type="color"
              class="w-10 h-10 rounded border border-gray-300 cursor-pointer"
            />
            <input
              type="text"
              class="input flex-1"
              placeholder="#000000"
            />
          </div>
        </div>
        
        <!-- Padding -->
        <div>
          <label class="label">Padding</label>
          <div class="grid grid-cols-4 gap-2">
            <input type="text" class="input text-center" placeholder="Top" />
            <input type="text" class="input text-center" placeholder="Right" />
            <input type="text" class="input text-center" placeholder="Bottom" />
            <input type="text" class="input text-center" placeholder="Left" />
          </div>
        </div>
        
        <!-- Margin -->
        <div>
          <label class="label">Margin</label>
          <div class="grid grid-cols-4 gap-2">
            <input type="text" class="input text-center" placeholder="Top" />
            <input type="text" class="input text-center" placeholder="Right" />
            <input type="text" class="input text-center" placeholder="Bottom" />
            <input type="text" class="input text-center" placeholder="Left" />
          </div>
        </div>
        
        <!-- Border Radius -->
        <div>
          <label class="label">Border Radius</label>
          <input type="text" class="input" placeholder="8px" />
        </div>
        
        <!-- Custom CSS -->
        <div v-if="canEditCustomCSS">
          <label class="label">Custom CSS</label>
          <textarea
            rows="4"
            class="input font-mono text-sm"
            placeholder=".my-class { ... }"
          />
        </div>
      </div>
      
      <!-- Animation Tab -->
      <div v-if="activeTab === 'animation'" class="p-4 space-y-4">
        <!-- Animation Lock Message -->
        <div v-if="!canEditAnimations" class="text-center py-8">
          <svg class="mx-auto w-12 h-12 text-gray-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" />
          </svg>
          <h4 class="mt-2 text-sm font-medium text-gray-900">Animation Editor Locked</h4>
          <p class="mt-1 text-sm text-gray-500">
            Upgrade to Enterprise or Exclusive to add animations
          </p>
          <NuxtLink to="/dashboard/subscription" class="btn btn-primary btn-sm mt-4">
            Upgrade Plan
          </NuxtLink>
        </div>
        
        <!-- Animation Controls -->
        <template v-else>
          <div>
            <label class="label">Animation Type</label>
            <select class="input">
              <option value="">None</option>
              <option value="fade">Fade</option>
              <option value="slide">Slide</option>
              <option value="bounce">Bounce</option>
              <option value="rotate">Rotate</option>
              <option value="scale">Scale</option>
            </select>
          </div>
          
          <div>
            <label class="label">Duration (ms)</label>
            <input
              type="number"
              min="100"
              max="10000"
              class="input"
              placeholder="500"
            />
          </div>
          
          <div>
            <label class="label">Delay (ms)</label>
            <input
              type="number"
              min="0"
              max="5000"
              class="input"
              placeholder="0"
            />
          </div>
          
          <div>
            <label class="label">Easing</label>
            <select class="input">
              <option value="linear">Linear</option>
              <option value="ease-in">Ease In</option>
              <option value="ease-out">Ease Out</option>
              <option value="ease-in-out">Ease In Out</option>
            </select>
          </div>
          
          <div>
            <label class="label">Trigger</label>
            <select class="input">
              <option value="load">On Load</option>
              <option value="scroll">On Scroll</option>
              <option value="hover">On Hover</option>
              <option value="click">On Click</option>
            </select>
          </div>
        </template>
      </div>
    </div>
  </div>
</template>
