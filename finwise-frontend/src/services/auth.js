const TOKEN_KEY = 'finwise_token'

export const authStorage = {
  setToken: (token) => localStorage.setItem(TOKEN_KEY, token),
  getToken: () => localStorage.getItem(TOKEN_KEY),
  removeToken: () => localStorage.removeItem(TOKEN_KEY),
  isLoggedIn: () => !!localStorage.getItem(TOKEN_KEY),
}