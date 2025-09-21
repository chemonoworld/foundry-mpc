# SSSui

A Rust and WebAssembly-based cryptographic library implementing Multi-Party Computation (MPC) algorithms, specifically Shamir Secret Sharing and Threshold Signature Schemes (to be added in future releases), across three elliptic curves: secp256k1, secp256r1 (NIST P-256), and ed25519.

## Overview

SSSui provides secure and efficient implementations of cryptographic protocols for distributed systems, enabling secure secret sharing and threshold cryptography across multiple participants. The library is designed to work seamlessly in both browser and Node.js environments through WebAssembly bindings.

## Components

### Demo app
https://github.com/Hyeong-soo/sssui-demo

### sssui-mpc-rs

Core Rust implementation of Shamir Secret Sharing algorithms with support for:

- **secp256k1**: Bitcoin's elliptic curve
- **secp256r1 (NIST P-256)**: NIST standard elliptic curve
- **ed25519**: Edwards curve for high-performance cryptography

### sssui-wasm

WebAssembly packaging layer that exposes the Rust functionality to JavaScript environments, enabling cross-platform deployment with native performance.

### sssui-mpc-ts(WIP)

JavaScript/TypeScript bindings and utilities for the WebAssembly module, providing:

- Type-safe interfaces for TypeScript projects
- Browser and Node.js compatibility
- Easy integration with existing JavaScript applications

## Features

- **Multi-curve Support**: Implementations across secp256k1, secp256r1, and ed25519
- **Cross-platform**: Native Rust performance with WebAssembly compatibility
- **Type Safety**: Full TypeScript support with comprehensive type definitions
- **Security**: Built on proven cryptographic primitives and thoroughly tested implementations
- **Performance**: Optimized for both computational efficiency and memory usage

## Installation

### Rust

```bash
cargo add sssui-mpc-rs
```

### JavaScript/TypeScript (via npm)

```bash
npm install sssui-mpc-ts
```

### WebAssembly (direct)

The WASM module can be found in the `sssui-wasm` package for direct integration.

## Repository Structure

```
sss-ui/
├── sssui-mpc-rs/          # Core Rust implementation
├── sssui-wasm/        # WebAssembly bindings
├── sssui-mpc-ts/          # TypeScript/JavaScript package
├── frost-core/        # FROST protocol core (threshold signatures) forked from ZcashFoundation/frost
├── frost-ed25519/     # FROST implementation for ed25519 forked from ZcashFoundation/frost
└── frost-rerandomized/ # Rerandomized FROST variant forked from ZcashFoundation/frost
```

## Roadmap

- ✅ Shamir Secret Sharing implementation
- 🔄 Threshold Signature Schemes (FROST protocol integration)
- 🔄 Enhanced browser optimization
- 🔄 Additional cryptographic primitives

## License

This project is dual-licensed under MIT OR Apache-2.0.

## Contributing

Contributions are welcome! Please ensure all cryptographic implementations are thoroughly tested and follow security best practices.

## Authors

- chemonoworld (jinwoo@keplr.app)
- Hyeongsoo Kim (hyeongsoo@postech.ac.kr)

## Repository

https://github.com/chemonoworld/sssui
