import React from "react";
import { useState } from "react";
import "./Signin.css";
import WalletConnect from "../components/WalletConnect";

function Signin({
  publicKey,
  setPublicKey,
  isConnected,
  setIsConnected,
}) {
  return (
    <div className="signin-main">
      <WalletConnect
        publicKey={publicKey}
        setPublicKey={setPublicKey}
        isConnected={isConnected}
        setIsConnected={setIsConnected}
      />
    </div>
  );
}


export default Signin;
