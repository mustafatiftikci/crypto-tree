use crypto_tree::{CryptoBinaryTree, Transaction, ProofStep};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct CryptoTreeWasm {
    tree: CryptoBinaryTree,
}

#[wasm_bindgen]
impl CryptoTreeWasm {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            tree: CryptoBinaryTree::new(),
        }
    }

    #[wasm_bindgen]
    pub fn insert(&mut self, id: &str, from: &str, to: &str, amount: u64, timestamp: Option<u64>) -> bool {
        let tx = Transaction {
            id: id.to_string(),
            from: from.to_string(),
            to: to.to_string(),
            amount,
            timestamp,
        };
        self.tree.insert(tx)
    }

    #[wasm_bindgen]
    pub fn search(&self, id: &str) -> Option<JsValue> {
        self.tree.search(id).map(|tx| {
            serde_wasm_bindgen::to_value(&tx).unwrap()
        })
    }

    #[wasm_bindgen]
    pub fn get_proof_of_inclusion(&self, id: &str) -> Option<JsValue> {
        self.tree.get_proof_of_inclusion(id).map(|proof| {
            serde_wasm_bindgen::to_value(&proof).unwrap()
        })
    }

    #[wasm_bindgen]
    pub fn verify_integrity(&self) -> bool {
        self.tree.verify_integrity()
    }

    #[wasm_bindgen]
    pub fn merkle_root(&self) -> String {
        self.tree.merkle_root().to_string()
    }

    #[wasm_bindgen]
    pub fn len(&self) -> usize {
        self.tree.len()
    }
}

#[wasm_bindgen]
pub fn greet() -> String {
    "Hello from CryptoTree WASM!".to_string()
}