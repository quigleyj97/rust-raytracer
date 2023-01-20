export class WasmBinary {
    private module?: typeof import("../../../raytracer-core/pkg");
    private isLoaded = false;
    
    public async init() {
        console.log("Importing WASM binary...");
        let module = await import("../../../raytracer-core/pkg");
        this.module = module;
        console.log("Imported WASM assembly");
        module.init_debug_hooks();
        console.log("Initted debug hooks");
        this.isLoaded = true;
        console.log("Rendering scene...");
        console.log(module.draw_scene());
    }
}
