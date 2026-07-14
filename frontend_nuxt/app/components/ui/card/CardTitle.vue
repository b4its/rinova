<script setup lang="ts">
import { cn } from '~/lib/utils'
interface Props { class?: string }
const props = defineProps<Props>()
</script>
<template>
  <div :class="cn('font-semibold leading-none tracking-tight', props.class)">
    <slot />
  </div>
</template>
