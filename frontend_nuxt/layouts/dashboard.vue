<script setup lang="ts">
const userStore = useUserStore()
const workspaceStore = useWorkspaceStore()
const router = useRouter()

// Redirect to login if not authenticated
if (!userStore.isAuthenticated) {
  router.push('/login')
}

// Navigation items
const navigation = computed(() => {
  const subscription = useSubscriptionStore()
  
  return [
    { name: 'Dashboard', href: '/dashboard', icon: 'home' },
    { name: 'Projects', href: '/dashboard/projects', icon: 'folder' },
    { 
      name: 'Analytics', 
      href: '/dashboard/analytics', 
      icon: 'chart',
      locked: !subscription.isFeatureEnabled('analyticsDashboard')
    },
    { 
      name: 'Asset Marketplace', 
      href: '/dashboard/marketplace', 
      icon: 'shopping-bag',
      locked: !subscription.isFeatureEnabled('assetMarketplace')
    },
    { name: 'Settings', href: '/dashboard/settings', icon: 'cog' }
  ]
})

const showWorkspaceDropdown = ref(false)
const showUserMenu = ref(false)

// Close dropdowns on outside click
function closeDropdowns() {
  showWorkspaceDropdown.value = false
  showUserMenu.value = false
}

async function handleLogout() {
  const { logout } = useAuth()
  await logout()
  router.push('/login')
}
</script>

<template>
  <div class="min-h-screen bg-gray-50" @click="closeDropdowns">
    <!-- Top Navigation -->
    <header class="fixed top-0 left-0 right-0 z-40 bg-white border-b border-gray-200">
      <div class="flex items-center justify-between h-16 px-4 lg:px-6">
        <!-- Logo & Workspace Switcher -->
        <div class="flex items-center gap-4">
          <NuxtLink to="/dashboard" class="text-xl font-bold text-primary-600">
            Rinova
          </NuxtLink>
          
          <!-- Workspace Switcher -->
          <div v-if="workspaceStore.currentWorkspace" class="relative">
            <button
              class="flex items-center gap-2 px-3 py-2 rounded-lg hover:bg-gray-100 transition-colors"
              @click.stop="showWorkspaceDropdown = !showWorkspaceDropdown"
            >
              <div
                v-if="workspaceStore.currentWorkspace.logoUrl"
                class="w-6 h-6 rounded bg-gray-200 bg-cover"
                :style="{ backgroundImage: `url(${workspaceStore.currentWorkspace.logoUrl})` }"
              />
              <div
                v-else
                class="w-6 h-6 rounded bg-primary-100 flex items-center justify-center text-xs font-medium text-primary-600"
              >
                {{ workspaceStore.currentWorkspace.name.charAt(0).toUpperCase() }}
              </div>
              <span class="text-sm font-medium text-gray-700">
                {{ workspaceStore.currentWorkspace.name }}
              </span>
              <svg class="w-4 h-4 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
              </svg>
            </button>
            
            <!-- Workspace Dropdown -->
            <div
              v-if="showWorkspaceDropdown"
              class="dropdown left-0"
              @click.stop
            >
              <div class="px-4 py-2 text-xs font-semibold text-gray-400 uppercase">
                Your Workspaces
              </div>
              
              <button
                v-for="workspace in workspaceStore.workspaces"
                :key="workspace.id"
                class="dropdown-item w-full text-left"
                :class="{ 'bg-primary-50': workspace.id === workspaceStore.currentWorkspaceId }"
                @click="workspaceStore.switchWorkspace(workspace.id)"
              >
                <div class="flex items-center gap-3">
                  <div
                    class="w-8 h-8 rounded bg-gray-100 flex items-center justify-center text-sm font-medium"
                    :class="workspace.type === 'company' ? 'bg-purple-100 text-purple-600' : 'bg-primary-100 text-primary-600'"
                  >
                    {{ workspace.name.charAt(0).toUpperCase() }}
                  </div>
                  <div>
                    <div class="font-medium text-gray-900">{{ workspace.name }}</div>
                    <div class="text-xs text-gray-500 capitalize">{{ workspace.type }}</div>
                  </div>
                </div>
              </button>
              
              <div class="border-t border-gray-100 my-1"></div>
              
              <NuxtLink
                to="/dashboard/workspaces/new"
                class="dropdown-item text-primary-600"
              >
                <svg class="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
                </svg>
                Create Workspace
              </NuxtLink>
            </div>
          </div>
        </div>
        
        <!-- Right Side -->
        <div class="flex items-center gap-4">
          <!-- User Menu -->
          <div class="relative">
            <button
              class="flex items-center gap-2 px-3 py-2 rounded-lg hover:bg-gray-100 transition-colors"
              @click.stop="showUserMenu = !showUserMenu"
            >
              <div class="w-8 h-8 rounded-full bg-primary-100 flex items-center justify-center">
                <span class="text-sm font-medium text-primary-600">
                  {{ userStore.userName.charAt(0).toUpperCase() }}
                </span>
              </div>
              <svg class="w-4 h-4 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
              </svg>
            </button>
            
            <!-- User Dropdown -->
            <div
              v-if="showUserMenu"
              class="dropdown right-0"
              @click.stop
            >
              <div class="px-4 py-3 border-b border-gray-100">
                <div class="font-medium text-gray-900">{{ userStore.userName }}</div>
                <div class="text-sm text-gray-500">{{ userStore.userEmail }}</div>
              </div>
              
              <NuxtLink to="/dashboard/settings" class="dropdown-item">
                <svg class="w-5 h-5 mr-2 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                </svg>
                Settings
              </NuxtLink>
              
              <NuxtLink to="/dashboard/subscription" class="dropdown-item">
                <svg class="w-5 h-5 mr-2 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 10h18M7 15h1m4 0h1m-7 4h12a3 3 0 003-3V8a3 3 0 00-3-3H6a3 3 0 00-3 3v8a3 3 0 003 3z" />
                </svg>
                Subscription
              </NuxtLink>
              
              <div class="border-t border-gray-100 my-1"></div>
              
              <button
                class="dropdown-item w-full text-left text-red-600"
                @click="handleLogout"
              >
                <svg class="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1" />
                </svg>
                Sign out
              </button>
            </div>
          </div>
        </div>
      </div>
    </header>

    <!-- Main Content Area -->
    <div class="flex pt-16">
      <!-- Sidebar -->
      <aside class="fixed left-0 top-16 bottom-0 w-64 bg-white border-r border-gray-200 overflow-y-auto">
        <nav class="p-4 space-y-1">
          <NuxtLink
            v-for="item in navigation"
            :key="item.name"
            :to="item.href"
            class="sidebar-item"
            :class="[
              $route.path === item.href ? 'sidebar-item-active' : '',
              item.locked ? 'opacity-60' : ''
            ]"
          >
            <!-- Home Icon -->
            <svg v-if="item.icon === 'home'" class="w-5 h-5 mr-3 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" />
            </svg>
            
            <!-- Folder Icon -->
            <svg v-else-if="item.icon === 'folder'" class="w-5 h-5 mr-3 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
            </svg>
            
            <!-- Chart Icon -->
            <svg v-else-if="item.icon === 'chart'" class="w-5 h-5 mr-3 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
            </svg>
            
            <!-- Shopping Bag Icon -->
            <svg v-else-if="item.icon === 'shopping-bag'" class="w-5 h-5 mr-3 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 11V7a4 4 0 00-8 0v4M5 9h14l1 12H4L5 9z" />
            </svg>
            
            <!-- Cog Icon -->
            <svg v-else-if="item.icon === 'cog'" class="w-5 h-5 mr-3 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
            </svg>
            
            <span>{{ item.name }}</span>
            
            <!-- Locked Badge -->
            <span v-if="item.locked" class="ml-auto badge badge-locked">Locked</span>
          </NuxtLink>
        </nav>
        
        <!-- Subscription Status -->
        <div class="p-4 mt-4 border-t border-gray-200">
          <SubscriptionStatus />
        </div>
      </aside>

      <!-- Main Content -->
      <main class="flex-1 ml-64 p-6">
        <slot />
      </main>
    </div>
  </div>
</template>
