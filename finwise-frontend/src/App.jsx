import React, { useState } from "react";
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

export default function App() {
  const [publicKey, setPublicKey] = useState(null);
  const [isConnected, setIsConnected] = useState(false);
  const [historyRefreshTrigger, setHistoryRefreshTrigger] = useState(0);
  const [transactionStatus, setTransactionStatus] = useState(null);
const [transactionData, setTransactionData] = useState(null);
const [balanceRefreshTrigger, setBalanceRefreshTrigger] = useState(0);  

  const handleTransactionComplete = (result) => {
    console.log("Transaction completed:", result);

    setTransactionStatus("success");
    setTransactionData({
      hash: result.hash,
      ledger: result.ledger,
      fee: result.fee_charged,
    });

    setBalanceRefreshTrigger((prev) => prev + 1);
    setHistoryRefreshTrigger((prev) => prev + 1);

    setTimeout(() => {
      handleCloseStatus();
    }, 5000);
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
            <Route path="/dashboard" element={<Dashboard />} />
            <Route path="/piggy" element={<PiggyBank />} />
            <Route
              path="/sendtransaction"
              element={
<SendTransaction
  publicKey={publicKey}
  isConnected={isConnected}
  onTransactionComplete={handleTransactionComplete}
  refreshTrigger={balanceRefreshTrigger}
/>
              }
            />
            <Route path="/txhistory" element={<TxHistory
                  publicKey={publicKey}
                  refreshTrigger={historyRefreshTrigger}
                />} />
            <Route
              path="/signin"
              element={
                <Signin
                  publicKey={publicKey}
                  setPublicKey={setPublicKey}
                  isConnected={isConnected}
                  setIsConnected={setIsConnected}
                />
              }
            />
          </Routes>
        </main>
      </div>
    </BrowserRouter>
  );
}
