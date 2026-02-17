import React, { useState, useEffect } from "react";
import {
  connectWallet,
  isFreighterInstalled,
  formatAddress,
} from "../pages/stellarService";
import "./WalletConnect.css";


function WalletConnect({
  publicKey,
  setPublicKey,
  isConnected,
  setIsConnected,
}) {
  const [error, setError] = useState(null);
  const [loading, setLoading] = useState(false);
  const [freighterInstalled, setFreighterInstalled] = useState(false);

  useEffect(() => {
    const checkFreighter = async () => {
      const installed = await isFreighterInstalled();
      setFreighterInstalled(installed);
    };

    checkFreighter();

    const interval = setInterval(checkFreighter, 1000);
    return () => clearInterval(interval);
  }, []);

  const handleConnect = async () => {
    setLoading(true);
    setError(null);

    try {
      const pubKey = await connectWallet();
      setPublicKey(pubKey);
      setIsConnected(true);
    } catch (err) {
      setError(err.message || "Failed to connect wallet");
      setIsConnected(false);
    } finally {
      setLoading(false);
    }
  };

  const handleDisconnect = () => {
    setPublicKey(null);
    setIsConnected(false);
  };

  return (
    <div className="wallet-connect-container">
      <div className="coverContainer">
        <h2>Wallet Connection</h2>

        {!freighterInstalled && (
          <div className="warning-message">
             Freighter wallet not detected. Please install{" "}
            <a
              href="https://freighter.app/"
              target="_blank"
              rel="noopener noreferrer"
            >
              Freighter
            </a>
          </div>
        )}

        {error && <div className="error-message"> {error}</div>}

        {!isConnected ? (
          <button
            onClick={handleConnect}
            disabled={loading || !freighterInstalled}
            className="connect-button"
          >
            {loading ? "Connecting..." : " Connect Freighter Wallet"}
          </button>
        ) : (
          <div className="connected-info">
            <div className="wallet-address">
              <span className="label">Connected:</span>
              <span className="address" title={publicKey}>
                {formatAddress(publicKey)}
              </span>
              <button
                onClick={() => navigator.clipboard.writeText(publicKey)}
                className="copy-button"
                title="Copy full address"
              >
                Copy
              </button>
            </div>
            <button onClick={handleDisconnect} className="disconnect-button">
              Disconnect
            </button>
          </div>
        )}
      </div>
    </div>
  );
}

export default WalletConnect;
