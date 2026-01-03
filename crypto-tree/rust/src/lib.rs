use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};

/// A transaction in the CryptoTree
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Transaction {
    pub id: String,
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub timestamp: Option<u64>,
}

/// A node in the AVL tree
#[derive(Debug)]
pub struct CryptoTreeNode {
    pub transaction: Transaction,
    pub left: Option<Box<CryptoTreeNode>>,
    pub right: Option<Box<CryptoTreeNode>>,
    pub height: i32,
    pub hash: String, // SHA-256 hex string
}

impl CryptoTreeNode {
    pub fn new(transaction: Transaction) -> Self {
        let hash = Self::calculate_hash(&transaction, &None, &None, 1);
        Self {
            transaction,
            left: None,
            right: None,
            height: 1,
            hash,
        }
    }

    fn calculate_hash(transaction: &Transaction, left_hash: &Option<String>, right_hash: &Option<String>, height: i32) -> String {
        let zero_string = "0".to_string();
        let left_hash_str = left_hash.as_ref().unwrap_or(&zero_string);
        let right_hash_str = right_hash.as_ref().unwrap_or(&zero_string);
        
        let node_data = CryptoTreeNodeData {
            transaction: transaction.clone(),
            left_hash: left_hash_str.clone(),
            right_hash: right_hash_str.clone(),
            height, // Use the actual height instead of hardcoded 1
        };
        
        let json_str = serde_json::to_string(&node_data).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(json_str.as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    fn update_hash(&mut self, left_hash: &Option<String>, right_hash: &Option<String>) {
        self.hash = Self::calculate_hash(&self.transaction, left_hash, right_hash, self.height);
    }

    fn get_balance_factor(&self) -> i32 {
        let left_height = self.left.as_ref().map_or(0, |n| n.height);
        let right_height = self.right.as_ref().map_or(0, |n| n.height);
        left_height - right_height
    }

    fn update_height(&mut self) {
        let left_height = self.left.as_ref().map_or(0, |n| n.height);
        let right_height = self.right.as_ref().map_or(0, |n| n.height);
        self.height = std::cmp::max(left_height, right_height) + 1;
    }
}

/// Data structure used for deterministic serialization
#[derive(Serialize, Deserialize, Clone, Debug)]
struct CryptoTreeNodeData {
    transaction: Transaction,
    left_hash: String,
    right_hash: String,
    height: i32,
}

/// The main CryptoTree structure
#[derive(Debug)]
pub struct CryptoBinaryTree {
    root: Option<Box<CryptoTreeNode>>,
    size: usize,
    merkle_root: String,
}

impl Default for CryptoBinaryTree {
    fn default() -> Self {
        Self::new()
    }
}

impl CryptoBinaryTree {
    pub fn new() -> Self {
        Self {
            root: None,
            size: 0,
            merkle_root: "0".to_string(),
        }
    }

    pub fn insert(&mut self, transaction: Transaction) -> bool {
        if self.root.is_none() {
            self.root = Some(Box::new(CryptoTreeNode::new(transaction)));
            self.size = 1;
            self._update_merkle_root();
            return true;
        }

        let mut inserted = false;
        let root = std::mem::take(&mut self.root);
        self.root = Self::_insert_recursive(root, transaction, &mut inserted);
        if inserted {
            self.size += 1;
            self._update_merkle_root();
        }
        inserted
    }

    fn _insert_recursive(
        node: Option<Box<CryptoTreeNode>>, 
        transaction: Transaction, 
        inserted: &mut bool
    ) -> Option<Box<CryptoTreeNode>> {
        match node {
            None => {
                *inserted = true;
                Some(Box::new(CryptoTreeNode::new(transaction)))
            }
            Some(mut n) => {
                let tx_id = &transaction.id;
                let node_tx_id = &n.transaction.id;

                if tx_id == node_tx_id {
                    // Duplicate
                    return Some(n);
                }

                if tx_id < node_tx_id {
                    n.left = Self::_insert_recursive(n.left, transaction.clone(), inserted);
                } else {
                    n.right = Self::_insert_recursive(n.right, transaction.clone(), inserted);
                }

                if *inserted {
                    // Update height first
                    n.update_height();
                    
                    // Balance the node
                    n = Self::_balance_node(n);
                    
                    // Now update the hash after balancing
                    let left_hash = n.left.as_ref().map(|l| l.hash.clone());
                    let right_hash = n.right.as_ref().map(|r| r.hash.clone());
                    n.update_hash(&left_hash, &right_hash);
                }

                Some(n)
            }
        }
    }

    fn _balance_node(mut node: Box<CryptoTreeNode>) -> Box<CryptoTreeNode> {
        let balance = node.get_balance_factor();

        // Left heavy
        if balance > 1 {
            if node.left.as_ref().map_or(0, |l| l.get_balance_factor()) < 0 {
                // Left-Right case
                node.left = Some(Self::_rotate_left(Box::new(*node.left.unwrap())));
            }
            // Left-Left case
            node = Self::_rotate_right(node); // ✅ Fixed: return is Box, assign directly
        }
        // Right heavy
        else if balance < -1 {
            if node.right.as_ref().map_or(0, |r| r.get_balance_factor()) > 0 {
                // Right-Left case
                node.right = Some(Self::_rotate_right(Box::new(*node.right.unwrap())));
            }
            // Right-Right case
            node = Self::_rotate_left(node); // ✅ Fixed: return is Box, assign directly
        }

        node
    }

    fn _rotate_left(mut z: Box<CryptoTreeNode>) -> Box<CryptoTreeNode> {
        // Update heights before rotation
        z.update_height();
        
        // Get the right child (y)
        let mut y = z.right.take().unwrap();
        y.update_height();
        
        // Perform the rotation
        let t2 = y.left.take();
        z.right = t2;
        y.left = Some(z);
        
        // Update heights after rotation
        y.left.as_mut().unwrap().update_height();
        y.update_height();
        
        // Update hashes after rotation using current children
        let z_node = y.left.as_mut().unwrap();
        let z_left_hash = z_node.left.as_ref().map(|l| l.hash.clone());
        let z_right_hash = z_node.right.as_ref().map(|r| r.hash.clone());
        z_node.update_hash(&z_left_hash, &z_right_hash);
        
        let y_left_hash = y.left.as_ref().map(|l| l.hash.clone());
        let y_right_hash = y.right.as_ref().map(|r| r.hash.clone());
        y.update_hash(&y_left_hash, &y_right_hash);

        y
    }

    fn _rotate_right(mut z: Box<CryptoTreeNode>) -> Box<CryptoTreeNode> {
        // Update heights before rotation
        z.update_height();
        
        // Get the left child (y)
        let mut y = z.left.take().unwrap();
        y.update_height();
        
        // Perform the rotation
        let t3 = y.right.take();
        z.left = t3;
        y.right = Some(z);
        
        // Update heights after rotation
        y.right.as_mut().unwrap().update_height();
        y.update_height();
        
        // Update hashes after rotation using current children
        let z_node = y.right.as_mut().unwrap();
        let z_left_hash = z_node.left.as_ref().map(|l| l.hash.clone());
        let z_right_hash = z_node.right.as_ref().map(|r| r.hash.clone());
        z_node.update_hash(&z_left_hash, &z_right_hash);
        
        let y_left_hash = y.left.as_ref().map(|l| l.hash.clone());
        let y_right_hash = y.right.as_ref().map(|r| r.hash.clone());
        y.update_hash(&y_left_hash, &y_right_hash);

        y
    }

    pub fn search<'a>(&'a self, tx_id: &str) -> Option<&'a Transaction> {
        Self::_search_recursive(&self.root, tx_id)
    }

    fn _search_recursive<'a>(node: &'a Option<Box<CryptoTreeNode>>, tx_id: &str) -> Option<&'a Transaction> {
        match node {
            None => None,
            Some(n) => {
                if tx_id == n.transaction.id {
                    Some(&n.transaction)
                } else if tx_id < &n.transaction.id {
                    Self::_search_recursive(&n.left, tx_id)
                } else {
                    Self::_search_recursive(&n.right, tx_id)
                }
            }
        }
    }

    pub fn verify_integrity(&self) -> bool {
        Self::_verify_recursive(&self.root)
    }

    fn _verify_recursive(node: &Option<Box<CryptoTreeNode>>) -> bool {
        match node {
            None => true,
            Some(n) => {
                let left_hash = n.left.as_ref().map(|l| l.hash.clone());
                let right_hash = n.right.as_ref().map(|r| r.hash.clone());
                let expected_hash = CryptoTreeNode::calculate_hash(&n.transaction, &left_hash, &right_hash, n.height);
                if n.hash != expected_hash {
                    eprintln!("❌ Hash mismatch at transaction {}", n.transaction.id);
                    return false;
                }
                Self::_verify_recursive(&n.left) && Self::_verify_recursive(&n.right)
            }
        }
    }

    fn _update_merkle_root(&mut self) {
        self.merkle_root = self.root.as_ref().map(|n| n.hash.clone()).unwrap_or("0".to_string());
    }

    pub fn get_proof_of_inclusion(&self, tx_id: &str) -> Option<Vec<ProofStep>> {
        let mut proof = Vec::new();
        if Self::_get_proof_recursive(&self.root, tx_id, &mut proof) {
            Some(proof)
        } else {
            None
        }
    }

    fn _get_proof_recursive(node: &Option<Box<CryptoTreeNode>>, tx_id: &str, proof: &mut Vec<ProofStep>) -> bool {
        match node {
            None => false,
            Some(n) => {
                if tx_id == n.transaction.id {
                    true
                } else if tx_id < &n.transaction.id {
                    if let Some(ref right) = n.right {
                        proof.push(ProofStep {
                            side: "right".to_string(),
                            hash: right.hash.clone(),
                        });
                    }
                    Self::_get_proof_recursive(&n.left, tx_id, proof)
                } else {
                    if let Some(ref left) = n.left {
                        proof.push(ProofStep {
                            side: "left".to_string(),
                            hash: left.hash.clone(),
                        });
                    }
                    Self::_get_proof_recursive(&n.right, tx_id, proof)
                }
            }
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn merkle_root(&self) -> &str {
        &self.merkle_root
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProofStep {
    pub side: String, // "left" or "right"
    pub hash: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_single() {
        let mut tree = CryptoBinaryTree::new();
        let tx = Transaction {
            id: "tx_001".to_string(),
            from: "Alice".to_string(),
            to: "Bob".to_string(),
            amount: 100,
            timestamp: Some(1640995200),
        };
        assert!(tree.insert(tx));
        assert_eq!(tree.len(), 1);
        assert!(tree.search("tx_001").is_some());
    }

    #[test]
    fn test_duplicate_insert() {
        let mut tree = CryptoBinaryTree::new();
        let tx = Transaction {
            id: "tx_001".to_string(),
            from: "Alice".to_string(),
            to: "Bob".to_string(),
            amount: 100,
            timestamp: Some(1640995200),
        };
        assert!(tree.insert(tx.clone())); // First insert - clone for ownership
        assert!(!tree.insert(tx)); // Duplicate - use original (now moved)
        assert_eq!(tree.len(), 1);
    }

    #[test]
    fn test_search_nonexistent() {
        let mut tree = CryptoBinaryTree::new();
        let tx = Transaction {
            id: "tx_001".to_string(),
            from: "Alice".to_string(),
            to: "Bob".to_string(),
            amount: 100,
            timestamp: Some(1640995200),
        };
        tree.insert(tx);
        assert!(tree.search("tx_999").is_none());
    }

    #[test]
    fn test_integrity_after_insert() {
        let mut tree = CryptoBinaryTree::new();
        let transactions = vec![
            Transaction {
                id: "tx_003".to_string(),
                from: "Bob".to_string(),
                to: "Charlie".to_string(),
                amount: 50,
                timestamp: Some(1640995300),
            },
            Transaction {
                id: "tx_001".to_string(),
                from: "Alice".to_string(),
                to: "Bob".to_string(),
                amount: 100,
                timestamp: Some(1640995200),
            },
            Transaction {
                id: "tx_005".to_string(),
                from: "Charlie".to_string(),
                to: "Dave".to_string(),
                amount: 25,
                timestamp: Some(1640995400),
            },
        ];

        for tx in transactions {
            tree.insert(tx);
        }

        assert!(tree.verify_integrity());
    }

    #[test]
    fn test_proof_of_inclusion() {
        let mut tree = CryptoBinaryTree::new();
        let transactions = vec![
            Transaction {
                id: "tx_005".to_string(),
                from: "Alice".to_string(),
                to: "Bob".to_string(),
                amount: 100,
                timestamp: Some(1640995200),
            },
            Transaction {
                id: "tx_003".to_string(),
                from: "Bob".to_string(),
                to: "Charlie".to_string(),
                amount: 50,
                timestamp: Some(1640995300),
            },
            Transaction {
                id: "tx_007".to_string(),
                from: "Charlie".to_string(),
                to: "Dave".to_string(),
                amount: 25,
                timestamp: Some(1640995400),
            },
            Transaction {
                id: "tx_001".to_string(),
                from: "Dave".to_string(),
                to: "Eve".to_string(),
                amount: 75,
                timestamp: Some(1640995500),
            },
            Transaction {
                id: "tx_009".to_string(),
                from: "Eve".to_string(),
                to: "Frank".to_string(),
                amount: 30,
                timestamp: Some(1640995600),
            },
        ];

        for tx in transactions {
            tree.insert(tx);
        }

        let proof = tree.get_proof_of_inclusion("tx_003");
        assert!(proof.is_some());
        let proof = proof.unwrap();
        assert!(!proof.is_empty());

        for step in &proof {
            assert!(step.side == "left" || step.side == "right");
            assert_eq!(step.hash.len(), 64); // SHA-256 hex
        }
    }

    #[test]
    fn test_avl_balance() {
        let mut tree = CryptoBinaryTree::new();
        // Insert in sorted order to force imbalance
        for i in 1..=100 {
            let tx = Transaction {
                id: format!("tx_{:03}", i),
                from: "A".to_string(),
                to: "B".to_string(),
                amount: i as u64,
                timestamp: Some(1640995200 + i),
            };
            tree.insert(tx);
        }
        assert_eq!(tree.len(), 100);
        assert!(tree.verify_integrity());
        assert!(tree.search("tx_050").is_some());
    }
}
