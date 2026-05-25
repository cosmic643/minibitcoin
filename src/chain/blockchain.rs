use crate::primitives::block::Block;

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

    
}