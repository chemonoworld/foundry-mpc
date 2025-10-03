# Repository Guidelines

## Project Structure & Module Organization
foundry-mpc is a mixed Rust + JavaScript workspace. `foundry-mpc-rs/` contains the Shamir Secret Sharing crate; toggle extra curves with cargo features `k256` and `p256`. `foundry-mpc-wasm/` wraps the crate for WebAssembly and emits `pkg-*` bundles that `foundry-mpc-ts/` copies into `dist/`. Vendored FROST forks (`frost-core/`, `frost-ed25519/`, `frost-rerandomized/`) shadow ZcashFoundation upstreams—record the upstream commit hash in your PR before diverging.

## Build, Test, and Development Commands
Run `cargo build -p foundry-mpc-rs` for Rust builds and `cargo test --workspace --all-features` before every PR. When touching WebAssembly bindings, execute `yarn workspace foundry-mpc-wasm build:web` (swap for `build:nodejs` or `build:bundler` as needed) and `yarn workspace foundry-mpc-wasm typecheck`. TypeScript development uses `yarn workspace foundry-mpc-ts build` to compile and copy WASM artifacts, `yarn workspace foundry-mpc-ts dev` for iterative runs, and `yarn workspace foundry-mpc-ts test` to execute Jest. Use `yarn ci` to mirror the automation pipeline.

## Coding Style & Naming Conventions
Rust code relies on stable `rustfmt`; run `cargo fmt` before committing. Keep functions snake_case, public types UpperCamelCase, and modules descriptive (`participants`, `curve::k256`, `shares`). Lint protocol changes with `cargo clippy --workspace --all-features`. TypeScript stays ESM, strictly typed, and 2-space indented. Format WASM bindings through `yarn workspace foundry-mpc-wasm exec prettier --check src`, and expose reusable utilities as named exports from `src/index.ts`.

## Testing Guidelines
Augment Rust modules with property-style tests next to their implementations and add integration coverage when flows span multiple crates. Always run `cargo test --workspace` before submitting. Co-locate TypeScript specs (`src/sharing.spec.ts`) with the code they exercise, and add smoke tests for new WASM exports that import the generated `pkg` bundle in Node and browser contexts. Seed randomness explicitly so cryptographic vectors stay deterministic.

## Commit & Pull Request Guidelines
Write concise, imperative commit subjects (`Add deterministic resharing vector`) as in the existing history. Keep each PR focused, summarize curve coverage in the description, link related issues, and list the verification commands you ran. Request review from a maintainer of the affected package and call out any FROST upstream sync.

## Security & Configuration Tips
Never commit live keys or secrets; rely on generated fixtures under `scripts/`. Preserve browser compatibility when touching randomness by keeping the `rand_core` `getrandom` feature enabled. Document new environment variables in `README.md` and keep Yarn pinned to 4.7.0 to prevent resolution drift.
