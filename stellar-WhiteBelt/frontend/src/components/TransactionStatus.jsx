import React from 'react';

function TransactionStatus({ status, transactionData, onClose }) {
  if (!status) {
    return null;
  }
  const getStatusIcon = () => {
    switch (status) {
      case 'pending':
        return 'pending...';
      case 'success':
        return 'Sucess!...';
      case 'failure':
        return 'Failed!...';
      default:
        return 'ℹworking...';
    }
  };

  const getStatusMessage = () => {
    switch (status) {
      case 'pending':
        return 'Transaction is being processed...';
      case 'success':
        return 'Transaction successful!';
      case 'failure':
        return 'Transaction failed';
      default:
        return 'Unknown status';
    }
  };


  const getExplorerUrl = (hash) => {
    return `https://stellar.expert/explorer/testnet/tx/${hash}`;
  };

  return (
    <div className={`transaction-status status-${status}`}>
      <div className="status-header">
        <span className="status-icon">{getStatusIcon()}</span>
        <h3>{getStatusMessage()}</h3>
      </div>

      {transactionData && (
        <div className="status-details">
          {transactionData.hash && (
            <div className="detail-row">
              <span className="label">Transaction Hash:</span>
              <span className="value hash">
                {transactionData.hash.slice(0, 8)}...{transactionData.hash.slice(-8)}
              </span>
            </div>
          )}

          {transactionData.ledger && (
            <div className="detail-row">
              <span className="label">Ledger:</span>
              <span className="value">{transactionData.ledger}</span>
            </div>
          )}

          {transactionData.fee && (
            <div className="detail-row">
              <span className="label">Fee:</span>
              <span className="value">{transactionData.fee} stroops</span>
            </div>
          )}

          {transactionData.hash && (
            <a 
              href={getExplorerUrl(transactionData.hash)}
              target="_blank"
              rel="noopener noreferrer"
              className="explorer-link"
            >
              View on Stellar Expert →
            </a>
          )}
        </div>
      )}

      {transactionData?.error && (
        <div className="error-details">
          <p>{transactionData.error}</p>
        </div>
      )}

      {onClose && (
        <button onClick={onClose} className="close-button">
          Close
        </button>
      )}
    </div>
  );
}

export default TransactionStatus;