import Hero from "./components/Hero";
import { useState } from "react";
import Dashboard from "./pages/Dashboard";

import Signin from "./pages/Signin";
import { Routes, Route } from "react-router-dom";
import "./App.css";

function App() {
  const [publicKey, setPublicKey] = useState(null);
  const [isConnected, setIsConnected] = useState(false);

  return (
    <Routes>
      <Route path="/" element={<Hero />} />

      <Route
        path="/dashboard"
        element={
          <Dashboard
            publicKey={publicKey}
            isConnected={isConnected}
          />
        }
      />

      <Route
        path="/signin"
        element={
          <Signin
            publicKey={publicKey}
            setPublicKey={setPublicKey}
            isConnected={isConnected}
            setIsConnected={setIsConnected}
          />
        }
      />
    </Routes>
  );
}


export default App;
