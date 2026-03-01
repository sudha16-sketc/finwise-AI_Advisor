import React, { useState, useEffect } from "react";
import { 
  sendTransaction, 
  isValidAddress,
  fetchBalance,
  fetchBalanceDirect 
} from "../pages/stellarService";
import "../App.css";

function SendTransaction({ publicKey, onTransactionComplete, isConnected, refreshTrigger }) {
  const [destination, setDestination] = useState("");
  const [amount, setAmount] = useState("");
  const [sending, setSending] = useState(false);
  const [error, setError] = useState(null);
  const [validationError, setValidationError] = useState("");
  const [balance, setBalance] = useState(null);
  const [balanceLoading, setBalanceLoading] = useState(false);
  const [balanceError, setBalanceError] = useState(null);

  const validateDestination = (address) => {
    if (!address) {
      setValidationError("");
      return false;
    }

    if (!isValidAddress(address)) {
      setValidationError("Invalid Stellar address");
      return false;
    }

    setValidationError("");
    return true;
  };

  const handleDestinationChange = (e) => {
    const value = e.target.value;
    setDestination(value);
    validateDestination(value);
  };

  const handleAmountChange = (e) => {
    const value = e.target.value;

    if (value === "" || /^\d*\.?\d*$/.test(value)) {
      setAmount(value);
    }
  };

  const handleSubmit = async (e) => {
    e.preventDefault();

    if (!validateDestination(destination)) {
      setError("Please enter a valid destination address");
      return;
    }

    if (!amount || parseFloat(amount) <= 0) {
      setError("Please enter a valid amount");
      return;
    }

    setSending(true);
    setError(null);

    try {
      const result = await sendTransaction(publicKey, destination, amount);

      setDestination("");
      setAmount("");

      if (onTransactionComplete) {
        onTransactionComplete(result);
      }
    } catch (err) {
      setError(err.message || "Transaction failed");
      console.error("Transaction error:", err);
    } finally {
      setSending(false);
    }
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
}, [publicKey, refreshTrigger]);

  if (!publicKey) {
    return null;
  }

  return (
    <div className="send-transaction-container">
      <h2>Send XLM</h2>

      {isConnected && (
        <div className="dashboard-section wallet-info">
          <div className="wallet-address">
            <strong>Wallet:</strong> {publicKey.slice(0, 6)}...
            {publicKey.slice(-6)}
          </div>

          <button onClick={() => navigator.clipboard.writeText(publicKey)}>
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

      <form onSubmit={handleSubmit} className="transaction-form">
        <div className="form-group">
          <label htmlFor="destination">Destination Address:</label>
          <input
            id="destination"
            type="text"
            value={destination}
            onChange={handleDestinationChange}
            placeholder="G..."
            disabled={sending}
            className={validationError ? "input-error" : ""}
          />
          {validationError && (
            <span className="validation-error">{validationError}</span>
          )}
        </div>

        <div className="form-group">
          <label htmlFor="amount">Amount (XLM):</label>
          <input
            id="amount"
            type="text"
            value={amount}
            onChange={handleAmountChange}
            placeholder="0.00"
            disabled={sending}
          />
        </div>

        {error && <div className="error-message">❌ {error}</div>}

        <button
          type="submit"
          disabled={sending || !destination || !amount || !!validationError}
          className="send-button"
        >
          {sending ? "⏳ Sending..." : "📤 Send Transaction"}
        </button>
      </form>

      <div className="transaction-info">
        <small>💡 Make sure destination account exists on testnet</small>
      </div>
    </div>
  );
}

export default SendTransaction;
