 # CryptoTree - Rust Implementation

> **The first self-balancing, cryptographic AVL tree for on-chain transaction search — O(log n) with Merkle proofs.**

This is the **Rust implementation** of the `crypto-tree` protocol, designed for high-performance, secure, and verifiable blockchain indexing.

## ✅ Features

- ✅ **AVL Tree**: Guaranteed O(log n) search, insert, delete
- ✅ **Cryptographic Hashing**: SHA-256 with deterministic serialization
- ✅ **Merkle Root Commitment**: Updated on every insert
- ✅ **Inclusion Proofs**: Structured proofs for light clients
- ✅ **Full Integrity Verification**: Detect tampering at any node
- ✅ **Zero unsafe code**: 100% safe Rust
- ✅ **WASM-ready**: Compile to WebAssembly for browser use

## 🚀 Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
crypto_tree = { git = "https://github.com/yourusername/crypto-tree", branch = "main" }
```

### Example

```rust
use crypto_tree::{CryptoBinaryTree, Transaction};

fn main() {
    let mut tree = CryptoBinaryTree::new();
    
    let tx = Transaction {
        id: "tx_001".to_string(),
        from: "Alice".to_string(),
        to: "Bob".to_string(),
        amount: 100,
        timestamp: Some(1640995200),
    };
    
    tree.insert(tx);
    
    // Search
    if let Some(transaction) = tree.search("tx_001") {
        println!("Found: {} -> {}", transaction.from, transaction.to);
    }
    
    // Get proof of inclusion
    if let Some(proof) = tree.get_proof_of_inclusion("tx_001") {
        println!("Proof has {} steps", proof.len());
    }
    
    // Verify integrity
    assert!(tree.verify_integrity());
    println!("Merkle root: {}", tree.merkle_root());
}
```

## 🔐 Security

- All hashes are computed using **SHA-256**
- Serialization uses **canonical JSON** (sorted keys)
- No mutable fields after insertion
- No external dependencies beyond `serde`, `sha2`

## 📦 Build & Test

```bash
# Build
cargo build --release

# Run tests
cargo test

# Check for safety
cargo clippy

# Format
cargo fmt
```

## 🌐 Future: WebAssembly (WASM)

This crate compiles to WASM with:

```bash
cargo build --target wasm32-unknown-unknown --release
```

Use with `wasm-bindgen` to integrate into browser wallets or dApps.

## 🤝 Contributing

See [../CONTRIBUTING.md](../CONTRIBUTING.md)

## 📜 License

MIT
