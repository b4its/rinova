export default defineNuxtConfig({
  compatibilityDate: '2025-07-15',
  devtools: { enabled: true },
  modules: ['@nuxtjs/tailwindcss', '@pinia/nuxt'],
  tailwindcss: {
    exposeConfig: true
  },
  typescript: {
    strict: true
  },
  runtimeConfig: {
    jwtSecret: process.env.JWT_SECRET || 'your-secret-key',
    // Server-only: Redis connection for the landing-page static cache.
    redisUrl: process.env.REDIS_URL || 'redis://:rinova_redis_secret@localhost:6379',
    public: {
      apiUrl: process.env.NUXT_PUBLIC_API_URL || 'http://localhost:8080',
      wsUrl: process.env.NUXT_PUBLIC_WS_URL || 'ws://localhost:8080'
    }
  },
  app: {
    head: {
      title: 'Rinova - Professional Website Builder',
      meta: [
        { name: 'description', content: 'Create professional websites with drag & drop editor.' },
        { name: 'viewport', content: 'width=device-width, initial-scale=1' }
      ],
      link: [
        { rel: 'stylesheet', href: 'https://unpkg.com/aos@2.3.1/dist/aos.css' }
      ],
      script: [
        { src: 'https://unpkg.com/aos@2.3.1/dist/aos.js', defer: true }
      ]
    }
  },
  ssr: true,
  vite: {
    server: {
      port: 3000,
      host: '0.0.0.0'
    }
  }
})
