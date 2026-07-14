<script setup lang="ts">
definePageMeta({ layout: 'panel', sidebarSection: 'projects' })

import { relativeTime, type Project } from '~/composables/useProjects'

const subscriptionStore = useSubscriptionStore()
const { listProjects } = useProjects()

const projects = ref<Project[]>([])
const isLoading = ref(true)
const loadError = ref<string | null>(null)

async function loadProjects() {
  isLoading.value = true
  loadError.value = null
  try {
    projects.value = await listProjects()
  } catch (e) {
    loadError.value = e instanceof Error ? e.message : 'Failed to load projects'
  } finally {
    isLoading.value = false
  }
}

onMounted(loadProjects)

const searchQuery = ref('')
const viewMode = ref<'grid' | 'list'>('grid')

const canCreateProject = computed(() => {
  return subscriptionStore.checkLimit('maxProjects', projects.value.length)
})

const filteredProjects = computed(() => {
  if (!searchQuery.value) return projects.value
  const q = searchQuery.value.toLowerCase()
  return projects.value.filter(p => p.name.toLowerCase().includes(q))
})
</script>

<template>
  <div class="space-y-6">
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-2xl font-bold tracking-tight">Projects</h1>
        <p class="text-muted-foreground text-sm mt-1">Manage and organize your website projects</p>
      </div>
      <button
        class="btn btn-primary"
        :disabled="!canCreateProject"
        @click="$router.push('/panel/projects/new')"
      >
        <svg class="w-4 h-4 mr-1.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
        </svg>
        New Project
      </button>
    </div>

    <div v-if="!canCreateProject" class="flex items-start gap-3 rounded-lg border border-amber-200 bg-amber-50 dark:border-amber-900 dark:bg-amber-950/50 p-4">
      <svg class="w-5 h-5 text-amber-600 shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
      </svg>
      <div class="text-sm">
        <p class="font-medium text-amber-800 dark:text-amber-300">Project limit reached</p>
        <p class="text-amber-700 dark:text-amber-400 mt-0.5">
          You've reached the maximum projects for your plan. <NuxtLink to="/panel/subscription" class="underline font-medium">Upgrade</NuxtLink> to create more.
        </p>
      </div>
    </div>

    <div class="flex items-center gap-4">
      <div class="relative flex-1 max-w-sm">
        <svg class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
        </svg>
        <input v-model="searchQuery" type="search" placeholder="Search projects..." class="input pl-9 h-9 text-sm" />
      </div>
      <div class="flex items-center rounded-lg border overflow-hidden">
        <button
          class="h-8 w-8 flex items-center justify-center transition-colors"
          :class="viewMode === 'grid' ? 'bg-accent text-accent-foreground' : 'text-muted-foreground hover:bg-accent hover:text-accent-foreground'"
          @click="viewMode = 'grid'"
        >
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zM14 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zM14 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z" />
          </svg>
        </button>
        <button
          class="h-8 w-8 flex items-center justify-center transition-colors border-l"
          :class="viewMode === 'list' ? 'bg-accent text-accent-foreground' : 'text-muted-foreground hover:bg-accent hover:text-accent-foreground'"
          @click="viewMode = 'list'"
        >
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 10h16M4 14h16M4 18h16" />
          </svg>
        </button>
      </div>
    </div>

    <div v-if="filteredProjects.length === 0" class="card">
      <div class="card-body text-center py-12">
        <div class="w-12 h-12 rounded-full bg-muted flex items-center justify-center mx-auto mb-3">
          <svg class="w-6 h-6 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
          </svg>
        </div>
        <h3 class="text-sm font-medium">{{ searchQuery ? 'No projects found' : 'No projects yet' }}</h3>
        <p class="text-sm text-muted-foreground mt-1">{{ searchQuery ? 'Try a different search' : 'Create your first project to get started.' }}</p>
        <NuxtLink v-if="!searchQuery && canCreateProject" to="/panel/projects/new" class="btn btn-primary btn-sm mt-4">
          <svg class="w-4 h-4 mr-1.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
          </svg>
          New Project
        </NuxtLink>
      </div>
    </div>

    <div v-else :class="viewMode === 'grid' ? 'grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4' : 'space-y-3'">
      <NuxtLink
        v-for="project in filteredProjects"
        :key="project.id"
        :to="`/editor/${project.id}`"
        class="card p-4 group hover:shadow-md transition-all"
        :class="viewMode === 'list' ? 'flex items-center gap-4' : ''"
      >
        <div :class="viewMode === 'list' ? 'flex items-center gap-4 flex-1 min-w-0' : ''">
          <div class="w-10 h-10 rounded-xl bg-gradient-to-br from-primary/10 to-primary/5 flex items-center justify-center shrink-0 group-hover:scale-110 transition-transform" :class="viewMode === 'list' ? '' : 'mb-3'">
            <svg class="w-5 h-5 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
            </svg>
          </div>
          <div class="min-w-0">
            <div class="flex items-center gap-2">
              <h3 class="font-semibold group-hover:text-primary transition-colors truncate">{{ project.name }}</h3>
              <span
                class="badge text-[10px] h-5 shrink-0"
                :class="project.status === 'published' ? 'bg-green-100 text-green-700 dark:bg-green-900/40 dark:text-green-400 border-green-200 dark:border-green-800' : project.status === 'draft' ? 'bg-yellow-100 text-yellow-700 dark:bg-yellow-900/40 dark:text-yellow-400 border-yellow-200 dark:border-yellow-800' : 'bg-muted text-muted-foreground border'"
              >
                {{ project.status }}
              </span>
            </div>
            <p v-if="viewMode === 'grid'" class="text-sm text-muted-foreground mt-1 line-clamp-2">{{ project.description }}</p>
            <div class="flex items-center gap-2 text-xs text-muted-foreground mt-1">
              <span>{{ project.template }}</span>
              <span>&middot;</span>
              <span>{{ project.updatedAt }}</span>
            </div>
          </div>
        </div>
        <div v-if="viewMode === 'list'" class="flex items-center gap-2 shrink-0">
          <span class="btn btn-sm btn-secondary">Edit</span>
        </div>
      </NuxtLink>
    </div>
  </div>
</template>
