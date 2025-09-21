# sssui-mpc-rs

A Rust implementation of Shamir Secret Sharing (SSS) for Multi-Party Computation (MPC) specifically designed for SUI chain integration. This crate provides secure and efficient cryptographic primitives for distributed secret sharing across multiple elliptic curves.

## Overview

`sssui-mpc-rs` is the core Rust implementation that powers the SSSui cryptographic library. It implements Shamir Secret Sharing algorithms with support for multiple elliptic curves, enabling secure secret distribution and reconstruction in distributed systems.

## Supported Curves

- **secp256k1**: Bitcoin's elliptic curve (via k256 feature)
- **secp256r1 (NIST P-256)**: NIST standard elliptic curve (via p256 feature)
- **ed25519**: Edwards curve for high-performance cryptography

## Features

- **Multi-curve Support**: Generic implementation across secp256k1, secp256r1, and ed25519
- **Secure Secret Sharing**: Threshold-based secret splitting and reconstruction
- **Point-based Architecture**: Efficient handling of cryptographic points with x,y coordinates
- **Polynomial Mathematics**: Custom polynomial implementation for Lagrange interpolation
- **Keyshare Management**: Structured handling of distributed key shares
- **Serialization**: Full serde support for all cryptographic structures

## Core Components

### Secret Sharing (`sss` module)

- `split()`: Splits a 32-byte secret into threshold shares
- `combine()`: Reconstructs the original secret from sufficient shares
- Supports configurable threshold values (t-of-n schemes)

### Ed25519 Specific (`sss_ed25519` module)

- Specialized implementation for Edwards curve
- Optimized Lagrange interpolation for ed25519
- Native curve arithmetic operations

### Key Share Management (`keyshares` module)

- `KeysharePoints`: Container for managing distributed key shares
- Validation of unique x-coordinates
- Point containment and membership checking

### Point Operations (`point` module)

- `Point256`: 32-byte x,y coordinate representation
- Curve-agnostic scalar conversion utilities
- Serializable point structures

### Mathematical Utilities (`math` module)

- Polynomial implementation for secret sharing
- Lagrange interpolation algorithms
- Field arithmetic operations

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
sssui-mpc-rs = "0.1.0"
```

### Feature Flags

```toml
[dependencies]
sssui-mpc-rs = { version = "0.1.0", features = ["k256", "p256"] }
```

- `k256`: Enable secp256k1 support (default)
- `p256`: Enable secp256r1/NIST P-256 support (default)

## Usage

### Basic Secret Sharing

```rust
use sssui_mpc_rs::sss::{split, combine};
use sssui_mpc_rs::Secp256k1;

// Split a secret into 3 shares with threshold 2
let secret = [1u8; 32];
let point_xs = vec![[1u8; 32], [2u8; 32], [3u8; 32]];
let threshold = 2;

let shares = split::<Secp256k1>(secret, point_xs, threshold)?;

// Reconstruct secret from 2 shares
let reconstructed = combine::<Secp256k1>(&shares[0..2])?;
assert_eq!(secret, reconstructed);
```

### Working with Key Shares

```rust
use sssui_mpc_rs::keyshares::KeysharePoints;
use sssui_mpc_rs::point::Point256;

let points = vec![
    Point256 { x: [1u8; 32], y: [2u8; 32] },
    Point256 { x: [3u8; 32], y: [4u8; 32] },
];

let keyshares = KeysharePoints::new(points)?;
```

## Security Considerations

- All cryptographic operations use secure random number generation
- Field arithmetic is performed using constant-time operations where possible
- Input validation prevents common attack vectors (duplicate coordinates, insufficient shares)
- Polynomial coefficients are generated using cryptographically secure randomness

## Dependencies

- `frost-ed25519`, `frost-core`: FROST protocol implementations
- `k256`, `p256`: Elliptic curve implementations
- `elliptic-curve`: Generic curve traits and utilities
- `serde`: Serialization support
- `rand_core`: Secure random number generation

## License

This project is licensed under the MIT License.

## Authors

- Jinwoo Lee (jinwoo0454@gmail.com)

## Repository

https://github.com/chemonoworld/sssui
