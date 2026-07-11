import type { ComponentType } from '~/types'

interface DragState {
  isDragging: boolean
  draggedType: ComponentType | null
  draggedId: string | null
  dragOffset: { x: number; y: number }
}

interface DropPosition {
  parentId: string | null
  index: number
  rect: DOMRect | null
}

/**
 * Composable for drag and drop functionality
 * Validates: Requirements 5.1, 5.2 (200 FPS performance), 6.2, 6.3, 6.4
 */
export function useDragDrop() {
  const state = reactive<DragState>({
    isDragging: false,
    draggedType: null,
    draggedId: null,
    dragOffset: { x: 0, y: 0 }
  })

  const dropPosition = ref<DropPosition>({
    parentId: null,
    index: -1,
    rect: null
  })

  // RAF-based drag optimization for 200 FPS
  let rafId: number | null = null
  let lastMousePosition = { x: 0, y: 0 }

  /**
   * Start dragging a new component from sidebar
   * Requirement 5.1: 200 FPS during drag
   */
  function startDragFromSidebar(
    componentType: ComponentType, 
    event: MouseEvent | DragEvent
  ): void {
    const target = event.target as HTMLElement
    const rect = target.getBoundingClientRect()
    
    state.isDragging = true
    state.draggedType = componentType
    state.draggedId = null
    state.dragOffset = {
      x: event.clientX - rect.left,
      y: event.clientY - rect.top
    }
    
    lastMousePosition = { x: event.clientX, y: event.clientY }
  }

  /**
   * Start dragging an existing component on canvas
   * Requirement 6.5: Move component with position update
   */
  function startDragFromCanvas(
    componentId: string,
    event: MouseEvent | DragEvent
  ): void {
    const target = event.target as HTMLElement
    const rect = target.getBoundingClientRect()
    
    state.isDragging = true
    state.draggedType = null
    state.draggedId = componentId
    state.dragOffset = {
      x: event.clientX - rect.left,
      y: event.clientY - rect.top
    }
    
    lastMousePosition = { x: event.clientX, y: event.clientY }
  }

  /**
   * Handle drag move with RAF optimization
   * Requirement 5.2: Render position in < 5ms
   */
  function handleDragMove(event: MouseEvent): void {
    if (!state.isDragging) return
    
    // Store position for RAF processing
    lastMousePosition = { x: event.clientX, y: event.clientY }
    
    // Use RAF for smooth 200 FPS updates
    if (rafId === null) {
      rafId = requestAnimationFrame(() => {
        // RAF callback - update drop indicator position
        updateDropIndicator(lastMousePosition)
        rafId = null
      })
    }
  }

  /**
   * End drag operation
   */
  function endDrag(): void {
    if (rafId !== null) {
      cancelAnimationFrame(rafId)
      rafId = null
    }
    
    state.isDragging = false
    state.draggedType = null
    state.draggedId = null
    state.dragOffset = { x: 0, y: 0 }
    
    dropPosition.value = {
      parentId: null,
      index: -1,
      rect: null
    }
  }

  /**
   * Update drop indicator position
   * Requirement 6.2: Show drop indicator for placement preview
   */
  function updateDropIndicator(mousePos: { x: number; y: number }): void {
    // Find drop target under mouse
    const elements = document.elementsFromPoint(mousePos.x, mousePos.y)
    
    for (const element of elements) {
      const dropZone = element.closest('[data-drop-zone]')
      if (dropZone) {
        const rect = dropZone.getBoundingClientRect()
        const parentId = dropZone.getAttribute('data-component-id') || null
        
        // Calculate drop index based on mouse position
        const children = dropZone.querySelectorAll(':scope > [data-component-id]')
        let index = children.length
        
        children.forEach((child, i) => {
          const childRect = child.getBoundingClientRect()
          const midY = childRect.top + childRect.height / 2
          
          if (mousePos.y < midY) {
            index = Math.min(index, i)
          }
        })
        
        dropPosition.value = {
          parentId,
          index,
          rect
        }
        return
      }
    }
    
    // No valid drop zone found
    dropPosition.value = {
      parentId: null,
      index: -1,
      rect: null
    }
  }

  /**
   * Get drop position for drop handler
   * Requirement 6.4: Handle component drop at correct position
   */
  function getDropPosition(): DropPosition {
    return { ...dropPosition.value }
  }

  /**
   * Check if currently dragging
   */
  const isDragging = computed(() => state.isDragging)

  /**
   * Get dragged component type (for new components)
   */
  const draggedType = computed(() => state.draggedType)

  /**
   * Get dragged component ID (for existing components)
   */
  const draggedId = computed(() => state.draggedId)

  return {
    // State
    isDragging,
    draggedType,
    draggedId,
    dropPosition: readonly(dropPosition),
    
    // Actions
    startDragFromSidebar,
    startDragFromCanvas,
    handleDragMove,
    endDrag,
    getDropPosition
  }
}
