# WASM Cross-Environment 패키지 계획

## 목표

- `wasm-pack`의 분리된 `web`, `bundler`, `nodejs` 타겟 대신, 하나의 빌드 아티팩트로 Node.js와 브라우저를 모두 지원.
- `foundry-mpc-ts`에서 추가 래퍼 없이 동일한 초기화 API(`init`, `initSync`)를 노출.

## 현황 요약

- 현재 `foundry-mpc-wasm`은 `pkg-web/`, `pkg-nodejs/`, `pkg-bundler/`에 각각 별도 빌드를 생성.
- `foundry-mpc-ts`는 `pkg-bundler`를 복사해 `dist/pkg`로 노출하며, 실행 환경별 분기 처리는 소비자(사용 앱)에게 맡기고 있음.

## 제안 아키텍처

1. **단일 wasm + 다중 로더**
   - `wasm-pack build wasm --target bundler --out-dir ../pkg-universal --out-name foundry_mpc` 명령을 추가(`build:universal`).
     ㅋ 2. **환경 감지 로더 작성**
   - `foundry-mpc-ts/src/runtime/universal.ts`에서 런타임을 감지하여 로딩 전략 분기.
   - Node: `import fs from 'node:fs/promises'`로 `.wasm` 파일을 읽고 `WebAssembly.instantiate` 호출.
   - Browser: `fetch(new URL('./pkg/foundry_mpc_bg.wasm', import.meta.url))`와 `WebAssembly.instantiateStreaming` 사용, 폴백으로 `instantiate` 제공.
   - 예시 코드 조각:
     ```ts
     const wasmUrl = new URL("../pkg/foundry_mpc_bg.wasm", import.meta.url);
     export async function init(input?: RequestInfo | URL) {
       const source = isNode
         ? await fs.readFile(wasmPathFromNodeResolution(wasmUrl))
         : await fetch(input ?? wasmUrl);
       const { instance, module } = await WebAssembly.instantiate(
         source,
         imports
       );
       return finalizeInit(instance, module);
     }
     ```
2. **단일 패키지 진입점 구성**
   - `foundry-mpc-ts/package.json`의 `exports` 필드에 환경 조건 추가:
     ```json
     {
       "name": "foundry-mpc-ts",
       "exports": {
         ".": {
           "browser": "./dist/src/universal-browser.js",
           "node": "./dist/src/universal-node.js",
           "default": "./dist/src/universal.js"
         }
       }
     }
     ```
   - 세 파일은 동일한 wasm 바이트코드를 import하지만 초기화 방식만 다르게 한 thin wrapper (빌드 단계에서 `esbuild`로 ESM 변환).
3. **빌드 파이프라인 정리**
   - `build` 스크립트 내에서 순서 변경:
     1. `yarn workspace foundry-mpc-wasm build:universal`
     2. `tsc --project tsconfig.json`
     3. `esbuild`로 universal 래퍼를 브라우저/노드 전용 진입점으로 번들.
   - `copy:wasm` 스크립트는 `pkg-universal`만 복사하도록 수정.
4. **테스트 및 검증**
   - Node: `node --experimental-wasm-modules scripts/probe-node.mjs`로 `init()` 호출 후 핵심 API smoke test.
   - Browser: `vite` or `webpack` 샌드박스에서 `init()` 불러 실사용 API 확인.
   - 번들 크기 검증: `wasm-opt -Os` 적용 고려.

## 장점

- 동일 wasm 파일을 공유해 릴리스 과정 단순화.
- 런타임 감지 로더로 소비자 설정 최소화.
- `exports` 조건을 통해 Node/브라우저 번들러가 자동으로 최적 진입점을 선택.

## 후속 작업

1. `foundry-mpc-wasm/package.json`에 `build:universal` 추가 및 기존 빌드 스크립트 정리.
2. `foundry-mpc-ts`에 universal 로더 구현 및 타입 정의 업데이트.
3. 예제 프로젝트(예: `scripts/ci`)에서 새 패키지로 교체 검증.
4. README/AGENTS 업데이트로 새 명령과 사용법 문서화.
