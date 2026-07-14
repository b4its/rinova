<script setup lang="ts">
definePageMeta({
  layout: 'default'
})

const route = useRoute()
const router = useRouter()
const config = useRuntimeConfig()

const token = computed(() => route.query.token as string)
const email = computed(() => route.query.email as string)

const password = ref('')
const confirmPassword = ref('')
const showPassword = ref(false)
const isLoading = ref(false)
const error = ref<string | null>(null)
const success = ref(false)

// Password validation
const passwordRequirements = computed(() => ({
  length: password.value.length >= 8,
  uppercase: /[A-Z]/.test(password.value),
  lowercase: /[a-z]/.test(password.value),
  number: /[0-9]/.test(password.value)
}))

const passwordValid = computed(() => 
  Object.values(passwordRequirements.value).every(Boolean)
)

const passwordsMatch = computed(() => 
  password.value === confirmPassword.value && password.value.length > 0
)

const canSubmit = computed(() => 
  passwordValid.value && passwordsMatch.value && token.value && email.value
)

async function handleSubmit() {
  if (!canSubmit.value) return
  
  isLoading.value = true
  error.value = null
  
  try {
    await $fetch(`${config.public.apiUrl}/api/v1/auth/reset-password`, {
      method: 'POST',
      body: {
        token: token.value,
        email: email.value,
        password: password.value
      }
    })
    
    success.value = true
  } catch (e: unknown) {
    const apiError = e as { data?: { message?: string } }
    error.value = apiError.data?.message || 'Failed to reset password. The link may have expired.'
  } finally {
    isLoading.value = false
  }
}

// Redirect if no token
onMounted(() => {
  if (!token.value || !email.value) {
    router.push('/forgot-password')
  }
})
</script>

<template>
  <div class="min-h-screen flex items-center justify-center bg-gradient-to-br from-primary-50 to-primary-100 dark:from-gray-950 dark:to-gray-900 py-12 px-4">
    <div class="max-w-md w-full">
      <!-- Logo -->
      <div class="text-center mb-8">
        <NuxtLink to="/" class="inline-block">
          <h1 class="text-3xl font-bold text-primary-600">Rinova</h1>
        </NuxtLink>
      </div>

      <!-- Success State -->
      <div v-if="success" class="card">
        <div class="card-body text-center">
          <div class="mx-auto w-16 h-16 bg-green-100 rounded-full flex items-center justify-center mb-4">
            <svg class="w-8 h-8 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
            </svg>
          </div>
          <h3 class="text-lg font-semibold text-gray-900 mb-2">Password Reset!</h3>
          <p class="text-sm text-gray-600 mb-6">
            Your password has been reset successfully. You can now log in with your new password.
          </p>
          <NuxtLink to="/login" class="btn btn-primary">
            Sign In
          </NuxtLink>
        </div>
      </div>

      <!-- Form -->
      <div v-else class="card">
        <div class="card-body">
          <div class="text-center mb-6">
            <h2 class="text-2xl font-bold text-gray-900">Reset Password</h2>
            <p class="text-sm text-gray-600 mt-2">
              Enter your new password below
            </p>
          </div>
          
          <form @submit.prevent="handleSubmit">
            <!-- Error -->
            <div v-if="error" class="p-4 bg-red-50 border border-red-200 rounded-lg mb-4">
              <p class="text-sm text-red-600">{{ error }}</p>
            </div>
            
            <!-- Password -->
            <div class="mb-4">
              <label for="password" class="label">New Password</label>
              <div class="relative">
                <input
                  id="password"
                  v-model="password"
                  :type="showPassword ? 'text' : 'password'"
                  required
                  class="input pr-10"
                  placeholder="••••••••"
                />
                <button
                  type="button"
                  class="absolute right-3 top-1/2 -translate-y-1/2 text-gray-400"
                  @click="showPassword = !showPassword"
                >
                  <svg v-if="showPassword" class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13.875 18.825A10.05 10.05 0 0112 19c-4.478 0-8.268-2.943-9.543-7a9.97 9.97 0 011.563-3.029m5.858.908a3 3 0 114.243 4.243M9.878 9.878l4.242 4.242M9.88 9.88l-3.29-3.29m7.532 7.532l3.29 3.29M3 3l3.59 3.59m0 0A9.953 9.953 0 0112 5c4.478 0 8.268 2.943 9.543 7a10.025 10.025 0 01-4.132 5.411m0 0L21 21" />
                  </svg>
                  <svg v-else class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
                  </svg>
                </button>
              </div>
              
              <!-- Password Requirements -->
              <div v-if="password" class="mt-2 space-y-1">
                <div
                  v-for="(met, requirement) in passwordRequirements"
                  :key="requirement"
                  class="flex items-center text-xs"
                  :class="met ? 'text-green-600' : 'text-gray-400'"
                >
                  <svg class="w-4 h-4 mr-1.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path v-if="met" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
                    <circle v-else cx="12" cy="12" r="10" stroke="currentColor" stroke-width="2" fill="none" />
                  </svg>
                  <span v-if="requirement === 'length'">At least 8 characters</span>
                  <span v-else-if="requirement === 'uppercase'">One uppercase letter</span>
                  <span v-else-if="requirement === 'lowercase'">One lowercase letter</span>
                  <span v-else-if="requirement === 'number'">One number</span>
                </div>
              </div>
            </div>
            
            <!-- Confirm Password -->
            <div class="mb-6">
              <label for="confirmPassword" class="label">Confirm Password</label>
              <input
                id="confirmPassword"
                v-model="confirmPassword"
                type="password"
                required
                class="input"
                :class="{ 'input-error': confirmPassword && !passwordsMatch }"
                placeholder="••••••••"
              />
              <p v-if="confirmPassword && !passwordsMatch" class="form-error">
                Passwords do not match
              </p>
            </div>
            
            <!-- Submit -->
            <button
              type="submit"
              class="btn btn-primary w-full"
              :disabled="!canSubmit || isLoading"
            >
              <svg v-if="isLoading" class="animate-spin -ml-1 mr-2 h-4 w-4 text-white" fill="none" viewBox="0 0 24 24">
                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
              </svg>
              {{ isLoading ? 'Resetting...' : 'Reset Password' }}
            </button>
          </form>
        </div>
      </div>
    </div>
  </div>
</template>
