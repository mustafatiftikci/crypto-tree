# CryptoTree: Searchable Merkle Binary Tree for Blockchain

> **A cryptographically secure, self-balancing, on-chain searchable tree for fast transaction lookups — O(log n) with verifiable proofs.**

![CryptoTree Architecture](https://via.placeholder.com/800x400?text=CryptoTree+Architecture+Diagram)

## 🚀 Why This Exists

Blockchain is a linked list. Searching for a transaction is **O(n)** — requiring scanning millions of blocks.

Bitcoin and Ethereum solve this with **off-chain indexes** (TheGraph, Dune, Nansen) — but they’re centralized, trust-dependent, and expensive.

**CryptoTree** is the first **on-chain, self-balancing, Merkle-verified binary search tree** designed for transactions. It enables:

- ✅ O(log n) transaction search
- ✅ Cryptographic proofs of inclusion (for light clients)
- ✅ Decentralized alternative to TheGraph
- ✅ Built-in integrity verification
- ✅ Multi-index support (tx_id, from, to, timestamp)

This is not a library — it’s a **new blockchain data primitive**.

---

## 📊 Performance Comparison

| Search Type | Operations (1M tx) | Time Estimate |
|-------------|-------------------|---------------|
| Blockchain (linked list) | 1,000,000 | ~10s |
| CryptoTree (AVL) | ~20 | ~0.001s |
| Bitcoin Index (hashmap) | 1 | ~0.0001s |

> ⚠️ Bitcoin’s index is fast — but **centralized**. CryptoTree is **decentralized and verifiable**.

---

## 📦 Installation

```bash
pip install crypto-tree
```

## 🛠️ Usage

```python
from crypto_tree import CryptoBinaryTree

# Initialize
tree = CryptoBinaryTree()

# Insert transactions
transactions = [
    {"id": "tx_001", "from": "Alice", "to": "Bob", "amount": 100, "timestamp": 1640995200},
    {"id": "tx_002", "from": "Bob", "to": "Charlie", "amount": 50, "timestamp": 1640995300},
]

for tx in transactions:
    tree.insert(tx)

# Search by tx_id (O(log n))
result = tree.search("tx_001")
print(result)  # {'id': 'tx_001', ...}

# Get cryptographic proof of inclusion
proof = tree.get_proof_of_inclusion("tx_001")
print(f"Proof has {len(proof)} hashes")

# Verify integrity of entire tree
is_valid = tree.verify_integrity()
print(f"Tree integrity: {is_valid}")
```

## 🔐 Security & Design

- Uses **SHA-256** for cryptographic hashing
- **AVL tree** ensures O(log n) worst-case performance
- Hashes include: transaction data + child hashes + height
- Merkle root updated on every insert
- Proofs are structured: `{ path: ["left", "right"], siblings: ["hash1", "hash2"] }`

## 📚 Technical Deep Dive

See [docs/spec.md](docs/spec.md) for formal specification, including:

- AVL balancing rotations
- Hash serialization (CBOR)
- Proof verification algorithm
- Multi-index architecture

## 🌍 Roadmap

- [x] Python implementation (AVL + Merkle)
- [ ] Rust port (for performance & WASM)
- [ ] WebAssembly browser demo
- [ ] Ethereum L2 integration
- [ ] Bitcoin index compatibility layer

## 🤝 Contribute

PRs welcome! See [CONTRIBUTING.md](CONTRIBUTING.md).

## 📜 License

MIT
