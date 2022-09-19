use blockchainlib::*;

fn main() {
    let difficulty = 0x000fffffffffffffffffffffffffffff;

    let mut block = Block::new(0, 0, vec![0; 32], 0, "Another block".to_owned(), difficulty);

    block.mine();

    println!(" mined genesis block {:?}", &block);

    let mut last_hash = block.hash.clone();

    let mut blockchain = Blockchain {
        blocks: vec![block],
    };

    for i in 1..=10 {
        let mut block = Block::new(i, 0, last_hash, 0, "Another block".to_owned(), difficulty);

        block.mine();

        println!(" mined block {:?}", &block);

        last_hash = block.hash.clone();

        blockchain.blocks.push(block);
    }
}
