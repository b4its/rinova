export default defineNuxtPlugin(() => {
  if (import.meta.client) {
    const checkAOS = setInterval(() => {
      const AOS = (window as any).AOS
      if (AOS && typeof AOS.init === 'function') {
        AOS.init({
          duration: 800,
          easing: 'ease-in-out',
          once: true,
          offset: 100,
        })
        clearInterval(checkAOS)
      }
    }, 100)
    setTimeout(() => clearInterval(checkAOS), 5000)
  }
})
