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
      const newScale = 1.15 - progress * 0.75;
      setScale(newScale);
    };

    window.addEventListener("scroll", onScroll, { passive: true });
    return () => window.removeEventListener("scroll", onScroll);
  }, []);

  return (
    <section className="hero-section">
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
      <div className="flex flex-col sm:flex-row gap-4 justify-center">
        <button
          onClick={() => navigate("/analyze")}
          className="absolute bottom-8 left-[32%] mb-4 px-10 py-4 bg-gradient-to-r from-sky-500 to-violet-600 text-white font-semibold rounded-2xl shadow-lg hover:shadow-sky-300 hover:scale-105 transition-all duration-200 text-lg"
        >
          Start Free Analysis →
        </button>
        <button
          onClick={() => navigate("/signin")}
          className=" absolute bottom-8 left-[52%] mb-4 px-8 py-4 bg-white text-slate-700 font-semibold rounded-2xl shadow border border-slate-200 hover:border-sky-300 hover:text-sky-700 transition-all duration-200 text-lg"
        >
          Sign Up
        </button>
      </div>

      
    </section>
  );
}

export default Hero;
