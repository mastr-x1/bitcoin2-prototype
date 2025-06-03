use randomx_rs::{RandomXCache, RandomXDataset, RandomXFlags, RandomXVM};
use std::sync::Arc;

pub struct RandomXMiner {
    dataset: Arc<RandomXDataset>,
}

impl RandomXMiner {
    pub fn new(seed: &[u8]) -> Self {
        let flags = RandomXFlags::get_recommended_flags();
        let cache = RandomXCache::new(flags, seed).expect("Failed to create cache");
        let dataset = RandomXDataset::new(flags, cache, 0, 1).expect("Failed to create dataset");
        
        Self {
            dataset: Arc::new(dataset),
        }
    }

    pub fn mine_block(&self, block_header: &[u8], difficulty: u32) -> (u64, Vec<u8>) {
        let vm = RandomXVM::new(RandomXFlags::get_recommended_flags(), self.dataset.clone())
            .expect("Failed to create VM");
            
        let target = Self::calculate_target(difficulty);
        let mut nonce: u64 = 0;
        
        loop {
            let mut input = Vec::with_capacity(block_header.len() + 8);
            input.extend_from_slice(block_header);
            input.extend_from_slice(&nonce.to_le_bytes());
            
            let hash = vm.calculate_hash(&input);
            
            if self.is_valid_hash(&hash, &target) {
                return (nonce, hash);
            }
            
            nonce += 1;
        }
    }

    fn calculate_target(difficulty: u32) -> [u8; 32] {
        let mut target = [0xff; 32];
        let zero_bytes = (difficulty / 8) as usize;
        let remainder = (difficulty % 8) as u8;
        
        for i in 0..zero_bytes {
            if i < 32 {
                target[i] = 0;
            }
        }
        
        if zero_bytes < 32 {
            target[zero_bytes] = 0xff >> remainder;
        }
        
        target
    }

    fn is_valid_hash(&self, hash: &[u8], target: &[u8; 32]) -> bool {
        for i in 0..32 {
            if hash[i] > target[i] {
                return false;
            } else if hash[i] < target[i] {
                return true;
            }
        }
        true
    }
}