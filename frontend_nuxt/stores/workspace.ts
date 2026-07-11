import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

export type WorkspaceType = 'personal' | 'company'
export type WorkspaceRole = 'owner' | 'admin' | 'member'

export interface Workspace {
  id: string
  name: string
  type: WorkspaceType
  role: WorkspaceRole
  logoUrl: string | null
  createdAt: string
  updatedAt: string
}

export const useWorkspaceStore = defineStore('workspace', () => {
  // State
  const workspaces = ref<Workspace[]>([])
  const currentWorkspaceId = ref<string | null>(null)
  const isLoading = ref(false)

  // Getters
  const currentWorkspace = computed(() => 
    workspaces.value.find(w => w.id === currentWorkspaceId.value) ?? null
  )
  
  const personalWorkspace = computed(() => 
    workspaces.value.find(w => w.type === 'personal') ?? null
  )
  
  const companyWorkspaces = computed(() => 
    workspaces.value.filter(w => w.type === 'company')
  )

  // Actions
  function setWorkspaces(workspaceList: Workspace[]): void {
    workspaces.value = workspaceList
  }

  function setCurrentWorkspace(workspaceId: string): void {
    currentWorkspaceId.value = workspaceId
  }

  function addWorkspace(workspace: Workspace): void {
    workspaces.value.push(workspace)
  }

  function removeWorkspace(workspaceId: string): void {
    const index = workspaces.value.findIndex(w => w.id === workspaceId)
    if (index !== -1) {
      workspaces.value.splice(index, 1)
    }
  }

  function clearWorkspaces(): void {
    workspaces.value = []
    currentWorkspaceId.value = null
  }

  async function fetchWorkspaces(): Promise<void> {
    isLoading.value = true
    try {
      // TODO: Implement API call
      // const response = await $fetch('/api/v1/workspaces')
      // setWorkspaces(response.workspaces)
      // if (response.workspaces.length > 0 && !currentWorkspaceId.value) {
      //   setCurrentWorkspace(response.workspaces[0].id)
      // }
    } finally {
      isLoading.value = false
    }
  }

  async function switchWorkspace(workspaceId: string): Promise<void> {
    setCurrentWorkspace(workspaceId)
    // Could trigger additional data fetches here
  }

  return {
    // State
    workspaces,
    currentWorkspaceId,
    isLoading,
    // Getters
    currentWorkspace,
    personalWorkspace,
    companyWorkspaces,
    // Actions
    setWorkspaces,
    setCurrentWorkspace,
    addWorkspace,
    removeWorkspace,
    clearWorkspaces,
    fetchWorkspaces,
    switchWorkspace
  }
})
