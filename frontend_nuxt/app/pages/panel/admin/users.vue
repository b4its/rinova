<script setup lang="ts">
definePageMeta({ layout: 'panel', sidebarSection: 'admin-users' })

interface ApiUser {
  id: string
  email: string
  full_name: string | null
  account_type: string
  role: 'user' | 'superuser'
  email_verified: boolean
  created_at: string
  updated_at: string
}

const api = useApi()
const items = ref<ApiUser[]>([])
const loading = ref(true)
const search = ref('')
const dialogOpen = ref(false)
const editingUser = ref<ApiUser | null>(null)
const formRole = ref<'user' | 'superuser'>('user')
const saving = ref(false)
const error = ref('')

// Pagination
const currentPage = ref(1)
const perPage = ref(10)

async function fetchUsers() {
  loading.value = true
  try {
    items.value = await api.get<ApiUser[]>('/admin/users')
    error.value = ''
  } catch {
    // API unavailable (Docker not rebuilt) → load from localStorage
    const raw = localStorage.getItem('admin_users')
    if (raw) {
      try {
        items.value = JSON.parse(raw).map((u: any) => ({
          id: u.id,
          email: u.email,
          full_name: u.name,
          account_type: 'personal',
          role: u.role,
          email_verified: u.status === 'active',
          created_at: u.created + 'T00:00:00Z',
          updated_at: u.created + 'T00:00:00Z',
        }))
      } catch { items.value = [] }
    }
  } finally {
    loading.value = false
  }
}
fetchUsers()

const filtered = computed(() => {
  if (!search.value) return items.value
  const q = search.value.toLowerCase()
  return items.value.filter(u =>
    (u.full_name ?? '').toLowerCase().includes(q) ||
    u.email.toLowerCase().includes(q)
  )
})

const totalPages = computed(() => Math.max(1, Math.ceil(filtered.value.length / perPage.value)))

const paginated = computed(() => {
  const start = (currentPage.value - 1) * perPage.value
  return filtered.value.slice(start, start + perPage.value)
})

watch(search, () => { currentPage.value = 1 })

function openEdit(u: ApiUser) {
  editingUser.value = u
  formRole.value = u.role
  dialogOpen.value = true
}

async function saveRole() {
  if (!editingUser.value) return
  saving.value = true
  const oldRole = editingUser.value.role
  editingUser.value.role = formRole.value
  try {
    const updated = await api.put<ApiUser>(`/admin/users/${editingUser.value.id}/role`, { role: formRole.value })
    const idx = items.value.findIndex(u => u.id === updated.id)
    if (idx !== -1) items.value[idx] = updated
    syncLocalRole(updated.email, updated.role)
    dialogOpen.value = false
  } catch (e) {
    editingUser.value.role = oldRole
    // Fallback: update localStorage directly even if API fails (dev without Docker)
    syncLocalRole(editingUser.value.email, formRole.value)
    const idx = items.value.findIndex(u => u.id === editingUser.value!.id)
    if (idx !== -1) items.value[idx] = { ...items.value[idx], role: formRole.value }
    dialogOpen.value = false
  } finally {
    saving.value = false
  }
}

async function removeUser(id: string) {
  if (!confirm('Are you sure you want to delete this user?')) return
  const user = items.value.find(u => u.id === id)
  try {
    await api.del(`/admin/users/${id}`)
    items.value = items.value.filter(u => u.id !== id)
    if (user) removeLocalUser(user.email)
  } catch (e) {
    error.value = e instanceof Error ? e.message : 'Failed to delete user'
  }
}

function syncLocalRole(email: string, role: 'user' | 'superuser') {
  const raw = localStorage.getItem('admin_users')
  if (!raw) return
  try {
    const users = JSON.parse(raw)
    const idx = users.findIndex((u: any) => u.email === email)
    if (idx !== -1) {
      users[idx].role = role
      localStorage.setItem('admin_users', JSON.stringify(users))
    }
  } catch { /* ignore */ }
}

function removeLocalUser(email: string) {
  const raw = localStorage.getItem('admin_users')
  if (!raw) return
  try {
    const users = JSON.parse(raw).filter((u: any) => u.email !== email)
    localStorage.setItem('admin_users', JSON.stringify(users))
  } catch { /* ignore */ }
}

function displayName(u: ApiUser): string {
  return u.full_name ?? u.email.split('@')[0]
}

function formatDate(iso: string): string {
  return iso.split('T')[0]
}
</script>

<template>
  <div class="mx-auto max-w-6xl space-y-6">
    <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
      <div>
        <h1 class="text-xl sm:text-2xl font-bold tracking-tight">Users</h1>
        <p class="text-xs sm:text-sm text-muted-foreground mt-1">Manage all user accounts and their roles.</p>
      </div>
    </div>

    <div v-if="error" class="rounded-md bg-destructive/10 border border-destructive/20 px-4 py-3 text-sm text-destructive">{{ error }}</div>

    <!-- Search -->
    <div class="flex w-full max-w-sm items-center gap-2 rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm">
      <svg class="w-4 h-4 text-muted-foreground shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" /></svg>
      <input v-model="search" type="text" placeholder="Search users..." class="flex-1 bg-transparent outline-none placeholder:text-muted-foreground text-sm">
    </div>

    <!-- Table -->
    <div class="rounded-xl border bg-card text-card-foreground shadow">
      <div class="relative w-full overflow-auto">
        <table class="w-full caption-bottom text-sm">
          <thead class="[&_tr]:border-b">
            <tr class="border-b border-border transition-colors hover:bg-muted/50">
              <th class="h-10 px-2 text-left align-middle font-medium text-muted-foreground">Name</th>
              <th class="h-10 px-2 text-left align-middle font-medium text-muted-foreground">Email</th>
              <th class="h-10 px-2 text-left align-middle font-medium text-muted-foreground">Role</th>
              <th class="h-10 px-2 text-left align-middle font-medium text-muted-foreground">Status</th>
              <th class="h-10 px-2 text-left align-middle font-medium text-muted-foreground">Created</th>
              <th class="h-10 px-2 text-left align-middle font-medium text-muted-foreground">Actions</th>
            </tr>
          </thead>
          <tbody class="[&_tr:last-child]:border-0">
            <tr v-for="u in paginated" :key="u.id" class="border-b border-border transition-colors hover:bg-muted/50">
              <td class="p-2 align-middle font-medium">{{ displayName(u) }}</td>
              <td class="p-2 align-middle text-muted-foreground">{{ u.email }}</td>
              <td class="p-2 align-middle">
                <span class="inline-flex items-center rounded-md border px-2.5 py-0.5 text-xs font-semibold"
                  :class="u.role === 'superuser' ? 'border-transparent bg-purple-100 text-purple-700 dark:bg-purple-900/40 dark:text-purple-400' : 'border-transparent bg-secondary text-secondary-foreground'"
                >{{ u.role }}</span>
              </td>
              <td class="p-2 align-middle">
                <span class="inline-flex items-center rounded-md border px-2.5 py-0.5 text-xs font-semibold"
                  :class="u.email_verified ? 'border-transparent bg-emerald-100 text-emerald-700 dark:bg-emerald-900/40 dark:text-emerald-400' : 'border-transparent bg-destructive/10 text-destructive'"
                >{{ u.email_verified ? 'active' : 'inactive' }}</span>
              </td>
              <td class="p-2 align-middle text-muted-foreground">{{ formatDate(u.created_at) }}</td>
              <td class="p-2 align-middle">
                <div class="flex items-center gap-1">
                  <button class="inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-xs font-medium transition-colors hover:bg-accent hover:text-accent-foreground h-8 px-2" @click="openEdit(u)">Edit</button>
                  <button class="inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-xs font-medium transition-colors hover:bg-destructive/10 hover:text-destructive h-8 px-2" @click="removeUser(u.id)">Delete</button>
                </div>
              </td>
            </tr>
            <tr v-if="loading">
              <td colspan="6" class="p-2 align-middle text-center text-muted-foreground py-8">Loading users...</td>
            </tr>
            <tr v-else-if="paginated.length === 0">
              <td colspan="6" class="p-2 align-middle text-center text-muted-foreground py-8">No users found.</td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <!-- Pagination -->
    <div v-if="totalPages > 1" class="flex items-center justify-between">
      <p class="text-sm text-muted-foreground">Page {{ currentPage }} of {{ totalPages }}</p>
      <div class="flex gap-1">
        <button class="inline-flex items-center justify-center rounded-md border border-input bg-background px-3 py-1.5 text-sm font-medium shadow-sm hover:bg-accent hover:text-accent-foreground transition-colors disabled:opacity-50 disabled:pointer-events-none" :disabled="currentPage <= 1" @click="currentPage--">Previous</button>
        <button class="inline-flex items-center justify-center rounded-md border border-input bg-background px-3 py-1.5 text-sm font-medium shadow-sm hover:bg-accent hover:text-accent-foreground transition-colors disabled:opacity-50 disabled:pointer-events-none" :disabled="currentPage >= totalPages" @click="currentPage++">Next</button>
      </div>
    </div>

    <!-- Edit Role Dialog -->
    <div v-if="dialogOpen && editingUser" class="fixed inset-0 z-50 flex items-center justify-center">
      <div class="fixed inset-0 bg-black/80" @click="dialogOpen = false" />
      <div class="fixed left-[50%] top-[50%] z-50 grid w-full max-w-lg translate-x-[-50%] translate-y-[-50%] gap-4 border bg-background text-foreground p-6 shadow-lg duration-200 sm:rounded-lg">
        <div class="flex flex-col space-y-1.5">
          <h2 class="text-lg font-semibold leading-none tracking-tight">Edit User Role</h2>
          <p class="text-sm text-muted-foreground">{{ editingUser.full_name ?? editingUser.email }}</p>
        </div>
        <div class="space-y-4">
          <div class="space-y-1">
            <label class="text-sm font-medium leading-none">Role</label>
            <select v-model="formRole" class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring">
              <option value="user">User</option>
              <option value="superuser">Superuser</option>
            </select>
          </div>
        </div>
        <div class="flex flex-col-reverse sm:flex-row sm:justify-end sm:space-x-2">
          <button class="inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-colors hover:bg-accent hover:text-accent-foreground h-9 px-4 py-2" @click="dialogOpen = false">Cancel</button>
          <button class="inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-colors bg-primary text-primary-foreground shadow hover:bg-primary/90 h-9 px-4 py-2" :disabled="saving" @click="saveRole">{{ saving ? 'Saving...' : 'Save' }}</button>
        </div>
      </div>
    </div>
  </div>
</template>
