use minibitcoin::primitives::{block::Block, hash::hash_to_hex};
use minibitcoin::chain::blockchain::Blockchain;

fn main() {
    let genesis_block = Block::genesis();
    let hash = genesis_block.hash();
    println!("{:?}", hash);
    println!("{}", hash_to_hex(&hash));

    let block1 = Block::next(&genesis_block);
    let block2 = Block::next(&block1);

    println!("{:?}", block1.header.previous_hash);
    println!("{}", hash_to_hex(&block1.header.previous_hash));

    println!("{:?}", block1.hash());
    println!("{}", hash_to_hex(&block1.hash()));

    println!("{:?}", block2.header.previous_hash);
    println!("{}", hash_to_hex(&block2.header.previous_hash));
    
    println!("{:?}", block2.hash());
    println!("{}", hash_to_hex(&block2.hash()));

    println!("------------------------------------");

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
}
