import type { ComponentNode, ComponentStyles, AnimationConfig, ViewportMode } from '~/types'
import { useSubscription } from './useSubscription'

/**
 * Composable for canvas state management
 * Validates: Requirements 5 (Drag and Drop Editor Performance), 6 (Component Management)
 */
export function useCanvas(projectId: string) {
  const subscription = useSubscription()
  
  // State - using shallowRef for performance with large trees
  // Requirement 5.4: Use shallow reference to prevent excessive reactivity
  const componentTree = shallowRef<Map<string, ComponentNode>>(new Map())
  const rootComponentIds = ref<string[]>([])
  const selectedComponentId = ref<string | null>(null)
  const viewportMode = ref<ViewportMode>('desktop')
  const isDirty = ref(false)
  
  // Auto-save state
  const lastSaved = ref<Date | null>(null)
  const autoSaveInterval = ref<NodeJS.Timeout | null>(null)

  // History for undo/redo
  // Requirement 5.7: Restore canvas state in < 50ms
  const history = ref<string[]>([])
  const historyIndex = ref(-1)
  const maxHistoryLength = 50

  // Computed
  const selectedComponent = computed(() => {
    if (!selectedComponentId.value) return null
    return componentTree.value.get(selectedComponentId.value) ?? null
  })

  const componentCount = computed(() => componentTree.value.size)

  // Check if responsive design is available
  const canEditResponsiveStyles = computed(() => 
    subscription.canUseResponsiveDesign.value
  )

  // Check if animations are available
  const canAddAnimations = computed(() => 
    subscription.canAccessAnimationEditor.value
  )

  /**
   * Add component to canvas
   * Requirement 6.3: Add component to correct position
   * Requirement 5.5: Update only affected components
   */
  function addComponent(component: ComponentNode, parentId: string | null = null, index?: number): void {
    const tree = new Map(componentTree.value)
    
    // Add component to tree
    tree.set(component.id, component)
    
    // Update parent's children array
    if (parentId) {
      const parent = tree.get(parentId)
      if (parent) {
        const children = [...parent.children]
        if (typeof index === 'number') {
          children.splice(index, 0, component.id)
        } else {
          children.push(component.id)
        }
        tree.set(parentId, { ...parent, children })
      }
    } else {
      // Add to root
      const roots = [...rootComponentIds.value]
      if (typeof index === 'number') {
        roots.splice(index, 0, component.id)
      } else {
        roots.push(component.id)
      }
      rootComponentIds.value = roots
    }
    
    componentTree.value = tree
    saveToHistory()
    isDirty.value = true
  }

  /**
   * Update component properties
   * Requirement 6.7: Update component display in < 100ms
   */
  function updateComponent(componentId: string, updates: Partial<ComponentNode>): void {
    const tree = new Map(componentTree.value)
    const existing = tree.get(componentId)
    
    if (!existing) return
    
    const updated = { ...existing, ...updates }
    tree.set(componentId, updated)
    componentTree.value = tree
    
    saveToHistory()
    isDirty.value = true
  }

  /**
   * Update component styles
   * Requirement 18: Responsive styles per breakpoint
   */
  function updateComponentStyles(
    componentId: string, 
    styles: Partial<ComponentStyles>,
    breakpoint: ViewportMode = 'desktop'
  ): void {
    const tree = new Map(componentTree.value)
    const existing = tree.get(componentId)
    
    if (!existing) return
    
    const currentStyles = existing.styles
    const updatedStyles: ComponentStyles = {
      ...currentStyles,
      [breakpoint]: { ...currentStyles[breakpoint], ...styles[breakpoint] }
    }
    
    tree.set(componentId, { ...existing, styles: updatedStyles })
    componentTree.value = tree
    
    saveToHistory()
    isDirty.value = true
  }

  /**
   * Add animation to component
   * Requirement 19: Animation support for Enterprise/Exclusive
   */
  function addAnimation(componentId: string, animation: AnimationConfig): boolean {
    if (!canAddAnimations.value) {
      return false
    }
    
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

  /**
   * Delete component
   * Requirement 6.9: Show confirmation for nested components
   */
  function deleteComponent(componentId: string): { deleted: boolean; hasChildren: boolean } {
    const tree = new Map(componentTree.value)
    const component = tree.get(componentId)
    
    if (!component) {
      return { deleted: false, hasChildren: false }
    }
    
    // Check if has children
    const hasChildren = component.children.length > 0
    
    // Remove from parent
    if (component.parentId) {
      const parent = tree.get(component.parentId)
      if (parent) {
        const children = parent.children.filter(id => id !== componentId)
        tree.set(component.parentId, { ...parent, children })
      }
    } else {
      // Remove from root
      rootComponentIds.value = rootComponentIds.value.filter(id => id !== componentId)
    }
    
    // Recursively delete children
    function deleteRecursive(id: string): void {
      const node = tree.get(id)
      if (node) {
        node.children.forEach(deleteRecursive)
        tree.delete(id)
      }
    }
    
    deleteRecursive(componentId)
    
    componentTree.value = tree
    
    if (selectedComponentId.value === componentId) {
      selectedComponentId.value = null
    }
    
    saveToHistory()
    isDirty.value = true
    
    return { deleted: true, hasChildren }
  }

  /**
   * Move component to new position
   * Requirement 6.5: Implement component move with position update
   */
  function moveComponent(componentId: string, newParentId: string | null, newIndex: number): void {
    const tree = new Map(componentTree.value)
    const component = tree.get(componentId)
    
    if (!component) return
    
    // Remove from old parent
    if (component.parentId) {
      const oldParent = tree.get(component.parentId)
      if (oldParent) {
        oldParent.children = oldParent.children.filter(id => id !== componentId)
        tree.set(component.parentId, { ...oldParent })
      }
    } else {
      rootComponentIds.value = rootComponentIds.value.filter(id => id !== componentId)
    }
    
    // Add to new parent
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
    
    // Update component's parentId
    tree.set(componentId, { ...component, parentId: newParentId })
    componentTree.value = tree
    
    saveToHistory()
    isDirty.value = true
  }

  /**
   * Select component
   * Requirement 6.5: Show selection border with 4 resize handles
   */
  function selectComponent(componentId: string | null): void {
    selectedComponentId.value = componentId
  }

  /**
   * Undo last action
   * Requirement 5.7: Restore canvas state in < 50ms
   */
  function undo(): void {
    if (historyIndex.value <= 0) return
    
    historyIndex.value--
    const state = JSON.parse(history.value[historyIndex.value])
    restoreState(state)
  }

  /**
   * Redo last undone action
   */
  function redo(): void {
    if (historyIndex.value >= history.value.length - 1) return
    
    historyIndex.value++
    const state = JSON.parse(history.value[historyIndex.value])
    restoreState(state)
  }

  /**
   * Save current state to history
   */
  function saveToHistory(): void {
    const state = {
      components: Object.fromEntries(componentTree.value),
      rootIds: rootComponentIds.value
    }
    
    // Remove any future history if we're not at the end
    if (historyIndex.value < history.value.length - 1) {
      history.value = history.value.slice(0, historyIndex.value + 1)
    }
    
    history.value.push(JSON.stringify(state))
    
    // Limit history size
    if (history.value.length > maxHistoryLength) {
      history.value.shift()
    } else {
      historyIndex.value++
    }
  }

  /**
   * Restore state from history
   */
  function restoreState(state: { components: Record<string, ComponentNode>; rootIds: string[] }): void {
    componentTree.value = new Map(Object.entries(state.components))
    rootComponentIds.value = state.rootIds
  }

  /**
   * Set viewport mode for responsive preview
   * Requirement 18.1: Preview modes for mobile, tablet, desktop
   */
  function setViewportMode(mode: ViewportMode): void {
    viewportMode.value = mode
  }

  /**
   * Auto-save canvas state
   * Requirement 5.5: Auto-save every 30 seconds
   */
  function startAutoSave(): void {
    if (autoSaveInterval.value) return
    
    autoSaveInterval.value = setInterval(async () => {
      if (isDirty.value) {
        await saveToBackend()
      }
    }, 30000) // 30 seconds
  }

  function stopAutoSave(): void {
    if (autoSaveInterval.value) {
      clearInterval(autoSaveInterval.value)
      autoSaveInterval.value = null
    }
  }

  /**
   * Save to backend
   */
  async function saveToBackend(): Promise<boolean> {
    // Implementation will call API to save
    isDirty.value = false
    lastSaved.value = new Date()
    return true
  }

  /**
   * Load canvas from backend
   */
  async function loadFromBackend(): Promise<boolean> {
    // Implementation will fetch from API
    return true
  }

  // Cleanup
  onUnmounted(() => {
    stopAutoSave()
  })

  return {
    // State
    componentTree: readonly(componentTree),
    rootComponentIds: readonly(rootComponentIds),
    selectedComponentId: readonly(selectedComponentId),
    selectedComponent: readonly(selectedComponent),
    viewportMode: readonly(viewportMode),
    isDirty: readonly(isDirty),
    lastSaved: readonly(lastSaved),
    componentCount: readonly(componentCount),
    
    // Feature checks
    canEditResponsiveStyles,
    canAddAnimations,
    
    // Actions
    addComponent,
    updateComponent,
    updateComponentStyles,
    addAnimation,
    deleteComponent,
    moveComponent,
    selectComponent,
    undo,
    redo,
    setViewportMode,
    startAutoSave,
    stopAutoSave,
    saveToBackend,
    loadFromBackend
  }
}
