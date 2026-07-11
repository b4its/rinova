// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: '2025-07-15',
  devtools: { enabled: true },

  modules: [
    '@pinia/nuxt',
    '@nuxtjs/tailwindcss'
  ],

  typescript: {
    strict: true
  },

  runtimeConfig: {
    // Server-side only
    jwtSecret: process.env.JWT_SECRET || 'your-secret-key',
    
    // Public runtime config (exposed to client)
    public: {
      apiUrl: process.env.NUXT_PUBLIC_API_URL || 'http://localhost:8080',
      wsUrl: process.env.NUXT_PUBLIC_WS_URL || 'ws://localhost:8080'
    }
  },

  app: {
    head: {
      title: 'Rinova - Professional Website Builder',
      meta: [
        { name: 'description', content: 'Create professional websites with drag & drop editor. High performance, blockchain-powered proof of ownership.' },
        { name: 'viewport', content: 'width=device-width, initial-scale=1' }
      ],
      link: [
        { rel: 'icon', type: 'image/x-icon', href: '/favicon.ico' }
      ]
    }
  },

  pinia: {
    storesDirs: ['./stores/**']
  },

  // Enable SSR
  ssr: true,

  // Vite configuration
  vite: {
    server: {
      port: 3000,
      host: '0.0.0.0'
    }
  }
})
