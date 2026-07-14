/**
 * Nitro startup plugin: refresh the landing cache.
 *
 * On every server start (i.e. after a build/deploy), CLEAR the existing
 * landing cache in Redis, then REPOPULATE it with freshly built static
 * content. This satisfies the "rebuild -> flush + reseed" requirement while
 * keeping only static data in Redis.
 */
import { useRedis, LANDING_CACHE_KEY } from '../utils/redis'
import { buildLandingContent } from '../utils/landingContent'

export default defineNitroPlugin(async () => {
  try {
    const redis = useRedis()
    // 1. Clear stale cache.
    await redis.del(LANDING_CACHE_KEY)
    // 2. Repopulate with fresh static content.
    const content = buildLandingContent()
    await redis.set(LANDING_CACHE_KEY, JSON.stringify(content))
    console.log('[landing-cache] flushed and reseeded on startup')
  } catch (err) {
    console.warn('[landing-cache] skipped reseed (redis unavailable):', (err as Error).message)
  }
})
