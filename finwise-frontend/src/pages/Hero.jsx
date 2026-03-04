import "./Hero.css";
import Background3D from "../components/Background3D";
import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";

function Hero() {
  const [scale, setScale] = useState(1);
  const navigate = useNavigate();

  useEffect(() => {
    const onScroll = () => {
      const y = window.scrollY;
      const progress = Math.min(y / 700, 1);
      const newScale = 1.1 - progress * 0.5;
      setScale(newScale);
    };

    window.addEventListener("scroll", onScroll, { passive: true });
    return () => window.removeEventListener("scroll", onScroll);
  }, []);

  return (
    <section className="hero-section">
      <Background3D />

      <div
        className="hero-text"
        style={{
          transform: `scale(${scale})`,
          transition: "transform 0.1s linear",
        }}
      >
        <h1>FIN</h1>
        <h1>WISE</h1>
      </div>

      <div className="hero-buttons">
        <button
          onClick={() => navigate("/analyze")}
          className="primary-btn"
        >
          Start Free Analysis →
        </button>

        <button
          onClick={() => navigate("/signin")}
          className="secondary-btn"
        >
          Sign Up
        </button>
      </div>
    </section>
  );
}

export default Hero;