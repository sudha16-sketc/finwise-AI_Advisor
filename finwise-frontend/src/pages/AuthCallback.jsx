// src/pages/AuthCallback.jsx
import { useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { API_BASE } from "../services/api";

/**
 * AuthCallback — handles the Google OAuth token handoff.
 *
 * WHY this page exists:
 *   After Google OAuth, the backend redirects here with ?token=...
 *   Instead of trusting a cookie set during the redirect (which browsers
 *   block cross-site), this page makes a direct credentialed fetch to
 *   /api/verify-token. That direct fetch properly receives and stores the
 *   session cookie from onrender.com in the browser for future requests.
 */
export default function AuthCallback() {
  const navigate = useNavigate();

  useEffect(() => {
    const params = new URLSearchParams(window.location.search);
    const token = params.get("token");
    const error = params.get("error");

    if (error) {
      navigate(`/login?error=${error}`);
      return;
    }

    if (!token) {
      navigate("/login?error=no_token");
      return;
    }

    fetch(`${API_BASE}/api/verify-token?token=${token}`, {
      method: "GET",
      credentials: "include", // ← critical: this is what sets the session cookie correctly
    })
      .then((res) => res.json())
      .then((data) => {
        if (data.authenticated) {
          navigate("/dashboard");
        } else {
          navigate("/login?error=auth_failed");
        }
      })
      .catch((err) => {
        console.error("Auth callback failed:", err);
        navigate("/login?error=network_error");
      });
  }, []);

  return (
    <div style={{
      display: "flex",
      justifyContent: "center",
      alignItems: "center",
      height: "100vh",
      fontSize: "1.2rem",
      color: "#666"
    }}>
      Signing you in...
    </div>
  );
}