<script setup lang="ts">
definePageMeta({ layout: 'panel', sidebarSection: 'projects' })

const router = useRouter()
const { createProject: createProjectApi } = useProjects()
const name = ref('')
const description = ref('')
const template = ref('Blank')
const isCreating = ref(false)
const createError = ref<string | null>(null)

const templates = [
  { id: 'Blank', name: 'Blank Canvas', desc: 'Start from scratch' },
  { id: 'Business', name: 'Business', desc: 'Professional company site' },
  { id: 'Portfolio', name: 'Portfolio', desc: 'Showcase your work' },
  { id: 'Blog', name: 'Blog', desc: 'Personal blog' },
  { id: 'Store', name: 'Store', desc: 'E-commerce store' },
]

async function createProject() {
  if (!name.value) return
  isCreating.value = true
  createError.value = null
  try {
    const project = await createProjectApi(name.value, description.value, template.value)
    await router.push(`/editor/${project.id}`)
  } catch (e) {
    createError.value = e instanceof Error ? e.message : 'Failed to create project'
    isCreating.value = false
  }
}
</script>

<template>
  <div class="max-w-2xl mx-auto space-y-6">
    <div>
      <NuxtLink to="/panel/projects" class="text-sm text-muted-foreground hover:text-foreground inline-flex items-center gap-1 mb-2 transition-colors">
        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
        </svg>
        Back to Projects
      </NuxtLink>
      <h1 class="text-2xl font-bold tracking-tight">New Project</h1>
      <p class="text-muted-foreground text-sm mt-1">Create a new website project</p>
    </div>

    <div class="card">
      <div class="card-body space-y-6">
        <div class="space-y-2">
          <label class="label">Project Name</label>
          <input v-model="name" type="text" class="input" placeholder="My Awesome Website" />
        </div>

        <div class="space-y-2">
          <label class="label">Description</label>
          <textarea v-model="description" class="input min-h-[80px]" rows="3" placeholder="Brief description of your project..." />
        </div>

        <div class="space-y-2">
          <label class="label">Template</label>
          <div class="grid grid-cols-2 gap-3">
            <button
              v-for="t in templates"
              :key="t.id"
              class="p-4 rounded-lg border-2 text-left transition-all"
              :class="template === t.id ? 'border-primary bg-primary/5' : 'border-border hover:border-muted-foreground/30'"
              @click="template = t.id"
            >
              <div class="font-medium text-sm">{{ t.name }}</div>
              <div class="text-xs text-muted-foreground mt-0.5">{{ t.desc }}</div>
            </button>
          </div>
        </div>

        <div class="flex items-center gap-3 pt-2">
          <button class="btn btn-primary" :disabled="!name || isCreating" @click="createProject">
            <svg v-if="isCreating" class="animate-spin -ml-1 mr-2 h-4 w-4" fill="none" viewBox="0 0 24 24">
              <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
              <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
            </svg>
            {{ isCreating ? 'Creating...' : 'Create Project' }}
          </button>
          <NuxtLink to="/panel/projects" class="btn btn-secondary">Cancel</NuxtLink>
        </div>
      </div>
    </div>
  </div>
</template>
