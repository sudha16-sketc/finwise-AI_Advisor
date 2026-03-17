import axios from 'axios'

const BASE_URL ='https://finwise-aiadvisor-production.up.railway.app'

const api = axios.create({
  baseURL: BASE_URL,
  headers: { 'Content-Type': 'application/json' },
  timeout: 30_000,
  withCredentials: true,   
})

// Log every request in dev
api.interceptors.request.use((config) => {
  console.log(`[API] ${config.method?.toUpperCase()} ${config.url}`)
  return config
})

// Normalize errors to plain Error with a message string
api.interceptors.response.use(
  (res) => res,
  (err) => {
    const message = err.response?.data?.error || err.message || 'An unknown error occurred'
    throw new Error(message)
  }
)

export const finwiseApi = {
  /** POST /api/analyze — AI financial analysis */
  analyze: async (payload) => {
    const { data } = await api.post('/api/analyze', payload)
    return data
  },

  /** POST /api/piggy/deposit */
  deposit: async (payload) => {
    const { data } = await api.post('/api/piggy/deposit', payload)
    return data
  },

  /** GET /api/piggy/stats/:userId */
  getPiggyStats: async (userId) => {
    const { data } = await api.get(`/api/piggy/stats/${userId}`)
    return data
  },

  /** GET /health */
  health: async () => {
    const { data } = await api.get('/health')
    return data
  },

  /** GET /api/profile */
  getProfile: async () => {
    const { data } = await api.get('/api/profile')
    return data
  },

  /** GET /api/check-auth */   
  checkAuth: async () => {
    const { data } = await api.get('/api/check-auth')
    return data
  },
}