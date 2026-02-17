import React, { useState, useEffect } from "react";
import { fetchBalance, fetchBalanceDirect } from "../pages/stellarService";

function BalanceDisplay({ publicKey, refreshTrigger }) {
  const [balance, setBalance] = useState(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(null);

  const loadBalance = async () => {
    if (!publicKey) return;

    setLoading(true);
    setError(null);

    try {

      const balanceData = await fetchBalance(publicKey);
      setBalance(balanceData);
    } catch (apiError) {
      console.warn("Backend API failed, trying direct Stellar call:", apiError);


      try {
        const directBalance = await fetchBalanceDirect(publicKey);
        setBalance(directBalance);
      } catch (directError) {
        setError("Failed to fetch balance");
        console.error("Balance fetch error:", directError);
      }
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadBalance();
  }, [publicKey, refreshTrigger]);


  if (!publicKey) {
    return null;
  }

  return (
    <div className="balance-display-container">
      <h2>Account Balance</h2>

      <div className="balance-content">
        {loading ? (
          <div className="loading-spinner">Loading balance...</div>
        ) : error ? (
          <div className="error-message">❌ {error}</div>
        ) : (
          <div className="balance-amount">
            <span className="amount">{balance || "0"}</span>
            <span className="currency">XLM</span>
          </div>
        )}
      </div>
    </div>
  );
}

export default BalanceDisplay;
