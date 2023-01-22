import { WasmBinary } from "./binary";

declare global {
    namespace JSX {
        interface IntrinsicElements {
            [HTMLRaytracerViewElement.TAG_NAME]: React.DetailedHTMLProps<React.HTMLAttributes<HTMLRaytracerViewElement>, HTMLRaytracerViewElement>
        }
    }
}

export class HTMLRaytracerViewElement extends HTMLElement {
    public static readonly TAG_NAME = 'ray-tracer';

    private canvas?: HTMLCanvasElement;
    private renderingContext?: CanvasRenderingContext2D;
    private binary: WasmBinary;
    private isReady = false;

    // eventually these will be configurable, and this component will own that
    // on the UI side
    private static WIDTH = 720;
    private static HEIGHT = 405;

    constructor() {
        super();
        this.binary = new WasmBinary();
        this.binary.init()
            .then(() => {
                this.isReady = true;
            })
            .catch(e => {
                console.error("An exception occurred during WASM initialization");
                console.error(e);
            });
    }

    public render() {
        this.innerHTML = '';
        this.canvas = document.createElement('canvas');
        this.canvas.width = HTMLRaytracerViewElement.WIDTH;
        this.canvas.height = HTMLRaytracerViewElement.HEIGHT;
        this.appendChild(this.canvas);
        let ctx = this.canvas.getContext('2d');
        if (ctx == null) {
            console.warn("Failed to acquire rendering context to canvas");
            return;
        }
        this.renderingContext = ctx;
    }

    public connectedCallback() {
        this.render();
    }

    public renderAndPaint() {
        if (!this.isReady) {
            throw new Error("WASM binary not yet initialized, cannot render");
        }
        const data = this.binary.render_image();
        const resultBuffer = new Uint8ClampedArray(data);
        const imageData = new ImageData(resultBuffer, HTMLRaytracerViewElement.WIDTH, HTMLRaytracerViewElement.HEIGHT);
        this.drawImageData(imageData);
    }

    private drawImageData(data: ImageData) {
        if (!this.renderingContext) throw new Error("No rendering context has been setup yet");

        this.renderingContext.putImageData(data, 0, 0);
    }
}

window.customElements.define(HTMLRaytracerViewElement.TAG_NAME, HTMLRaytracerViewElement);
