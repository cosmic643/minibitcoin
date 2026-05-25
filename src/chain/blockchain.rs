use crate::{consensus::pow::is_valid_pow, primitives::block::Block};
use crate::primitives::hash::hash_to_hex;

pub struct Blockchain{
    pub blocks: Vec<Block>,
}

impl Blockchain{
    pub fn new() -> Self {
        Blockchain{
            blocks: vec![Block::genesis()],
        }
    }

    pub fn latest_block(&self) -> &Block{
        self.blocks.last().unwrap()
    }
    
    pub fn add_block(&mut self){
        let new_block = Block::next(self.latest_block());
        self.blocks.push(new_block);
    }

    pub fn validate_chain(&self) -> bool{
        for i in 1..self.blocks.len(){
            let current = &self.blocks[i];
            let previous = &self.blocks[i-1];

            if current.header.previous_hash != previous.hash(){
                return false;
            }
            if !is_valid_pow(&hash_to_hex(&current.hash()), current.header.difficulty){
                return false
            }
        }
        true
    }
    // pub fn validate_chain(&self) -> bool {
    //     for pair in self.blocks.windows(2) {
    //         let previous = &pair[0];
    //         let current = &pair[1];
    
    //         let linked = current.header.previous_hash == previous.hash();
    //         let mined = is_valid_pow(
    //             &hash_to_hex(&current.hash()),
    //             current.header.difficulty,
    //         );
    
    //         if !linked || !mined {
    //             return false;
    //         }
    //     }
    
    //     true
    // }
}