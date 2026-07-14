<script setup lang="ts">
import { useBlockchain, type SubscriptionRecord } from '~/composables/useBlockchain'

definePageMeta({ layout: 'panel', sidebarSection: 'blockchain' })

const userStore = useUserStore()
const { getSubscriptionHistory } = useBlockchain()

const records = ref<SubscriptionRecord[]>([])
const isLoading = ref(true)
const loadError = ref<string | null>(null)
const selected = ref<SubscriptionRecord | null>(null)

async function load() {
  if (!userStore.user) return
  isLoading.value = true
  loadError.value = null
  try {
    records.value = await getSubscriptionHistory(userStore.user.id)
  } catch (e) {
    loadError.value = e instanceof Error ? e.message : 'Failed to load blockchain records'
  } finally {
    isLoading.value = false
  }
}

onMounted(load)

function formatDate(ms: number): string {
  return new Date(ms).toLocaleString()
}

function formatAmount(cents: number, currency: string): string {
  if (cents === 0) return 'Free'
  return new Intl.NumberFormat('en-US', { style: 'currency', currency: currency.toUpperCase() }).format(cents / 100)
}

function shortHash(hash: string): string {
  if (hash.length <= 18) return hash
  return `${hash.slice(0, 10)}…${hash.slice(-8)}`
}

async function copyHash(hash: string) {
  if (import.meta.client && navigator.clipboard) {
    await navigator.clipboard.writeText(hash)
  }
}

const actionColor: Record<string, string> = {
  upgrade: 'bg-emerald-100 text-emerald-700 dark:bg-emerald-900/40 dark:text-emerald-400 border-emerald-200 dark:border-emerald-800',
  downgrade: 'bg-blue-100 text-blue-700 dark:bg-blue-900/40 dark:text-blue-400 border-blue-200 dark:border-blue-800',
  cancel: 'bg-red-100 text-red-700 dark:bg-red-900/40 dark:text-red-400 border-red-200 dark:border-red-800',
  create: 'bg-teal-100 text-teal-700 dark:bg-teal-900/40 dark:text-teal-400 border-teal-200 dark:border-teal-800',
}
</script>

<template>
  <div class="space-y-6">
    <div>
      <h1 class="text-2xl font-bold tracking-tight">Blockchain Explorer</h1>
      <p class="text-muted-foreground text-sm mt-1">
        Subscription transactions recorded immutably on the Hyperledger Besu chain. Verify each transaction hash and its stored data.
      </p>
    </div>

    <div v-if="isLoading" class="card">
      <div class="card-body flex items-center justify-center py-12">
        <svg class="animate-spin h-6 w-6 text-primary" fill="none" viewBox="0 0 24 24">
          <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
          <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
        </svg>
      </div>
    </div>

    <div v-else-if="loadError" class="rounded-lg border border-red-200 bg-red-50 dark:border-red-900 dark:bg-red-950/50 px-4 py-3 text-sm text-red-700 dark:text-red-400">
      {{ loadError }}
    </div>

    <div v-else-if="records.length === 0" class="card">
      <div class="card-body text-center py-12">
        <div class="w-12 h-12 rounded-full bg-gradient-to-br from-blue-500/10 to-emerald-500/10 flex items-center justify-center mx-auto mb-3">
          <svg class="w-6 h-6 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
          </svg>
        </div>
        <h3 class="text-sm font-medium">No blockchain records yet</h3>
        <p class="text-sm text-muted-foreground mt-1">
          Your subscription transactions will appear here once you upgrade or change your plan.
        </p>
        <NuxtLink to="/panel/subscription" class="btn btn-primary btn-sm mt-4">View Plans</NuxtLink>
      </div>
    </div>

    <div v-else class="grid gap-4 lg:grid-cols-2">
      <div
        v-for="record in records"
        :key="record.txHash"
        class="card p-5 cursor-pointer hover:shadow-md transition-all"
        :class="selected?.txHash === record.txHash ? 'ring-2 ring-primary' : ''"
        @click="selected = record"
      >
        <div class="flex items-center justify-between mb-3">
          <span class="badge text-[11px] capitalize" :class="actionColor[record.action] ?? 'border'">
            {{ record.action }} → {{ record.planType }}
          </span>
          <span class="text-xs text-muted-foreground">Block #{{ record.blockNumber }}</span>
        </div>
        <div class="space-y-2 text-sm">
          <div class="flex items-center justify-between gap-2">
            <span class="text-muted-foreground">Tx Hash</span>
            <button class="font-mono text-xs text-primary hover:underline" @click.stop="copyHash(record.txHash)">
              {{ shortHash(record.txHash) }}
            </button>
          </div>
          <div class="flex items-center justify-between">
            <span class="text-muted-foreground">Amount</span>
            <span class="font-medium">{{ formatAmount(record.amountCents, record.currency) }}</span>
          </div>
          <div class="flex items-center justify-between">
            <span class="text-muted-foreground">Recorded</span>
            <span>{{ formatDate(record.timestamp) }}</span>
          </div>
        </div>
      </div>
    </div>

    <!-- Detail drawer -->
    <div v-if="selected" class="modal-overlay" @click.self="selected = null">
      <div class="modal max-w-lg">
        <div class="modal-header">
          <h3 class="text-lg font-semibold">Transaction Detail</h3>
          <button class="text-muted-foreground hover:text-foreground" @click="selected = null">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>
        <div class="modal-body space-y-3 text-sm">
          <div>
            <div class="text-muted-foreground text-xs mb-1">Transaction Hash</div>
            <div class="font-mono text-xs break-all bg-muted rounded-lg p-2">{{ selected.txHash }}</div>
          </div>
          <div>
            <div class="text-muted-foreground text-xs mb-1">Content Hash (SHA-256)</div>
            <div class="font-mono text-xs break-all bg-muted rounded-lg p-2">{{ selected.contentHash }}</div>
          </div>
          <div class="grid grid-cols-2 gap-3">
            <div>
              <div class="text-muted-foreground text-xs">Action</div>
              <div class="font-medium capitalize">{{ selected.action }}</div>
            </div>
            <div>
              <div class="text-muted-foreground text-xs">Plan</div>
              <div class="font-medium capitalize">{{ selected.planType }}</div>
            </div>
            <div>
              <div class="text-muted-foreground text-xs">Amount</div>
              <div class="font-medium">{{ formatAmount(selected.amountCents, selected.currency) }}</div>
            </div>
            <div>
              <div class="text-muted-foreground text-xs">Block Number</div>
              <div class="font-medium">#{{ selected.blockNumber }}</div>
            </div>
            <div>
              <div class="text-muted-foreground text-xs">Subscription ID</div>
              <div class="font-mono text-xs break-all">{{ selected.subscriptionId }}</div>
            </div>
            <div>
              <div class="text-muted-foreground text-xs">Timestamp</div>
              <div class="font-medium">{{ formatDate(selected.timestamp) }}</div>
            </div>
          </div>
        </div>
        <div class="modal-footer">
          <button class="btn btn-secondary btn-sm" @click="copyHash(selected.txHash)">Copy Tx Hash</button>
          <button class="btn btn-primary btn-sm" @click="selected = null">Close</button>
        </div>
      </div>
    </div>
  </div>
</template>
