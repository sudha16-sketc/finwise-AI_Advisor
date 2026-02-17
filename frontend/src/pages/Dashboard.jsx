import React, { useState } from "react";

import SendTransaction from "../components/SendTransaction";
import TransactionHistory from "../pages/TxHistory";
import "./Dashboard.css";
import { fetchBalance, fetchBalanceDirect } from "../pages/stellarService";
import { useEffect } from "react";

function Dashboard({ publicKey, isConnected }) {

  const [transactionStatus, setTransactionStatus] = useState(null);
  const [transactionData, setTransactionData] = useState(null);

  const [balanceRefreshTrigger, setBalanceRefreshTrigger] = useState(0);
  const [historyRefreshTrigger, setHistoryRefreshTrigger] = useState(0);
  const [activeSection, setActiveSection] = useState("send");
  const [balance, setBalance] = useState(null);
  const [balanceLoading, setBalanceLoading] = useState(false);
  const [balanceError, setBalanceError] = useState(null);

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
  const loadBalance = async () => {
    if (!publicKey) return;

    setBalanceLoading(true);
    setBalanceError(null);

    try {
      const balanceData = await fetchBalance(publicKey);
      setBalance(balanceData);
    } catch (apiError) {
      console.warn("Backend failed, using direct Stellar:", apiError);

      try {
        const directBalance = await fetchBalanceDirect(publicKey);
        setBalance(directBalance);
      } catch (directError) {
        setBalanceError("Failed to fetch balance");
        console.error(directError);
      }
    } finally {
      setBalanceLoading(false);
    }
  };
  useEffect(() => {
    loadBalance();
  }, [publicKey, balanceRefreshTrigger]);

  const handleCloseStatus = () => {
    setTransactionStatus(null);
    setTransactionData(null);
  };

  return (
    <div className="dashboard-container">
      <div className="dashboard-grid">
        <div className="dashboard-main">
          <div className="dashboard-navbar">

            <div className="profile-menu">
              <button className="dots-btn">⋮</button>
            </div>

            <div className="profile-circle"></div>

            <h1 className="profile-name">xyz</h1>
            <p className="profile-tagline"></p>

            <ul className="nav-list">
              <li onClick={() => setActiveSection("send")}>
                Send a Transaction
              </li>
              <li onClick={() => setActiveSection("history")}>
                View Transaction history
              </li>

              <li onClick={() => setActiveSection("piggy")}>
                View your piggy bank
              </li>
            </ul>
          </div>

          <div className="dashboard-upbar">
            {isConnected && (
              <div className="dashboard-section wallet-info">
                
                <div className="wallet-address">
                  <strong>Wallet:</strong> {publicKey.slice(0, 6)}...
                  {publicKey.slice(-6)}
                </div>

                <button
                  onClick={() => navigator.clipboard.writeText(publicKey)}
                >
                  Copy
                </button>

               
                <div className="balance-display-container">
                  <h3>Balance</h3>

                  {balanceLoading ? (
                    <div>Loading balance...</div>
                  ) : balanceError ? (
                    <div> {balanceError}</div>
                  ) : (
                    <div className="balance-amount">
                      <span className="amount">{balance || "0"}</span>
                      <span className="currency"> XLM</span>
                    </div>
                  )}
                </div>
              </div>
            )}
          </div>

          <div className="dashboard-content">
            {isConnected && activeSection === "send" && (
              <div className="dashboard-section">
                <SendTransaction
                  publicKey={publicKey}
                  onTransactionComplete={handleTransactionComplete}
                />
              </div>
            )}

            

            {isConnected && activeSection === "history" && (
              <div className="dashboard-section full-width">
                <TransactionHistory
                  publicKey={publicKey}
                  refreshTrigger={historyRefreshTrigger}
                />
              </div>
            )}

            {activeSection === "piggy" && (
              <div className="dashboard-section">
                <h2>Piggy bank coming soon </h2>
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}

export default Dashboard;
