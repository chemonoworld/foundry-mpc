import * as wasmGlue from "../pkg/sssui_wasm_bg.js";
import { __wbg_set_wasm } from "../pkg/sssui_wasm_bg.js";
export { wasmGlue };
export async function initSSSuiWasm() {
    try {
        await initWasm(wasmGlue, "../pkg/sssui_wasm_bg.wasm");
        console.log("[sssui-mpc-ts] WASM initialized");
    }
    catch (error) {
        console.error("[sssui-mpc-ts] Error initializing WASM, err: %s", error);
        throw error;
    }
}
let isInitialized = false;
let wasmInstance = null;
const exportedFunctions = {};
export async function initWasm(glueModule, wasmPath) {
    var _a;
    if (isInitialized && wasmInstance) {
        return exportedFunctions;
    }
    try {
        let wasmBytes;
        const isNode = typeof process !== "undefined" && ((_a = process.versions) === null || _a === void 0 ? void 0 : _a.node);
        if (isNode) {
            const { readFile } = await import("fs/promises");
            const { fileURLToPath } = await import("url");
            const path = await import("path");
            const thisFileUrl = import.meta.url;
            const thisFilePath = fileURLToPath(thisFileUrl);
            const distDir = path.dirname(thisFilePath);
            const wasmAbsPath = path.resolve(distDir, wasmPath);
            const wasmBuffer = await readFile(wasmAbsPath);
            wasmBytes = wasmBuffer.buffer.slice(wasmBuffer.byteOffset, wasmBuffer.byteOffset + wasmBuffer.byteLength);
        }
        else {
            const response = await fetch(wasmPath);
            if (!response.ok) {
                throw new Error(`Cannot load WASM file: ${response.status} ${response.statusText}`);
            }
            wasmBytes = await response.arrayBuffer();
        }
        const importsMap = {
            "./sssui_wasm_bg.js": glueModule,
        };
        const { instance } = await globalThis.WebAssembly.instantiate(wasmBytes, importsMap);
        __wbg_set_wasm(instance.exports);
        if (typeof glueModule.init === "function") {
            glueModule.init();
        }
        wasmInstance = glueModule;
        Object.keys(glueModule)
            .filter((key) => typeof glueModule[key] === "function")
            .forEach((funcName) => {
            exportedFunctions[funcName] = (...args) => glueModule[funcName](...args);
        });
        isInitialized = true;
        return exportedFunctions;
    }
    catch (error) {
        throw error;
    }
}
export function isWasmInitialized() {
    return isInitialized;
}
//# sourceMappingURL=init.js.map