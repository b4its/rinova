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

/** Raw workspace shape from workspace_service (snake_case). */
interface BackendWorkspace {
  id: string
  name: string
  type: WorkspaceType
  logo_url: string | null
  current_user_role: WorkspaceRole
  created_at: string
  updated_at: string
}

function mapWorkspace(w: BackendWorkspace): Workspace {
  return {
    id: w.id,
    name: w.name,
    type: w.type,
    role: w.current_user_role,
    logoUrl: w.logo_url,
    createdAt: w.created_at,
    updatedAt: w.updated_at
  }
}

export const useWorkspaceStore = defineStore('workspace', () => {
  const workspaces = ref<Workspace[]>([])
  const currentWorkspaceId = ref<string | null>(null)
  const isLoading = ref(false)

  const currentWorkspace = computed(() =>
    workspaces.value.find(w => w.id === currentWorkspaceId.value) ?? null
  )

  const personalWorkspace = computed(() =>
    workspaces.value.find(w => w.type === 'personal') ?? null
  )

  const companyWorkspaces = computed(() =>
    workspaces.value.filter(w => w.type === 'company')
  )

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
    const userStore = useUserStore()
    if (!userStore.user) return
    const api = useApi()
    isLoading.value = true
    try {
      const res = await api.get<{ workspaces: BackendWorkspace[] }>(
        `/workspaces?user_id=${userStore.user.id}`
      )
      workspaces.value = res.workspaces.map(mapWorkspace)
      // Default to the personal workspace, else the first one.
      if (!currentWorkspaceId.value && workspaces.value.length > 0) {
        const personal = workspaces.value.find(w => w.type === 'personal')
        currentWorkspaceId.value = personal?.id ?? workspaces.value[0]!.id
      }
    } catch (e) {
      // Don't let a workspace fetch failure crash the whole app.
      console.warn('[workspace] failed to fetch workspaces:', e)
      workspaces.value = []
    } finally {
      isLoading.value = false
    }
  }

  async function switchWorkspace(workspaceId: string): Promise<void> {
    setCurrentWorkspace(workspaceId)
  }

  return {
    workspaces, currentWorkspaceId, isLoading,
    currentWorkspace, personalWorkspace, companyWorkspaces,
    setWorkspaces, setCurrentWorkspace, addWorkspace,
    removeWorkspace, clearWorkspaces, fetchWorkspaces, switchWorkspace
  }
})
