<script setup lang="ts">
definePageMeta({ layout: false })

const route = useRoute()
const templateId = route.params.id as string

const viewportMode = ref<'desktop' | 'tablet' | 'mobile'>('desktop')
const template = ref<{ name: string; code: string } | null>(null)

onMounted(() => {
  const saved = localStorage.getItem('admin_templates')
  if (saved) {
    const items = JSON.parse(saved)
    template.value = items.find((t: any) => t.id === Number(templateId)) || items[0] || null
  }
})

function resizeIframe(e: Event) {
  const iframe = e.target as HTMLIFrameElement
  iframe.style.height = iframe.contentWindow?.document.body.scrollHeight + 'px'
}
</script>

<template>
  <div v-if="template" class="h-screen flex flex-col bg-background">
    <!-- Navbar -->
    <header class="h-12 bg-card border-b border-border flex items-center justify-between px-2 sm:px-4 shrink-0">
      <div class="flex items-center gap-1.5 sm:gap-3 min-w-0">
        <NuxtLink to="/panel/admin/templates" class="flex items-center gap-1 text-xs sm:text-sm text-muted-foreground hover:text-foreground transition-colors truncate">
          <svg class="w-3.5 h-3.5 sm:w-4 sm:h-4 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" /></svg>
          <span class="hidden sm:inline">Kembali ke Templates</span>
          <span class="sm:hidden">Kembali</span>
        </NuxtLink>
        <div class="w-5 h-5 sm:w-6 sm:h-6 bg-gradient-to-br from-primary-500 to-purple-600 rounded flex items-center justify-center shrink-0">
          <svg class="w-2.5 h-2.5 sm:w-3 sm:h-3 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" /></svg>
        </div>
        <span class="text-xs sm:text-sm font-bold text-primary-600 dark:text-primary-400 truncate">Rinova</span>
      </div>

      <div class="flex items-center gap-1.5 sm:gap-3 shrink-0">
        <span class="hidden sm:flex items-center gap-1 px-2 py-0.5 rounded-full bg-amber-100 dark:bg-amber-900/30 text-amber-700 dark:text-amber-400 text-[10px] font-semibold uppercase tracking-wider">
          <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" /><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" /></svg>
          Preview
        </span>
        <span class="text-xs font-medium text-muted-foreground truncate max-w-[120px] sm:max-w-none">{{ template.name }}</span>
        <div class="flex items-center gap-0.5 bg-muted rounded-md p-0.5">
          <button v-for="mode in ['desktop', 'tablet', 'mobile'] as const" :key="mode"
            class="p-1 sm:p-1.5 rounded transition-colors"
            :class="viewportMode === mode ? 'bg-card shadow-sm' : 'hover:bg-accent'"
            @click="viewportMode = mode">
            <svg class="w-3 h-3 sm:w-3.5 sm:h-3.5 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <template v-if="mode === 'desktop'"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" /></template>
              <template v-else-if="mode === 'tablet'"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 18h.01M7 21h10a2 2 0 002-2V5a2 2 0 00-2-2H7a2 2 0 00-2 2v14a2 2 0 002 2z" /></template>
              <template v-else><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 18h.01M8 21h8a2 2 0 002-2V5a2 2 0 00-2-2H8a2 2 0 00-2 2v14a2 2 0 002 2z" /></template>
            </svg>
          </button>
        </div>
      </div>
    </header>

    <!-- Preview content -->
    <div class="flex-1 overflow-auto bg-muted p-2 sm:p-4 md:p-6 lg:p-8">
      <div class="flex justify-center transition-all duration-200 shrink-0"
        :class="viewportMode === 'mobile' ? 'w-[375px]' : viewportMode === 'tablet' ? 'w-[768px]' : 'w-[1280px]'">
        <!-- Desktop frame -->
        <div v-if="viewportMode === 'desktop'" class="w-full bg-card rounded-lg overflow-hidden shadow-2xl border border-border">
          <div class="bg-muted px-3 py-1.5 flex items-center gap-1.5 border-b border-border">
            <span class="w-2.5 h-2.5 rounded-full bg-red-400" />
            <span class="w-2.5 h-2.5 rounded-full bg-yellow-400" />
            <span class="w-2.5 h-2.5 rounded-full bg-green-400" />
            <span class="ml-2 text-[10px] text-muted-foreground font-mono truncate">{{ template.name }}</span>
          </div>
          <iframe sandbox="allow-same-origin allow-scripts" :srcdoc="template.code" class="w-full border-0 min-h-[500px]" @load="resizeIframe($event)" />
        </div>
        <!-- Tablet frame -->
        <div v-else-if="viewportMode === 'tablet'" class="w-full rounded-[24px] shadow-2xl p-2.5 bg-card">
          <div class="bg-background rounded-[16px] overflow-hidden">
            <iframe sandbox="allow-same-origin allow-scripts" :srcdoc="template.code" class="w-full border-0 min-h-[600px]" @load="resizeIframe($event)" />
          </div>
        </div>
        <!-- Mobile frame -->
        <div v-else class="w-full rounded-[32px] shadow-2xl p-2 relative bg-card">
          <div class="absolute top-0 left-1/2 -translate-x-1/2 w-28 h-5 rounded-b-xl z-10 bg-card" />
          <div class="bg-background rounded-[24px] overflow-hidden">
            <iframe sandbox="allow-same-origin allow-scripts" :srcdoc="template.code" class="w-full border-0 min-h-[600px]" @load="resizeIframe($event)" />
          </div>
          <div class="absolute bottom-1.5 left-1/2 -translate-x-1/2 w-24 h-1 rounded-full bg-muted-foreground/30" />
        </div>
      </div>
    </div>
  </div>

  <div v-else class="h-screen flex items-center justify-center bg-background">
    <p class="text-sm text-muted-foreground">Template not found.</p>
  </div>
</template>