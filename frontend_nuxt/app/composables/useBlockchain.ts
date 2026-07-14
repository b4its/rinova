/**
 * Blockchain API helpers (blockchain_service via gateway).
 *
 * Subscription transactions are recorded on-chain; this exposes their hashes
 * and stored data for the explorer page.
 */

export interface SubscriptionRecord {
  userId: string
  subscriptionId: string
  planType: string
  action: string
  amountCents: number
  currency: string
  paymentReference: string | null
  contentHash: string
  txHash: string
  blockNumber: number
  timestamp: number
}

interface BackendSubscriptionRecord {
  user_id: string
  subscription_id: string
  plan_type: string
  action: string
  amount_cents: number
  currency: string
  payment_reference: string | null
  content_hash: string
  tx_hash: string
  block_number: number
  timestamp: number
}

interface HistoryResponse {
  user_id: string
  records: BackendSubscriptionRecord[]
  total: number
}

function mapRecord(r: BackendSubscriptionRecord): SubscriptionRecord {
  return {
    userId: r.user_id,
    subscriptionId: r.subscription_id,
    planType: r.plan_type,
    action: r.action,
    amountCents: r.amount_cents,
    currency: r.currency,
    paymentReference: r.payment_reference,
    contentHash: r.content_hash,
    txHash: r.tx_hash,
    blockNumber: r.block_number,
    timestamp: r.timestamp,
  }
}

export function useBlockchain() {
  const api = useApi()

  async function getSubscriptionHistory(userId: string): Promise<SubscriptionRecord[]> {
    const res = await api.get<HistoryResponse>(`/blockchain/subscription/${userId}`)
    return (res.records ?? []).map(mapRecord)
  }

  return { getSubscriptionHistory }
}
