# CryptoTree: Formal Specification

## 1. Overview

**CryptoTree** is a self-balancing, cryptographic binary search tree designed for on-chain transaction indexing. It provides:

- O(log n) search, insertion, and deletion
- Cryptographic integrity via Merkle hashing
- Verifiable inclusion proofs for light clients
- Deterministic serialization for consensus

Unlike traditional blockchains (O(n) search), CryptoTree enables efficient querying without centralized indexes.

---

## 2. Data Structure

### 2.1 Node Structure

Each node contains:

| Field | Type | Description |
|-------|------|-------------|
| `transaction` | `dict` | Transaction data (must include `id`) |
| `timestamp` | `int` (optional) | Unix timestamp of transaction |
| `left` | `CryptoTreeNode` | Left child |
| `right` | `CryptoTreeNode` | Right child |
| `height` | `int` | Height of subtree (for AVL balancing) |
| `hash` | `str` (64-char hex) | SHA-256 hash of node data |

### 2.2 Hash Computation

The hash of a node is computed as:

```python
node_data = {
    "transaction": transaction,  # serialized as JSON
    "left_hash": left.hash if left else "0",
    "right_hash": right.hash if right else "0",
    "height": height
}

hash = SHA256(json.dumps(node_data, sort_keys=True, separators=(',', ':')))
```

> ✅ **Determinism is critical**: Keys are sorted, no whitespace, no comments.

### 2.3 AVL Balancing

CryptoTree uses **AVL tree** rotations to guarantee O(log n) worst-case performance:

- **Balance factor** = left_height - right_height
- If |balance factor| > 1 → rotate
- Four cases: Left-Left, Right-Right, Left-Right, Right-Left

Rotations update `height` and `hash` of affected nodes.

---

## 3. Operations

### 3.1 Insertion

1. Perform standard BST insertion by `tx_id`
2. Update `height` and `hash` of all ancestors
3. Check balance factor at each ancestor
4. Apply rotations if unbalanced
5. Update Merkle root

### 3.2 Search

1. Start at root
2. Compare `tx_id` with current node’s `tx_id`
3. Go left if smaller, right if larger
4. Return transaction if found, `None` otherwise

Time: **O(log n)**

### 3.3 Inclusion Proof

Returns a structured proof: `[{"side": "left|right", "hash": "..."}, ...]`

**Verification Algorithm**:

```python
def verify_proof(target_tx_id, proof, root_hash):
    current_hash = root_hash
    for step in proof:
        if step["side"] == "left":
            # Recompute hash assuming right sibling = step["hash"]
            current_hash = compute_node_hash(
                transaction=target_tx_id,
                left_hash="0",  # placeholder
                right_hash=step["hash"]
            )
        else:  # "right"
            current_hash = compute_node_hash(
                transaction=target_tx_id,
                left_hash=step["hash"],
                right_hash="0"
            )
    return current_hash == target_tx_hash
```

> ⚠️ **Note**: Full verification requires reconstructing the path from leaf to root using sibling hashes.

### 3.4 Integrity Verification

Recursively verify:
- Each node’s `hash` matches computed hash from children
- All subtrees are valid

Returns `True` if entire tree is cryptographically sound.

---

## 4. Merkle Root

- Updated after every insert
- `merkle_root = root.hash`
- Used as commitment in block headers or smart contracts

---

## 5. Multi-Index Support (Future)

Future versions will support:

| Index | Key | Use Case |
|-------|-----|----------|
| `tx_id` | `tx_id` | Transaction lookup |
| `from_addr` | `from` | Wallet history |
| `to_addr` | `to` | Wallet history |
| `timestamp` | `timestamp` | Time-range queries |

Each index is a separate AVL tree, all anchored to the same Merkle root.

---

## 6. Security Considerations

- **Hash collision resistance**: SHA-256 used
- **Serialization determinism**: JSON with sorted keys
- **Input validation**: `tx_id` must be string
- **No mutable fields**: All data immutable after insertion

---

## 7. Performance Benchmarks (Expected)

| Size | Search Time (AVL) | Search Time (Linked List) | Speedup |
|------|-------------------|---------------------------|---------|
| 1K | 0.01ms | 1.2ms | 120x |
| 10K | 0.02ms | 12ms | 600x |
| 100K | 0.03ms | 120ms | 4000x |
| 1M | 0.04ms | 1.2s | 30,000x |

---

## 8. References

- [AVL Tree - Knuth, TAOCP](https://en.wikipedia.org/wiki/AVL_tree)
- [Merkle Tree - Merkle, 1987](https://en.wikipedia.org/wiki/Merkle_tree)
- [Bitcoin Merkle Tree](https://en.bitcoin.it/wiki/Protocol_documentation#Merkle_Trees)
- [Ethereum Patricia Trie](https://ethereum.org/en/developers/docs/data-structures-and-encoding/patricia-merkle-trie/)

---

> ✅ This specification is designed for implementation, audit, and academic citation.