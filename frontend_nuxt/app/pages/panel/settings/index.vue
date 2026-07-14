<script setup lang="ts">
definePageMeta({ layout: 'panel', sidebarSection: 'settings' })

const userStore = useUserStore()
const api = useApi()

const fullName = ref(userStore.user?.fullName || '')
const email = ref(userStore.user?.email || '')
const isSaving = ref(false)
const successMessage = ref('')
const errorMessage = ref('')

async function handleSaveProfile() {
  isSaving.value = true
  successMessage.value = ''
  errorMessage.value = ''
  try {
    const updated = await api.put<{ id: string; full_name: string | null }>(
      '/users/me',
      { full_name: fullName.value }
    )
    if (userStore.user) {
      userStore.setUser({ ...userStore.user, fullName: updated.full_name ?? fullName.value })
    }
    successMessage.value = 'Profile updated successfully'
  } catch (e) {
    errorMessage.value = e instanceof Error ? e.message : 'Failed to update profile'
  } finally {
    isSaving.value = false
  }
}
</script>

<template>
  <div class="space-y-6 max-w-2xl mx-auto">
    <div>
      <h1 class="text-2xl font-bold tracking-tight">Settings</h1>
      <p class="text-muted-foreground text-sm mt-1">Manage your account settings and preferences</p>
    </div>

    <div class="card p-5">
      <div class="card-header">
        <h2 class="card-title">Profile</h2>
        <p class="card-description">Update your personal information</p>
      </div>
      <div class="card-body space-y-6">
        <div v-if="successMessage" class="rounded-lg border border-green-200 bg-green-50 dark:border-green-900 dark:bg-green-950/50 px-4 py-3 text-sm text-green-700 dark:text-green-400">
          {{ successMessage }}
        </div>
        <div v-if="errorMessage" class="rounded-lg border border-red-200 bg-red-50 dark:border-red-900 dark:bg-red-950/50 px-4 py-3 text-sm text-red-700 dark:text-red-400">
          {{ errorMessage }}
        </div>

        <div class="flex items-center gap-4">
          <div class="w-14 h-14 rounded-full bg-primary flex items-center justify-center shrink-0">
            <span class="text-lg font-semibold text-primary-foreground">{{ (fullName || 'U').charAt(0).toUpperCase() }}</span>
          </div>
          <button class="btn btn-secondary btn-sm">Change Avatar</button>
        </div>

        <div class="space-y-2">
          <label class="label">Full Name</label>
          <input v-model="fullName" type="text" class="input" />
        </div>

        <div class="space-y-2">
          <label class="label">Email</label>
          <input :value="email" type="email" class="input" disabled />
          <p class="text-xs text-muted-foreground">Email cannot be changed. Contact support if needed.</p>
        </div>

        <button class="btn btn-primary" :disabled="isSaving" @click="handleSaveProfile">
          <svg v-if="isSaving" class="animate-spin -ml-1 mr-2 h-4 w-4" fill="none" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
          </svg>
          {{ isSaving ? 'Saving...' : 'Save Changes' }}
        </button>
      </div>
    </div>

    <div class="card p-5">
      <div class="card-header">
        <h2 class="card-title">Password</h2>
        <p class="card-description">Update your account password</p>
      </div>
      <div class="card-body">
        <NuxtLink to="/panel/settings/password" class="btn btn-secondary btn-sm">
          Change Password
        </NuxtLink>
      </div>
    </div>

    <div class="card border-destructive/50 p-5">
      <div class="card-header">
        <h2 class="card-title text-destructive dark:text-red-500 font-semibold text-lg">Danger Zone</h2>
        <p class="card-description">Irreversible actions. Proceed with caution.</p>
      </div>
      <div class="card-body">
        <p class="text-sm text-muted-foreground mb-4">Once you delete your account, there is no going back.</p>
        <button class="btn bg-destructive text-destructive-foreground hover:bg-destructive/90 btn-sm">Delete Account</button>
      </div>
    </div>
  </div>
</template>
