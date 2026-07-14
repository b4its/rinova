<script setup lang="ts">
const userStore = useUserStore()
const workspaceStore = useWorkspaceStore()
const themeStore = useThemeStore()
const route = useRoute()
const router = useRouter()

const sidebarOpen = ref(false)
const isDesktopSidebar = ref(false)

onMounted(() => {
  const mq = window.matchMedia('(min-width: 1024px)')
  isDesktopSidebar.value = mq.matches
  sidebarOpen.value = mq.matches
  mq.addEventListener('change', (e) => {
    isDesktopSidebar.value = e.matches
    if (e.matches) sidebarOpen.value = true
    else sidebarOpen.value = false
  })
})

const navigation = computed(() => {
  const sub = useSubscriptionStore()
  return [
    { sectionId: 'dashboard', name: 'Dashboard', href: '/panel', icon: 'home', locked: false },
    { sectionId: 'projects', name: 'Projects', href: '/panel/projects', icon: 'folder', locked: false },
    {
      sectionId: 'analytics',
      name: 'Analytics',
      href: '/panel/analytics',
      icon: 'chart',
      locked: !sub.isFeatureEnabled('analyticsDashboard')
    },
    {
      sectionId: 'marketplace',
      name: 'Marketplace',
      href: '/panel/marketplace',
      icon: 'shopping-bag',
      locked: !sub.isFeatureEnabled('assetMarketplace')
    },
    { sectionId: 'blockchain', name: 'Blockchain', href: '/panel/blockchain', icon: 'shield', locked: false },
    { sectionId: 'settings', name: 'Settings', href: '/panel/settings', icon: 'cog', locked: false }
  ]
})

const adminNavigation = computed(() => [
  { sectionId: 'admin-dashboard', name: 'Admin', href: '/panel/admin', icon: 'shield' },
  { sectionId: 'admin-users', name: 'Users', href: '/panel/admin/users', icon: 'users' },
  { sectionId: 'admin-products', name: 'Products', href: '/panel/admin/products', icon: 'package' },
  { sectionId: 'admin-components', name: 'Components', href: '/panel/admin/components', icon: 'grid' },
])

const projectsExpanded = ref(false)

const activeSectionId = computed(() => route.meta?.sidebarSection as string | undefined)

function isActive(item: { sectionId: string }) {
  return activeSectionId.value === item.sectionId
}

const showWorkspaceDropdown = ref(false)
const showUserMenu = ref(false)

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
  <div class="min-h-screen bg-background" @click="closeDropdowns">
    <!-- Sidebar -->
    <aside
      class="fixed inset-y-0 left-0 z-40 flex flex-col border-r bg-sidebar text-sidebar-foreground transition-all duration-300 ease-in-out"
      :class="sidebarOpen ? 'translate-x-0 w-60' : '-translate-x-full w-60 lg:translate-x-0 lg:w-16'"
    >
      <!-- Logo -->
      <div class="flex h-14 items-center border-b border-sidebar-border px-4 shrink-0" :class="!sidebarOpen && 'lg:justify-center'">
        <NuxtLink to="/panel" class="flex items-center gap-2 overflow-hidden">
          <div class="w-8 h-8 shrink-0 rounded-lg bg-primary flex items-center justify-center">
            <svg class="w-5 h-5 text-primary-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
            </svg>
          </div>
          <span class="text-base font-bold truncate transition-opacity duration-200" :class="sidebarOpen ? 'opacity-100' : 'lg:opacity-0 hidden lg:block'">
            Rinova
          </span>
        </NuxtLink>
      </div>

      <!-- Sidebar Toggle -->
      <button
        class="hidden lg:flex absolute -right-3 top-14 z-50 h-6 w-6 items-center justify-center rounded-full border bg-background shadow-sm text-muted-foreground hover:text-foreground transition-colors"
        @click.stop="sidebarOpen = !sidebarOpen"
        :title="sidebarOpen ? 'Collapse sidebar' : 'Expand sidebar'"
      >
        <svg
          class="w-3 h-3 transition-transform duration-200"
          :class="sidebarOpen ? '' : 'rotate-180'"
          fill="none" stroke="currentColor" viewBox="0 0 24 24"
        >
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
        </svg>
      </button>

      <!-- Navigation -->
      <nav class="flex-1 overflow-y-auto p-3 space-y-1">
        <NuxtLink
          v-for="item in navigation"
          :key="item.name"
          :to="item.href"
          class="flex items-center gap-3 rounded-lg px-3 py-2 text-sm font-medium transition-colors hover:bg-accent hover:text-accent-foreground"
          :class="[isActive(item) ? 'bg-accent text-accent-foreground' : '', item.locked ? 'opacity-50 pointer-events-none' : '']"
        >
          <svg v-if="item.icon === 'home'" class="w-5 h-5 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" />
          </svg>
          <svg v-else-if="item.icon === 'folder'" class="w-5 h-5 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
          </svg>
          <svg v-else-if="item.icon === 'chart'" class="w-5 h-5 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
          </svg>
          <svg v-else-if="item.icon === 'shopping-bag'" class="w-5 h-5 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 11V7a4 4 0 00-8 0v4M5 9h14l1 12H4L5 9z" />
          </svg>
          <svg v-else-if="item.icon === 'cog'" class="w-5 h-5 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
          </svg>
          <span class="truncate transition-opacity duration-200" :class="sidebarOpen ? 'opacity-100' : 'lg:opacity-0 opacity-0'">
            {{ item.name }}
          </span>
          <span
            v-if="item.locked"
            class="ml-auto badge text-[10px] px-1.5 py-0 bg-muted text-muted-foreground border-0 shrink-0"
            :class="!sidebarOpen && 'hidden'"
          >
            Locked
          </span>
        </NuxtLink>
      </nav>

      <!-- Admin Navigation (superuser only) -->
      <template v-if="userStore.isSuperuser">
        <div class="px-3 py-1">
          <div class="flex items-center gap-2 px-3">
            <svg class="w-3.5 h-3.5 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" />
            </svg>
            <span class="text-[10px] font-semibold uppercase tracking-wider text-muted-foreground/60" :class="!sidebarOpen && 'hidden'">Admin</span>
          </div>
        </div>
        <nav class="flex-0 px-3 pb-2 space-y-1">
          <NuxtLink
            v-for="item in adminNavigation"
            :key="item.name"
            :to="item.href"
            class="flex items-center gap-3 rounded-lg px-3 py-2 text-sm font-medium transition-colors hover:bg-accent hover:text-accent-foreground"
            :class="isActive(item) ? 'bg-accent text-accent-foreground' : ''"
          >
            <svg v-if="item.icon === 'shield'" class="w-5 h-5 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z" />
            </svg>
            <svg v-else-if="item.icon === 'users'" class="w-5 h-5 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z" />
            </svg>
            <svg v-else-if="item.icon === 'package'" class="w-5 h-5 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4" />
            </svg>
            <svg v-else-if="item.icon === 'grid'" class="w-5 h-5 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zm10 0a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zm10 0a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z" />
            </svg>
            <span class="truncate transition-opacity duration-200" :class="sidebarOpen ? 'opacity-100' : 'lg:opacity-0 opacity-0'">
              {{ item.name }}
            </span>
          </NuxtLink>

          <!-- Projects dropdown -->
          <div>
            <button class="flex items-center gap-3 rounded-lg px-3 py-2 text-sm font-medium transition-colors hover:bg-accent hover:text-accent-foreground w-full text-left" :class="projectsExpanded || activeSectionId === 'admin-kategori' || activeSectionId === 'admin-templates' ? 'bg-accent text-accent-foreground' : ''" @click="projectsExpanded = !projectsExpanded">
              <svg class="w-5 h-5 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" /></svg>
              <span class="truncate flex-1 transition-opacity duration-200" :class="sidebarOpen ? 'opacity-100' : 'lg:opacity-0 opacity-0'">Projects</span>
              <svg class="w-3.5 h-3.5 transition-transform duration-200" :class="projectsExpanded ? 'rotate-90' : ''" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" /></svg>
            </button>
            <div v-if="projectsExpanded" class="ml-2 mt-0.5 space-y-0.5 border-l-2 border-muted pl-2">
              <NuxtLink to="/panel/admin/kategori" class="flex items-center gap-3 rounded-lg px-3 py-1.5 text-sm font-medium transition-colors hover:bg-accent hover:text-accent-foreground" :class="activeSectionId === 'admin-kategori' ? 'bg-accent text-accent-foreground' : 'text-muted-foreground'">
                <svg class="w-4 h-4 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 7h.01M7 3h5c.512 0 1.024.195 1.414.586l7 7a2 2 0 010 2.828l-7 7a2 2 0 01-2.828 0l-7-7A1.994 1.994 0 013 12V7a4 4 0 014-4z" /></svg>
                <span class="truncate transition-opacity duration-200" :class="sidebarOpen ? 'opacity-100' : 'lg:opacity-0 opacity-0'">Kategori</span>
              </NuxtLink>
              <NuxtLink to="/panel/admin/templates" class="flex items-center gap-3 rounded-lg px-3 py-1.5 text-sm font-medium transition-colors hover:bg-accent hover:text-accent-foreground" :class="activeSectionId === 'admin-templates' ? 'bg-accent text-accent-foreground' : 'text-muted-foreground'">
                <svg class="w-4 h-4 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 5a1 1 0 011-1h14a1 1 0 011 1v2a1 1 0 01-1 1H5a1 1 0 01-1-1V5zM4 13a1 1 0 011-1h6a1 1 0 011 1v6a1 1 0 01-1 1H5a1 1 0 01-1-1v-6zM16 13a1 1 0 011-1h2a1 1 0 011 1v6a1 1 0 01-1 1h-2a1 1 0 01-1-1v-6z" /></svg>
                <span class="truncate transition-opacity duration-200" :class="sidebarOpen ? 'opacity-100' : 'lg:opacity-0 opacity-0'">Templates</span>
              </NuxtLink>
            </div>
          </div>
        </nav>
      </template>

      <!-- Bottom section -->
      <div class="border-t border-sidebar-border p-3 space-y-2">
        <NuxtLink
          to="/"
          class="flex items-center gap-3 rounded-lg px-3 py-2 text-sm font-medium text-muted-foreground hover:text-foreground hover:bg-accent transition-colors"
        >
          <svg class="w-5 h-5 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
          </svg>
          <span class="truncate transition-opacity duration-200" :class="sidebarOpen ? 'opacity-100' : 'lg:opacity-0 opacity-0'">
            Back to Website
          </span>
        </NuxtLink>
        <div class="transition-opacity duration-200" :class="sidebarOpen ? 'opacity-100' : 'lg:opacity-0 opacity-0 overflow-hidden'">
          <div v-if="sidebarOpen">
            <SubscriptionStatus />
          </div>
        </div>
      </div>
    </aside>

    <!-- Mobile backdrop -->
    <div
      v-if="sidebarOpen"
      class="fixed inset-0 z-30 bg-black/50 transition-opacity duration-300 lg:hidden"
      @click="sidebarOpen = false"
    />

    <!-- Main area -->
    <div class="transition-all duration-300 ease-in-out" :class="sidebarOpen ? 'lg:ml-60' : 'lg:ml-16'">
      <!-- Top Header -->
      <header class="sticky top-0 z-30 flex h-14 items-center gap-4 border-b bg-background px-6">
        <!-- Mobile menu toggle -->
        <button
          class="lg:hidden h-9 w-9 rounded-lg flex items-center justify-center text-muted-foreground hover:bg-accent hover:text-accent-foreground transition-colors"
          @click.stop="sidebarOpen = !sidebarOpen"
        >
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
          </svg>
        </button>

        <!-- Workspace switcher -->
        <div v-if="workspaceStore.currentWorkspace" class="relative">
          <button
            class="flex items-center gap-2 h-9 rounded-lg px-3 text-sm font-medium text-muted-foreground hover:text-foreground hover:bg-accent transition-colors"
            @click.stop="showWorkspaceDropdown = !showWorkspaceDropdown"
          >
            <div class="w-6 h-6 rounded bg-primary/10 flex items-center justify-center text-xs font-semibold text-primary">
              {{ workspaceStore.currentWorkspace.name.charAt(0).toUpperCase() }}
            </div>
            <span class="hidden sm:inline">{{ workspaceStore.currentWorkspace.name }}</span>
            <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
            </svg>
          </button>

          <div v-if="showWorkspaceDropdown" class="dropdown left-0 mt-1 min-w-[14rem]" @click.stop>
            <div class="px-3 py-1.5 text-xs font-semibold text-muted-foreground">Workspaces</div>
            <button
              v-for="w in workspaceStore.workspaces"
              :key="w.id"
              class="dropdown-item w-full gap-3"
              :class="{ 'bg-accent': w.id === workspaceStore.currentWorkspaceId }"
              @click="workspaceStore.switchWorkspace(w.id)"
            >
              <div
                class="w-7 h-7 rounded-lg flex items-center justify-center text-xs font-semibold shrink-0"
                :class="w.type === 'company' ? 'bg-purple-100 text-purple-700 dark:bg-purple-900/40 dark:text-purple-400' : 'bg-primary/10 text-primary'"
              >
                {{ w.name.charAt(0).toUpperCase() }}
              </div>
              <div class="text-left min-w-0">
                <div class="text-sm font-medium truncate">{{ w.name }}</div>
                <div class="text-xs text-muted-foreground capitalize">{{ w.type }}</div>
              </div>
            </button>
            <div class="separator my-1" />
            <NuxtLink to="/panel/workspaces/new" class="dropdown-item text-primary">
              <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
              </svg>
              Create Workspace
            </NuxtLink>
          </div>
        </div>

        <div class="flex-1" />

        <div class="flex items-center gap-2">
          <!-- Theme Toggle (shadcn switch) -->
          <button
            class="relative inline-flex h-5 w-9 shrink-0 cursor-pointer items-center rounded-full border-2 border-transparent transition-colors duration-200 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:ring-offset-background"
            :class="themeStore.mode === 'dark' ? 'bg-primary' : 'bg-input'"
            @click="themeStore.toggle()"
            role="switch"
            :aria-checked="themeStore.mode === 'dark'"
          >
            <span
              class="pointer-events-none block h-4 w-4 rounded-full bg-background shadow-sm ring-0 transition-transform duration-200 flex items-center justify-center"
              :class="themeStore.mode === 'dark' ? 'translate-x-4' : 'translate-x-0'"
            >
              <svg v-if="themeStore.mode === 'dark'" class="w-2.5 h-2.5 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z" />
              </svg>
              <svg v-else class="w-2.5 h-2.5 text-amber-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z" />
              </svg>
            </span>
          </button>

          <!-- User -->
          <div class="relative">
            <button
              class="flex items-center gap-2 h-9 rounded-lg px-3 text-sm font-medium text-muted-foreground hover:text-foreground hover:bg-accent transition-colors"
              @click.stop="showUserMenu = !showUserMenu"
            >
              <div class="w-7 h-7 rounded-full bg-primary flex items-center justify-center text-xs font-semibold text-primary-foreground">
                {{ userStore.userName?.charAt(0)?.toUpperCase() || 'U' }}
              </div>
              <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
              </svg>
            </button>

            <div v-if="showUserMenu" class="dropdown right-0 mt-1 min-w-[13rem]" @click.stop>
              <div class="px-3 py-2 border-b">
                <div class="text-sm font-medium truncate">{{ userStore.userName }}</div>
                <div class="text-xs text-muted-foreground truncate">{{ userStore.userEmail }}</div>
              </div>
              <NuxtLink to="/panel/settings" class="dropdown-item">
                <svg class="w-4 h-4 mr-2 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                </svg>
                Settings
              </NuxtLink>
              <NuxtLink to="/panel/subscription" class="dropdown-item">
                <svg class="w-4 h-4 mr-2 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 10h18M7 15h1m4 0h1m-7 4h12a3 3 0 003-3V8a3 3 0 00-3-3H6a3 3 0 00-3 3v8a3 3 0 003 3z" />
                </svg>
                Subscription
              </NuxtLink>
              <div class="separator my-1" />
              <button class="dropdown-item w-full text-destructive" @click="handleLogout">
                <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1" />
                </svg>
                Sign out
              </button>
            </div>
          </div>
        </div>
      </header>

      <!-- Page Content -->
      <main class="p-6">
        <slot />
      </main>
    </div>
  </div>
</template>
