import * as StellarSdk from "@stellar/stellar-sdk";

import {
  isConnected,
  requestAccess,
  signTransaction
} from "@stellar/freighter-api";



import axios from 'axios';

// Stellar testnet configuration
const TESTNET_URL = 'https://horizon-testnet.stellar.org';
const server = new StellarSdk.Horizon.Server(TESTNET_URL);
const BACKEND_API = '/api'; // Proxied through Vite to localhost:8080

/**
 * Check if Freighter wallet is installed
 */


export const isFreighterInstalled = async () => {
  try {
    const result = await isConnected();
    return !result.error;
  } catch {
    return false;
  }
};





/**
 * Connect to Freighter wallet and get public key
 */
export const connectWallet = async () => {
  try {
    const result = await requestAccess();

    if (result.error) {
      throw new Error(result.error);
    }

    return result.address;
  } catch (error) {
    console.error("Wallet connection failed:", error);
    throw new Error("Connection rejected");
  }
};




/**
 * Fetch XLM balance from backend API
 */
export const fetchBalance = async (publicKey) => {
  try {
    const response = await axios.get(`${BACKEND_API}/balance/${publicKey}`);
    return response.data.balance;
  } catch (error) {
    console.error('Balance fetch failed:', error);
    throw error;
  }
};

/**
 * Fetch XLM balance directly from Stellar network (fallback)
 */
export const fetchBalanceDirect = async (publicKey) => {
  try {
    const account = await server.loadAccount(publicKey);
    const xlmBalance = account.balances.find(
      balance => balance.asset_type === 'native'
    );
    return xlmBalance ? xlmBalance.balance : '0';
  } catch (error) {
    console.error('Direct balance fetch failed:', error);
    throw error;
  }
};

/**
 * Send XLM transaction via backend API
 */
export const sendTransaction = async (sourcePublicKey, destinationAddress, amount) => {
  try {
    // Build transaction using Stellar SDK
    const sourceAccount = await server.loadAccount(sourcePublicKey);
    
    const transaction = new StellarSdk.TransactionBuilder(sourceAccount, {
      fee: StellarSdk.BASE_FEE,
      networkPassphrase: StellarSdk.Networks.TESTNET,
    })
      .addOperation(
        StellarSdk.Operation.payment({
          destination: destinationAddress,
          asset: StellarSdk.Asset.native(),
          amount: amount.toString(),
        })
      )
      .setTimeout(30)
      .build();

    // Sign transaction with Freighter
    const signed = await signTransaction(transaction.toXDR(), {
      networkPassphrase: StellarSdk.Networks.TESTNET,
    });

    const response = await axios.post(`${BACKEND_API}/send`, {
      xdr: signed.signedTxXdr,
    });

    return response.data;
  } catch (error) {
    console.error('Transaction failed:', error.response?.data || error);
throw new Error(
  error.response?.data?.message ||
  'Transaction failed'
);

  }
};

/**
 * Fetch transaction history from backend API
 */
export const fetchTransactions = async (publicKey) => {
  try {
    const response = await axios.get(`${BACKEND_API}/transactions/${publicKey}`);
    return response.data.transactions;
  } catch (error) {
    console.error('Transaction history fetch failed:', error);
    throw error;
  }
};

/**
 * Fetch transaction history directly from Stellar network (fallback)
 */
export const fetchTransactionsDirect = async (publicKey, limit = 10) => {
  try {
    const transactions = await server
      .transactions()
      .forAccount(publicKey)
      .order('desc')
      .limit(limit)
      .call();

    return transactions.records.map(tx => ({
      id: tx.id,
      hash: tx.hash,
      created_at: tx.created_at,
      source_account: tx.source_account,
      fee_charged: tx.fee_charged,
      operation_count: tx.operation_count,
      successful: tx.successful,
    }));
  } catch (error) {
    console.error('Direct transaction fetch failed:', error);
    throw error;
  }
};

/**
 * Format Stellar address (shorten for display)
 */
export const formatAddress = (address) => {
  if (!address) return '';
  return `${address.slice(0, 4)}...${address.slice(-4)}`;
};

/**
 * Validate Stellar address
 */
export const isValidAddress = (address) => {
  try {
    return StellarSdk.StrKey.isValidEd25519PublicKey(address);
  } catch {
    return false;
  }
};