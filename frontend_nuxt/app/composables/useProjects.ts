/**
 * Project API helpers (project_service, PostgreSQL for metadata + MongoDB for
 * the component tree). Description and template are stored in the project
 * `metadata` JSON blob since the SQL schema only has name/status/metadata.
 */

export interface ProjectMetadata {
  description?: string
  template?: string
  [key: string]: unknown
}

/** Raw project shape from project_service (snake_case). */
interface BackendProject {
  id: string
  workspace_id: string
  owner_id: string
  name: string
  status: 'draft' | 'published' | 'archived'
  metadata: ProjectMetadata
  last_published_at: string | null
  created_at: string
  updated_at: string
}

export interface Project {
  id: string
  workspaceId: string
  ownerId: string
  name: string
  description: string
  template: string
  status: 'draft' | 'published' | 'archived'
  lastPublishedAt: string | null
  createdAt: string
  updatedAt: string
}

interface ProjectListResponse {
  projects: BackendProject[]
  total: number
  page: number
  page_size: number
  total_pages: number
}

function mapProject(p: BackendProject): Project {
  const meta = p.metadata ?? {}
  return {
    id: p.id,
    workspaceId: p.workspace_id,
    ownerId: p.owner_id,
    name: p.name,
    description: typeof meta.description === 'string' ? meta.description : '',
    template: typeof meta.template === 'string' ? meta.template : 'Blank',
    status: p.status,
    lastPublishedAt: p.last_published_at,
    createdAt: p.created_at,
    updatedAt: p.updated_at,
  }
}

/** Format an ISO timestamp as a relative "x ago" string for display. */
export function relativeTime(iso: string): string {
  const then = new Date(iso).getTime()
  const diff = Date.now() - then
  const mins = Math.floor(diff / 60000)
  if (mins < 1) return 'just now'
  if (mins < 60) return `${mins} minute${mins === 1 ? '' : 's'} ago`
  const hrs = Math.floor(mins / 60)
  if (hrs < 24) return `${hrs} hour${hrs === 1 ? '' : 's'} ago`
  const days = Math.floor(hrs / 24)
  if (days < 7) return `${days} day${days === 1 ? '' : 's'} ago`
  const weeks = Math.floor(days / 7)
  if (weeks < 5) return `${weeks} week${weeks === 1 ? '' : 's'} ago`
  return new Date(iso).toLocaleDateString()
}

export function useProjects() {
  const api = useApi()
  const workspaceStore = useWorkspaceStore()

  async function listProjects(): Promise<Project[]> {
    const wsId = workspaceStore.currentWorkspaceId
    const query = wsId ? `?workspace_id=${wsId}&page_size=100` : '?page_size=100'
    const res = await api.get<ProjectListResponse>(`/projects${query}`)
    return res.projects.map(mapProject)
  }

  async function getProject(id: string): Promise<Project> {
    const res = await api.get<BackendProject>(`/projects/${id}`)
    return mapProject(res)
  }

  async function createProject(
    name: string,
    description: string,
    template: string
  ): Promise<Project> {
    const wsId = workspaceStore.currentWorkspaceId
    if (!wsId) throw new Error('No workspace selected')
    const res = await api.post<BackendProject>('/projects', {
      name,
      workspace_id: wsId,
      metadata: { description, template },
    })
    return mapProject(res)
  }

  async function deleteProject(id: string): Promise<void> {
    await api.del(`/projects/${id}`)
  }

  return { listProjects, getProject, createProject, deleteProject }
}
