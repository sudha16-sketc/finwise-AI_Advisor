import React, { Suspense, useRef } from "react";
import { Canvas, useFrame } from "@react-three/fiber";
import { OrbitControls, useGLTF, Environment } from "@react-three/drei";
import "./Hero.css";


const MODEL_PATH = "/models/computer.glb";

function Model(props) {
  const group = useRef();
  const { scene } = useGLTF(MODEL_PATH);

  useFrame((state) => {
    const t = state.clock.getElapsedTime();
    if (group.current) {
      group.current.position.y = Math.sin(t) * 0.05;
    }
  });

  return (
    <group ref={group} {...props}>
      <primitive object={scene} position={[0, 0, 0]}/>
    </group>
  );
}

useGLTF.preload(MODEL_PATH);

export default function FloatingModel() {
  return (
    <div className="floating-model">
      <Canvas camera={{ position: [1,1,1], fov: 45 }}>
        
        <ambientLight intensity={0.6} />
        <directionalLight position={[5, 5, 5]} intensity={1} />

       
        <Environment preset="city" />

      
        <Suspense fallback={null}>
          <Model scale={0.2} />
        </Suspense>

      
        <OrbitControls enableZoom={false} enablePan={false} />
      </Canvas>
    </div>
  );
}
