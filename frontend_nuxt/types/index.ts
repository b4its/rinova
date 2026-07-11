// Global type definitions for Rinova Website Builder

export interface ApiResponse<T = unknown> {
  data: T
  message?: string
  meta?: {
    page?: number
    limit?: number
    total?: number
  }
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
  pagination: {
    page: number
    limit: number
    total: number
    totalPages: number
  }
}

// Component types for the builder
export type ComponentType = 
  | 'Page'
  | 'Hero'
  | 'Header'
  | 'Footer'
  | 'Text'
  | 'Image'
  | 'Button'
  | 'Form'
  | 'Gallery'
  | 'Video'
  | 'Divider'
  | 'Container'

export interface ComponentStyles {
  desktop: Record<string, string | number>
  tablet?: Record<string, string | number>
  mobile?: Record<string, string | number>
}

export interface AnimationConfig {
  type: 'fade' | 'slide' | 'bounce' | 'rotate' | 'scale'
  duration: number // 100ms - 10000ms
  delay: number // 0ms - 5000ms
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

// Project types
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

// Published site types
export interface PublishedSite {
  id: string
  projectId: string
  subdomain: string
  customDomain: string | null
  sslStatus: 'pending' | 'active' | 'failed'
  publishedAt: string
}

// Notification types
export type NotificationType = 
  | 'workspace_invitation'
  | 'publish_complete'
  | 'blockchain_transaction'
  | 'subscription_update'

export interface Notification {
  id: string
  type: NotificationType
  title: string
  message: string
  read: boolean
  createdAt: string
  metadata?: Record<string, unknown>
}

// SEO types
export interface SEOMetadata {
  title: string // max 60 chars
  description: string // max 160 chars
  keywords: string[] // max 10 keywords
  ogImage?: string
  canonicalUrl?: string
}

// Breakpoint types for responsive design
export type ViewportMode = 'desktop' | 'tablet' | 'mobile'

export interface BreakpointConfig {
  mobile: number // < 768px
  tablet: number // 768px - 1024px
  desktop: number // > 1024px
}
