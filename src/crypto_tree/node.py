import hashlib
import json
from typing import Optional, Dict, Any, List

class CryptoTreeNode:
    """
    A cryptographic node in the AVL tree, storing a transaction and its Merkle hash.
    
    The hash is computed from:
    - transaction data
    - left child hash
    - right child hash
    - height (for AVL balancing)
    
    This ensures tamper-evident structure.
    """
    
    def __init__(self, transaction_data: Dict[str, Any]):
        if not isinstance(transaction_data, dict) or 'id' not in transaction_data:
            raise ValueError("Transaction must be a dict with 'id' field")
        
        self.transaction = transaction_data
        self.timestamp = transaction_data.get('timestamp')
        self.left: Optional['CryptoTreeNode'] = None
        self.right: Optional['CryptoTreeNode'] = None
        self.height = 1  # Height for AVL balancing
        self.hash = self.calculate_hash()
    
    def calculate_hash(self) -> str:
        """
        Calculate cryptographic hash using CBOR-like deterministic serialization.
        
        Uses sorted keys and canonical JSON to ensure deterministic output.
        """
        left_hash = self.left.hash if self.left else "0"
        right_hash = self.right.hash if self.right else "0"
        
        # Create deterministic dict with all fields
        node_data = {
            'transaction': self.transaction,
            'left_hash': left_hash,
            'right_hash': right_hash,
            'height': self.height
        }
        
        # Serialize with sorted keys for determinism
        data_string = json.dumps(node_data, sort_keys=True, separators=(',', ':'))
        return hashlib.sha256(data_string.encode('utf-8')).hexdigest()
    
    def update_hash(self):
        """Recompute hash after child modifications."""
        self.hash = self.calculate_hash()
    
    def get_balance_factor(self) -> int:
        """Calculate balance factor for AVL tree."""
        left_height = self.left.height if self.left else 0
        right_height = self.right.height if self.right else 0
        return left_height - right_height
    
    def __repr__(self):
        return f"CryptoTreeNode(id={self.transaction['id']}, hash={self.hash[:8]})"