import hashlib
import json
from typing import Optional, Dict, Any, List, Tuple
from .node import CryptoTreeNode

class CryptoBinaryTree:
    """
    A self-balancing AVL tree for cryptographic transaction indexing.
    
    Supports:
    - O(log n) insertion and search
    - Merkle root commitment
    - Cryptographic inclusion proofs
    - Full tree integrity verification
    
    Unlike BST, this uses AVL rotations to guarantee O(log n) worst-case performance.
    """
    
    def __init__(self):
        self.root: Optional[CryptoTreeNode] = None
        self.size = 0
        self.merkle_root = "0"
    
    def insert(self, transaction: Dict[str, Any]) -> bool:
        """
        Insert a transaction into the tree. Returns True if inserted, False if duplicate.
        
        Uses AVL rotations to maintain balance after insertion.
        """
        if not isinstance(transaction, dict) or 'id' not in transaction:
            raise ValueError("Transaction must be a dict with 'id' field")
        
        if not self.root:
            self.root = CryptoTreeNode(transaction)
            self.size = 1
            self._update_merkle_root()
            return True
        
        # Insert recursively and balance
        inserted = self._insert_recursive(self.root, transaction)
        if inserted:
            self.size += 1
            self._update_merkle_root()
        return inserted
    
    def _insert_recursive(self, node: CryptoTreeNode, transaction: Dict[str, Any]) -> bool:
        tx_id = transaction['id']
        node_tx_id = node.transaction['id']
        
        # Avoid duplicates
        if tx_id == node_tx_id:
            return False
        
        # Insert into left subtree
        if tx_id < node_tx_id:
            if node.left is None:
                node.left = CryptoTreeNode(transaction)
                node.update_hash()
                self._update_height(node)
                self._balance_node(node)
                return True
            else:
                inserted = self._insert_recursive(node.left, transaction)
                if inserted:
                    node.update_hash()
                    self._update_height(node)
                    self._balance_node(node)
                return inserted
        
        # Insert into right subtree
        elif tx_id > node_tx_id:
            if node.right is None:
                node.right = CryptoTreeNode(transaction)
                node.update_hash()
                self._update_height(node)
                self._balance_node(node)
                return True
            else:
                inserted = self._insert_recursive(node.right, transaction)
                if inserted:
                    node.update_hash()
                    self._update_height(node)
                    self._balance_node(node)
                return inserted
        
        return False
    
    def _update_height(self, node: CryptoTreeNode):
        """Update height of node based on children."""
        left_height = node.left.height if node.left else 0
        right_height = node.right.height if node.right else 0
        node.height = max(left_height, right_height) + 1
    
    def _balance_node(self, node: CryptoTreeNode):
        """Balance the node using AVL rotations if needed."""
        balance = node.get_balance_factor()
        
        # Left heavy
        if balance > 1:
            if node.left and node.left.get_balance_factor() < 0:
                # Left-Right case
                node.left = self._rotate_left(node.left)
            # Left-Left case
            node = self._rotate_right(node)
        
        # Right heavy
        elif balance < -1:
            if node.right and node.right.get_balance_factor() > 0:
                # Right-Left case
                node.right = self._rotate_right(node.right)
            # Right-Right case
            node = self._rotate_left(node)
        
        return node
    
    def _rotate_left(self, z: CryptoTreeNode) -> CryptoTreeNode:
        """Perform left rotation on node z."""
        y = z.right
        T2 = y.left
        
        # Perform rotation
        y.left = z
        z.right = T2
        
        # Update heights
        self._update_height(z)
        self._update_height(y)
        
        # Update hashes
        z.update_hash()
        y.update_hash()
        
        return y
    
    def _rotate_right(self, z: CryptoTreeNode) -> CryptoTreeNode:
        """Perform right rotation on node z."""
        y = z.left
        T3 = y.right
        
        # Perform rotation
        y.right = z
        z.left = T3
        
        # Update heights
        self._update_height(z)
        self._update_height(y)
        
        # Update hashes
        z.update_hash()
        y.update_hash()
        
        return y
    
    def search(self, tx_id: str) -> Optional[Dict[str, Any]]:
        """
        Search for a transaction by ID. Returns transaction data or None.
        Time complexity: O(log n)
        """
        return self._search_recursive(self.root, tx_id)
    
    def _search_recursive(self, node: Optional[CryptoTreeNode], tx_id: str) -> Optional[Dict[str, Any]]:
        if not node:
            return None
        
        node_tx_id = node.transaction['id']
        
        if tx_id == node_tx_id:
            return node.transaction
        elif tx_id < node_tx_id:
            return self._search_recursive(node.left, tx_id)
        else:
            return self._search_recursive(node.right, tx_id)
    
    def verify_integrity(self) -> bool:
        """
        Verify the entire tree's cryptographic integrity.
        Returns True if all hashes are valid.
        """
        return self._verify_recursive(self.root)
    
    def _verify_recursive(self, node: Optional[CryptoTreeNode]) -> bool:
        if not node:
            return True
        
        # Recompute hash and compare
        expected_hash = node.calculate_hash()
        if node.hash != expected_hash:
            print(f"❌ Hash mismatch at transaction {node.transaction['id']}")
            return False
        
        # Recursively verify children
        return (self._verify_recursive(node.left) and 
                self._verify_recursive(node.right))
    
    def _update_merkle_root(self):
        """Update the Merkle root to be the hash of the root node."""
        self.merkle_root = self.root.hash if self.root else "0"
    
    def get_proof_of_inclusion(self, tx_id: str) -> Optional[List[Dict[str, str]]]:
        """
        Get a cryptographic proof that a transaction exists in the tree.
        
        Returns a list of {"side": "left|right", "hash": "..."} objects
        that can be used to reconstruct the path from root to target.
        
        This enables light clients to verify inclusion without the full tree.
        """
        proof = []
        found = self._get_proof_recursive(self.root, tx_id, proof)
        return proof if found else None
    
    def _get_proof_recursive(
        self, 
        node: Optional[CryptoTreeNode], 
        tx_id: str, 
        proof: List[Dict[str, str]]
    ) -> bool:
        if not node:
            return False
        
        node_tx_id = node.transaction['id']
        
        if tx_id == node_tx_id:
            return True
        
        # Go left
        if tx_id < node_tx_id:
            if node.right:
                proof.append({"side": "right", "hash": node.right.hash})
            return self._get_proof_recursive(node.left, tx_id, proof)
        
        # Go right
        else:
            if node.left:
                proof.append({"side": "left", "hash": node.left.hash})
            return self._get_proof_recursive(node.right, tx_id, proof)
    
    def __len__(self):
        return self.size
    
    def __repr__(self):
        return f"CryptoBinaryTree(size={self.size}, merkle_root={self.merkle_root[:16]}...)"