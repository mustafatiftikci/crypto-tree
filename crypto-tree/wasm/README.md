# CryptoTree - WASM Demo

A browser-based demonstration of the `crypto-tree` Rust library, compiled to WebAssembly.

## Overview

This project demonstrates how a Merkle AVL Tree can run entirely client-side, allowing for:
1.  **Local Insertion**: Building a verifiable tree structure in the browser memory.
2.  **Verification**: Generating and verifying Merkle proofs without a backend server.

## Running Locally

### Prerequisites
- `rustup`
- `wasm-pack`: `cargo install wasm-pack`
- A static file server (e.g., `serve`): `npm install -g serve`

### Build & Run

1.  **Build WASM**:
    ```bash
    wasm-pack build --target web
    ```

2.  **Serve**:
    ```bash
    serve -s pkg
    ```

3.  **Open**:
    Navigate to `http://localhost:5000` (or the port shown by your server).

## Structure
- `src/lib.rs`: WASM bindings for the Rust core.
- `index.html`: Vanilla JS UI for interacting with the WASM module.