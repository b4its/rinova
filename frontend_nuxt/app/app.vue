<script setup lang="ts">
const themeStore = useThemeStore()
onMounted(() => themeStore.load())
useHead({
  link: [
    { rel: 'stylesheet', href: 'https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.5.2/css/all.min.css', crossorigin: 'anonymous' }
  ]
})
</script>

<template>
  <div>
    <NuxtLayout>
      <NuxtPage />
    </NuxtLayout>
  </div>
</template>

<style>
:root {
  --background: 0 0% 100%;
  --foreground: 240 10% 3.9%;
  --card: 0 0% 100%;
  --card-foreground: 240 10% 3.9%;
  --popover: 0 0% 100%;
  --popover-foreground: 240 10% 3.9%;
  --primary: 221 83% 53%;
  --primary-foreground: 0 0% 100%;
  --secondary: 160 40% 96%;
  --secondary-foreground: 165 60% 20%;
  --muted: 210 20% 96%;
  --muted-foreground: 215 16% 47%;
  --accent: 160 84% 39%;
  --accent-foreground: 0 0% 100%;
  --destructive: 0 84% 60%;
  --destructive-foreground: 0 0% 98%;
  --border: 214 20% 90%;
  --input: 214 20% 90%;
  --ring: 221 83% 53%;
  --radius: 0.5rem;
  --sidebar-background: 0 0% 98%;
  --sidebar-foreground: 240 5% 26%;
  --sidebar-accent: 240 5% 96%;
  --sidebar-accent-foreground: 240 6% 10%;
  --sidebar-border: 220 13% 91%;
  --sidebar-primary: 240 6% 10%;
  --sidebar-primary-foreground: 0 0% 98%;
  --sidebar-ring: 217 91% 60%;
}
.dark {
  --background: 240 10% 3.9%;
  --foreground: 0 0% 98%;
  --card: 240 10% 3.9%;
  --card-foreground: 0 0% 98%;
  --popover: 240 10% 3.9%;
  --popover-foreground: 0 0% 98%;
  --primary: 217 91% 60%;
  --primary-foreground: 0 0% 100%;
  --secondary: 200 12% 16%;
  --secondary-foreground: 0 0% 98%;
  --muted: 200 12% 16%;
  --muted-foreground: 215 15% 65%;
  --accent: 160 84% 39%;
  --accent-foreground: 0 0% 100%;
  --destructive: 0 63% 31%;
  --destructive-foreground: 0 0% 98%;
  --border: 200 12% 18%;
  --input: 200 12% 18%;
  --ring: 217 91% 60%;
  --sidebar-background: 240 6% 10%;
  --sidebar-foreground: 240 5% 96%;
  --sidebar-accent: 240 4% 16%;
  --sidebar-accent-foreground: 240 5% 96%;
  --sidebar-border: 240 4% 16%;
  --sidebar-primary: 224 76% 48%;
  --sidebar-primary-foreground: 0 0% 100%;
  --sidebar-ring: 217 91% 60%;
}

.card {
  background-color: hsl(var(--card));
  color: hsl(var(--card-foreground));
  border-radius: var(--radius);
  border: 1px solid hsl(var(--border));
  box-shadow: 0 1px 2px 0 rgb(0 0 0 / 0.05);
}
.card-body {
  padding: 1.5rem;
}

.label {
  display: block;
  font-size: 0.875rem;
  font-weight: 500;
  color: hsl(var(--foreground));
  margin-bottom: 0.5rem;
}
.input {
  display: block;
  width: 100%;
  border-radius: calc(var(--radius) - 2px);
  border: 1px solid hsl(var(--input));
  background-color: hsl(var(--background));
  padding: 0.5rem 0.75rem;
  font-size: 0.875rem;
  color: hsl(var(--foreground));
  transition: border-color 0.15s ease;
}
.input:focus {
  outline: none;
  border-color: hsl(var(--ring));
  box-shadow: 0 0 0 2px hsl(var(--ring) / 0.2);
}
.input-error {
  border-color: hsl(var(--destructive));
}
.form-error {
  font-size: 0.75rem;
  color: hsl(var(--destructive));
  margin-top: 0.25rem;
}

.btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  border-radius: calc(var(--radius) - 2px);
  font-size: 0.875rem;
  font-weight: 500;
  padding: 0.5rem 1rem;
  transition: all 0.15s ease;
  cursor: pointer;
  border: 1px solid transparent;
}
.btn-sm {
  padding: 0.375rem 0.75rem;
  font-size: 0.75rem;
}
.btn-primary {
  background-color: hsl(var(--primary));
  color: hsl(var(--primary-foreground));
}
.btn-primary:hover {
  opacity: 0.9;
}
.btn-secondary {
  background-color: hsl(var(--secondary));
  color: hsl(var(--secondary-foreground));
}
.btn-secondary:hover {
  background-color: hsl(var(--accent));
}
.btn-outline {
  border-color: hsl(var(--border));
  background-color: hsl(var(--background));
  color: hsl(var(--foreground));
}
.btn-outline:hover {
  background-color: hsl(var(--accent));
}

.badge {
  display: inline-flex;
  align-items: center;
  border-radius: 9999px;
  padding: 0.125rem 0.625rem;
  font-size: 0.75rem;
  font-weight: 500;
  border: 1px solid hsl(var(--border));
}

.dropdown {
  position: absolute;
  z-index: 50;
  min-width: 12rem;
  border-radius: var(--radius);
  border: 1px solid hsl(var(--border));
  background-color: hsl(var(--popover));
  color: hsl(var(--popover-foreground));
  box-shadow: 0 10px 15px -3px rgb(0 0 0 / 0.1);
  padding: 0.25rem;
}
.dropdown-item {
  display: flex;
  align-items: center;
  width: 100%;
  padding: 0.375rem 0.5rem;
  font-size: 0.875rem;
  border-radius: calc(var(--radius) - 2px);
  cursor: pointer;
  background: none;
  border: none;
  text-align: left;
  color: inherit;
}
.dropdown-item:hover {
  background-color: hsl(var(--accent));
  color: hsl(var(--accent-foreground));
}

.separator {
  height: 1px;
  background-color: hsl(var(--border));
  margin: 0.25rem 0;
}

.section-title {
  font-size: 1.25rem;
  font-weight: 600;
  color: hsl(var(--foreground));
}
.section-subtitle {
  font-size: 0.875rem;
  color: hsl(var(--muted-foreground));
  max-width: 36rem;
  margin-left: auto;
  margin-right: auto;
}

.modal-overlay {
  position: fixed;
  inset: 0;
  z-index: 50;
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: rgb(0 0 0 / 0.5);
  padding: 1rem;
}
.modal {
  position: relative;
  width: 100%;
  max-width: 32rem;
  border-radius: var(--radius);
  border: 1px solid hsl(var(--border));
  background-color: hsl(var(--background));
  box-shadow: 0 20px 25px -5px rgb(0 0 0 / 0.1);
}
.modal-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 1rem 1.5rem;
  border-bottom: 1px solid hsl(var(--border));
}
.modal-body {
  padding: 1.5rem;
}
.modal-footer {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: 0.5rem;
  padding: 1rem 1.5rem;
  border-top: 1px solid hsl(var(--border));
}
</style>
