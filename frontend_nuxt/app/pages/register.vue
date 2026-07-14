<script setup lang="ts">
definePageMeta({
  layout: 'default'
})

const { register, isLoading, error } = useAuth()
const router = useRouter()

const email = ref('')
const password = ref('')
const confirmPassword = ref('')
const fullName = ref('')
const accountType = ref<'personal' | 'company'>('personal')
const acceptTerms = ref(false)
const showPassword = ref(false)
const showConfirmPassword = ref(false)
const formError = ref<string | null>(null)
const registrationSuccess = ref(false)

const passwordRequirements = computed(() => ({
  length: password.value.length >= 8,
  uppercase: /[A-Z]/.test(password.value),
  lowercase: /[a-z]/.test(password.value),
  number: /[0-9]/.test(password.value)
}))

const passwordStrength = computed(() => {
  const met = Object.values(passwordRequirements.value).filter(Boolean).length
  if (met === 0) return { level: 0, label: '', color: '' }
  if (met === 1) return { level: 1, label: 'Weak', color: 'bg-red-500' }
  if (met === 2) return { level: 2, label: 'Fair', color: 'bg-yellow-500' }
  if (met === 3) return { level: 3, label: 'Good', color: 'bg-blue-500' }
  return { level: 4, label: 'Strong', color: 'bg-green-500' }
})

const emailError = computed(() => {
  if (!email.value) return null
  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/
  return emailRegex.test(email.value) ? null : 'Please enter a valid email address'
})

const passwordError = computed(() => {
  if (!password.value) return null
  if (!passwordRequirements.value.length) return 'Password must be at least 8 characters'
  if (!passwordRequirements.value.uppercase) return 'Password must contain an uppercase letter'
  if (!passwordRequirements.value.lowercase) return 'Password must contain a lowercase letter'
  if (!passwordRequirements.value.number) return 'Password must contain a number'
  return null
})

const confirmPasswordError = computed(() => {
  if (!confirmPassword.value) return null
  return password.value === confirmPassword.value ? null : 'Passwords do not match'
})

const canSubmit = computed(() => {
  return (
    email.value &&
    password.value &&
    confirmPassword.value &&
    fullName.value &&
    acceptTerms.value &&
    !emailError.value &&
    !passwordError.value &&
    !confirmPasswordError.value
  )
})

async function handleRegister() {
  if (!canSubmit.value) return
  formError.value = null
  const success = await register(email.value, password.value, fullName.value, accountType.value)
  if (success) {
    registrationSuccess.value = true
  } else {
    formError.value = error.value || 'Registration failed'
  }
}
</script>

<template>
  <div class="min-h-screen flex items-center justify-center bg-gradient-to-br from-primary-50 to-primary-100 dark:from-gray-950 dark:to-gray-900 py-12 px-4 sm:px-6 lg:px-8">
    <div class="max-w-md w-full">
      <NuxtLink to="/" class="inline-flex items-center gap-1.5 text-sm text-muted-foreground hover:text-foreground transition-colors mb-6">
        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
        </svg>
        Back to Rinova
      </NuxtLink>

      <div class="text-center mb-8">
        <NuxtLink to="/" class="inline-block">
          <h1 class="text-3xl font-bold text-primary">Rinova</h1>
        </NuxtLink>
        <h2 class="mt-4 text-2xl font-bold text-foreground">
          Create your account
        </h2>
        <p class="mt-2 text-sm text-muted-foreground">
          Start building professional websites today
        </p>
      </div>

      <div v-if="registrationSuccess" class="card">
        <div class="card-body text-center">
          <div class="mx-auto w-16 h-16 bg-green-100 dark:bg-green-900/40 rounded-full flex items-center justify-center mb-4">
            <svg class="w-8 h-8 text-green-600 dark:text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
            </svg>
          </div>
          <h3 class="text-lg font-semibold text-foreground mb-2">Check your email</h3>
          <p class="text-sm text-muted-foreground mb-4">
            We've sent a verification link to <strong>{{ email }}</strong>. 
            Please verify your email to activate your account.
          </p>
          <p class="text-xs text-muted-foreground">
            The verification link will expire in 24 hours.
          </p>
          <NuxtLink to="/login" class="btn btn-primary mt-6">
            Back to Login
          </NuxtLink>
        </div>
      </div>

      <div v-else class="card">
        <form class="card-body space-y-5" @submit.prevent="handleRegister">
          <div v-if="formError" class="p-4 bg-destructive/10 border border-destructive/20 rounded-lg">
            <p class="text-sm text-destructive">{{ formError }}</p>
          </div>

          <div>
            <label for="fullName" class="label">Full Name</label>
            <input
              id="fullName"
              v-model="fullName"
              type="text"
              autocomplete="name"
              required
              class="input"
              placeholder="John Doe"
            >
          </div>

          <div>
            <label for="email" class="label">Email address</label>
            <input
              id="email"
              v-model="email"
              type="email"
              autocomplete="email"
              required
              class="input"
              :class="{ 'input-error': emailError }"
              placeholder="you@example.com"
            >
            <p v-if="emailError" class="form-error">{{ emailError }}</p>
          </div>

          <div>
            <label class="label">Account Type</label>
            <div class="grid grid-cols-2 gap-3">
              <button
                type="button"
                class="p-4 rounded-lg border-2 text-left transition-all"
                :class="accountType === 'personal' 
                  ? 'border-primary bg-primary/5' 
                  : 'border-border hover:border-muted-foreground/30'"
                @click="accountType = 'personal'"
              >
                <div class="flex items-center gap-3">
                  <svg class="w-6 h-6" :class="accountType === 'personal' ? 'text-primary' : 'text-muted-foreground'" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
                  </svg>
                  <div>
                    <div class="font-medium text-foreground">Personal</div>
                    <div class="text-xs text-muted-foreground">For individual use</div>
                  </div>
                </div>
              </button>
              <button
                type="button"
                class="p-4 rounded-lg border-2 text-left transition-all"
                :class="accountType === 'company' 
                  ? 'border-primary bg-primary/5' 
                  : 'border-border hover:border-muted-foreground/30'"
                @click="accountType = 'company'"
              >
                <div class="flex items-center gap-3">
                  <svg class="w-6 h-6" :class="accountType === 'company' ? 'text-primary' : 'text-muted-foreground'" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4" />
                  </svg>
                  <div>
                    <div class="font-medium text-foreground">Company</div>
                    <div class="text-xs text-muted-foreground">For teams</div>
                  </div>
                </div>
              </button>
            </div>
          </div>

          <div>
            <label for="password" class="label">Password</label>
            <div class="relative">
              <input
                id="password"
                v-model="password"
                :type="showPassword ? 'text' : 'password'"
                autocomplete="new-password"
                required
                class="input pr-10"
                :class="{ 'input-error': passwordError }"
                placeholder="&bull;&bull;&bull;&bull;&bull;&bull;&bull;&bull;"
              >
              <button
                type="button"
                class="absolute right-3 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground"
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

            <div v-if="password" class="mt-2">
              <div class="flex gap-1 mb-1">
                <div
                  v-for="i in 4"
                  :key="i"
                  class="h-1 flex-1 rounded-full transition-colors"
                  :class="i <= passwordStrength.level ? passwordStrength.color : 'bg-muted'"
                />
              </div>
              <p class="text-xs text-muted-foreground">{{ passwordStrength.label }}</p>
            </div>

            <div v-if="password" class="mt-2 space-y-1">
              <div
                v-for="(met, requirement) in passwordRequirements"
                :key="requirement"
                class="flex items-center text-xs"
                :class="met ? 'text-green-600 dark:text-green-400' : 'text-muted-foreground'"
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

          <div>
            <label for="confirmPassword" class="label">Confirm Password</label>
            <div class="relative">
              <input
                id="confirmPassword"
                v-model="confirmPassword"
                :type="showConfirmPassword ? 'text' : 'password'"
                autocomplete="new-password"
                required
                class="input pr-10"
                :class="{ 'input-error': confirmPasswordError }"
                placeholder="&bull;&bull;&bull;&bull;&bull;&bull;&bull;&bull;"
              >
              <button
                type="button"
                class="absolute right-3 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground"
                @click="showConfirmPassword = !showConfirmPassword"
              >
                <svg v-if="showConfirmPassword" class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13.875 18.825A10.05 10.05 0 0112 19c-4.478 0-8.268-2.943-9.543-7a9.97 9.97 0 011.563-3.029m5.858.908a3 3 0 114.243 4.243M9.878 9.878l4.242 4.242M9.88 9.88l-3.29-3.29m7.532 7.532l3.29 3.29M3 3l3.59 3.59m0 0A9.953 9.953 0 0112 5c4.478 0 8.268 2.943 9.543 7a10.025 10.025 0 01-4.132 5.411m0 0L21 21" />
                </svg>
                <svg v-else class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
                </svg>
              </button>
            </div>
            <p v-if="confirmPasswordError" class="form-error">{{ confirmPasswordError }}</p>
          </div>

          <div class="flex items-start">
            <input
              id="terms"
              v-model="acceptTerms"
              type="checkbox"
              class="mt-1 w-4 h-4 text-primary border-border rounded focus:ring-primary"
            >
            <label for="terms" class="ml-2 text-sm text-muted-foreground">
              I agree to the 
              <NuxtLink to="/terms" class="text-primary hover:text-primary/80">Terms of Service</NuxtLink>
              and
              <NuxtLink to="/privacy" class="text-primary hover:text-primary/80">Privacy Policy</NuxtLink>
            </label>
          </div>

          <button
            type="submit"
            :disabled="!canSubmit || isLoading"
            class="btn btn-primary w-full"
          >
            <svg v-if="isLoading" class="animate-spin -ml-1 mr-2 h-4 w-4 text-white" fill="none" viewBox="0 0 24 24">
              <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
              <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z" />
            </svg>
            {{ isLoading ? 'Creating account...' : 'Create account' }}
          </button>
        </form>
      </div>

      <p v-if="!registrationSuccess" class="mt-6 text-center text-sm text-muted-foreground">
        Already have an account?
        <NuxtLink to="/login" class="font-medium text-primary hover:text-primary/80">
          Sign in
        </NuxtLink>
      </p>
    </div>
  </div>
</template>
