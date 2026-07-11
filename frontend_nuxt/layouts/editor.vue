<script setup lang="ts">
const route = useRoute()
const projectId = computed(() => route.params.id as string)

const userStore = useUserStore()
const subscriptionStore = useSubscriptionStore()
const workspaceStore = useWorkspaceStore()

// Redirect if not authenticated
if (!userStore.isAuthenticated) {
  navigateTo('/login')
}

// Viewport mode
const viewportMode = ref<'desktop' | 'tablet' | 'mobile'>('desktop')
const viewportWidth = computed(() => {
  switch (viewportMode.value) {
    case 'mobile': return '375px'
    case 'tablet': return '768px'
    default: return '100%'
  }
})

// Canvas state
const canvas = useCanvas(projectId.value)
const selectedComponentId = computed(() => canvas.selectedComponentId.value)

// Undo/Redo
function handleUndo() {
  canvas.undo()
}

function handleRedo() {
  canvas.redo()
}

// Save
const isSaving = ref(false)
async function handleSave() {
  isSaving.value = true
  await canvas.saveToBackend()
  isSaving.value = false
}

// Preview
const showPreview = ref(false)

// Publish
const canPublish = computed(() => subscriptionStore.isFeatureEnabled('oneClickPublish'))
</script>

<template>
  <div class="h-screen flex flex-col bg-gray-100">
    <!-- Top Toolbar -->
    <header class="h-14 bg-white border-b border-gray-200 flex items-center justify-between px-4 z-30">
      <!-- Left: Logo & Project Name -->
      <div class="flex items-center gap-4">
        <NuxtLink to="/dashboard" class="text-lg font-bold text-primary-600">
          Rinova
        </NuxtLink>
        <div class="h-6 w-px bg-gray-200"></div>
        <div class="flex items-center gap-2">
          <input
            type="text"
            value="My Project"
            class="text-sm font-medium text-gray-700 bg-transparent border-0 focus:ring-0 p-0"
          />
        </div>
      </div>
      
      <!-- Center: Viewport Controls -->
      <div class="flex items-center gap-2 bg-gray-100 rounded-lg p-1">
        <button
          v-for="mode in ['desktop', 'tablet', 'mobile'] as const"
          :key="mode"
          class="p-2 rounded"
          :class="viewportMode === mode ? 'bg-white shadow-sm' : 'hover:bg-gray-200'"
          @click="viewportMode = mode"
        >
          <svg v-if="mode === 'desktop'" class="w-4 h-4 text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
          </svg>
          <svg v-else-if="mode === 'tablet'" class="w-4 h-4 text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 18h.01M7 21h10a2 2 0 002-2V5a2 2 0 00-2-2H7a2 2 0 00-2 2v14a2 2 0 002 2z" />
          </svg>
          <svg v-else class="w-4 h-4 text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 18h.01M8 21h8a2 2 0 002-2V5a2 2 0 00-2-2H8a2 2 0 00-2 2v14a2 2 0 002 2z" />
          </svg>
        </button>
      </div>
      
      <!-- Right: Actions -->
      <div class="flex items-center gap-2">
        <!-- Undo/Redo -->
        <button
          class="p-2 text-gray-600 hover:bg-gray-100 rounded-lg"
          title="Undo"
          @click="handleUndo"
        >
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 10h10a8 8 0 018 8v2M3 10l6 6m-6-6l6-6" />
          </svg>
        </button>
        <button
          class="p-2 text-gray-600 hover:bg-gray-100 rounded-lg"
          title="Redo"
          @click="handleRedo"
        >
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 10h-10a8 8 0 00-8 8v2M21 10l-6 6m6-6l-6-6" />
          </svg>
        </button>
        
        <div class="h-6 w-px bg-gray-200 mx-2"></div>
        
        <!-- Save -->
        <button
          class="btn btn-secondary btn-sm"
          :disabled="isSaving"
          @click="handleSave"
        >
          <svg v-if="isSaving" class="animate-spin w-4 h-4 mr-1" fill="none" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
          </svg>
          {{ isSaving ? 'Saving...' : 'Save' }}
        </button>
        
        <!-- Preview -->
        <button class="btn btn-secondary btn-sm" @click="showPreview = true">
          <svg class="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
          </svg>
          Preview
        </button>
        
        <!-- Publish -->
        <button
          v-if="canPublish"
          class="btn btn-primary btn-sm"
        >
          <svg class="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12" />
          </svg>
          Publish
        </button>
      </div>
    </header>
    
    <!-- Main Editor Area -->
    <div class="flex-1 flex overflow-hidden">
      <!-- Left Sidebar: Components -->
      <aside class="w-64 bg-white border-r border-gray-200 overflow-y-auto">
        <SidebarComponents />
      </aside>
      
      <!-- Canvas Area -->
      <main class="flex-1 overflow-hidden p-4">
        <div class="h-full flex items-center justify-center">
          <div
            class="canvas-container h-full transition-all duration-200"
            :style="{ width: viewportWidth, maxWidth: '100%' }"
          >
            <BuilderCanvas
              :project-id="projectId"
              :viewport-mode="viewportMode"
            />
          </div>
        </div>
      </main>
      
      <!-- Right Sidebar: Properties -->
      <aside class="w-80 bg-white border-l border-gray-200 overflow-y-auto">
        <PropertyPanel :selected-component-id="selectedComponentId" />
      </aside>
    </div>
  </div>
</template>
