use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u128,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
    pub difficulty: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub sender: String,
    pub recipient: String,
    pub amount: f64,
    pub signature: String,
    pub public_key: String,
}

impl Block {
    pub fn new(index: u64, transactions: Vec<Transaction>, previous_hash: String, difficulty: u32) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
            
        let mut block = Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            hash: String::new(),
            nonce: 0,
            difficulty,
        };
        
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let data = format!(
            "{}{}{:?}{}{}",
            self.index,
            self.timestamp,
            self.transactions,
            self.previous_hash,
            self.nonce
        );
        
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }

    pub fn mine(&mut self) {
        let target = "0".repeat(self.difficulty as usize);
        while &self.hash[0..self.difficulty as usize] != target {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
    }
}

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub pending_transactions: Vec<Transaction>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block::new(
            0,
            vec![],
            "0".to_string(),
            4 // difficulty
        );
        
        Blockchain {
            chain: vec![genesis_block],
            pending_transactions: vec![],
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.pending_transactions.push(transaction);
    }

    pub fn mine_pending_transactions(&mut self, miner_address: &str) {
        let mut block = Block::new(
            self.chain.len() as u64,
            self.pending_transactions.clone(),
            self.chain.last().unwrap().hash.clone(),
            self.chain.last().unwrap().difficulty,
        );
        
        block.mine();
        self.chain.push(block);
        self.pending_transactions = vec![];
    }

    pub fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];
            
            if current.previous_hash != previous.hash {
                return false;
            }
            
            if current.hash != current.calculate_hash() {
                return false;
            }
        }
        true
    }
}