"""
CryptoTree: A cryptographically secure, self-balancing AVL tree for blockchain transaction indexing.

Enables O(log n) search, Merkle proofs, and on-chain integrity verification.
"""

from .crypto_tree import CryptoBinaryTree
from .node import CryptoTreeNode

__all__ = [
    "CryptoBinaryTree",
    "CryptoTreeNode",
]

__version__ = "0.1.0"
__author__ = "Your Name"
__license__ = "MIT"