import React from "react";
import "./Signin.css";
import WalletConnect from "../components/WalletConnect";

function Signin({
  publicKey,
  setPublicKey,
  isConnected,
  setIsConnected,
  isAuthenticated,        // ← receive from parent
  setIsAuthenticated,     // ← receive from parent
}) {
  return (
    <div className="signin-main">
      <WalletConnect
        publicKey={publicKey}
        setPublicKey={setPublicKey}
        isConnected={isConnected}
        setIsConnected={setIsConnected}
        isAuthenticated={isAuthenticated}
        setIsAuthenticated={setIsAuthenticated}
      />
    </div>
  );
}

export default Signin;