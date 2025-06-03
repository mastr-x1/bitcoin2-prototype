use crate::blockchain::{Blockchain, Block};
use crate::consensus::randomx::RandomXMiner;
use crate::network::p2p::P2PNetwork;
use crate::crypto::quantum_sign::QuantumSigner;
use tokio::sync::Mutex;
use std::sync::Arc;
use std::time::Duration;

pub struct Node {
    pub blockchain: Arc<Mutex<Blockchain>>,
    pub p2p_network: Arc<Mutex<P2PNetwork>>,
    pub miner: RandomXMiner,
    pub wallet: QuantumSigner,
    pub mining_enabled: bool,
}

impl Node {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let blockchain = Arc::new(Mutex::new(Blockchain::new()));
        let p2p_network = Arc::new(Mutex::new(P2PNetwork::new(blockchain.clone()).await?));
        let miner = RandomXMiner::new(b"bitcoin2-seed");
        let wallet = QuantumSigner::new("Dilithium2")?;
        let mining_enabled = true;
        
        Ok(Self {
            blockchain,
            p2p_network,
            miner,
            wallet,
            mining_enabled,
        })
    }
    
    pub async fn start(&self) {
        // Start mining loop
        if self.mining_enabled {
            let blockchain = self.blockchain.clone();
            let p2p_network = self.p2p_network.clone();
            let miner = self.miner.clone();
            
            tokio::spawn(async move {
                loop {
                    {
                        let mut chain = blockchain.lock().await;
                        if !chain.pending_transactions.is_empty() {
                            let last_block = chain.chain.last().unwrap().clone();
                            let mut new_block = Block::new(
                                chain.chain.len() as u64,
                                chain.pending_transactions.clone(),
                                last_block.hash.clone(),
                                last_block.difficulty,
                            );
                            
                            let header = format!(
                                "{}{}{:?}{}",
                                new_block.index,
                                new_block.timestamp,
                                new_block.transactions,
                                new_block.previous_hash,
                            ).into_bytes();
                            
                            let (nonce, hash) = miner.mine_block(&header, new_block.difficulty);
                            new_block.nonce = nonce;
                            new_block.hash = hex::encode(hash);
                            
                            chain.chain.push(new_block.clone());
                            chain.pending_transactions = vec![];
                            
                            // Broadcast new block
                            p2p_network.lock().await.broadcast_block(new_block).await;
                        }
                    }
                    
                    tokio::time::sleep(Duration::from_secs(10)).await;
                }
            });
        }
        
        // Start network event loop
        let p2p_network = self.p2p_network.clone();
        tokio::spawn(async move {
            p2p_network.lock().await.start().await;
        });
    }
    
    pub async fn create_transaction(&self, recipient: &str, amount: f64) {
        let mut chain = self.blockchain.lock().await;
        let sender = hex::encode(self.wallet.public_key());
        
        let message = format!("{}{}{}", sender, recipient, amount);
        let signature = self.wallet.sign(message.as_bytes())
            .expect("Failed to sign transaction");
        
        let tx = Transaction {
            sender,
            recipient: recipient.to_string(),
            amount,
            signature: hex::encode(signature),
            public_key: hex::encode(self.wallet.public_key()),
        };
        
        chain.add_transaction(tx.clone());
        self.p2p_network.lock().await.broadcast_transaction(tx).await;
    }
}