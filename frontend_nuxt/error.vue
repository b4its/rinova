<script setup lang="ts">
import type { NuxtError } from '#app'

const props = defineProps<{
  error: NuxtError
}>()

const handleError = () => clearError({ redirect: '/' })
</script>

<template>
  <div class="min-h-screen flex items-center justify-center bg-gradient-to-br from-primary-50 to-primary-100">
    <div class="max-w-md w-full text-center px-4">
      <h1 class="text-9xl font-bold text-primary-600 mb-4">
        {{ error.statusCode }}
      </h1>
      <h2 class="text-2xl font-bold text-gray-900 mb-4">
        {{ error.statusCode === 404 ? 'Page Not Found' : 'Something Went Wrong' }}
      </h2>
      <p class="text-gray-600 mb-8">
        {{ error.statusCode === 404 
          ? "The page you're looking for doesn't exist or has been moved." 
          : 'An unexpected error occurred. Please try again later.' }}
      </p>
      <div class="flex gap-4 justify-center">
        <button
          class="btn btn-primary"
          @click="handleError"
        >
          Go Home
        </button>
        <button
          class="btn btn-secondary"
          @click="$router.back()"
        >
          Go Back
        </button>
      </div>
    </div>
  </div>
</template>
