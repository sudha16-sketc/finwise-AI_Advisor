import React, { useState } from 'react';
import { sendTransaction, isValidAddress } from '../pages/stellarService';

function SendTransaction({ publicKey, onTransactionComplete }) {
  const [destination, setDestination] = useState('');
  const [amount, setAmount] = useState('');
  const [sending, setSending] = useState(false);
  const [error, setError] = useState(null);
  const [validationError, setValidationError] = useState('');


  const validateDestination = (address) => {
    if (!address) {
      setValidationError('');
      return false;
    }
    
    if (!isValidAddress(address)) {
      setValidationError('Invalid Stellar address');
      return false;
    }
    
    setValidationError('');
    return true;
  };

  const handleDestinationChange = (e) => {
    const value = e.target.value;
    setDestination(value);
    validateDestination(value);
  };

  const handleAmountChange = (e) => {
    const value = e.target.value;

    if (value === '' || /^\d*\.?\d*$/.test(value)) {
      setAmount(value);
    }
  };


  const handleSubmit = async (e) => {
    e.preventDefault();

    if (!validateDestination(destination)) {
      setError('Please enter a valid destination address');
      return;
    }

    if (!amount || parseFloat(amount) <= 0) {
      setError('Please enter a valid amount');
      return;
    }

    setSending(true);
    setError(null);

    try {

      const result = await sendTransaction(publicKey, destination, amount);
      

      setDestination('');
      setAmount('');
      

      if (onTransactionComplete) {
        onTransactionComplete(result);
      }
    } catch (err) {
      setError(err.message || 'Transaction failed');
      console.error('Transaction error:', err);
    } finally {
      setSending(false);
    }
  };

  if (!publicKey) {
    return null;
  }

  return (
    <div className="send-transaction-container">
      <h2>Send XLM</h2>
      
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
            className={validationError ? 'input-error' : ''}
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

        {error && (
          <div className="error-message">❌ {error}</div>
        )}

        <button 
          type="submit" 
          disabled={sending || !destination || !amount || !!validationError}
          className="send-button"
        >
          {sending ? '⏳ Sending...' : '📤 Send Transaction'}
        </button>
      </form>

      <div className="transaction-info">
        <small>💡 Make sure destination account exists on testnet</small>
      </div>
    </div>
  );
}

export default SendTransaction;