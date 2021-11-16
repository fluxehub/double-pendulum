import React, { useEffect, useRef } from 'react';
import init, { test } from 'simulator';

function Pendulum() {
    const canvasRef = useRef<HTMLCanvasElement>(null);

    // Initialize WASM and then load the canvas
    useEffect(() => {
        if (canvasRef.current !== null) {
            // Get 2D context and load it into the simulator
            const ctx = canvasRef.current.getContext('2d')!;
            init().then(() => test(ctx));
        }
    }, [canvasRef]);

    return <canvas width="400" height="400" ref={canvasRef}></canvas>
}

export default Pendulum;