<script setup lang="ts">
definePageMeta({
  layout: 'default'
})

const config = useRuntimeConfig()

const email = ref('')
const isLoading = ref(false)
const isSubmitted = ref(false)
const error = ref<string | null>(null)

async function handleSubmit() {
  if (!email.value) return
  
  isLoading.value = true
  error.value = null
  
  try {
    // TODO: Implement API call
    await $fetch(`${config.public.apiUrl}/api/v1/auth/forgot-password`, {
      method: 'POST',
      body: { email: email.value }
    })
    
    isSubmitted.value = true
  } catch (e: unknown) {
    const apiError = e as { data?: { message?: string } }
    error.value = apiError.data?.message || 'Failed to send reset email. Please try again.'
  } finally {
    isLoading.value = false
  }
}
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
      <div v-if="isSubmitted" class="card">
        <div class="card-body text-center">
          <div class="mx-auto w-16 h-16 bg-green-100 rounded-full flex items-center justify-center mb-4">
            <svg class="w-8 h-8 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
            </svg>
          </div>
          <h3 class="text-lg font-semibold text-gray-900 mb-2">Check your email</h3>
          <p class="text-sm text-gray-600 mb-4">
            We've sent a password reset link to <strong>{{ email }}</strong>
          </p>
          <p class="text-xs text-gray-500">
            The link will expire in 1 hour. If you don't see the email, check your spam folder.
          </p>
          <NuxtLink to="/login" class="btn btn-primary mt-6">
            Back to Login
          </NuxtLink>
        </div>
      </div>

      <!-- Form -->
      <div v-else class="card">
        <div class="card-body">
          <div class="text-center mb-6">
            <h2 class="text-2xl font-bold text-gray-900">Forgot Password?</h2>
            <p class="text-sm text-gray-600 mt-2">
              Enter your email address and we'll send you a link to reset your password.
            </p>
          </div>
          
          <form @submit.prevent="handleSubmit">
            <!-- Error -->
            <div v-if="error" class="p-4 bg-red-50 border border-red-200 rounded-lg mb-4">
              <p class="text-sm text-red-600">{{ error }}</p>
            </div>
            
            <!-- Email -->
            <div class="mb-6">
              <label for="email" class="label">Email address</label>
              <input
                id="email"
                v-model="email"
                type="email"
                required
                class="input"
                placeholder="you@example.com"
              />
            </div>
            
            <!-- Submit -->
            <button
              type="submit"
              class="btn btn-primary w-full"
              :disabled="!email || isLoading"
            >
              <svg v-if="isLoading" class="animate-spin -ml-1 mr-2 h-4 w-4 text-white" fill="none" viewBox="0 0 24 24">
                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
              </svg>
              {{ isLoading ? 'Sending...' : 'Send Reset Link' }}
            </button>
          </form>
        </div>
        
        <div class="card-footer text-center">
          <p class="text-sm text-gray-600">
            Remember your password?
            <NuxtLink to="/login" class="text-primary-600 hover:text-primary-700 font-medium">
              Sign in
            </NuxtLink>
          </p>
        </div>
      </div>
    </div>
  </div>
</template>
