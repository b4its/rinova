/**
 * Server-only Redis client (singleton).
 *
 * Used exclusively for caching STATIC landing-page content. Dynamic data
 * (users, projects, subscriptions) is never cached here.
 */
import Redis from 'ioredis'

let client: Redis | null = null

export function useRedis(): Redis {
  if (client) return client

  const { redisUrl } = useRuntimeConfig()
  client = new Redis(redisUrl, {
    lazyConnect: false,
    maxRetriesPerRequest: 2,
    // Don't spam reconnects if Redis is down; landing falls back to source.
    retryStrategy: (times) => (times > 3 ? null : Math.min(times * 200, 1000)),
  })

  client.on('error', (err) => {
    // Keep this quiet-ish; the landing route degrades gracefully.
    console.warn('[redis] connection error:', err.message)
  })

  return client
}

/** Cache key namespace for landing content. */
export const LANDING_CACHE_KEY = 'rinova:landing:content:v1'
