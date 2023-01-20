import React from "react";
import ReactDOM from "react-dom";
import { WasmBinary } from "./wasm"

export {};

console.log("Hello, world!");

const binary = new WasmBinary();

(window as any).__binary = binary;

const HelloWasm = () => {
    return (<p>Hello, world!!!</p>);
};

const root = document.querySelector("#root");
ReactDOM.render(<HelloWasm />, root);
