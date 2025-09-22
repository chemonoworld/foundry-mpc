import * as wasmGlue from "../pkg/sssui_wasm_bg.js";
import { __wbg_set_wasm } from "../pkg/sssui_wasm_bg.js";

export { wasmGlue };

export async function initSSSuiWasm() {
  try {
    await initWasm(wasmGlue, "../pkg/sssui_wasm_bg.wasm");
    console.log("[sssui-mpc-ts] WASM initialized");
  } catch (error) {
    console.error("[sssui-mpc-ts] Error initializing WASM, err: %s", error);
    throw error;
  }
}

// Flag to track module initialization status
let isInitialized = false;
// Store the initialized WASM module
let wasmInstance: any = null;
// Store exported functions
const exportedFunctions: Record<string, Function> = {};

type WasmModule = typeof wasmGlue;
export type WasmFunctionNames = keyof WasmModule;

/**
 * Function to initialize WASM module and export its functions globally
 *
 * @param wasmModule - WASM module (import * as wasmModule from "my-wasm-module")
 * @param wasmPath - WASM binary file path (default: "/threshold_ecdsa_wasm_bg.wasm")
 * @returns Object containing all exported functions
 */
export async function initWasm(
  glueModule: any,
  wasmPath: string
): Promise<Record<string, Function>> {
  // const wasmModule: any = originalWasmModule;
  // const wasmPath = "@/../pkg/cait_sith_keplr_wasm_bg.wasm";
  // Return cached functions if already initialized
  if (isInitialized && wasmInstance) {
    return exportedFunctions;
  }

  try {
    // console.log(`Initializing WASM module: ${wasmPath}`);

    // Load WASM binary file (browser vs node)
    let wasmBytes: ArrayBuffer;
    const isNode = typeof process !== "undefined" && process.versions?.node;
    if (isNode) {
      const { readFile } = await import("fs/promises");
      const { fileURLToPath } = await import("url");
      const path = await import("path");
      // Resolve wasm relative to this file's compiled location in dist
      const thisFileUrl = import.meta.url;
      const thisFilePath = fileURLToPath(thisFileUrl);
      const distDir = path.dirname(thisFilePath);
      const wasmAbsPath = path.resolve(distDir, wasmPath);
      const wasmBuffer = await readFile(wasmAbsPath);
      wasmBytes = wasmBuffer.buffer.slice(
        wasmBuffer.byteOffset,
        wasmBuffer.byteOffset + wasmBuffer.byteLength
      ) as ArrayBuffer;
    } else {
      const response = await fetch(wasmPath);
      if (!response.ok) {
        throw new Error(
          `Cannot load WASM file: ${response.status} ${response.statusText}`
        );
      }
      wasmBytes = await response.arrayBuffer();
    }

    // Instantiate WASM and bind to glue
    const importsMap: Record<string, any> = {
      "./sssui_wasm_bg.js": glueModule,
    };
    const { instance } = await (globalThis as any).WebAssembly.instantiate(
      wasmBytes,
      importsMap
    );
    __wbg_set_wasm(instance.exports as any);

    if (typeof (glueModule as any).init === "function") {
      (glueModule as any).init();
    }

    // Store the initialized glue module
    wasmInstance = glueModule;

    // Extract all functions from the glue module
    Object.keys(glueModule)
      .filter((key) => typeof glueModule[key] === "function")
      .forEach((funcName) => {
        exportedFunctions[funcName] = (...args: any[]) =>
          glueModule[funcName](...args);
      });

    // Mark initialization as successful
    isInitialized = true;
    // console.log("WASM module initialization complete!");

    return exportedFunctions;
  } catch (error) {
    // console.error("WASM module initialization error:", error);
    throw error;
  }
}

/**
 * Helper function to check if WASM is initialized
 */
export function isWasmInitialized(): boolean {
  return isInitialized;
}
