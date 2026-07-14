<script setup lang="ts">
const themeStore = useThemeStore()
</script>

<template>
  <div>
    <slot />

    <!-- Floating Theme Toggle (professional toggle switch) -->
    <button
      class="fixed bottom-6 right-6 z-50 flex items-center gap-2 px-4 py-2.5 rounded-full shadow-lg backdrop-blur-xl transition-all duration-200 hover:scale-105"
      :class="themeStore.mode === 'dark' ? 'bg-card/90 text-foreground border-border' : 'bg-card/90 text-foreground border-border'"
      @click="themeStore.toggle()"
      :aria-label="themeStore.mode === 'dark' ? 'Switch to light mode' : 'Switch to dark mode'"
    >
      <svg v-if="themeStore.mode === 'dark'" class="w-4 h-4 text-yellow-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z" />
      </svg>
      <svg v-else class="w-4 h-4 text-amber-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z" />
      </svg>
      <span class="text-xs font-medium">{{ themeStore.mode === 'dark' ? 'Light' : 'Dark' }}</span>
    </button>
  </div>
</template>
