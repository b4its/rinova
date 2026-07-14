/**
 * GET /api/landing
 *
 * Serves static landing-page content via a Redis read-through cache.
 * If Redis is unavailable, degrades gracefully to the in-code source.
 */
import { useRedis, LANDING_CACHE_KEY } from '../utils/redis'
import { buildLandingContent } from '../utils/landingContent'

export default defineEventHandler(async () => {
  try {
    const redis = useRedis()
    const cached = await redis.get(LANDING_CACHE_KEY)
    if (cached) {
      return { source: 'cache', ...JSON.parse(cached) }
    }

    // Cache miss: build, store, return. TTL is a safety net; the build-time
    // plugin is the primary (re)population mechanism.
    const content = buildLandingContent()
    await redis.set(LANDING_CACHE_KEY, JSON.stringify(content), 'EX', 60 * 60 * 24)
    return { source: 'origin', ...content }
  } catch {
    // Redis down -> serve from source so the landing page still renders.
    return { source: 'fallback', ...buildLandingContent() }
  }
})
