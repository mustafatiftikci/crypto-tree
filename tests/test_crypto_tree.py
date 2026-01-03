import unittest
import random
from src.crypto_tree import CryptoBinaryTree, CryptoTreeNode

class TestCryptoTree(unittest.TestCase):
    
    def setUp(self):
        self.tree = CryptoBinaryTree()
        
    def test_insert_single(self):
        tx = {"id": "tx_001", "from": "Alice", "to": "Bob", "amount": 100}
        self.assertTrue(self.tree.insert(tx))
        self.assertEqual(len(self.tree), 1)
        self.assertIsNotNone(self.tree.search("tx_001"))
    
    def test_duplicate_insert(self):
        tx = {"id": "tx_001", "from": "Alice", "to": "Bob", "amount": 100}
        self.assertTrue(self.tree.insert(tx))
        self.assertFalse(self.tree.insert(tx))  # Duplicate
        self.assertEqual(len(self.tree), 1)
    
    def test_search_nonexistent(self):
        tx = {"id": "tx_001", "from": "Alice", "to": "Bob", "amount": 100}
        self.tree.insert(tx)
        self.assertIsNone(self.tree.search("tx_999"))
    
    def test_integrity_after_insert(self):
        transactions = [
            {"id": "tx_003", "from": "Bob", "to": "Charlie", "amount": 50},
            {"id": "tx_001", "from": "Alice", "to": "Bob", "amount": 100},
            {"id": "tx_005", "from": "Charlie", "to": "Dave", "amount": 25},
        ]
        
        for tx in transactions:
            self.tree.insert(tx)
        
        self.assertTrue(self.tree.verify_integrity())
    
    def test_proof_of_inclusion(self):
        transactions = [
            {"id": "tx_005", "from": "Alice", "to": "Bob", "amount": 100},
            {"id": "tx_003", "from": "Bob", "to": "Charlie", "amount": 50},
            {"id": "tx_007", "from": "Charlie", "to": "Dave", "amount": 25},
            {"id": "tx_001", "from": "Dave", "to": "Eve", "amount": 75},
            {"id": "tx_009", "from": "Eve", "to": "Frank", "amount": 30},
        ]
        
        for tx in transactions:
            self.tree.insert(tx)
        
        proof = self.tree.get_proof_of_inclusion("tx_003")
        self.assertIsNotNone(proof)
        self.assertGreater(len(proof), 0)
        
        # Verify proof structure
        for item in proof:
            self.assertIn("side", item)
            self.assertIn("hash", item)
            self.assertIsInstance(item["hash"], str)
            self.assertEqual(len(item["hash"]), 64)  # SHA-256
    
    def test_large_tree_integrity(self):
        # Test with 1000 transactions
        transactions = [{"id": f"tx_{i:04d}", "from": f"user_{i}", "to": f"recipient_{i}", "amount": i} 
                       for i in range(1000)]
        
        for tx in transactions:
            self.tree.insert(tx)
        
        self.assertEqual(len(self.tree), 1000)
        self.assertTrue(self.tree.verify_integrity())
        
        # Test search on random transaction
        target = random.choice(transactions)["id"]
        result = self.tree.search(target)
        self.assertIsNotNone(result)
        self.assertEqual(result["id"], target)
    
    def test_avl_balance(self):
        # Insert in sorted order to force imbalance
        transactions = [{"id": f"tx_{i}", "from": "A", "to": "B", "amount": i} 
                       for i in range(100)]
        
        for tx in transactions:
            self.tree.insert(tx)
        
        # AVL should keep tree balanced
        # We can't easily check height, but we can verify no crash and integrity
        self.assertTrue(self.tree.verify_integrity())
        
        # Search should still be fast (no degeneration to O(n))
        result = self.tree.search("tx_50")
        self.assertIsNotNone(result)
        
if __name__ == '__main__':
    unittest.main()