<script setup lang="ts">
import type { ComponentNode } from '~/types'
import BuilderCanvas from '~/components/BuilderCanvas.vue'

definePageMeta({ layout: false })

const route = useRoute()
const projectId = route.params.id as string

const componentTree = shallowRef<Map<string, ComponentNode>>(new Map())
const rootComponentIds = ref<string[]>([])
const viewportMode = ref<'desktop' | 'tablet' | 'mobile'>('desktop')
const loaded = ref(false)

onMounted(() => {
  try {
    const saved = localStorage.getItem(`project_${projectId}`)
    if (saved) {
      const data = JSON.parse(saved)
      componentTree.value = new Map(Object.entries(data.components))
      rootComponentIds.value = data.rootIds
    } else {
      const hero = {
        id: 'comp_hero_001',
        type: 'Hero' as const,
        props: { heading: 'Build Your Dream Website', subheading: 'Create stunning landing pages with our drag-and-drop builder. No coding required.', ctaText: 'Get Started Free' },
        styles: { desktop: {} },
        children: [],
        parentId: null
      }
      componentTree.value = new Map([[hero.id, hero]])
      rootComponentIds.value = [hero.id]
    }
  } catch {}
  loaded.value = true
})
</script>

<template>
  <div class="h-screen flex flex-col bg-background">
    <!-- Preview navbar -->
    <header class="h-12 bg-card border-b border-border flex items-center justify-between px-4 shrink-0">
      <div class="flex items-center gap-3">
        <NuxtLink :to="`/editor/${projectId}`" class="flex items-center gap-1.5 text-sm text-muted-foreground hover:text-foreground transition-colors">
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" /></svg>
          Kembali ke Editor
        </NuxtLink>
        <div class="w-6 h-6 bg-gradient-to-br from-primary-500 to-purple-600 rounded flex items-center justify-center">
          <svg class="w-3 h-3 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" /></svg>
        </div>
        <span class="text-sm font-bold text-primary-600 dark:text-primary-400">Rinova</span>
      </div>

      <div class="flex items-center gap-3">
        <span class="flex items-center gap-1 px-2 py-0.5 rounded-full bg-amber-100 dark:bg-amber-900/30 text-amber-700 dark:text-amber-400 text-[10px] font-semibold uppercase tracking-wider">
          <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" /><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" /></svg>
          Preview
        </span>

        <div class="flex items-center gap-0.5 bg-muted rounded-md p-0.5">
          <button v-for="mode in ['desktop', 'tablet', 'mobile'] as const" :key="mode"
            class="p-1.5 rounded transition-colors"
            :class="viewportMode === mode ? 'bg-card shadow-sm' : 'hover:bg-accent'"
            @click="viewportMode = mode">
            <svg v-if="mode === 'desktop'" class="w-3.5 h-3.5 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" /></svg>
            <svg v-else-if="mode === 'tablet'" class="w-3.5 h-3.5 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 18h.01M7 21h10a2 2 0 002-2V5a2 2 0 00-2-2H7a2 2 0 00-2 2v14a2 2 0 002 2z" /></svg>
            <svg v-else class="w-3.5 h-3.5 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 18h.01M8 21h8a2 2 0 002-2V5a2 2 0 00-2-2H8a2 2 0 00-2 2v14a2 2 0 002 2z" /></svg>
          </button>
        </div>
      </div>
    </header>

    <!-- Preview content -->
    <div class="flex-1 overflow-auto bg-background p-4 sm:p-6 lg:p-8">
      <div v-if="!loaded" class="flex items-center justify-center h-full text-muted-foreground text-sm">Loading...</div>
      <div v-else class="flex justify-center" :class="viewportMode === 'mobile' ? 'max-w-sm mx-auto' : viewportMode === 'tablet' ? 'max-w-2xl mx-auto' : ''">
        <div class="w-full bg-card shadow-xl rounded-lg overflow-hidden" :class="viewportMode !== 'desktop' ? 'border border-border' : ''">
          <BuilderCanvas
            :component-tree="componentTree"
            :root-component-ids="rootComponentIds"
            :selected-component-id="null"
            :viewport-mode="viewportMode"
            :preview="true"
          />
        </div>
      </div>
    </div>
  </div>
</template>