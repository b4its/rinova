import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

export type UserRole = 'user' | 'superuser'

export interface User {
  id: string
  email: string
  fullName: string | null
  accountType: 'personal' | 'company'
  role: UserRole
  emailVerified: boolean
  createdAt: string
  updatedAt: string
}

export const useUserStore = defineStore('user', () => {
  const user = ref<User | null>(null)
  const isLoading = ref(false)

  const isAuthenticated = computed(() => user.value !== null)
  const isSuperuser = computed(() => user.value?.role === 'superuser')
  const userEmail = computed(() => user.value?.email ?? null)
  const userName = computed(() => user.value?.fullName ?? user.value?.email?.split('@')[0] ?? 'User')

  function setUser(userData: User): void {
    user.value = userData
  }

  function clearUser(): void {
    user.value = null
  }

  function setLoading(value: boolean): void {
    isLoading.value = value
  }

  return {
    user, isLoading,
    isAuthenticated, isSuperuser, userEmail, userName,
    setUser, clearUser, setLoading
  }
})
