import "./Hero.css";
import Background3D from "./Background3D";
import { useEffect, useState } from "react";
import Navbar from "./Navbar";
import { Link } from "react-router-dom";
import Signin from "../pages/Signin";
import FloatingModel from "./Screen3dmodel";

function Hero() {
  const [scale, setScale] = useState(1);

  useEffect(() => {
    const onScroll = () => {
      const y = window.scrollY;
      const progress = Math.min(y / 700, 1);
      const newScale = 1.15 - progress * 0.75; 
      setScale(newScale);
    };

    window.addEventListener("scroll", onScroll, { passive: true });
    return () => window.removeEventListener("scroll", onScroll);
  }, []);

  return (
    <section className="hero-section">
      <Navbar />
      <div className="container">
      <div className="model">
        <Background3D />
      </div>

      <div
        className="hero-text"
        style={{
          transform: `scale(${scale})`,
          transition: "transform 0.08s linear",
        }}
      >
        <h1>FIN</h1>
        <h1>WISE</h1>
      </div>
      </div>

      <Link to="/signin" className="get-started-link">
        <div className="get-startedbtn">GET STARTED </div>
      </Link>

      <FloatingModel />
    </section>
  );
}

export default Hero;
