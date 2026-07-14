<script setup lang="ts">
definePageMeta({
  layout: 'default'
})

const route = useRoute()
const router = useRouter()
const { verifyEmail, resendVerification, isLoading, error } = useAuth()

const token = computed(() => route.query.token as string)
const verificationStatus = ref<'pending' | 'success' | 'failed'>('pending')
const resendEmail = ref('')

// Auto-verify if token is present
onMounted(async () => {
  if (token.value) {
    const success = await verifyEmail(token.value)
    verificationStatus.value = success ? 'success' : 'failed'
  }
})

async function handleResend() {
  if (!resendEmail.value) return
  await resendVerification(resendEmail.value)
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

      <!-- Pending State -->
      <div v-if="verificationStatus === 'pending'" class="card">
        <div class="card-body text-center">
          <div class="mx-auto w-16 h-16 bg-primary-100 rounded-full flex items-center justify-center mb-4">
            <svg class="animate-spin w-8 h-8 text-primary-600" fill="none" viewBox="0 0 24 24">
              <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
              <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z" />
            </svg>
          </div>
          <h3 class="text-lg font-semibold text-gray-900 mb-2">Verifying your email</h3>
          <p class="text-sm text-gray-600">Please wait while we verify your email address...</p>
        </div>
      </div>

      <!-- Success State -->
      <div v-else-if="verificationStatus === 'success'" class="card">
        <div class="card-body text-center">
          <div class="mx-auto w-16 h-16 bg-green-100 rounded-full flex items-center justify-center mb-4">
            <svg class="w-8 h-8 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
            </svg>
          </div>
          <h3 class="text-lg font-semibold text-gray-900 mb-2">Email Verified!</h3>
          <p class="text-sm text-gray-600 mb-6">
            Your email has been verified successfully. You can now access all features of your account.
          </p>
          <NuxtLink to="/panel" class="btn btn-primary">
            Go to Dashboard
          </NuxtLink>
        </div>
      </div>

      <!-- Failed State -->
      <div v-else class="card">
        <div class="card-body text-center">
          <div class="mx-auto w-16 h-16 bg-red-100 rounded-full flex items-center justify-center mb-4">
            <svg class="w-8 h-8 text-red-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
            </svg>
          </div>
          <h3 class="text-lg font-semibold text-gray-900 mb-2">Verification Failed</h3>
          <p class="text-sm text-gray-600 mb-4">
            {{ error || 'The verification link is invalid or has expired. Please request a new one.' }}
          </p>
          
          <div class="border-t border-gray-100 pt-4 mt-4">
            <p class="text-sm text-gray-600 mb-2">Resend verification email</p>
            <div class="flex gap-2">
              <input
                v-model="resendEmail"
                type="email"
                class="input flex-1"
                placeholder="your@email.com"
              />
              <button
                class="btn btn-primary"
                :disabled="!resendEmail || isLoading"
                @click="handleResend"
              >
                Resend
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- No Token State -->
      <div v-if="!token" class="card">
        <div class="card-body text-center">
          <div class="mx-auto w-16 h-16 bg-yellow-100 rounded-full flex items-center justify-center mb-4">
            <svg class="w-8 h-8 text-yellow-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
            </svg>
          </div>
          <h3 class="text-lg font-semibold text-gray-900 mb-2">No Verification Token</h3>
          <p class="text-sm text-gray-600 mb-4">
            Please click the verification link in your email to verify your account.
          </p>
          <NuxtLink to="/login" class="btn btn-primary">
            Back to Login
          </NuxtLink>
        </div>
      </div>
    </div>
  </div>
</template>
