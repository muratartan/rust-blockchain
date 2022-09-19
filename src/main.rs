use blockchainlib::*;

fn main() {
    let difficulty = 0x000fffffffffffffffffffffffffffff;

    let mut blockchain = Blockchain {
        blocks: vec![block],
    };

    for i in 1..=10 {
        let mut block = Block::new(i, 0, vec![0; 32], 0, "Another block".to_owned(), difficulty);

        block.mine();

        println!(" mined genesis block {:?}", &block);

        blockchain.blocks.push(block);
    }
}
