# Contributing to CryptoTree

Thank you for considering contributing to CryptoTree! This project aims to build the first decentralized, searchable, cryptographic tree for blockchain transactions.

## 🚀 How to Contribute

### 1. Find an Issue

Check the [issues page](https://github.com/yourusername/crypto-tree/issues) for:
- "help wanted"
- "good first issue"
- "enhancement"

### 2. Fork & Clone

```bash
git clone https://github.com/yourusername/crypto-tree.git
cd crypto-tree
```

### 3. Set Up Development Environment

```bash
python -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate
pip install -e .[dev]
```

### 4. Run Tests

```bash
python -m unittest discover tests
```

### 5. Code Style

- Use **black** for formatting
- Use **ruff** for linting
- Follow **PEP 8**
- Write docstrings for all public functions

```bash
black src/
ruff check src/
```

### 6. Write Tests

Add tests in `tests/` for any new feature or bug fix.

### 7. Commit & Push

Use clear, imperative commit messages:

```
feat: add multi-index support for from_address
fix: handle null timestamp in hash computation
docs: update spec.md with proof verification algorithm
```

### 8. Open Pull Request

- Title: `[feat/fix/docs] Brief description`
- Description: Explain what you changed and why
- Link to issue if applicable

## 🧠 Design Philosophy

- **Security first**: No trust assumptions
- **Determinism**: Same input → same hash
- **Simplicity**: Avoid over-engineering
- **Extensibility**: Designed for multi-index, WASM, Rust

## 📜 License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

> 💡 *This is not just a library — it’s a new blockchain primitive. Your code could become part of the next generation of decentralized systems.*