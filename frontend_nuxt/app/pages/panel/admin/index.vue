<script setup lang="ts">
definePageMeta({ layout: 'panel', sidebarSection: 'admin-dashboard' })
const userStore = useUserStore()

const stats = [
  { label: 'Total Users', value: '1,234', change: '+12%', icon: 'users' },
  { label: 'Total Projects', value: '892', change: '+8%', icon: 'folder' },
  { label: 'Products', value: '48', change: '+3', icon: 'package' },
  { label: 'Components', value: '24', change: '-', icon: 'grid' },
]

const recentUsers = [
  { name: 'Alice Johnson', email: 'alice@example.com', role: 'user', status: 'active', date: '2026-07-12' },
  { name: 'Bob Smith', email: 'bob@example.com', role: 'user', status: 'active', date: '2026-07-11' },
  { name: 'Charlie Brown', email: 'charlie@example.com', role: 'user', status: 'inactive', date: '2026-07-10' },
  { name: 'Diana Prince', email: 'diana@example.com', role: 'superuser', status: 'active', date: '2026-07-09' },
]
</script>

<template>
  <div class="space-y-6">
    <div>
      <h1 class="text-2xl font-bold tracking-tight">Admin Dashboard</h1>
      <p class="text-muted-foreground">Manage your platform from one place.</p>
    </div>

    <!-- Stats -->
    <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
      <div v-for="s in stats" :key="s.label" class="rounded-xl border bg-card text-card-foreground shadow">
        <div class="p-6 flex flex-col gap-1">
          <div class="flex items-center justify-between">
            <svg v-if="s.icon === 'users'" class="w-4 h-4 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z" />
            </svg>
            <svg v-else-if="s.icon === 'folder'" class="w-4 h-4 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
            </svg>
            <svg v-else-if="s.icon === 'package'" class="w-4 h-4 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4" />
            </svg>
            <span class="text-xs font-medium text-emerald-600 dark:text-emerald-400">{{ s.change }}</span>
          </div>
          <span class="text-2xl font-bold">{{ s.value }}</span>
          <span class="text-xs text-muted-foreground">{{ s.label }}</span>
        </div>
      </div>
    </div>

    <!-- Quick Actions -->
    <div class="rounded-xl border bg-card text-card-foreground shadow">
      <div class="p-6">
        <h2 class="text-lg font-semibold mb-4">Quick Actions</h2>
        <div class="grid gap-3 sm:grid-cols-2 lg:grid-cols-4">
          <NuxtLink to="/panel/admin/users" class="flex items-center gap-3 rounded-lg border p-4 hover:bg-accent transition-colors">
            <svg class="w-5 h-5 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M18 9v3m0 0v3m0-3h3m-3 0h-3m-2-5a4 4 0 11-8 0 4 4 0 018 0zM3 20a6 6 0 0112 0v1H3v-1z" />
            </svg>
            <div>
              <div class="text-sm font-medium">Manage Users</div>
              <div class="text-xs text-muted-foreground">Add or edit user accounts</div>
            </div>
          </NuxtLink>
          <NuxtLink to="/panel/admin/products" class="flex items-center gap-3 rounded-lg border p-4 hover:bg-accent transition-colors">
            <svg class="w-5 h-5 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4" />
            </svg>
            <div>
              <div class="text-sm font-medium">Manage Products</div>
              <div class="text-xs text-muted-foreground">Add or edit products</div>
            </div>
          </NuxtLink>
          <NuxtLink to="/panel/admin/components" class="flex items-center gap-3 rounded-lg border p-4 hover:bg-accent transition-colors">
            <svg class="w-5 h-5 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zm10 0a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zm10 0a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z" />
            </svg>
            <div>
              <div class="text-sm font-medium">Manage Components</div>
              <div class="text-xs text-muted-foreground">Configure builder components</div>
            </div>
          </NuxtLink>
          <NuxtLink to="/panel/settings" class="flex items-center gap-3 rounded-lg border p-4 hover:bg-accent transition-colors">
            <svg class="w-5 h-5 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
            </svg>
            <div>
              <div class="text-sm font-medium">Site Settings</div>
              <div class="text-xs text-muted-foreground">Global configuration</div>
            </div>
          </NuxtLink>
        </div>
      </div>
    </div>

    <!-- Recent Users -->
    <div class="rounded-xl border bg-card text-card-foreground shadow">
      <div class="p-6">
        <div class="flex items-center justify-between mb-4">
          <h2 class="text-lg font-semibold">Recent Users</h2>
          <NuxtLink to="/panel/admin/users" class="text-sm text-primary hover:underline">View all</NuxtLink>
        </div>
        <div class="relative w-full overflow-auto">
          <table class="w-full caption-bottom text-sm">
            <thead class="[&_tr]:border-b">
            <tr class="border-b border-border transition-colors hover:bg-muted/50">
              <th class="h-10 px-2 text-left align-middle font-medium text-muted-foreground">Name</th>
                <th class="h-10 px-2 text-left align-middle font-medium text-muted-foreground">Email</th>
                <th class="h-10 px-2 text-left align-middle font-medium text-muted-foreground">Role</th>
                <th class="h-10 px-2 text-left align-middle font-medium text-muted-foreground">Status</th>
              </tr>
            </thead>
            <tbody class="[&_tr:last-child]:border-0">
              <tr v-for="u in recentUsers" :key="u.email" class="border-b border-border transition-colors hover:bg-muted/50">
                <td class="p-2 align-middle font-medium">{{ u.name }}</td>
                <td class="p-2 align-middle text-muted-foreground">{{ u.email }}</td>
                <td class="p-2 align-middle">
                  <span class="inline-flex items-center rounded-md border px-2.5 py-0.5 text-xs font-semibold"
                    :class="u.role === 'superuser' ? 'border-transparent bg-purple-100 text-purple-700 dark:bg-purple-900/40 dark:text-purple-400' : 'border-transparent bg-secondary text-secondary-foreground'"
                  >{{ u.role }}</span>
                </td>
                <td class="p-2 align-middle">
                  <span class="inline-flex items-center rounded-md border px-2.5 py-0.5 text-xs font-semibold"
                    :class="u.status === 'active' ? 'border-transparent bg-emerald-100 text-emerald-700 dark:bg-emerald-900/40 dark:text-emerald-400' : 'border-transparent bg-destructive/10 text-destructive'"
                  >{{ u.status }}</span>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </div>
  </div>
</template>
