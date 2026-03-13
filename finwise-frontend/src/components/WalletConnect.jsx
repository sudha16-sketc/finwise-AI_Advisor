/**
 * WalletConnect.jsx
 *
 * Auth + wallet connection UI for FinWise.
 *
 * Wallet logic is fully delegated to Stellar Wallets Kit via walletManager.js.
 * The kit's authModal() handles wallet selection, QR codes for WalletConnect,
 * extension detection, and session persistence — no custom code needed here.
 */

import React, { useState, useEffect } from "react";
import {
  openWalletModal,
  getConnectedAddress,
  disconnect as kitDisconnect,
  onKitEvent,
  KitEventType,
} from "../services/walletManager";
import { formatAddress } from "../pages/stellarService";
import "./WalletConnect.css";

const API = "https://finwise-backend.up.railway.app";

// ── Component ─────────────────────────────────────────────────────────────────

function WalletConnect({ publicKey, setPublicKey, isConnected, setIsConnected }) {
  // ── Auth state ──
  const [username, setUsername]           = useState("");
  const [email, setEmail]                 = useState("");
  const [password, setPassword]           = useState("");
  const [isLogin, setIsLogin]             = useState(false);
  const [isAuthenticated, setIsAuthenticated] = useState(false);

  // ── Wallet state ──
  const [walletError, setWalletError]     = useState(null);
  const [walletLoading, setWalletLoading] = useState(false);

  // ── Restore session on mount ─────────────────────────────────────────────

  useEffect(() => {
    // 1. Check backend session (email/Google auth)
    const checkBackendAuth = async () => {
      try {
        const res  = await fetch(`${API}/api/check-auth`, { credentials: "include" });
        const data = await res.json();
        if (data.authenticated) {
          setIsAuthenticated(true);
          if (data.user.wallet_address) {
            setPublicKey(data.user.wallet_address);
            setIsConnected(true);
          }
        }
      } catch (err) {
        console.error("Auth check failed:", err);
      }
    };

    // 2. Restore any existing kit wallet session (kit persists across reloads)
    const restoreWalletSession = async () => {
      const address = await getConnectedAddress();
      if (address) {
        setPublicKey(address);
        setIsConnected(true);
      }
    };

    checkBackendAuth();
    restoreWalletSession();
  }, []);

  // ── Listen to kit events (disconnect / account change) ───────────────────

  useEffect(() => {
    const unsub = onKitEvent((event) => {
      if (event.eventType === KitEventType.DISCONNECT) {
        setPublicKey(null);
        setIsConnected(false);
      }
      if (event.eventType === KitEventType.STATE_UPDATED) {
        const addr = event.payload.address;
        if (addr) {
          setPublicKey(addr);
          setIsConnected(true);
        } else {
          setPublicKey(null);
          setIsConnected(false);
        }
      }
    });
    return unsub; // clean up on unmount
  }, []);

  // ── Wallet handlers ──────────────────────────────────────────────────────

  const handleConnect = async () => {
    setWalletLoading(true);
    setWalletError(null);
    try {
      // Opens the kit's built-in modal — handles all wallets + WalletConnect QR
      const address = await openWalletModal();
      setPublicKey(address);
      setIsConnected(true);
    } catch (err) {
      setWalletError(err.message || "Failed to connect wallet");
    } finally {
      setWalletLoading(false);
    }
  };

  const handleDisconnect = () => {
    kitDisconnect();
    setPublicKey(null);
    setIsConnected(false);
  };

  // ── Auth handlers ────────────────────────────────────────────────────────

  const handleSignup = async (e) => {
    e.preventDefault();
    if (!username || !email || !password) {
      alert("Please fill in all fields!");
      return;
    }
    try {
      const res  = await fetch(`${API}/api/signup`, {
        method:      "POST",
        headers:     { "Content-Type": "application/json" },
        credentials: "include",
        body: JSON.stringify({ username, email, password, walletAddress: publicKey }),
      });
      const data = await res.json();
      if (res.ok) {
        alert("Signup successful!");
        setIsAuthenticated(true);
        setUsername(""); setEmail(""); setPassword("");
      } else {
        alert(data.message);
      }
    } catch {
      alert("Server error. Try again later.");
    }
  };

  const handleLogin = async (e) => {
    e.preventDefault();
    if (!email || !password) { alert("Please fill in all fields!"); return; }
    try {
      const res  = await fetch(`${API}/api/login`, {
        method:      "POST",
        headers:     { "Content-Type": "application/json" },
        credentials: "include",
        body: JSON.stringify({ email, password }),
      });
      const data = await res.json();
      if (res.ok) { alert("Login successful!"); setIsAuthenticated(true); }
      else          { alert(data.message); }
    } catch {
      alert("Server error. Try again later.");
    }
  };

  const handleSignout = async () => {
    try {
      await fetch(`${API}/api/logout`, {
        method: "POST", headers: { "Content-Type": "application/json" }, credentials: "include",
      });
      handleDisconnect();
      setIsAuthenticated(false);
      window.location.href = "/";
    } catch {
      alert("Logout failed. Try again later.");
    }
  };

  // ── Render ───────────────────────────────────────────────────────────────

  return (
    <div className="wallet-connect-container">

      {/* ── Auth panel ── */}
      <div className="connect-with-google">
        <div className="toggle-buttons align-center">
          <button className={!isLogin ? "active-btn" : ""} onClick={() => setIsLogin(false)}>
            Sign Up
          </button>
          <button className={isLogin ? "active-btn" : ""} onClick={() => setIsLogin(true)}>
            Login
          </button>
        </div>

        {!isLogin ? (
          <form className="form" onSubmit={handleSignup}>
            <input type="text"     placeholder="Enter username" value={username}  onChange={e => setUsername(e.target.value)}  required />
            <input type="email"    placeholder="Email"          value={email}     onChange={e => setEmail(e.target.value)}     required />
            <input type="password" placeholder="Password"       value={password}  onChange={e => setPassword(e.target.value)} required />
            <button type="submit">SIGN UP</button>
          </form>
        ) : (
          <form className="form" onSubmit={handleLogin}>
            <input type="email"    placeholder="Email"    value={email}    onChange={e => setEmail(e.target.value)}    required />
            <input type="password" placeholder="Password" value={password} onChange={e => setPassword(e.target.value)} required />
            <button type="submit">LOGIN</button>
          </form>
        )}

        <div className="alt-buttons">
          OR
          <button
            type="button"
            className="google-btn"
            onClick={() => (window.location.href = `${API}/auth/google`)}
          >
            Sign in with Google
          </button>
          {isAuthenticated && (
            <button
              className="connectbtn rounded bg-red-500 font-bold text-white hover:bg-red-600"
              onClick={handleSignout}
            >
              SIGN OUT
            </button>
          )}
        </div>
      </div>

      {/* ── Wallet panel ── */}
      <div className="coverContainer">
        <div className="wallet-connect-section">
          <h2 className="wallet-connect">Wallet Connection</h2>

          {walletError && <div className="error-message">{walletError}</div>}

          {isConnected ? (
            <div className="connected-info">
              <div className="wallet-address">
                <span className="label">Connected:</span>
                <span className="address" title={publicKey}>
                  {formatAddress(publicKey)}
                </span>
                <button
                  onClick={() => navigator.clipboard.writeText(publicKey)}
                  className="copy-button"
                  title="Copy full address"
                >
                  Copy
                </button>
              </div>
              <button onClick={handleDisconnect} className="disconnect-button">
                Disconnect
              </button>
            </div>
          ) : (
            <div className="wallet-options">
              <button
                onClick={handleConnect}
                className="connect-button"
                disabled={walletLoading}
              >
                {walletLoading ? "Opening wallet picker…" : "Connect Wallet"}
              </button>
              <p className="wallet-hint">
                Supports Freighter, Albedo, xBull, Rabet, Lobstr, WalletConnect&nbsp;(mobile), and more.
              </p>
            </div>
          )}
        </div>
      </div>

    </div>
  );
}

export default WalletConnect;