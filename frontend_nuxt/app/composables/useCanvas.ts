import type { ComponentNode, ComponentStyles, AnimationConfig, ViewportMode } from '~/types'

export function useCanvas(projectId: string) {
  const subscription = useSubscription()

  const componentTree = shallowRef<Map<string, ComponentNode>>(new Map())
  const rootComponentIds = ref<string[]>([])
  const selectedComponentId = ref<string | null>(null)
  const viewportMode = ref<ViewportMode>('desktop')
  const isDirty = ref(false)

  const lastSaved = ref<Date | null>(null)
  let autoSaveTimer: ReturnType<typeof setInterval> | null = null

  const history = ref<string[]>([])
  const historyIndex = ref(-1)
  const maxHistoryLength = 50

  const selectedComponent = computed(() => {
    if (!selectedComponentId.value) return null
    return componentTree.value.get(selectedComponentId.value) ?? null
  })

  const componentCount = computed(() => componentTree.value.size)

  const canEditResponsiveStyles = computed(() => subscription.canUseResponsiveDesign.value)
  const canAddAnimations = computed(() => subscription.canAccessAnimationEditor.value)

  function saveToHistory(): void {
    const state = {
      components: Object.fromEntries(componentTree.value),
      rootIds: rootComponentIds.value
    }
    if (historyIndex.value < history.value.length - 1) {
      history.value = history.value.slice(0, historyIndex.value + 1)
    }
    history.value.push(JSON.stringify(state))
    if (history.value.length > maxHistoryLength) {
      history.value.shift()
    } else {
      historyIndex.value++
    }
  }

  function restoreState(state: { components: Record<string, ComponentNode>; rootIds: string[] }): void {
    componentTree.value = new Map(Object.entries(state.components))
    rootComponentIds.value = state.rootIds
  }

  function addComponent(component: ComponentNode, parentId: string | null = null, index?: number): void {
    const tree = new Map(componentTree.value)
    tree.set(component.id, component)
    if (parentId) {
      const parent = tree.get(parentId)
      if (parent) {
        const children = [...parent.children]
        typeof index === 'number' ? children.splice(index, 0, component.id) : children.push(component.id)
        tree.set(parentId, { ...parent, children })
      }
    } else {
      const roots = [...rootComponentIds.value]
      typeof index === 'number' ? roots.splice(index, 0, component.id) : roots.push(component.id)
      rootComponentIds.value = roots
    }
    componentTree.value = tree
    saveToHistory()
    isDirty.value = true
  }

  function updateComponent(componentId: string, updates: Partial<ComponentNode>): void {
    const tree = new Map(componentTree.value)
    const existing = tree.get(componentId)
    if (!existing) return
    tree.set(componentId, { ...existing, ...updates })
    componentTree.value = tree
    saveToHistory()
    isDirty.value = true
  }

  function updateComponentStyles(componentId: string, styles: Partial<ComponentStyles>, breakpoint: ViewportMode = 'desktop'): void {
    const tree = new Map(componentTree.value)
    const existing = tree.get(componentId)
    if (!existing) return
    const updatedStyles: ComponentStyles = {
      ...existing.styles,
      [breakpoint]: { ...existing.styles[breakpoint], ...styles[breakpoint] }
    }
    tree.set(componentId, { ...existing, styles: updatedStyles })
    componentTree.value = tree
    saveToHistory()
    isDirty.value = true
  }

  function addAnimation(componentId: string, animation: AnimationConfig): boolean {
    if (!canAddAnimations.value) return false
    const tree = new Map(componentTree.value)
    const existing = tree.get(componentId)
    if (!existing) return false
    const animations = [...(existing.animations ?? []), animation]
    tree.set(componentId, { ...existing, animations })
    componentTree.value = tree
    saveToHistory()
    isDirty.value = true
    return true
  }

  function deleteComponent(componentId: string): { deleted: boolean; hasChildren: boolean } {
    const tree = new Map(componentTree.value)
    const component = tree.get(componentId)
    if (!component) return { deleted: false, hasChildren: false }
    const hasChildren = component.children.length > 0
    if (component.parentId) {
      const parent = tree.get(component.parentId)
      if (parent) {
        tree.set(component.parentId, { ...parent, children: parent.children.filter(id => id !== componentId) })
      }
    } else {
      rootComponentIds.value = rootComponentIds.value.filter(id => id !== componentId)
    }
    const deleteRecursive = (id: string): void => {
      const node = tree.get(id)
      if (node) {
        node.children.forEach(deleteRecursive)
        tree.delete(id)
      }
    }
    deleteRecursive(componentId)
    componentTree.value = tree
    if (selectedComponentId.value === componentId) selectedComponentId.value = null
    saveToHistory()
    isDirty.value = true
    return { deleted: true, hasChildren }
  }

  function moveComponent(componentId: string, newParentId: string | null, newIndex: number): void {
    const tree = new Map(componentTree.value)
    const component = tree.get(componentId)
    if (!component) return
    if (component.parentId) {
      const oldParent = tree.get(component.parentId)
      if (oldParent) {
        oldParent.children = oldParent.children.filter(id => id !== componentId)
        tree.set(component.parentId, { ...oldParent })
      }
    } else {
      rootComponentIds.value = rootComponentIds.value.filter(id => id !== componentId)
    }
    if (newParentId) {
      const newParent = tree.get(newParentId)
      if (newParent) {
        const children = [...newParent.children]
        children.splice(newIndex, 0, componentId)
        tree.set(newParentId, { ...newParent, children })
      }
    } else {
      const roots = [...rootComponentIds.value]
      roots.splice(newIndex, 0, componentId)
      rootComponentIds.value = roots
    }
    tree.set(componentId, { ...component, parentId: newParentId })
    componentTree.value = tree
    saveToHistory()
    isDirty.value = true
  }

  function selectComponent(componentId: string | null): void {
    selectedComponentId.value = componentId
  }

  function undo(): void {
    if (historyIndex.value <= 0) return
    historyIndex.value--
    const snapshot = history.value[historyIndex.value]
    if (snapshot) restoreState(JSON.parse(snapshot))
  }

  function redo(): void {
    if (historyIndex.value >= history.value.length - 1) return
    historyIndex.value++
    const snapshot = history.value[historyIndex.value]
    if (snapshot) restoreState(JSON.parse(snapshot))
  }

  function setViewportMode(mode: ViewportMode): void {
    viewportMode.value = mode
  }

  function startAutoSave(): void {
    if (autoSaveTimer) return
    autoSaveTimer = setInterval(async () => {
      if (isDirty.value) await saveToBackend()
    }, 30000)
  }

  function stopAutoSave(): void {
    if (autoSaveTimer) {
      clearInterval(autoSaveTimer)
      autoSaveTimer = null
    }
  }

  // The backend (MongoDB) stores component nodes with snake_case `parent_id`
  // and a `type` field; the frontend model uses `parentId`/`type`. Map both
  // ways on save/load.
  function toBackendNode(node: ComponentNode): Record<string, unknown> {
    const { parentId, ...rest } = node
    return { ...rest, parent_id: parentId ?? null }
  }

  function fromBackendNode(raw: Record<string, unknown>): ComponentNode {
    const { parent_id, ...rest } = raw
    return { ...(rest as object), parentId: (parent_id as string | null) ?? null } as ComponentNode
  }

  async function saveToBackend(): Promise<boolean> {
    try {
      const api = useApi()
      const components: Record<string, unknown> = {}
      for (const [id, node] of componentTree.value) {
        components[id] = toBackendNode(node)
      }
      await api.put(`/projects/${projectId}/components`, {
        components,
        root_ids: rootComponentIds.value
      })
      isDirty.value = false
      lastSaved.value = new Date()
      return true
    } catch {
      return false
    }
  }

  function setDefaultContent() {
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

  async function loadFromBackend(): Promise<boolean> {
    if (componentTree.value.size > 0) return true
    try {
      const api = useApi()
      const doc = await api.get<{
        components: Record<string, Record<string, unknown>>
        root_ids: string[]
        updated_at?: string
      }>(`/projects/${projectId}/components`)
      const entries = Object.entries(doc.components ?? {}).map(
        ([id, raw]) => [id, fromBackendNode(raw)] as const
      )
      if (entries.length > 0) {
        componentTree.value = new Map(entries)
        rootComponentIds.value = doc.root_ids ?? []
        lastSaved.value = doc.updated_at ? new Date(doc.updated_at) : null
        return true
      }
    } catch { /* no saved document yet -> fall through to default */ }
    setDefaultContent()
    return true
  }

  onUnmounted(() => stopAutoSave())

  return {
    componentTree: readonly(componentTree),
    rootComponentIds: readonly(rootComponentIds),
    selectedComponentId: readonly(selectedComponentId),
    selectedComponent: readonly(selectedComponent),
    viewportMode: readonly(viewportMode),
    isDirty: readonly(isDirty),
    lastSaved: readonly(lastSaved),
    componentCount: readonly(componentCount),
    canEditResponsiveStyles,
    canAddAnimations,
    addComponent, updateComponent, updateComponentStyles,
    addAnimation, deleteComponent, moveComponent, selectComponent,
    undo, redo, setViewportMode,
    startAutoSave, stopAutoSave, saveToBackend, loadFromBackend
  }
}
