import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

export interface User {
  id: string
  email: string
  fullName: string | null
  accountType: 'personal' | 'company'
  emailVerified: boolean
  createdAt: string
  updatedAt: string
}

export interface UserState {
  user: User | null
  isAuthenticated: boolean
  isLoading: boolean
}

export const useUserStore = defineStore('user', () => {
  // State
  const user = ref<User | null>(null)
  const isLoading = ref(false)

  // Getters
  const isAuthenticated = computed(() => user.value !== null)
  const userEmail = computed(() => user.value?.email ?? null)
  const userName = computed(() => user.value?.fullName ?? user.value?.email?.split('@')[0] ?? 'User')

  // Actions
  function setUser(userData: User): void {
    user.value = userData
  }

  function clearUser(): void {
    user.value = null
  }

  function setLoading(value: boolean): void {
    isLoading.value = value
  }

  async function fetchUser(): Promise<void> {
    setLoading(true)
    try {
      // TODO: Implement API call to fetch current user
      // const response = await $fetch('/api/v1/users/me')
      // setUser(response.user)
    } catch (error) {
      clearUser()
    } finally {
      setLoading(false)
    }
  }

  async function login(_email: string, _password: string): Promise<void> {
    setLoading(true)
    try {
      // TODO: Implement login API call
      // const response = await $fetch('/api/v1/auth/login', {
      //   method: 'POST',
      //   body: { email, password }
      // })
      // setUser(response.user)
    } finally {
      setLoading(false)
    }
  }

  async function logout(): Promise<void> {
    setLoading(true)
    try {
      // TODO: Implement logout API call
      // await $fetch('/api/v1/auth/logout', { method: 'POST' })
    } finally {
      clearUser()
      setLoading(false)
    }
  }

  return {
    // State
    user,
    isLoading,
    // Getters
    isAuthenticated,
    userEmail,
    userName,
    // Actions
    setUser,
    clearUser,
    setLoading,
    fetchUser,
    login,
    logout
  }
})
