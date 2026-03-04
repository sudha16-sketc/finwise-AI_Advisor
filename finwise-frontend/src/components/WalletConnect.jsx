import React, { useState, useEffect } from "react";
import {
  connectWallet,
  isFreighterInstalled,
  formatAddress,
} from "../pages/stellarService";
import "./WalletConnect.css";
import { connectWithWallet, wallets } from "../services/walletManager";


function WalletConnect({
  publicKey,
  setPublicKey,
  isConnected,
  setIsConnected,
}) {
  const [error, setError] = useState(null);
  const [loading, setLoading] = useState(false);
  const [freighterInstalled, setFreighterInstalled] = useState(false);
  const [username, setUsername] = useState("");
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [isLogin, setIsLogin] = useState(false);
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [status, setStatus] = useState("");

  useEffect(() => {
    const checkFreighter = async () => {
      const installed = await isFreighterInstalled();
      setFreighterInstalled(installed);
    };

    checkFreighter();

    const interval = setInterval(checkFreighter, 1000);
    return () => clearInterval(interval);
  }, []);

  const handleConnect = async () => {
    setLoading(true);
    setError(null);

    try {
      const pubKey = await connectWallet();
      setPublicKey(pubKey);
      setIsConnected(true);
    } catch (err) {
      setError(err.message || "Failed to connect wallet");
      setIsConnected(false);
    } finally {
      setLoading(false);
    }
  };

  const handleConnectOther = async (walletId) => {
    setLoading(true);
    setError(null);

    try {
      const pubKey = await connectWithWallet(walletId);
      setPublicKey(pubKey);
      setIsConnected(true);
    } catch (err) {
      setError(err.message || "Failed to connect wallet");
      setIsConnected(false);
    } finally {
      setLoading(false);
    }
  };

  const handleDisconnect = () => {
    setPublicKey(null);
    setIsConnected(false);
    localStorage.removeItem("publicKey");
  };
  const handleSignup = async (e) => {
    e.preventDefault();

    if (!username || !email || !password) {
      alert("Please fill in all fields!");
      return;
    }

    try {
      const response = await fetch("http://localhost:8080/api/signup", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        credentials: "include",
        body: JSON.stringify({
          username,
          email,
          password,
          walletAddress: publicKey,
        }),
      });

      const data = await response.json();
      if (response.ok) {
        setStatus("Signup successful!");
        alert("Signup successful!");
        setIsAuthenticated(true);
        setUsername("");
        setEmail("");
        setPassword("");
      } else {
        setStatus(data.message);
        alert(data.message);
      }
    } catch (err) {
      console.error(err);
      setStatus("Server error. Try again later.");
    }
  };

  // Login
  const handleLogin = async (e) => {
    e.preventDefault();

    if (!email || !password) {
      alert("Please fill in all fields!");
      return;
    }

    try {
      const response = await fetch("http://localhost:8080/api/login", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        credentials: "include",
        body: JSON.stringify({ email, password }),
      });

      const data = await response.json();
      if (response.ok) {
        alert("Login successful!");
        setIsAuthenticated(true);
      } else {
        alert(data.message);
      }
    } catch (err) {
      console.error(err);
      alert("Server error. Try again later.");
    }
  };

  // Logout
  const handleSignout = async () => {
    try {
      await fetch("http://localhost:8080/api/logout", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        credentials: "include",
      });

      alert("Logout successful!");
      setPublicKey(null);
      setIsConnected(false);
      localStorage.removeItem("walletAddress");
      setIsAuthenticated(false);
      window.location.href = "/";
    } catch (err) {
      console.error(err);
      alert("Logout failed. Try again later.");
    }
  };

  useEffect(() => {
    const checkAuth = async () => {
      try {
        const res = await fetch("http://localhost:8080/api/check-auth", {
          credentials: "include",
        });
        const data = await res.json();

        if (data.authenticated) {
          setIsAuthenticated(true);
          setPublicKey(data.user.wallet_address || null);
        }
      } catch (err) {
        console.error("Auth check failed:", err);
      }
    };

    checkAuth();
  }, []);

  return (
    <div className="wallet-connect-container">
      <div className="connect-with-google">
        <div className="toggle-buttons align-center ">
          <button
            className={!isLogin ? "active-btn" : ""}
            onClick={() => setIsLogin(false)}
          >
            Sign Up
          </button>
          <button
            className={isLogin ? "active-btn" : ""}
            onClick={() => setIsLogin(true)}
          >
            Login
          </button>
        </div>

        {!isLogin ? (
          <form className="form" onSubmit={handleSignup}>
            <input
              type="text"
              placeholder="Enter username"
              value={username}
              onChange={(e) => setUsername(e.target.value)}
              required
            />
            <input
              type="email"
              placeholder="Email"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              required
            />
            <input
              type="password"
              placeholder="Password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              required
            />
            <button type="submit">SIGN UP</button>
          </form>
        ) : (
          <form className="form" onSubmit={handleLogin}>
            <input
              type="email"
              placeholder="Email"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              required
            />
            <input
              type="password"
              placeholder="Password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              required
            />
            <button type="submit">LOGIN</button>
          </form>
        )}

        <div className="alt-buttons">
          OR
          <button
            type="button"
            className="google-btn"
            onClick={() =>
              (window.location.href = "http://localhost:8080/auth/google")
            }
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
      <div className="coverContainer">
        
        <div className="wallet-connect-section">
          <h2 className="wallet-connect">Wallet Connection</h2>

          {!freighterInstalled && (
            <div className="warning-message">
              Freighter wallet not detected. Please install{" "}
              <a
                href="https://freighter.app/"
                target="_blank"
                rel="noopener noreferrer"
              >
                Freighter
              </a>
            </div>
          )}

          {error && <div className="error-message">{error}</div>}

          {isConnected ? (
            // Show connected info ONCE regardless of which wallet connected
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
            // Show all connect options ONCE when not connected
            <div className="wallet-options">
              <button onClick={handleConnect} className="connect-button">
                Connect Freighter
              </button>
              {wallets.map((wallet) => (
                <button
                  key={wallet.id}
                  onClick={() => handleConnectOther(wallet.id)}
                  className="connect-button"
                >
                  Connect {wallet.name}
                </button>
              ))}
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

export default WalletConnect;
