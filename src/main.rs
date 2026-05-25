use minibitcoin::primitives::{block::Block, hash::hash_to_hex};
use minibitcoin::chain::blockchain::Blockchain;

fn main() {



    

    let mut blockchain = Blockchain::new();
    blockchain.add_block();
    blockchain.add_block();
    blockchain.add_block();
    blockchain.add_block();
    blockchain.add_block();
    blockchain.add_block();
    for (i, block) in blockchain.blocks.iter().enumerate(){
        println!("Block: {i}");
        println!("Block Hash: {}",hash_to_hex(&block.hash()));
        println!("Previous Block Hash: {}",hash_to_hex(&block.header.previous_hash));
        println!("-----------------------");
    }

    println!("{}", blockchain.validate_chain());
    blockchain.blocks[2].header.nonce = 999;
    println!("{}", blockchain.validate_chain());
    
}
