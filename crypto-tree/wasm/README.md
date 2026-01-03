crypto-tree/wasm/README.md
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

2.  **Copy the UI**:
    ```bash
    cp index.html pkg/
    ```

3.  **Serve**:
    ```bash
    serve -s pkg
    ```

4.  **Open**:
    Navigate to `http://localhost:5000` (or the port shown by your server).

## Structure
- `src/lib.rs`: WASM bindings for the Rust core.
- `index.html`: Vanilla JS UI for interacting with the WASM module.

### ✅ Bonus: Automate It (Optional)

You can make this even smoother by adding a `build` script to `Cargo.toml` or a `package.json` in `wasm/`:

```json
crypto-tree/wasm/package.json
{
  "name": "crypto-tree-wasm",
  "version": "0.1.0",
  "scripts": {
    "build": "wasm-pack build --target web",
    "start": "npm run build && cp index.html pkg/ && serve -s pkg"
  },
  "devDependencies": {
    "serve": "^14.2.0"
  }
}
```

Then users can just run:

```bash
npm install
npm start
```