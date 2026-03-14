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

export default function App() {
  // Seed from localStorage so desktop users who previously connected
  // don't see a flash of "disconnected" on reload.
  // But we also re-check the kit session below so mobile/WalletConnect
  // sessions are restored correctly even when localStorage is stale.
  const [publicKey, setPublicKey] = useState(
    () => localStorage.getItem("publicKey") || null,
  );
  const [isConnected, setIsConnected] = useState(
    () => !!localStorage.getItem("publicKey"),
  );
  const [historyRefreshTrigger, setHistoryRefreshTrigger] = useState(0);
  const [transactionStatus, setTransactionStatus] = useState(null);
  const [transactionData, setTransactionData] = useState(null);
  const [balanceRefreshTrigger, setBalanceRefreshTrigger] = useState(0);

  // ── Sync wallet state from the kit ──────────────────────────────────────
  //
  // This is the core fix for mobile. The Stellar Wallets Kit manages its own
  // session (especially WalletConnect), and does NOT write to localStorage.
  // So App must:
  //   1. Ask the kit for the current address on mount (covers page reload)
  //   2. Listen to kit events for connect / disconnect during the session
  //
  useEffect(() => {
    // 1. Restore kit session on mount (async — WalletConnect takes ~1-2s on mobile)
    getConnectedAddress().then((address) => {
      if (address) {
        syncWalletState(address);
      } else {
        // Kit has no session — clear any stale localStorage from a previous visit
        clearWalletState();
      }
    });

    // 2. Listen for future connect / disconnect events from the kit modal
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

    return unsub;
  }, []);

  // Centralised helpers so localStorage and React state always stay in sync
  const syncWalletState = (address) => {
    setPublicKey(address);
    setIsConnected(true);
    localStorage.setItem("publicKey", address); // keep for fast desktop reload
  };

  const clearWalletState = () => {
    setPublicKey(null);
    setIsConnected(false);
    localStorage.removeItem("publicKey");
  };

  // ── Used by Signin page (email/password login restores wallet address) ───
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

  // ── Transaction callbacks ────────────────────────────────────────────────
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
                <Dashboard publicKey={publicKey} isConnected={isConnected} />
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
                />
              }
            />
          </Routes>
        </main>
      </div>
    </BrowserRouter>
  );
}