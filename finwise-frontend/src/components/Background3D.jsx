import { useEffect, useState, useRef } from "react";
import { Canvas, useFrame } from "@react-three/fiber";
import { OrbitControls, useGLTF } from "@react-three/drei";

function useScrollScale({ min = 0.75, max = 1.1, scrollRange = 700 } = {}) {
  const [scale, setScale] = useState(1);
  const [progress, setProgress] = useState(0);

  useEffect(() => {
    const onScroll = () => {
      const y = window.scrollY;
      const p = Math.min(y / scrollRange, 1);

      setProgress(p);

      const newScale = max - (max - min) * p;
      setScale(newScale);
    };

    window.addEventListener("scroll", onScroll, { passive: true });
    return () => window.removeEventListener("scroll", onScroll);
  }, [min, max, scrollRange]);

  return { scale, progress };
}

function Model({ scale, progress }) {
  const ref = useRef();
  const { scene } = useGLTF("/models/bag_of_money.glb");

  useFrame(() => {
    if (ref.current) {
      ref.current.scale.setScalar(scale * 15);

      
ref.current.rotation.y = progress * (Math.PI * 0.6);
ref.current.rotation.x = progress * 0.08;     
    }
  });

  return (
    <primitive
      ref={ref}
      object={scene}
      position={[-0.3, -0.6, 0]}
      rotation={[0, 0, 0]}
    />
  );
}

export default function Background3D() {
  const { scale, progress } = useScrollScale();

  return (
    <Canvas
      camera={{ position: [0, 1, 5], fov: 45 }}
      style={{
        position: "fixed",
        inset: 0,
        zIndex: -1,
      }}
    >
      <ambientLight intensity={1} />
      <directionalLight position={[5, 5, 5]} intensity={1} />

      <Model scale={scale} progress={progress} />

      <OrbitControls
        enableRotate={false}
        enableZoom={false}
        enablePan={false}
      />
    </Canvas>
  );
}
