// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: '2024-11-01',
  devtools: { enabled: true },
  
  future: {
    compatibilityVersion: 4
  },

  typescript: {
    strict: true,
    typeCheck: true
  },

  modules: [
    '@pinia/nuxt',
    '@nuxtjs/tailwindcss'
  ],

  pinia: {
    storesDirs: ['./stores/**']
  },

  tailwindcss: {
    cssPath: '~/assets/css/tailwind.css',
    configPath: 'tailwind.config.js',
    exposeConfig: false,
    viewer: true
  },

  app: {
    head: {
      title: 'Rinova Website Builder',
      meta: [
        { name: 'description', content: 'Professional website builder with drag & drop editor' }
      ]
    }
  }
})
