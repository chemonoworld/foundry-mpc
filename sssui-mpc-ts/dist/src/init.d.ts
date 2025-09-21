import * as wasmGlue from "../pkg/sssui_wasm_bg.js";
export { wasmGlue };
export declare function initSSSuiWasm(): Promise<void>;
type WasmModule = typeof wasmGlue;
export type WasmFunctionNames = keyof WasmModule;
export declare function initWasm(glueModule: any, wasmPath: string): Promise<Record<string, Function>>;
export declare function isWasmInitialized(): boolean;
