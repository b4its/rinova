/**
 * GET /health
 *
 * Lightweight liveness probe used by the Docker healthcheck.
 * Does not touch Redis or the backend so it stays fast and reliable.
 */
export default defineEventHandler(() => {
  return { status: 'ok', service: 'frontend', timestamp: new Date().toISOString() }
})
