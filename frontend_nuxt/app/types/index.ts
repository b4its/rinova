export interface ApiResponse<T = unknown> {
  data: T
  message?: string
  meta?: { page?: number; limit?: number; total?: number }
}

export interface ApiError {
  code: string
  message: string
  details?: Record<string, unknown>
  requestId: string
  timestamp: string
}

export interface PaginatedResponse<T> {
  items: T[]
  pagination: { page: number; limit: number; total: number; totalPages: number }
}

export type ComponentType =
  | 'Page' | 'Hero' | 'Header' | 'Footer' | 'Text'
  | 'Image' | 'Button' | 'Form' | 'Gallery' | 'Video' | 'Divider' | 'Container'
  | 'Features' | 'Testimonials' | 'Pricing' | 'FAQ' | 'Contact' | 'Stats'
  | 'Input' | 'Textarea' | 'Number' | 'Select' | 'Checkbox' | 'Radio' | 'Range' | 'Date'
  | 'Link' | 'Icon'

export interface ComponentStyles {
  desktop: Record<string, string | number>
  tablet?: Record<string, string | number>
  mobile?: Record<string, string | number>
}

export interface AnimationConfig {
  type: 'fade' | 'slide' | 'bounce' | 'rotate' | 'scale'
  duration: number
  delay: number
  easing: 'linear' | 'ease-in' | 'ease-out' | 'ease-in-out'
  iterationCount: number
  trigger: 'load' | 'scroll' | 'hover' | 'click'
}

export interface ComponentNode {
  id: string
  type: ComponentType
  props: Record<string, unknown>
  styles: ComponentStyles
  animations?: AnimationConfig[]
  children: string[]
  parentId: string | null
}

export interface Project {
  id: string
  workspaceId: string
  ownerId: string
  name: string
  status: 'draft' | 'published' | 'archived'
  metadata: Record<string, unknown>
  lastPublishedAt: string | null
  createdAt: string
  updatedAt: string
}

export interface PublishedSite {
  id: string
  projectId: string
  subdomain: string
  customDomain: string | null
  sslStatus: 'pending' | 'active' | 'failed'
  publishedAt: string
}

export type NotificationType =
  | 'workspace_invitation' | 'publish_complete'
  | 'blockchain_transaction' | 'subscription_update'

export interface Notification {
  id: string
  type: NotificationType
  title: string
  message: string
  read: boolean
  createdAt: string
  metadata?: Record<string, unknown>
}

export interface SEOMetadata {
  title: string
  description: string
  keywords: string[]
  ogImage?: string
  canonicalUrl?: string
}

export type ViewportMode = 'desktop' | 'tablet' | 'mobile'

export interface BreakpointConfig {
  mobile: number
  tablet: number
  desktop: number
}
