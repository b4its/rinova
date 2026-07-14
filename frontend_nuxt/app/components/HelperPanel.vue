<script setup lang="ts">
const props = defineProps<{ previewDark: boolean }>()
const emit = defineEmits<{ 'update:previewDark': [v: boolean] }>()

const showGrid = ref(false)
const snapToGrid = ref(true)
const showGuides = ref(false)
</script>

<template>
  <div class="border-t border-border">
    <div class="px-3 py-2 flex items-center justify-between">
      <span class="text-[10px] font-semibold text-gray-400 uppercase tracking-wider">Helper</span>
      <button
        class="p-1 rounded text-muted-foreground hover:text-foreground hover:bg-accent"
        @click="$emit('close')"
        title="Collapse helper"
      >
        <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
        </svg>
      </button>
    </div>

    <div class="px-3 pb-3 space-y-2.5">
      <!-- Theme Toggle (preview only) -->
      <button
        class="w-full flex items-center justify-between rounded-lg px-3 py-2 text-xs font-medium transition-colors hover:bg-accent"
        @click="emit('update:previewDark', !previewDark)"
      >
        <div class="flex items-center gap-2">
          <svg v-if="previewDark" class="w-4 h-4 text-yellow-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z" />
          </svg>
          <svg v-else class="w-4 h-4 text-amber-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z" />
          </svg>
          <span class="text-foreground">Preview Dark</span>
        </div>
        <div
          class="relative w-8 h-4 rounded-full transition-colors"
          :class="previewDark ? 'bg-primary' : 'bg-gray-300 dark:bg-gray-600'"
        >
          <span
            class="absolute top-0.5 w-3 h-3 rounded-full bg-white shadow transition-transform"
            :class="previewDark ? 'translate-x-4' : 'translate-x-0.5'"
          />
        </div>
      </button>

      <!-- Show Grid -->
      <button
        class="w-full flex items-center justify-between rounded-lg px-3 py-2 text-xs font-medium transition-colors hover:bg-accent"
        @click="showGrid = !showGrid"
      >
        <div class="flex items-center gap-2">
          <svg class="w-4 h-4 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zM14 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zM14 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z" />
          </svg>
          <span class="text-foreground">Show Grid</span>
        </div>
        <div
          class="w-8 h-4 rounded-full transition-colors relative"
          :class="showGrid ? 'bg-primary' : 'bg-gray-300 dark:bg-gray-600'"
        >
          <span
            class="absolute top-0.5 w-3 h-3 rounded-full bg-white shadow transition-transform"
            :class="showGrid ? 'translate-x-4' : 'translate-x-0.5'"
          />
        </div>
      </button>

      <!-- Snap to Grid -->
      <button
        class="w-full flex items-center justify-between rounded-lg px-3 py-2 text-xs font-medium transition-colors hover:bg-accent"
        @click="snapToGrid = !snapToGrid"
      >
        <div class="flex items-center gap-2">
          <svg class="w-4 h-4 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
          </svg>
          <span class="text-foreground">Snap to Grid</span>
        </div>
        <div
          class="w-8 h-4 rounded-full transition-colors relative"
          :class="snapToGrid ? 'bg-primary' : 'bg-gray-300 dark:bg-gray-600'"
        >
          <span
            class="absolute top-0.5 w-3 h-3 rounded-full bg-white shadow transition-transform"
            :class="snapToGrid ? 'translate-x-4' : 'translate-x-0.5'"
          />
        </div>
      </button>

      <!-- Show Guides -->
      <button
        class="w-full flex items-center justify-between rounded-lg px-3 py-2 text-xs font-medium transition-colors hover:bg-accent"
        @click="showGuides = !showGuides"
      >
        <div class="flex items-center gap-2">
          <svg class="w-4 h-4 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 12H4M12 4v16" />
          </svg>
          <span class="text-foreground">Show Guides</span>
        </div>
        <div
          class="w-8 h-4 rounded-full transition-colors relative"
          :class="showGuides ? 'bg-primary' : 'bg-gray-300 dark:bg-gray-600'"
        >
          <span
            class="absolute top-0.5 w-3 h-3 rounded-full bg-white shadow transition-transform"
            :class="showGuides ? 'translate-x-4' : 'translate-x-0.5'"
          />
        </div>
      </button>
    </div>
  </div>
</template>
