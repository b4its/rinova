<script setup lang="ts">
const emit = defineEmits<{
  close: []
  created: [project: { id: string; name: string }]
}>()

const workspaceStore = useWorkspaceStore()
const router = useRouter()

const projectName = ref('')
const projectDescription = ref('')
const isLoading = ref(false)
const error = ref<string | null>(null)

const canCreate = computed(() => projectName.value.trim().length >= 3)

async function handleCreate() {
  if (!canCreate.value) return
  
  isLoading.value = true
  error.value = null
  
  try {
    // TODO: Implement API call
    // const response = await $fetch('/api/v1/projects', {
    //   method: 'POST',
    //   body: {
    //     name: projectName.value,
    //     description: projectDescription.value,
    //     workspaceId: workspaceStore.currentWorkspaceId
    //   }
    // })
    
    // Navigate to editor
    // router.push(`/editor/${response.project.id}`)
    
    // For now, just close
    emit('close')
  } catch (e) {
    error.value = 'Failed to create project. Please try again.'
  } finally {
    isLoading.value = false
  }
}
</script>

<template>
  <div class="modal-overlay" @click.self="emit('close')">
    <div class="modal">
      <div class="modal-header">
        <h3 class="text-lg font-semibold text-gray-900">Create New Project</h3>
        <button
          class="absolute top-4 right-4 text-gray-400 hover:text-gray-600"
          @click="emit('close')"
        >
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>
      
      <form @submit.prevent="handleCreate">
        <div class="modal-body space-y-4">
          <!-- Error -->
          <div v-if="error" class="p-4 bg-red-50 border border-red-200 rounded-lg">
            <p class="text-sm text-red-600">{{ error }}</p>
          </div>
          
          <!-- Project Name -->
          <div>
            <label for="projectName" class="label">Project Name</label>
            <input
              id="projectName"
              v-model="projectName"
              type="text"
              required
              class="input"
              placeholder="My Awesome Website"
            >
            <p v-if="projectName && projectName.length < 3" class="form-error">
              Name must be at least 3 characters
            </p>
          </div>
          
          <!-- Project Description -->
          <div>
            <label for="projectDescription" class="label">
              Description <span class="text-gray-400">(optional)</span>
            </label>
            <textarea
              id="projectDescription"
              v-model="projectDescription"
              rows="3"
              class="input"
              placeholder="A brief description of your project"
            />
          </div>
          
          <!-- Workspace -->
          <div>
            <label class="label">Workspace</label>
            <div class="flex items-center gap-3 p-3 bg-gray-50 rounded-lg">
              <div class="w-8 h-8 rounded bg-primary-100 flex items-center justify-center">
                <span class="text-sm font-medium text-primary-600">
                  {{ workspaceStore.currentWorkspace?.name.charAt(0).toUpperCase() }}
                </span>
              </div>
              <div>
                <div class="font-medium text-gray-900">
                  {{ workspaceStore.currentWorkspace?.name }}
                </div>
                <div class="text-xs text-gray-500 capitalize">
                  {{ workspaceStore.currentWorkspace?.type }} workspace
                </div>
              </div>
            </div>
          </div>
        </div>
        
        <div class="modal-footer">
          <button type="button" class="btn btn-secondary" @click="emit('close')">
            Cancel
          </button>
          <button
            type="submit"
            class="btn btn-primary"
            :disabled="!canCreate || isLoading"
          >
            <svg v-if="isLoading" class="animate-spin -ml-1 mr-2 h-4 w-4 text-white" fill="none" viewBox="0 0 24 24">
              <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
              <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z" />
            </svg>
            {{ isLoading ? 'Creating...' : 'Create Project' }}
          </button>
        </div>
      </form>
    </div>
  </div>
</template>
