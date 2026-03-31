
import axios from 'axios'

const BASE_URL = import.meta.env.VITE_API_URL || 'https://finwise-ai-advisor.onrender.com'
console.log('[API] Base URL:', BASE_URL)

// Token store for localStorage
export const tokenStore = {
  get: () => localStorage.getItem('auth_token'),
  set: (token) => localStorage.setItem('auth_token', token),
  remove: () => localStorage.removeItem('auth_token'),
  isLoggedIn: () => !!localStorage.getItem('auth_token'),
}

const api = axios.create({
  baseURL: BASE_URL,
  headers: { 'Content-Type': 'application/json' },
  timeout: 30_000,
  withCredentials: true, // still send cookies for Google OAuth
})

// Attach Bearer token if present
api.interceptors.request.use((config) => {
  const token = tokenStore.get();
  if (token) {
    config.headers = config.headers || {};
    config.headers['Authorization'] = `Bearer ${token}`;
  }
  return config;
})

api.interceptors.response.use(
  (res) => res,
  (err) => {
    if (err.response?.status === 401) {
      const path = window.location.pathname;
      // Only redirect if not already on login/signup/auth page
      if (!/login|signup|auth/i.test(path)) {
        tokenStore.remove();
        window.location.href = '/login';
      }
    }
    const message =
      err.response?.data?.message ||
      err.response?.data?.error ||
      err.message ||
      'Unknown error';
    throw new Error(message);
  }
)

export const finwiseApi = {
  // Auth
  signup: async ({ username, email, password, walletAddress }) => {
    const { data } = await api.post('/api/signup', { username, email, password, walletAddress });
    if (data.token) tokenStore.set(data.token);
    return data;
  },

  login: async ({ email, password }) => {
    const { data } = await api.post('/api/login', { email, password });
    if (data.token) tokenStore.set(data.token);
    return data;
  },

  logout: async () => {
    const { data } = await api.post('/api/logout');
    tokenStore.remove();
    return data;
  },

  checkAuth: async () => {
    const { data } = await api.get('/api/check-auth');
    if (data.token) tokenStore.set(data.token);
    return data;
  },

  // Wallet / Stellar
  getBalance: async (address) => {
    const { data } = await api.get(`/api/balance/${address}`);
    return data;
  },

  getTransactions: async (address) => {
    const { data } = await api.get(`/api/transactions/${address}`);
    return data;
  },

  sendTransaction: async (xdr) => {
    const { data } = await api.post('/api/send', { xdr });
    return data;
  },

  // Features
  analyze: async (payload) => {
    const { data } = await api.post('/api/analyze', payload);
    return data;
  },

  getProfile: async () => {
    const { data } = await api.get('/api/profile');
    return data;
  },

  getMetrics: async () => {
    const { data } = await api.get('/api/metrics');
    return data;
  },

  health: async () => {
    const { data } = await api.get('/health');
    return data;
  },
}

export const API_BASE = BASE_URL