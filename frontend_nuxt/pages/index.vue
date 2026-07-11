<script setup lang="ts">
definePageMeta({
  layout: 'default'
})
</script>

<template>
  <div class="min-h-screen flex flex-col items-center justify-center bg-gradient-to-br from-primary-50 to-primary-100">
    <div class="text-center">
      <h1 class="text-4xl font-bold text-gray-900 mb-4">
        Welcome to Rinova
      </h1>
      <p class="text-lg text-gray-600 mb-8">
        Professional Website Builder with Drag & Drop Editor
      </p>
      <div class="flex gap-4 justify-center">
        <NuxtLink 
          to="/login" 
          class="btn btn-primary"
        >
          Get Started
        </NuxtLink>
        <NuxtLink 
          to="/register" 
          class="btn btn-secondary"
        >
          Create Account
        </NuxtLink>
      </div>
    </div>
  </div>
</template>
