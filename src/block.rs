use sha2::{Digest, Sha256};
use serde::{Serialize, Deserialize};
use md5;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: u128,
    pub data: String,
    pub previous_hash: String,
    pub nonce: u64,
    pub hash: String,
}
const DIFFICULTY_PREFIX: &str = "0000"; // Adjust difficulty as needed

impl Block {
    pub fn is_valid(&self, last_block: &Block) -> bool {
        if self.previous_hash != last_block.hash {
            return false;
        }

        let valid_hash = format!("{:x}", md5::compute(format!("{}{}{}{}", self.index, self.timestamp, self.data, self.nonce)));
        if self.hash != valid_hash || &self.hash[..2] != "00" {
            return false;
        }

        true
    }
    pub fn mine_block(last_block: &Block, data: String) -> Block {
        let index = last_block.index + 1;
        let timestamp = 0; // Replace with actual timestamp
        let previous_hash = last_block.hash.clone();
        let mut nonce = 0;
        let mut hash = String::new();

        // Simple Proof of Work loop (you can improve this)
        loop {
            hash = format!("{:x}", md5::compute(format!("{}{}{}{}", index, timestamp, data, nonce)));
            if &hash[..2] == "00" {
                break;
            }
            nonce += 1;
        }

        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
            nonce,
        }
    }

    pub fn genesis() -> Self {
        Block {
            index: 0,
            timestamp: 0,
            data: "Genesis Block".to_string(),
            previous_hash: "0".to_string(),
            hash: "GENESIS_HASH".to_string(),
            nonce: 0,
        }
    }
    pub fn new(index: u64, timestamp: u128, data: String, previous_hash: String) -> Self {
        let mut block = Block {
            index,
            timestamp,
            data,
            previous_hash,
            nonce: 0,
            hash: String::new(),
        };
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let input = format!(
            "{}{}{}{}{}",
            self.index, self.timestamp, self.data, self.previous_hash, self.nonce
        );
        let mut hasher = Sha256::new();
        hasher.update(input);
        format!("{:x}", hasher.finalize())
    }


}
