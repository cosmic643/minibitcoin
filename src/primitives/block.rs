use crate::primitives::hash::{Hash256, zero_hash};
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
                difficulty: 0,
                nonce: 0,
            },
        }
    }

    pub fn hash(&self) -> Hash256{
        let serialized_header = bincode::serialize(&self.header).unwrap();
        sha256(&serialized_header)
    }
}