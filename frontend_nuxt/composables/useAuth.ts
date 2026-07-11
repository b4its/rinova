import type { User } from '~/stores/user'

/**
 * Composable for authentication management
 * Validates: Requirements 1 (User Registration and Authentication)
 */
export function useAuth() {
  const userStore = useUserStore()
  const config = useRuntimeConfig()
  
  const isLoading = ref(false)
  const error = ref<string | null>(null)

  /**
   * Login with email and password
   * Requirement 1.6: Generate JWT with 7-day expiry, store in secure HTTP-only cookie
   */
  async function login(email: string, password: string): Promise<boolean> {
    isLoading.value = true
    error.value = null
    
    try {
      const response = await $fetch<{ user: User; token: string }>(
        `${config.public.apiUrl}/api/v1/auth/login`,
        {
          method: 'POST',
          body: { email, password }
        }
      )
      
      userStore.setUser(response.user)
      return true
    } catch (e: unknown) {
      const apiError = e as { data?: { message?: string } }
      // Requirement 1.7: Generic error for invalid credentials
      error.value = apiError.data?.message || 'Invalid credentials'
      return false
    } finally {
      isLoading.value = false
    }
  }

  /**
   * Register new user
   * Requirement 1.2: Create account and send verification email within 30 seconds
   */
  async function register(email: string, password: string, fullName: string, accountType: 'personal' | 'company'): Promise<boolean> {
    isLoading.value = true
    error.value = null
    
    try {
      const response = await $fetch<{ user: User; message: string }>(
        `${config.public.apiUrl}/api/v1/auth/register`,
        {
          method: 'POST',
          body: { email, password, fullName, accountType }
        }
      )
      
      return true
    } catch (e: unknown) {
      const apiError = e as { data?: { message?: string } }
      error.value = apiError.data?.message || 'Registration failed'
      return false
    } finally {
      isLoading.value = false
    }
  }

  /**
   * Verify email with token
   * Requirement 1.4: Verify token within 24-hour window
   */
  async function verifyEmail(token: string): Promise<boolean> {
    isLoading.value = true
    error.value = null
    
    try {
      const response = await $fetch<{ user: User; message: string }>(
        `${config.public.apiUrl}/api/v1/auth/verify-email`,
        {
          method: 'POST',
          body: { token }
        }
      )
      
      userStore.setUser(response.user)
      return true
    } catch (e: unknown) {
      const apiError = e as { data?: { message?: string } }
      error.value = apiError.data?.message || 'Email verification failed'
      return false
    } finally {
      isLoading.value = false
    }
  }

  /**
   * Logout user
   */
  async function logout(): Promise<void> {
    isLoading.value = true
    
    try {
      await $fetch(`${config.public.apiUrl}/api/v1/auth/logout`, {
        method: 'POST'
      })
    } finally {
      userStore.clearUser()
      isLoading.value = false
    }
  }

  /**
   * Fetch current user
   */
  async function fetchCurrentUser(): Promise<void> {
    if (!userStore.isAuthenticated) {
      return
    }
    
    isLoading.value = true
    
    try {
      const response = await $fetch<{ user: User }>(
        `${config.public.apiUrl}/api/v1/users/me`
      )
      
      userStore.setUser(response.user)
    } catch {
      userStore.clearUser()
    } finally {
      isLoading.value = false
    }
  }

  /**
   * Resend verification email
   * Requirement 1.5: Provide resend option for expired verification
   */
  async function resendVerification(email: string): Promise<boolean> {
    isLoading.value = true
    error.value = null
    
    try {
      await $fetch(`${config.public.apiUrl}/api/v1/auth/resend-verification`, {
        method: 'POST',
        body: { email }
      })
      
      return true
    } catch (e: unknown) {
      const apiError = e as { data?: { message?: string } }
      error.value = apiError.data?.message || 'Failed to resend verification email'
      return false
    } finally {
      isLoading.value = false
    }
  }

  return {
    isLoading: readonly(isLoading),
    error: readonly(error),
    login,
    register,
    verifyEmail,
    logout,
    fetchCurrentUser,
    resendVerification
  }
}
