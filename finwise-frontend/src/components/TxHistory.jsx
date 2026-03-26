import React, { useState, useEffect } from "react";
import {
  fetchTransactions,
  fetchTransactionsDirect,
} from "../pages/stellarService";
import "../App.css";
function TxHistory({ publicKey, refreshTrigger }) {
  const [transactions, setTransactions] = useState([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(null);

  const loadTransactions = async () => {
    if (!publicKey) return;

    setLoading(true);
    setError(null);

    try {
      const txData = await fetchTransactions(publicKey);
      setTransactions(Array.isArray(txData) ? txData : []);
    } catch (apiError) {
      console.warn("Backend API failed, trying direct Stellar call:", apiError);

      try {
        const directTx = await fetchTransactionsDirect(publicKey, 10);
        setTransactions(Array.isArray(directTx) ? directTx : []);
      } catch (directError) {
        setTransactions([]);
        setError("Failed to load transactions");
        console.error("Transaction history error:", directError);
      }
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadTransactions();
  }, [publicKey, refreshTrigger]);

  const formatDate = (dateString) => {
    const date = new Date(dateString);
    return date.toLocaleString();
  };

  const formatHash = (hash) => {
    return `${hash.slice(0, 8)}...${hash.slice(-8)}`;
  };

  const getExplorerUrl = (hash) => {
    return `https://stellar.expert/explorer/testnet/tx/${hash}`;
  };

  if (!publicKey) {
    return null;
  }

  return (
    <div className="transaction-history-container">

      {loading ? (
        <div className="loading-spinner">Loading transactions...</div>
      ) : error ? (
        <div className="error-message"> {error}</div>
      ) : !transactions || transactions.length === 0 ? (
        <div className="no-transactions">
          No transactions found for this account
        </div>
      ) : (
        <div className="tx-table-wrapper">
          <table className="tx-table">
            <thead>
              <tr>
                <th></th>
                <th>INVOICE ID</th>
                <th>HASH</th>
                <th>DATE</th>
                <th>FEE</th>
                <th>STATUS</th>
                <th></th>
              </tr>
            </thead>

            <tbody>
              {transactions.map((tx, index) => (
                <tr key={tx.id || index}>
                  <td>
                    <input type="checkbox" />
                  </td>

                  <td className="id-cell">ID: {index + 10000}</td>

                  <td className="hash-cell" title={tx.hash}>
                    {formatHash(tx.hash)}
                  </td>

                  <td>{formatDate(tx.created_at)}</td>

                  <td>${(tx.fee_charged / 10000000).toFixed(5)}</td>

                  <td>
                    <span
                      className={`status-pill ${tx.successful ? "success" : "failed"}`}
                    >
                      {tx.successful ? "Completed" : "Failed"}
                    </span>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>

          
        </div>
      )}
    </div>
  );
}

export default TxHistory;
