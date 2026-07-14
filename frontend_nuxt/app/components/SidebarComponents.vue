<script setup lang="ts">
import type { ComponentType } from '~/types'

const emit = defineEmits<{
  'add:component': [type: ComponentType]
}>()

const categories = [
  {
    name: '',
    items: [
      { type: 'Container' as ComponentType, name: 'Container', desc: 'Flexible wrapper for custom content' },
    ]
  },
  {
    name: 'Layout',
    items: [
      { type: 'Header' as ComponentType, name: 'Header', desc: 'Navigation bar with logo and menu' },
      { type: 'Footer' as ComponentType, name: 'Footer', desc: 'Footer with links and copyright' },
    ]
  },
  {
    name: 'Sections',
    items: [
      { type: 'Hero' as ComponentType, name: 'Hero', desc: 'Full-width banner with headline and CTA' },
      { type: 'Features' as ComponentType, name: 'Features', desc: 'Grid of feature columns' },
      { type: 'Testimonials' as ComponentType, name: 'Testimonials', desc: 'Customer testimonials' },
      { type: 'Pricing' as ComponentType, name: 'Pricing', desc: 'Pricing table with plans' },
      { type: 'FAQ' as ComponentType, name: 'FAQ', desc: 'Frequently asked questions' },
      { type: 'Contact' as ComponentType, name: 'Contact', desc: 'Contact form and information' },
      { type: 'Stats' as ComponentType, name: 'Stats', desc: 'Statistics and achievements' },
    ]
  },
  {
    name: 'Content',
    items: [
      { type: 'Text' as ComponentType, name: 'Text', desc: 'Paragraph or heading text block' },
      { type: 'Image' as ComponentType, name: 'Image', desc: 'Image with caption' },
      { type: 'Button' as ComponentType, name: 'Button', desc: 'Call-to-action button' },
      { type: 'Divider' as ComponentType, name: 'Divider', desc: 'Horizontal separator' },
      { type: 'Link' as ComponentType, name: 'Link', desc: 'Hyperlink with custom URL' },
      { type: 'Icon' as ComponentType, name: 'Icon', desc: 'Font Awesome or Tailwind icon' },
      { type: 'Form' as ComponentType, name: 'Form', desc: 'Multi-field contact form' },
    ]
  },
  {
    name: 'Form Inputs',
    items: [
      { type: 'Input' as ComponentType, name: 'Input', desc: 'Single-line text input' },
      { type: 'Textarea' as ComponentType, name: 'Textarea', desc: 'Multi-line text input' },
      { type: 'Number' as ComponentType, name: 'Number', desc: 'Numeric input with min/max' },
      { type: 'Select' as ComponentType, name: 'Select', desc: 'Dropdown option selector' },
      { type: 'Checkbox' as ComponentType, name: 'Checkbox', desc: 'Boolean toggle input' },
      { type: 'Radio' as ComponentType, name: 'Radio', desc: 'Single-select option group' },
      { type: 'Range' as ComponentType, name: 'Range', desc: 'Slider range input' },
      { type: 'Date' as ComponentType, name: 'Date', desc: 'Date picker input' },
    ]
  }
]

const icons: Record<string, string> = {
  Header: 'M4 6h16M4 10h16M4 14h8m-8 4h16',
  Footer: 'M4 6h16M12 10h8M12 14h8M4 18h16',
  Hero: 'M5 3v4M3 5h4M6 17v4m-2-2h4m5-16l2.286 6.857L21 12l-5.714 2.143L13 21l-2.286-6.857L5 12l5.714-2.143L13 3z',
  Features: 'M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zM14 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zM14 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z',
  Testimonials: 'M8 10h.01M12 10h.01M16 10h.01M9 16H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-5l-5 5v-5z',
  Pricing: 'M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z',
  FAQ: 'M8 10h.01M12 10h.01M16 10h.01M9 16H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-5l-5 5v-5z',
  Contact: 'M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z',
  Stats: 'M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z',
  Text: 'M4 6h16M4 12h16m-7 6h7',
  Image: 'M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z',
  Button: 'M15 15l-2 5L9 9l11 4-5 2zm0 0l5 5M7.188 2.239l.777 2.897M5.136 7.965l-2.898-.777M13.95 4.05l-2.122 2.122m-5.657 5.656l-2.12 2.122',
  Form: 'M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z',
  Divider: 'M20 12H4',
  Input: 'M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z',
  Textarea: 'M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z',
  Number: 'M7 7h3M7 12h3m-3 5h3m5-13l-4 4h3l-4 4h3l-4 4',
  Select: 'M8 9l4-4 4 4m0 6l-4 4-4-4',
  Checkbox: 'M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z',
  Radio: 'M12 8a4 4 0 100 8 4 4 0 000-8zm0 10a6 6 0 100-12 6 6 0 000 12z',
  Range: 'M5 12h14M12 5l7 7-7 7M5 5l7 7-7 7',
  Date: 'M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z',
  Container: 'M4 5h16v14H4V5zm2 2v10h12V7H6zm2 8h8v1H8v-1zm0-3h8v1H8v-1zm0-3h8v1H8V9z',
  Link: 'M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14',
  Icon: 'M12 2l2.4 7.2L22 9l-6 4.8L17.6 22 12 17.6 6.4 22 8 13.8 2 9l7.6-.2L12 2z',
}
</script>

<template>
  <div class="h-full flex flex-col bg-card overflow-y-auto">
    <div v-for="cat in categories" :key="cat.name" class="py-3">
      <h4 v-if="cat.name" class="text-xs font-semibold text-gray-400 uppercase tracking-wider mb-2 px-3">{{ cat.name }}</h4>
      <div class="space-y-0.5 px-2">
        <button
          v-for="comp in cat.items"
          :key="comp.type"
          class="sidebar-btn w-full flex items-center gap-2.5 px-2.5 py-2 rounded-lg hover:bg-accent transition-all text-left group"
          @click="emit('add:component', comp.type)"
        >
          <div class="w-7 h-7 rounded-md bg-muted flex items-center justify-center text-gray-500 group-hover:bg-primary-100 dark:group-hover:bg-primary-900/40 group-hover:text-primary-600 dark:group-hover:text-primary-400 transition-colors shrink-0">
            <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" :d="icons[comp.type] || 'M12 4v16m8-8H4'" /></svg>
          </div>
          <div class="min-w-0">
            <p class="text-sm font-medium text-foreground leading-tight">{{ comp.name }}</p>
            <p class="text-xs text-muted-foreground truncate leading-tight">{{ comp.desc }}</p>
          </div>
        </button>
      </div>
    </div>
  </div>
</template>
