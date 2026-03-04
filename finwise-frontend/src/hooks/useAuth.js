import { useState, useEffect } from 'react'
import { finwiseApi } from '../services/api'

export function useAuth() {
  const [user, setUser] = useState(null)
  const [loading, setLoading] = useState(true)

  useEffect(() => {
    finwiseApi.checkAuth()
      .then(data => setUser(data.authenticated ? data.user : null))
      .catch(() => setUser(null))
      .finally(() => setLoading(false))
  }, [])

  return { user, loading }
}