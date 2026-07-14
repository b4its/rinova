<script setup lang="ts">
definePageMeta({ layout: 'panel', sidebarSection: 'dashboard' })

import { relativeTime, type Project } from '~/composables/useProjects'

const userStore = useUserStore()
const subscriptionStore = useSubscriptionStore()
const workspaceStore = useWorkspaceStore()
const { listProjects } = useProjects()

const allProjects = ref<Project[]>([])

const recentProjects = computed(() =>
  [...allProjects.value]
    .sort((a, b) => new Date(b.updatedAt).getTime() - new Date(a.updatedAt).getTime())
    .slice(0, 3)
    .map(p => ({ id: p.id, name: p.name, updatedAt: relativeTime(p.updatedAt), status: p.status }))
)

onMounted(async () => {
  try {
    allProjects.value = await listProjects()
  } catch { /* handled by global 401 redirect; leave list empty otherwise */ }
})

const quickActions = computed(() => {
  const actions = [
    { name: 'New Project', icon: 'plus', href: '/panel/projects/new', primary: true, desc: 'Create a new website' }
  ]
  if (subscriptionStore.isFeatureEnabled('oneClickPublish')) {
    actions.push({ name: 'Publish Site', icon: 'upload', href: '/panel/projects', primary: false, desc: 'Deploy your site' })
  }
  if (subscriptionStore.isFeatureEnabled('assetMarketplace')) {
    actions.push({ name: 'Browse Marketplace', icon: 'shopping-bag', href: '/panel/marketplace', primary: false, desc: 'Discover assets' })
  }
  return actions
})

const stats = computed(() => [
  { label: 'Projects', value: allProjects.value.length, desc: 'Total projects', icon: 'folder' },
  { label: 'Team Members', value: workspaceStore.currentWorkspace?.type === 'company' ? '—' : 1, desc: 'In your workspace', icon: 'users' },
  { label: 'Plan', value: subscriptionStore.currentPlan, desc: 'Current subscription', icon: 'hard-drive' }
])
</script>

<template>
  <div class="space-y-8">
    <div>
      <h1 class="text-2xl font-bold tracking-tight">Welcome back, {{ userStore.userName }}!</h1>
      <p class="text-muted-foreground mt-1">Here's what's happening in your workspace</p>
    </div>

    <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
      <NuxtLink
        v-for="action in quickActions"
        :key="action.name"
        :to="action.href"
        class="card p-5 flex items-center gap-4 hover:shadow-md transition-all group"
        :class="action.primary ? 'border-primary/50 bg-primary/5 ring-1 ring-primary/10' : ''"
      >
        <div
          class="w-10 h-10 rounded-xl flex items-center justify-center shrink-0 transition-transform group-hover:scale-110"
          :class="action.primary ? 'bg-primary text-primary-foreground' : 'bg-secondary text-muted-foreground'"
        >
          <svg v-if="action.icon === 'plus'" class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
          </svg>
          <svg v-else-if="action.icon === 'upload'" class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12" />
          </svg>
          <svg v-else-if="action.icon === 'shopping-bag'" class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 11V7a4 4 0 00-8 0v4M5 9h14l1 12H4L5 9z" />
          </svg>
        </div>
        <div>
          <div class="font-semibold text-sm group-hover:text-primary transition-colors">{{ action.name }}</div>
          <div class="text-xs text-muted-foreground mt-0.5">{{ action.desc }}</div>
        </div>
      </NuxtLink>
    </div>

    <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
      <div v-for="stat in stats" :key="stat.label" class="card p-5 hover:shadow-md transition-all">
        <div class="flex items-start justify-between">
          <div>
            <div class="text-sm text-muted-foreground">{{ stat.label }}</div>
            <div class="text-2xl font-bold mt-1">{{ stat.value }}</div>
            <div class="text-xs text-muted-foreground mt-1">{{ stat.desc }}</div>
          </div>
          <div class="w-9 h-9 rounded-lg bg-primary/10 flex items-center justify-center shrink-0">
            <svg v-if="stat.icon === 'folder'" class="w-5 h-5 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
            </svg>
            <svg v-else-if="stat.icon === 'users'" class="w-5 h-5 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197m13.5-9a2.5 2.5 0 11-5 0 2.5 2.5 0 015 0z" />
            </svg>
            <svg v-else-if="stat.icon === 'hard-drive'" class="w-5 h-5 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4" />
            </svg>
          </div>
        </div>
      </div>
    </div>

    <div class="card p-5">
      <div class="card-header">
        <h2 class="card-title">Recent Projects</h2>
        <p class="card-description">Your most recently updated projects</p>
      </div>
      <div class="card-body">
        <div v-if="recentProjects.length === 0" class="text-center py-8">
          <div class="w-12 h-12 rounded-full bg-muted flex items-center justify-center mx-auto mb-3">
            <svg class="w-6 h-6 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
            </svg>
          </div>
          <h3 class="text-sm font-medium">No projects yet</h3>
          <NuxtLink to="/panel/projects/new" class="btn btn-primary btn-sm mt-4">
            <svg class="w-4 h-4 mr-1.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
            </svg>
            New Project
          </NuxtLink>
        </div>
        <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          <NuxtLink
            v-for="project in recentProjects"
            :key="project.id"
            :to="`/editor/${project.id}`"
            class="card p-4 hover:shadow-md transition-all group"
          >
            <div class="flex items-start justify-between mb-3">
              <div class="w-10 h-10 rounded-xl bg-gradient-to-br from-primary/10 to-primary/5 flex items-center justify-center group-hover:scale-110 transition-transform">
                <svg class="w-5 h-5 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
                </svg>
              </div>
              <span
                class="badge text-[10px] h-5"
                :class="project.status === 'published' ? 'bg-green-100 text-green-700 dark:bg-green-900/40 dark:text-green-400 border-green-200 dark:border-green-800' : 'bg-yellow-100 text-yellow-700 dark:bg-yellow-900/40 dark:text-yellow-400 border-yellow-200 dark:border-yellow-800'"
              >
                {{ project.status }}
              </span>
            </div>
            <h3 class="font-semibold group-hover:text-primary transition-colors">{{ project.name }}</h3>
            <p class="text-xs text-muted-foreground mt-1">Updated {{ project.updatedAt }}</p>
          </NuxtLink>
        </div>
      </div>
    </div>

    <div
      v-if="subscriptionStore.currentPlan === 'freemium'"
      class="card bg-gradient-to-r from-primary to-primary/80 border-primary/20"
    >
      <div class="card-body flex items-center justify-between gap-4">
        <div>
          <h3 class="text-lg font-semibold text-primary-foreground">Unlock all features</h3>
          <p class="text-primary-foreground/80 text-sm mt-1">
            Upgrade to access Animation Editor, Custom CSS, and more premium features.
          </p>
        </div>
        <NuxtLink to="/panel/subscription" class="btn bg-card text-card-foreground hover:bg-card/90 shrink-0">
          View Plans
        </NuxtLink>
      </div>
    </div>
  </div>
</template>
