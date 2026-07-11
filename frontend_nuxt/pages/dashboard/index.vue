<script setup lang="ts">
definePageMeta({
  layout: 'dashboard'
})

const userStore = useUserStore()
const workspaceStore = useWorkspaceStore()
const subscriptionStore = useSubscriptionStore()

// Recent projects placeholder
const recentProjects = ref([
  // Will be populated from API
])

// Quick actions
const quickActions = computed(() => {
  const actions = [
    { name: 'New Project', icon: 'plus', href: '/dashboard/projects/new', primary: true }
  ]
  
  if (subscriptionStore.isFeatureEnabled('oneClickPublish')) {
    actions.push({ name: 'Publish Site', icon: 'upload', href: '/dashboard/projects' })
  }
  
  if (subscriptionStore.isFeatureEnabled('assetMarketplace')) {
    actions.push({ name: 'Browse Marketplace', icon: 'shopping-bag', href: '/dashboard/marketplace' })
  }
  
  return actions
})

// Stats
const stats = computed(() => [
  { 
    label: 'Projects', 
    value: 0,
    max: subscriptionStore.currentPlan === 'exclusive' ? null : subscriptionStore.limits.maxProjects
  },
  { label: 'Team Members', value: 1, max: subscriptionStore.limits.maxTeamMembers },
  { label: 'Storage Used', value: '0 MB', max: `${subscriptionStore.limits.maxStorageMB} MB` }
])
</script>

<template>
  <div>
    <!-- Header -->
    <div class="mb-8">
      <h1 class="text-2xl font-bold text-gray-900">
        Welcome back, {{ userStore.userName }}!
      </h1>
      <p class="mt-1 text-gray-600">
        Here's what's happening in your workspace
      </p>
    </div>

    <!-- Quick Actions -->
    <div class="grid grid-cols-1 md:grid-cols-3 gap-4 mb-8">
      <NuxtLink
        v-for="action in quickActions"
        :key="action.name"
        :to="action.href"
        class="card p-4 flex items-center gap-3 hover:shadow-md transition-shadow"
        :class="action.primary ? 'bg-primary-50 border-primary-200' : ''"
      >
        <div
          class="w-10 h-10 rounded-lg flex items-center justify-center"
          :class="action.primary ? 'bg-primary-600 text-white' : 'bg-gray-100 text-gray-600'"
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
        <span class="font-medium text-gray-900">{{ action.name }}</span>
      </NuxtLink>
    </div>

    <!-- Stats Grid -->
    <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
      <div v-for="stat in stats" :key="stat.label" class="card p-6">
        <div class="text-sm font-medium text-gray-500 mb-1">{{ stat.label }}</div>
        <div class="text-2xl font-bold text-gray-900">
          {{ stat.value }}
          <span v-if="stat.max" class="text-sm font-normal text-gray-500">
            / {{ stat.max }}
          </span>
        </div>
      </div>
    </div>

    <!-- Recent Projects -->
    <div class="card">
      <div class="card-header flex items-center justify-between">
        <h2 class="text-lg font-semibold text-gray-900">Recent Projects</h2>
        <NuxtLink to="/dashboard/projects" class="text-sm text-primary-600 hover:text-primary-700">
          View all
        </NuxtLink>
      </div>
      
      <div class="card-body">
        <div v-if="recentProjects.length === 0" class="text-center py-12">
          <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
          </svg>
          <h3 class="mt-2 text-sm font-medium text-gray-900">No projects</h3>
          <p class="mt-1 text-sm text-gray-500">Get started by creating a new project.</p>
          <div class="mt-6">
            <NuxtLink to="/dashboard/projects/new" class="btn btn-primary">
              <svg class="-ml-1 mr-2 h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
              </svg>
              New Project
            </NuxtLink>
          </div>
        </div>
        
        <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          <!-- Project cards will be rendered here -->
        </div>
      </div>
    </div>

    <!-- Upgrade Banner (for freemium users) -->
    <div
      v-if="subscriptionStore.currentPlan === 'freemium'"
      class="mt-8 card bg-gradient-to-r from-primary-600 to-primary-700 border-0"
    >
      <div class="card-body flex items-center justify-between">
        <div class="text-white">
          <h3 class="text-lg font-semibold">Unlock all features</h3>
          <p class="text-primary-100 mt-1">
            Upgrade to Enterprise or Exclusive to access premium features like Animation Editor, Custom CSS, and more.
          </p>
        </div>
        <NuxtLink to="/dashboard/subscription" class="btn bg-white text-primary-600 hover:bg-primary-50">
          View Plans
        </NuxtLink>
      </div>
    </div>
  </div>
</template>
