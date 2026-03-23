// src/services/api.js
import axios from 'axios'

const BASE_URL = import.meta.env.VITE_API_URL || 'https://finwise-ai-advisor.onrender.com'

console.log('[API] Base URL:', BASE_URL) // Debug: confirm which URL is being used

const api = axios.create({
  baseURL: BASE_URL,
  headers: { 'Content-Type': 'application/json' },
  timeout: 30_000,
  withCredentials: true,
})

// ── Interceptors ──────────────────────────────────────────────────────────────

api.interceptors.request.use((config) => {
  console.log(`[API REQUEST] ${config.method?.toUpperCase()} ${config.baseURL}${config.url}`)
  console.log('[API REQUEST] Payload:', config.data || config.params || 'none')
  return config
})

api.interceptors.response.use(
  (res) => {
    console.log(`[API RESPONSE] ${res.status} ${res.config.url}`, res.data)
    return res
  },
  (err) => {
    console.error(`[API ERROR] ${err.config?.url}`, {
      status: err.response?.status,
      data: err.response?.data,
      message: err.message,
    })
    const message = err.response?.data?.message || err.response?.data?.error || err.message || 'Unknown error'
    throw new Error(message)
  }
)

// ── API Methods ───────────────────────────────────────────────────────────────

export const finwiseApi = {

  // Auth
  signup: async ({ username, email, password, walletAddress }) => {
    const { data } = await api.post('/api/signup', { username, email, password, walletAddress })
    return data
  },

  login: async ({ email, password }) => {
    const { data } = await api.post('/api/login', { email, password })
    return data
  },

  logout: async () => {
    const { data } = await api.post('/api/logout')
    return data
  },

  checkAuth: async () => {
    const { data } = await api.get('/api/check-auth')
    return data
  },

  // Wallet / Stellar
  getBalance: async (address) => {
    const { data } = await api.get(`/api/balance/${address}`)
    return data
  },

  getTransactions: async (address) => {
    const { data } = await api.get(`/api/transactions/${address}`)
    return data
  },

  sendTransaction: async (xdr) => {
    const { data } = await api.post('/api/send', { xdr })
    return data
  },

  // Features
  analyze: async (payload) => {
    const { data } = await api.post('/api/analyze', payload)
    return data
  },

  getProfile: async () => {
    const { data } = await api.get('/api/profile')
    return data
  },

  getMetrics: async () => {
    const { data } = await api.get('/api/metrics')
    return data
  },

  // Health
  health: async () => {
    const { data } = await api.get('/health')
    return data
  },
}

export const API_BASE = BASE_URL  // export for use in WalletConnect.jsx & stellarService.js