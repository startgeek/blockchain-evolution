use crate::block::Block;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            chain: vec![Block::genesis()],
        }
    }

    /// Adds a block if it's valid
    pub fn add_block(&mut self, data: String) -> Block {
        let last_block = self.chain.last().unwrap();
        let new_block = Block::mine_block(last_block, data);
        self.chain.push(new_block.clone());
        new_block
    }

    /// Receives a block from a peer, validates, and adds it if valid
    pub fn add_block_from_peer(&mut self, new_block: Block) -> bool {
        let last_block = self.chain.last().unwrap();
        
        if new_block.previous_hash == last_block.hash && new_block.is_valid(last_block) {
            self.chain.push(new_block);
            return true;
        }
        false
    }

    /// Replaces the local chain if the received chain is longer and valid
    pub fn replace_chain(&mut self, new_chain: Blockchain) -> bool {
        if new_chain.chain.len() > self.chain.len() && Blockchain::is_valid_chain(&new_chain.chain) {
            self.chain = new_chain.chain;
            return true;
        }
        false
    }

    /// Validates a full blockchain
    pub fn is_valid_chain(chain: &Vec<Block>) -> bool {
        for i in 1..chain.len() {
            if chain[i].previous_hash != chain[i - 1].hash || !chain[i].is_valid(&chain[i - 1]) {
                return false;
            }
        }
        true
    }
}
