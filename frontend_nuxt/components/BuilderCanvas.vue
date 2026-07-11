<script setup lang="ts">
import type { ViewportMode, ComponentNode } from '~/types'

const props = defineProps<{
  projectId: string
  viewportMode: ViewportMode
}>()

const emit = defineEmits<{
  'component:selected': [component: ComponentNode]
  'component:added': [component: ComponentNode]
  'component:deleted': [componentId: string]
  'component:moved': [event: { componentId: string; parentId: string | null; index: number }]
}>()

const canvas = useCanvas(props.projectId)
const dragDrop = useDragDrop()

// Canvas reference
const canvasRef = ref<HTMLElement | null>(null)

// Drop indicator position
const dropIndicatorStyle = ref<Record<string, string>>({})

// Handle drop
function handleDrop(event: DragEvent) {
  event.preventDefault()
  
  if (!event.dataTransfer) return
  
  const componentType = event.dataTransfer.getData('componentType') as ComponentType | undefined
  const droppedComponentId = event.dataTransfer.getData('componentId')
  
  const dropPos = dragDrop.getDropPosition()
  
  if (componentType) {
    // Create new component
    const newComponent = createComponent(componentType)
    canvas.addComponent(newComponent, dropPos.parentId, dropPos.index)
    emit('component:added', newComponent)
  } else if (droppedComponentId) {
    // Move existing component
    canvas.moveComponent(droppedComponentId, dropPos.parentId, dropPos.index)
    emit('component:moved', {
      componentId: droppedComponentId,
      parentId: dropPos.parentId,
      index: dropPos.index
    })
  }
  
  dragDrop.endDrag()
}

// Handle drag over
function handleDragOver(event: DragEvent) {
  event.preventDefault()
  if (event.dataTransfer) {
    event.dataTransfer.dropEffect = 'copy'
  }
  
  dragDrop.handleDragMove(event as unknown as MouseEvent)
  
  // Update drop indicator
  const dropPos = dragDrop.dropPosition.value
  if (dropPos.rect) {
    dropIndicatorStyle.value = {
      top: `${dropPos.rect.top}px`,
      left: `${dropPos.rect.left}px`,
      width: `${dropPos.rect.width}px`,
      height: '2px'
    }
  }
}

// Handle drag leave
function handleDragLeave() {
  dropIndicatorStyle.value = {}
}

// Create component
function createComponent(type: ComponentType): ComponentNode {
  const id = `comp-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`
  
  return {
    id,
    type,
    props: getDefaultProps(type),
    styles: {
      desktop: getDefaultStyles(type)
    },
    children: [],
    parentId: null
  }
}

// Default props per component type
function getDefaultProps(type: ComponentType): Record<string, unknown> {
  switch (type) {
    case 'Text':
      return { content: 'Click to edit text', tag: 'p' }
    case 'Button':
      return { label: 'Click me', variant: 'primary' }
    case 'Image':
      return { src: '', alt: 'Image' }
    case 'Hero':
      return { heading: 'Hero Title', subheading: 'Hero subtitle', ctaText: 'Get Started' }
    case 'Header':
      return { logo: '', menuItems: [] }
    case 'Footer':
      return { copyright: '© 2024 Your Company', links: [] }
    case 'Video':
      return { src: '', autoplay: false }
    case 'Divider':
      return { style: 'solid' }
    default:
      return {}
  }
}

// Default styles per component type
function getDefaultStyles(type: ComponentType): Record<string, string | number> {
  switch (type) {
    case 'Text':
      return { fontSize: '16px', color: '#333333', lineHeight: '1.5' }
    case 'Button':
      return { padding: '12px 24px', borderRadius: '8px', backgroundColor: '#3b82f6', color: '#ffffff' }
    case 'Image':
      return { width: '100%', height: 'auto', objectFit: 'cover' }
    case 'Hero':
      return { minHeight: '400px', display: 'flex', flexDirection: 'column', alignItems: 'center', justifyContent: 'center', padding: '40px', backgroundColor: '#f3f4f6' }
    case 'Container':
      return { width: '100%', minHeight: '100px', padding: '20px' }
    case 'Divider':
      return { width: '100%', height: '1px', backgroundColor: '#e5e7eb', margin: '20px 0' }
    default:
      return {}
  }
}

// Select component
function selectComponent(componentId: string | null) {
  canvas.selectComponent(componentId)
  if (componentId) {
    const component = canvas.componentTree.value.get(componentId)
    if (component) {
      emit('component:selected', component)
    }
  }
}
</script>

<template>
  <div
    ref="canvasRef"
    class="relative w-full h-full overflow-auto bg-white"
    data-drop-zone
    @drop="handleDrop"
    @dragover="handleDragOver"
    @dragleave="handleDragLeave"
  >
    <!-- Drop Indicator -->
    <div
      v-if="dragDrop.isDragging.value && Object.keys(dropIndicatorStyle).length > 0"
      class="drop-indicator"
      :style="dropIndicatorStyle"
    />
    
    <!-- Empty State -->
    <div
      v-if="canvas.componentCount.value === 0"
      class="absolute inset-0 flex items-center justify-center"
    >
      <div class="text-center p-8">
        <svg class="mx-auto w-16 h-16 text-gray-300 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" />
        </svg>
        <h3 class="text-lg font-medium text-gray-900 mb-2">Start Building</h3>
        <p class="text-sm text-gray-500">
          Drag components from the left sidebar to start creating your website
        </p>
      </div>
    </div>
    
    <!-- Component Tree -->
    <div v-else class="p-4">
      <!-- Render components -->
      <div
        v-for="rootId in canvas.rootComponentIds.value"
        :key="rootId"
        class="relative"
      >
        <!-- Component will be rendered here -->
        <div
          class="min-h-[100px] border-2 border-dashed border-gray-200 rounded-lg p-4"
          :class="{ 'ring-2 ring-primary-500': canvas.selectedComponentId.value === rootId }"
          @click.stop="selectComponent(rootId)"
        >
          <div class="text-sm text-gray-500 text-center">
            Component: {{ rootId }}
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
