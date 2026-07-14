<script setup lang="ts">
/**
 * Alias untuk link verifikasi email.
 *
 * Email verifikasi dari backend menautkan ke /auth/verify-email?token=...,
 * sementara halaman verifikasi sebenarnya ada di /verify-email. Halaman ini
 * meneruskan (redirect) sambil mempertahankan query token.
 */
definePageMeta({ layout: false })

const route = useRoute()

// Redirect sedini mungkin (server-side saat SSR) sambil membawa query token.
await navigateTo({ path: '/verify-email', query: route.query }, { redirectCode: 302 })
</script>

<template>
  <div />
</template>
