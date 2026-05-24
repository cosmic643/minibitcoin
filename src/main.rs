use minibitcoin::primitives::block::Block;

fn main() {
    let genesis_block = Block::genesis();
    let hash = genesis_block.hash();
    println!("{:?}", hash);
}
