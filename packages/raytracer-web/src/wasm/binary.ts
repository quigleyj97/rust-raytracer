
export class WasmBinary {
    private module?: typeof import("../../../raytracer-core/pkg");
    private isLoaded = false;
    
    public async init() {
        console.debug("Importing WASM binary...");

        const start = Date.now();
        let module = await import("../../../raytracer-core/pkg");
        const importTime = Date.now();
        this.module = module;
        console.debug("Imported WASM assembly");

        module.init_debug_hooks();
        const initTime = Date.now();
        console.debug("Initialized debug hooks");

        console.log("WASM binary loaded");
        console.group("Timing tree");
        console.log("Overall:", initTime - start, "ms");
        console.debug("Binary download time:", importTime - start, "ms");
        console.debug("Initialization time:", initTime - importTime, "ms");
        console.groupEnd();
        this.isLoaded = true;
    }

    public render_image(): Uint8Array {
        if (!this.isLoaded) {
            throw new Error("Module uninitialized");
        }
        console.log("Rendering scene...");
        const start = Date.now();
        performance.mark("beginDraw");
        const data = this.module!.draw_scene();
        performance.mark("endDraw");
        const end = Date.now();
        performance.measure("Render time", "beginDraw", "endDraw");
        console.log("Rendering complete, took ", end - start, "ms");
        return data;
    }
}
