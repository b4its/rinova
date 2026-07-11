<script setup lang="ts">
definePageMeta({
  layout: 'editor'
})

const route = useRoute()
const projectId = computed(() => route.params.id as string)
</script>

<template>
  <div class="h-full">
    <!-- Canvas is rendered by the editor layout -->
  </div>
</template>
