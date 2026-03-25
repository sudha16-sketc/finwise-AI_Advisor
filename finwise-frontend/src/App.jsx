import React, { useState, useEffect } from "react";
import { BrowserRouter, Routes, Route } from "react-router-dom";
import Navbar from "./components/Navbar";
import Hero from "./pages/Hero";
import Analyze from "./pages/Analyze";
import Dashboard from "./pages/Dashboard";
import PiggyBank from "./pages/PiggyBank";
import Signin from "./pages/Signin";
import About from "./pages/About";
import SendTransaction from "./components/SendTransaction";
import TxHistory from "./components/TxHistory";
import { getConnectedAddress, onKitEvent, KitEventType } from "./services/walletManager";
import { API_BASE } from "./services/api";

export default function App() {
  const [publicKey, setPublicKey] = useState(
    () => localStorage.getItem("publicKey") || null,
  );
  const [isConnected, setIsConnected] = useState(
    () => !!localStorage.getItem("publicKey"),
  );
  const [isAuthenticated, setIsAuthenticated] = useState(false); // ← ADD THIS
  const [walletLoading, setWalletLoading] = useState(true);
  const [historyRefreshTrigger, setHistoryRefreshTrigger] = useState(0);
  const [transactionStatus, setTransactionStatus] = useState(null);
  const [transactionData, setTransactionData] = useState(null);
  const [balanceRefreshTrigger, setBalanceRefreshTrigger] = useState(0);


  useEffect(() => {
    fetch(`${API_BASE}/api/check-auth`, { credentials: "include" })
      .then((r) => r.json())
      .then((data) => {
        if (data.authenticated) {
          setIsAuthenticated(true);
          if (data.user?.wallet_address) {
            syncWalletState(data.user.wallet_address);
          }
        }
      })
      .catch(() => {});
  }, []);

  // ── Restore wallet session from kit ─────────────────────────────────────
  useEffect(() => {
    let cancelled = false;

    getConnectedAddress()
      .then((address) => {
        if (cancelled) return;
        if (address) {
          syncWalletState(address);
        } else {
          const stored = localStorage.getItem("publicKey");
          if (!stored) {
            clearWalletState();
          }
        }
      })
      .catch(() => {
        if (!cancelled) {
          const stored = localStorage.getItem("publicKey");
          if (!stored) clearWalletState();
        }
      })
      .finally(() => {
        if (!cancelled) setWalletLoading(false);
      });

    const unsub = onKitEvent((event) => {
      if (event.eventType === KitEventType.STATE_UPDATED) {
        const addr = event.payload?.address;
        if (addr) {
          syncWalletState(addr);
        } else {
          clearWalletState();
        }
      }
      if (event.eventType === KitEventType.DISCONNECT) {
        clearWalletState();
      }
    });

    return () => {
      cancelled = true;
      unsub();
    };
  }, []);

  const syncWalletState = (address) => {
    setPublicKey(address);
    setIsConnected(true);
    localStorage.setItem("publicKey", address);
  };

  const clearWalletState = () => {
    setPublicKey(null);
    setIsConnected(false);
    localStorage.removeItem("publicKey");
  };

  const handleSetPublicKey = (key) => {
    if (key) {
      syncWalletState(key);
    } else {
      clearWalletState();
    }
  };

  const handleSetIsConnected = (val) => {
    if (!val) clearWalletState();
    else setIsConnected(true);
  };

  const handleTransactionComplete = (result) => {
    setTransactionStatus("success");
    setTransactionData({
      hash: result.hash,
      ledger: result.ledger,
      fee: result.fee_charged,
    });
    setBalanceRefreshTrigger((prev) => prev + 1);
    setHistoryRefreshTrigger((prev) => prev + 1);
    setTimeout(handleCloseStatus, 5000);
  };

  const handleCloseStatus = () => {
    setTransactionStatus(null);
    setTransactionData(null);
  };

  return (
    <BrowserRouter>
      <div className="min-h-screen flex flex-col">
        <Navbar />
        <main className="flex-1">
          <Routes>
            <Route
              path="/"
              element={
                <>
                  <Hero />
                  <About />
                </>
              }
            />
            <Route path="/analyze" element={<Analyze />} />
            <Route
              path="/dashboard"
              element={
                <Dashboard
                  publicKey={publicKey}
                  isConnected={isConnected}
                  walletLoading={walletLoading}
                />
              }
            />
            <Route path="/piggy" element={<PiggyBank />} />
            <Route
              path="/sendtransaction"
              element={
                <SendTransaction
                  publicKey={publicKey}
                  isConnected={isConnected}
                  onTransactionComplete={handleTransactionComplete}
                  refreshTrigger={balanceRefreshTrigger}
                  lastTx={transactionData?.hash}
                  fee="0.00001 XLM"
                  network="online"
                  status={
                    transactionStatus
                      ? { type: transactionStatus, message: "Transaction successful" }
                      : null
                  }
                />
              }
            />
            <Route
              path="/txhistory"
              element={
                <TxHistory
                  publicKey={publicKey}
                  refreshTrigger={historyRefreshTrigger}
                />
              }
            />
            <Route
              path="/signin"
              element={
                <Signin
                  publicKey={publicKey}
                  setPublicKey={handleSetPublicKey}
                  isConnected={isConnected}
                  setIsConnected={handleSetIsConnected}
                  isAuthenticated={isAuthenticated}        
                  setIsAuthenticated={setIsAuthenticated}  
                />
              }
            />
          </Routes>
        </main>
      </div>
    </BrowserRouter>
  );
}