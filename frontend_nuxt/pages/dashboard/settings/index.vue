<script setup lang="ts">
definePageMeta({
  layout: 'dashboard'
})

const userStore = useUserStore()
const { fetchCurrentUser } = useAuth()

// Form state
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
    // TODO: Implement API call to update profile
    await new Promise(resolve => setTimeout(resolve, 1000))
    successMessage.value = 'Profile updated successfully'
  } catch {
    errorMessage.value = 'Failed to update profile'
  } finally {
    isSaving.value = false
  }
}
</script>

<template>
  <div>
    <div class="mb-8">
      <h1 class="text-2xl font-bold text-gray-900">Settings</h1>
      <p class="mt-1 text-sm text-gray-600">
        Manage your account settings and preferences
      </p>
    </div>
    
    <div class="max-w-2xl">
      <!-- Profile Section -->
      <div class="card mb-6">
        <div class="card-header">
          <h2 class="text-lg font-semibold text-gray-900">Profile</h2>
        </div>
        
        <div class="card-body space-y-6">
          <!-- Success Message -->
          <div v-if="successMessage" class="p-4 bg-green-50 border border-green-200 rounded-lg">
            <p class="text-sm text-green-600">{{ successMessage }}</p>
          </div>
          
          <!-- Error Message -->
          <div v-if="errorMessage" class="p-4 bg-red-50 border border-red-200 rounded-lg">
            <p class="text-sm text-red-600">{{ errorMessage }}</p>
          </div>
          
          <!-- Avatar -->
          <div>
            <label class="label">Avatar</label>
            <div class="flex items-center gap-4">
              <div class="w-16 h-16 rounded-full bg-primary-100 flex items-center justify-center">
                <span class="text-xl font-medium text-primary-600">
                  {{ fullName.charAt(0).toUpperCase() || 'U' }}
                </span>
              </div>
              <button class="btn btn-secondary btn-sm">
                Change Avatar
              </button>
            </div>
          </div>
          
          <!-- Full Name -->
          <div>
            <label for="fullName" class="label">Full Name</label>
            <input
              id="fullName"
              v-model="fullName"
              type="text"
              class="input"
            />
          </div>
          
          <!-- Email -->
          <div>
            <label for="email" class="label">Email</label>
            <input
              id="email"
              v-model="email"
              type="email"
              class="input"
              disabled
            />
            <p class="text-xs text-gray-500 mt-1">
              Email cannot be changed. Contact support if needed.
            </p>
          </div>
          
          <!-- Save Button -->
          <button
            class="btn btn-primary"
            :disabled="isSaving"
            @click="handleSaveProfile"
          >
            <svg v-if="isSaving" class="animate-spin -ml-1 mr-2 h-4 w-4 text-white" fill="none" viewBox="0 0 24 24">
              <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
              <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
            </svg>
            {{ isSaving ? 'Saving...' : 'Save Changes' }}
          </button>
        </div>
      </div>
      
      <!-- Password Section -->
      <div class="card mb-6">
        <div class="card-header">
          <h2 class="text-lg font-semibold text-gray-900">Password</h2>
        </div>
        
        <div class="card-body">
          <NuxtLink to="/dashboard/settings/password" class="btn btn-secondary">
            Change Password
          </NuxtLink>
        </div>
      </div>
      
      <!-- Danger Zone -->
      <div class="card border-red-200">
        <div class="card-header bg-red-50">
          <h2 class="text-lg font-semibold text-red-600">Danger Zone</h2>
        </div>
        
        <div class="card-body">
          <p class="text-sm text-gray-600 mb-4">
            Once you delete your account, there is no going back. Please be certain.
          </p>
          <button class="btn btn-danger">
            Delete Account
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
