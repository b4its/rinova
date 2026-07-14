import { defineStore } from 'pinia'
import { ref, watch } from 'vue'

export type ThemeMode = 'light' | 'dark'

export const useThemeStore = defineStore('theme', () => {
  const mode = ref<ThemeMode>('light')

  function load() {
    if (import.meta.client) {
      const stored = localStorage.getItem('rinova-theme') as ThemeMode | null
      if (stored) mode.value = stored
    }
  }

  function toggle() {
    mode.value = mode.value === 'light' ? 'dark' : 'light'
  }

  function setTheme(val: ThemeMode) {
    mode.value = val
  }

  if (import.meta.client) {
    watch(mode, (val) => {
      localStorage.setItem('rinova-theme', val)
      document.documentElement.classList.toggle('dark', val === 'dark')
    })
  }

  return { mode, load, toggle, setTheme }
})
