# CryptoTree

> **A cryptographically verifiable Merkle AVL Tree implementation in Rust.**

CryptoTree is a high-performance, authenticated dictionary data structure designed for client-side state verification. It combines the self-balancing properties of an **AVL Tree** with the cryptographic integrity of a **Merkle Tree**, enabling O(log n) search operations with verifiable proofs of inclusion.

This library is written in Rust and compiles to WebAssembly (WASM), making it suitable for usage in both backend systems and browser-based light clients.

## Features

- **Authenticated Storage**: Every node is hashed (SHA-256), creating a tamper-evident root hash (Merkle Root).
- **Efficient Search**: O(log n) lookup, insertion, and proof generation via AVL balancing.
- **Verifiable Proofs**: Generate compact Merkle proofs that can be verified by any client with the root hash.
- **WASM Support**: First-class support for compiling to WebAssembly for browser environments.
- **Zero Dependencies**: Core logic depends only on standard crypto libraries.

## Architecture

CryptoTree differs from standard Merkle Trees (which are typically static or append-only) by supporting dynamic insertions while maintaining a balanced structure. This makes it ideal for use cases where state changes frequently but verification is required.

- **Tree Type**: Binary AVL Tree
- **Hashing**: SHA-256
- **Serialization**: Deterministic canonical JSON (for proof stability)

## Usage

### Rust

Add to your `Cargo.toml`:

```toml
[dependencies]
crypto_tree = { path = "crypto-tree/rust" }
```

```rust
use crypto_tree::{CryptoBinaryTree, Transaction};

let mut tree = CryptoBinaryTree::new();

// Insert data
let tx = Transaction::new("tx_123", "Alice", "Bob", 50);
tree.insert(tx);

// Generate Proof
let proof = tree.get_proof_of_inclusion("tx_123").unwrap();

// Verify (Client-side)
let is_valid = crypto_tree::verify_proof("tx_123", &proof, &tree.root_hash());
```

### WebAssembly (Browser)

See [crypto-tree/wasm/README.md](crypto-tree/wasm/README.md) for browser integration details.

## Project Structure

- `crypto-tree/rust`: Core Rust implementation.
- `crypto-tree/wasm`: WASM bindings and browser demo.

## License

MIT
