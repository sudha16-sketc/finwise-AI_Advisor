/**
 * WalletConnect.jsx
 *
 * Auth + wallet connection UI for FinWise.
 *
 * WHY createButton() instead of authModal():
 *   authModal() is a programmatic call that opens the modal immediately.
 *   In React, calling it from a button's onClick works fine for extension
 *   wallets, but WalletConnect's QR rendering relies on the kit's Web
 *   Component (<swk-app-modal>) already being live in the DOM at the moment
 *   the modal opens. In some React render cycles, the component hydrates
 *   after the call, causing the modal to appear empty or not at all.
 *
 *   StellarWalletsKit.createButton(ref) injects the kit's own <swk-button>
 *   Web Component into a container div. This button manages its own click
 *   → modal → QR flow entirely inside the kit's component tree, completely
 *   sidestepping the React timing issue. The kit's STATE_UPDATED / DISCONNECT
 *   events still fire normally so React state stays in sync.
 *
 *   We keep our own "Connect Wallet" button as a visible label, and hide the
 *   kit's button visually — clicking our button programmatically clicks the
 *   kit's button, triggering the kit's native flow.
 */

import React, { useState, useEffect, useRef } from "react";
import {
  StellarWalletsKit,
  getConnectedAddress,
  disconnect as kitDisconnect,
  onKitEvent,
  KitEventType,
} from "../services/walletManager";
import { formatAddress } from "../pages/stellarService";
import "./WalletConnect.css";

import { API_BASE as API } from "../services/api";

function WalletConnect({ publicKey, setPublicKey, isConnected, setIsConnected }) {
  // ── Auth state ──────────────────────────────────────────────────────────
  const [username, setUsername]               = useState("");
  const [email, setEmail]                     = useState("");
  const [password, setPassword]               = useState("");
  const [isLogin, setIsLogin]                 = useState(false);
  const [isAuthenticated, setIsAuthenticated] = useState(false);

  // ── Wallet state ────────────────────────────────────────────────────────
  const [walletError, setWalletError]         = useState(null);

  // Ref for the hidden div where the kit mounts its <swk-button>
  const kitButtonRef = useRef(null);

  // ── Mount kit button into its container div ──────────────────────────────
  //
  // This runs once after the component mounts. At this point the DOM node
  // referenced by kitButtonRef is guaranteed to exist, so the kit can safely
  // inject its Web Component into it.
  //
  useEffect(() => {
    if (!kitButtonRef.current) return;
    StellarWalletsKit.createButton(kitButtonRef.current);
  }, []);

  // ── Restore session on mount ─────────────────────────────────────────────
  useEffect(() => {
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

  // ── Listen to kit events ─────────────────────────────────────────────────
  useEffect(() => {
    const unsub = onKitEvent((event) => {
      if (event.eventType === KitEventType.DISCONNECT) {
        setPublicKey(null);
        setIsConnected(false);
      }
      if (event.eventType === KitEventType.STATE_UPDATED) {
        const addr = event.payload?.address;
        if (addr) {
          setPublicKey(addr);
          setIsConnected(true);
          setWalletError(null);
        } else {
          setPublicKey(null);
          setIsConnected(false);
        }
      }
    });
    return unsub;
  }, []);

  // ── Wallet handlers ──────────────────────────────────────────────────────

  // Clicking our styled button programmatically clicks the hidden kit button.
  // The kit then opens its own modal (with WalletConnect QR if configured).
  const handleConnect = () => {
    setWalletError(null);
    const kitBtn = kitButtonRef.current?.querySelector("swk-button");
    if (kitBtn) {
      kitBtn.click();
    } else {
      // Fallback: open modal directly (works for non-WC wallets)
      StellarWalletsKit.authModal().then(({ address }) => {
        if (address) {
          setPublicKey(address);
          setIsConnected(true);
        }
      }).catch((err) => setWalletError(err.message));
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
    if (!username || !email || !password) { alert("Please fill in all fields!"); return; }
    try {
      const res  = await fetch(`${API}/api/signup`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
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
    } catch { alert("Server error. Try again later."); }
  };

  const handleLogin = async (e) => {
    e.preventDefault();
    if (!email || !password) { alert("Please fill in all fields!"); return; }
    try {
      const res  = await fetch(`${API}/api/login`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        credentials: "include",
        body: JSON.stringify({ email, password }),
      });
      const data = await res.json();
      if (res.ok) { alert("Login successful!"); setIsAuthenticated(true); }
      else         { alert(data.message); }
    } catch { alert("Server error. Try again later."); }
  };

  const handleSignout = async () => {
    try {
      await fetch(`${API}/api/logout`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        credentials: "include",
      });
      handleDisconnect();
      setIsAuthenticated(false);
      window.location.href = "/";
    } catch { alert("Logout failed. Try again later."); }
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

          {/*
            Hidden container where the kit mounts its <swk-button> Web Component.
            It must be in the DOM at all times (not conditionally rendered) so
            the kit's component is always registered and clickable.
            We hide it visually — our own button triggers it via .click().
          */}
          <div ref={kitButtonRef} style={{ display: "none" }} />

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
              <button onClick={handleConnect} className="connect-button">
                Connect Wallet
              </button>
              <p className="wallet-hint">
                Supports Freighter, Albedo, xBull, Lobstr, WalletConnect&nbsp;(mobile QR), and more.
              </p>
            </div>
          )}
        </div>
      </div>

    </div>
  );
}

export default WalletConnect;