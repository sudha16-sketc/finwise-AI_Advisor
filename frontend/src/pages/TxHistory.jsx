import React, { useState, useEffect } from 'react';
import { fetchTransactions, fetchTransactionsDirect } from './stellarService';

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
      console.warn('Backend API failed, trying direct Stellar call:', apiError);

      try {
        const directTx = await fetchTransactionsDirect(publicKey, 10);
        setTransactions(Array.isArray(directTx) ? directTx : []);

      } catch (directError) {
        setTransactions([]);

        console.error('Transaction history error:', directError);
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
      <div className="history-header">
        <h2>Transaction History</h2>
        <button 
          onClick={loadTransactions} 
          disabled={loading}
          className="refresh-button-small"
        >
          {loading ? 'Refreshing...' : 'Refresh'}
        </button>
      </div>

      {loading ? (
        <div className="loading-spinner">Loading transactions...</div>
      ) : error ? (
        <div className="error-message"> {error}</div>
      ) : !transactions || transactions.length === 0? (
        <div className="no-transactions">
          No transactions found for this account
        </div>
      ) : (
        <div className="transactions-list">
          {transactions.map((tx, index) => (
            <div key={tx.id || index} className="transaction-item">
              <div className="tx-header">
                <span className="tx-status">
                  {tx.successful ? 'Sucess' : 'Failed'}
                </span>
                <span className="tx-hash" title={tx.hash}>
                  {formatHash(tx.hash)}
                </span>
              </div>
              
              <div className="tx-details">
                <div className="tx-detail">
                  <span className="label">Date:</span>
                  <span className="value">{formatDate(tx.created_at)}</span>
                </div>
                
                <div className="tx-detail">
                  <span className="label">Operations:</span>
                  <span className="value">{tx.operation_count}</span>
                </div>
                
                <div className="tx-detail">
                  <span className="label">Fee:</span>
                  <span className="value">{tx.fee_charged} stroops</span>
                </div>
              </div>

              <a 
                href={getExplorerUrl(tx.hash)}
                target="_blank"
                rel="noopener noreferrer"
                className="explorer-link-small"
              >
                View Details →
              </a>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}

export default TxHistory;