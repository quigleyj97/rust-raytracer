import React, { useCallback, useRef } from "react";
import { createRoot } from "react-dom/client";
import { HTMLRaytracerViewElement, WasmBinary } from "./wasm"

export {};

console.log("Hello, world!");

const binary = new WasmBinary();

(window as any).__binary = binary;

const HelloWasm = () => {
    const ref = useRef<HTMLRaytracerViewElement>(null);

    const draw = useCallback(() => {
        if (!ref.current) return;

        ref.current.renderAndPaint();
    }, [ref]);

    return (<p>Hello, world!
        <button onClick={draw}>Render image</button>
        <ray-tracer ref={ref}></ray-tracer>
    </p>);
};

const container = document.querySelector("#root")!;
const root = createRoot(container);
root.render(<HelloWasm />);
