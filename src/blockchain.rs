use super::*;

#[derive(Debug)]
pub enum BlockValidationErr {
    MismatchedIndex,
    InvalidHash,
    AchronologicalTimestamp,
    MismatchedPreviousHash,
    InvalidGenesisBlockFormat,
    InvalidInput,
    InsufficientInputValue,
    InvalidCoinbaseTransaction,
}
pub struct Blockchain {
    pub blocks: Vec<Block>,
}
impl Blockchain {
    pub fn update_with_block(&mut self, block: Block) -> bool {
        for (i, block) in self.blocks.iter().enumerate() {
            if block.index != i as u32 {
                println!(" index mismatch {} != {}", &block.index, &i);
                return false;
            } else if !block::check_difficulty(&block.hash(), block.difficulty) {
                println!("difficulty failed");
                return false;
            } else if i != 0 {
                // not genesis block
                let prev_block = &self.blocks[i - 1];
                if block.timestamp <= prev_block.timestamp {
                    println!("time did not increase");
                    return false;
                } else if block.prev_block_hash != prev_block.hash {
                    println!("hash mismatch");
                    return false;
                }
            } else {
                // genesis block
                if block.prev_block_hash != vec![0; 32] {
                    println!("genesis block prev_block_hash invalid");
                    return false;
                }
            }
        }
        true
    }
}
