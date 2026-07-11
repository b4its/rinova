<script setup lang="ts">
definePageMeta({
  layout: 'dashboard'
})

const subscriptionStore = useSubscriptionStore()

// Projects placeholder
const projects = ref([
  // Will be populated from API
])

const isLoading = ref(false)
const searchQuery = ref('')
const viewMode = ref<'grid' | 'list'>('grid')

// Check if can create new project
const canCreateProject = computed(() => {
  return subscriptionStore.checkLimit('maxProjects', projects.value.length)
})

// Filter projects
const filteredProjects = computed(() => {
  if (!searchQuery.value) return projects.value
  const query = searchQuery.value.toLowerCase()
  return projects.value.filter((p: { name: string }) => 
    p.name.toLowerCase().includes(query)
  )
})

// Create project modal
const showCreateModal = ref(false)
</script>

<template>
  <div>
    <!-- Header -->
    <div class="flex items-center justify-between mb-6">
      <div>
        <h1 class="text-2xl font-bold text-gray-900">Projects</h1>
        <p class="mt-1 text-sm text-gray-600">
          Manage and organize your website projects
        </p>
      </div>
      
      <button
        class="btn btn-primary"
        :disabled="!canCreateProject"
        @click="showCreateModal = true"
      >
        <svg class="-ml-1 mr-2 h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
        </svg>
        New Project
      </button>
    </div>

    <!-- Limit Warning -->
    <div
      v-if="!canCreateProject"
      class="mb-6 p-4 bg-yellow-50 border border-yellow-200 rounded-lg"
    >
      <div class="flex items-center gap-3">
        <svg class="w-5 h-5 text-yellow-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
        </svg>
        <div>
          <p class="font-medium text-yellow-800">Project limit reached</p>
          <p class="text-sm text-yellow-700">
            You've reached the maximum number of projects for your plan. 
            <NuxtLink to="/dashboard/subscription" class="underline">Upgrade</NuxtLink> to create more.
          </p>
        </div>
      </div>
    </div>

    <!-- Filters & View Toggle -->
    <div class="flex items-center gap-4 mb-6">
      <!-- Search -->
      <div class="flex-1 relative">
        <svg class="absolute left-3 top-1/2 -translate-y-1/2 w-5 h-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
        </svg>
        <input
          v-model="searchQuery"
          type="search"
          placeholder="Search projects..."
          class="input pl-10"
        >
      </div>
      
      <!-- View Toggle -->
      <div class="flex items-center border border-gray-200 rounded-lg overflow-hidden">
        <button
          class="p-2"
          :class="viewMode === 'grid' ? 'bg-gray-100' : 'bg-white hover:bg-gray-50'"
          @click="viewMode = 'grid'"
        >
          <svg class="w-5 h-5 text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zM14 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zM14 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z" />
          </svg>
        </button>
        <button
          class="p-2"
          :class="viewMode === 'list' ? 'bg-gray-100' : 'bg-white hover:bg-gray-50'"
          @click="viewMode = 'list'"
        >
          <svg class="w-5 h-5 text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 10h16M4 14h16M4 18h16" />
          </svg>
        </button>
      </div>
    </div>

    <!-- Projects Grid/List -->
    <div v-if="isLoading" class="flex justify-center py-12">
      <svg class="animate-spin w-8 h-8 text-primary-600" fill="none" viewBox="0 0 24 24">
        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
        <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z" />
      </svg>
    </div>

    <div v-else-if="filteredProjects.length === 0" class="card">
      <div class="card-body text-center py-12">
        <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
        </svg>
        <h3 class="mt-2 text-sm font-medium text-gray-900">
          {{ searchQuery ? 'No projects found' : 'No projects yet' }}
        </h3>
        <p class="mt-1 text-sm text-gray-500">
          {{ searchQuery ? 'Try a different search term' : 'Get started by creating your first project.' }}
        </p>
        <div v-if="!searchQuery && canCreateProject" class="mt-6">
          <button class="btn btn-primary" @click="showCreateModal = true">
            <svg class="-ml-1 mr-2 h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
            </svg>
            New Project
          </button>
        </div>
      </div>
    </div>

    <div
      v-else
      :class="viewMode === 'grid' ? 'grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6' : 'space-y-4'"
    >
      <!-- Project cards will be rendered here -->
      <div
        v-for="project in filteredProjects"
        :key="project.id"
        class="card group"
      >
        <!-- Project card content -->
      </div>
    </div>

    <!-- Create Project Modal -->
    <CreateProjectModal
      v-if="showCreateModal"
      @close="showCreateModal = false"
    />
  </div>
</template>
