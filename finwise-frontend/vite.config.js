import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

export default defineConfig({
  plugins: [react()],
  resolve: {
    dedupe: ['@stellar/stellar-sdk']
  },
  server: {
    port: 3000,
    proxy: {
      '/api': {
        target: 'https://finwise-ai-advisor.onrender.com',
        changeOrigin: true,
      }
    }
  }
})