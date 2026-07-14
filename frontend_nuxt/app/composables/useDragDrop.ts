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

export function useDragDrop() {
  const state = reactive<DragState>({
    isDragging: false,
    draggedType: null,
    draggedId: null,
    dragOffset: { x: 0, y: 0 }
  })

  const dropPosition = ref<DropPosition>({ parentId: null, index: -1, rect: null })

  let rafId: number | null = null
  let lastMousePosition = { x: 0, y: 0 }

  function startDragFromSidebar(componentType: ComponentType, event: MouseEvent | DragEvent): void {
    const target = event.target as HTMLElement
    const rect = target.getBoundingClientRect()
    state.isDragging = true
    state.draggedType = componentType
    state.draggedId = null
    state.dragOffset = { x: event.clientX - rect.left, y: event.clientY - rect.top }
    lastMousePosition = { x: event.clientX, y: event.clientY }
  }

  function startDragFromCanvas(componentId: string, event: MouseEvent | DragEvent): void {
    const target = event.target as HTMLElement
    const rect = target.getBoundingClientRect()
    state.isDragging = true
    state.draggedType = null
    state.draggedId = componentId
    state.dragOffset = { x: event.clientX - rect.left, y: event.clientY - rect.top }
    lastMousePosition = { x: event.clientX, y: event.clientY }
  }

  function handleDragMove(event: MouseEvent): void {
    if (!state.isDragging) return
    lastMousePosition = { x: event.clientX, y: event.clientY }
    if (rafId === null) {
      rafId = requestAnimationFrame(() => {
        updateDropIndicator(lastMousePosition)
        rafId = null
      })
    }
  }

  function endDrag(): void {
    if (rafId !== null) {
      cancelAnimationFrame(rafId)
      rafId = null
    }
    state.isDragging = false
    state.draggedType = null
    state.draggedId = null
    state.dragOffset = { x: 0, y: 0 }
    dropPosition.value = { parentId: null, index: -1, rect: null }
  }

  function updateDropIndicator(mousePos: { x: number; y: number }): void {
    const elements = document.elementsFromPoint(mousePos.x, mousePos.y)
    for (const element of elements) {
      const dropZone = element.closest('[data-drop-zone]')
      if (dropZone) {
        const rect = dropZone.getBoundingClientRect()
        const parentId = dropZone.getAttribute('data-component-id') || null
        const children = dropZone.querySelectorAll(':scope > [data-component-id]')
        let index = children.length
        children.forEach((child, i) => {
          const childRect = child.getBoundingClientRect()
          const midY = childRect.top + childRect.height / 2
          if (mousePos.y < midY) index = Math.min(index, i)
        })
        dropPosition.value = { parentId, index, rect }
        return
      }
    }
    dropPosition.value = { parentId: null, index: -1, rect: null }
  }

  function getDropPosition(): DropPosition {
    return { ...dropPosition.value }
  }

  const isDragging = computed(() => state.isDragging)
  const draggedType = computed(() => state.draggedType)
  const draggedId = computed(() => state.draggedId)

  return {
    isDragging, draggedType, draggedId,
    dropPosition: readonly(dropPosition),
    startDragFromSidebar, startDragFromCanvas,
    handleDragMove, endDrag, getDropPosition
  }
}
