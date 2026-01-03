# CryptoTree - Rust Crate

A Rust implementation of a Merkle AVL Tree, designed for verifiable data storage and retrieval.

## Features

- **Standard AVL Tree**: O(log n) operations.
- **Merkle Proofs**: Cryptographic proofs of inclusion for any node.
- **Safe Rust**: Implemented without `unsafe` blocks.
- **WASM Compatible**: Ready for compilation to `wasm32-unknown-unknown`.

## Usage

```rust
use crypto_tree::CryptoBinaryTree;

let mut tree = CryptoBinaryTree::new();
tree.insert(tx);

// O(log n) search
if let Some(found) = tree.search("tx_id") {
    println!("{:?}", found);
}
```

## Build

```bash
cargo build --release
cargo test
cargo clippy
```

## License

MIT
