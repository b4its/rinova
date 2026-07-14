<script setup lang="ts">
definePageMeta({ layout: 'panel', sidebarSection: 'admin-users' })

interface DummyUser {
  id: string
  name: string
  email: string
  role: 'user' | 'superuser'
  status: 'active' | 'inactive'
  created: string
}

const users = ref<DummyUser[]>([
  { id: '1', name: 'Alice Johnson', email: 'alice@example.com', role: 'user', status: 'active', created: '2026-01-15' },
  { id: '2', name: 'Bob Smith', email: 'bob@example.com', role: 'user', status: 'active', created: '2026-02-20' },
  { id: '3', name: 'Charlie Brown', email: 'charlie@example.com', role: 'user', status: 'inactive', created: '2026-03-10' },
  { id: '4', name: 'Diana Prince', email: 'diana@example.com', role: 'superuser', status: 'active', created: '2026-01-01' },
  { id: '5', name: 'Eve Davis', email: 'eve@example.com', role: 'user', status: 'active', created: '2026-04-22' },
])

const dialogOpen = ref(false)
const editingUser = ref<DummyUser | null>(null)
const formName = ref('')
const formEmail = ref('')
const formRole = ref<'user' | 'superuser'>('user')
const formPassword = ref('')

function openAdd() {
  editingUser.value = null
  formName.value = ''
  formEmail.value = ''
  formRole.value = 'user'
  formPassword.value = ''
  dialogOpen.value = true
}

function openEdit(user: DummyUser) {
  editingUser.value = user
  formName.value = user.name
  formEmail.value = user.email
  formRole.value = user.role
  formPassword.value = ''
  dialogOpen.value = true
}

function saveUser() {
  if (editingUser.value) {
    const idx = users.value.findIndex(u => u.id === editingUser.value!.id)
    if (idx !== -1) {
      users.value[idx] = { ...users.value[idx], name: formName.value, email: formEmail.value, role: formRole.value }
    }
  } else {
    users.value.push({
      id: String(Date.now()),
      name: formName.value,
      email: formEmail.value,
      role: formRole.value,
      status: 'active',
      created: new Date().toISOString().split('T')[0],
    })
  }
  dialogOpen.value = false
}

function removeUser(id: string) {
  users.value = users.value.filter(u => u.id !== id)
}

const search = ref('')
const filteredUsers = computed(() => {
  if (!search.value) return users.value
  const q = search.value.toLowerCase()
  return users.value.filter(u => u.name.toLowerCase().includes(q) || u.email.toLowerCase().includes(q))
})
</script>

<template>
  <div class="space-y-6">
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-2xl font-bold tracking-tight">Users</h1>
        <p class="text-muted-foreground">Manage all user accounts on the platform.</p>
      </div>
      <button class="inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50 bg-primary text-primary-foreground shadow hover:bg-primary/90 h-9 px-4 py-2" @click="openAdd">
        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
        </svg>
        Add User
      </button>
    </div>

    <!-- Search -->
    <div class="flex w-full max-w-sm items-center gap-2 rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm">
      <svg class="w-4 h-4 text-muted-foreground shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
      </svg>
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
            <tr v-for="u in filteredUsers" :key="u.id" class="border-b border-border transition-colors hover:bg-muted/50">
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
              <td class="p-2 align-middle text-muted-foreground">{{ u.created }}</td>
              <td class="p-2 align-middle">
                <div class="flex items-center gap-1">
                  <button class="inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-xs font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50 hover:bg-accent hover:text-accent-foreground h-8 px-2" @click="openEdit(u)">Edit</button>
                  <button class="inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-xs font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50 hover:bg-destructive/10 hover:text-destructive h-8 px-2" @click="removeUser(u.id)">Delete</button>
                </div>
              </td>
            </tr>
            <tr v-if="filteredUsers.length === 0">
              <td colspan="6" class="p-2 align-middle text-center text-muted-foreground py-8">No users found.</td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <!-- Dialog -->
    <div v-if="dialogOpen" class="fixed inset-0 z-50 flex items-center justify-center">
      <div class="fixed inset-0 bg-black/80" @click="dialogOpen = false" />
      <div class="fixed left-[50%] top-[50%] z-50 grid w-full max-w-lg translate-x-[-50%] translate-y-[-50%] gap-4 border bg-background text-foreground p-6 shadow-lg duration-200 sm:rounded-lg">
        <div class="flex flex-col space-y-1.5">
          <h2 class="text-lg font-semibold leading-none tracking-tight">{{ editingUser ? 'Edit User' : 'Add User' }}</h2>
          <p class="text-sm text-muted-foreground">{{ editingUser ? 'Update user details below.' : 'Fill in the details to create a new user.' }}</p>
        </div>
        <div class="space-y-4">
          <div class="space-y-1">
            <label class="text-sm font-medium leading-none">Name</label>
            <input v-model="formName" type="text" class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring" placeholder="Full name">
          </div>
          <div class="space-y-1">
            <label class="text-sm font-medium leading-none">Email</label>
            <input v-model="formEmail" type="email" class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring" placeholder="Email address">
          </div>
          <div class="space-y-1">
            <label class="text-sm font-medium leading-none">Password</label>
            <input v-model="formPassword" type="password" class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring" :placeholder="editingUser ? 'Leave blank to keep current' : 'Password'">
          </div>
          <div class="space-y-1">
            <label class="text-sm font-medium leading-none">Role</label>
            <select v-model="formRole" class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring">
              <option value="user">User</option>
              <option value="superuser">Superuser</option>
            </select>
          </div>
        </div>
        <div class="flex flex-col-reverse sm:flex-row sm:justify-end sm:space-x-2">
          <button class="inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50 hover:bg-accent hover:text-accent-foreground h-9 px-4 py-2" @click="dialogOpen = false">Cancel</button>
          <button class="inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50 bg-primary text-primary-foreground shadow hover:bg-primary/90 h-9 px-4 py-2" @click="saveUser">{{ editingUser ? 'Save' : 'Create' }}</button>
        </div>
      </div>
    </div>
  </div>
</template>
