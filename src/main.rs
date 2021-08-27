use std::io::Cursor;
use sha2::{Sha256, Digest};
use hex_literal::hex;
use byteorder::{BigEndian, ReadBytesExt};
use chain::{Block, Blockchain};
fn main() {
    let block_1 = Block::new(None, "This is a string".to_string());
    let mut chain = Blockchain {blocks: vec![block_1]};
    println!("Added first block to chain");

    let block_2 = Block::new(Some(&chain.blocks[0]), "Next string".to_string());
    println!("Created second block");
    

}
