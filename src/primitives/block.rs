use crate::primitives::hash::{Hash256, zero_hash, hash_to_hex};
use crate::consensus::pow::{is_valid_pow};
use serde::{Serialize, Deserialize};
use crate::primitives::hash::sha256;
use bincode;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockHeader{
    pub version: u32,
    pub previous_hash: Hash256,
    pub merkle_root: Hash256,
    pub timestamp: u64,
    pub difficulty: u32,
    pub nonce: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block{
    pub header: BlockHeader,
}

impl Block{
    pub fn genesis() -> Self{
        Block{
            header: BlockHeader{
                version: 1,
                previous_hash: zero_hash(),
                merkle_root: zero_hash(),
                timestamp: 0,
                difficulty: 4,
                nonce: 0,
            },
        }
    }

    pub fn hash(&self) -> Hash256{
        let serialized_header = bincode::serialize(&self.header).unwrap();
        sha256(&serialized_header)
    }

    
    pub fn next(previous: &Block) -> Self{
        let mut block =Block{
            header: BlockHeader { 
                version: 1,
                previous_hash: previous.hash(), 
                merkle_root: zero_hash(), 
                timestamp: previous.header.timestamp + 1,
                difficulty: previous.header.difficulty,
                nonce: 0 }
        };
        block.mine();
        block
    }

    pub fn mine(&mut self){
        loop{
            let block_hash = hash_to_hex(&self.hash());
            if is_valid_pow(&block_hash, self.header.difficulty){
                break;
            }
            self.header.nonce += 1;
        }
    }
}