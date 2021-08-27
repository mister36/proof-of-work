use std::time::SystemTime;
use sha2::{Sha256, Digest};
// use lazy_static::lazy_static;

// const TARGET: [u8; 32] = [0; 32];

// lazy_static! {
//     static ref TARGET: Vec<u8> = {
//         let target = vec![0; 32];
//         target
//     }
// }

pub struct Block {

    /// Data within block
    contents: String,

    /// Hash of contents
    hash: Vec<u8>,
    
    /// Hash of previous block
    prev_hash: Vec<u8>,

    // adjusted by miner until new hash < target hash
    // nonce: u32,

    /// time at creation
    timestamp: SystemTime
}

impl Block {
    /// Takes in the previous block & contents of new block
    pub fn new(prev_block_opt: Option<&Block>, contents: String) -> Self {
        // Sha-256 hasher and block
        let mut hasher = Sha256::new();
        let block;
        let hash;
        
        // Hashes content and stores in vector
        hasher.update(contents.as_bytes());
        
        // Not the genesis block
        if let Some(prev_block) = prev_block_opt {
            // Hashes previous hash + content -> H(contents || prev_hash)
            hasher.update(&prev_block.hash);
            hash = hasher.finalize()[..].to_vec();

            // mine
            mine(&prev_block.hash, &contents);

            block = Block {
                contents,
                prev_hash: prev_block.hash.clone(),
                hash,
                timestamp: SystemTime::now()
            }
        } else {
            // Concatenates 0 to content -> H(contents || 0)
            let genesis_hash = vec![0];

            hasher.update(genesis_hash);
            hash = hasher.finalize()[..].to_vec();

            block = Block {
                contents,
                prev_hash: vec![0],
                hash,
                timestamp: SystemTime::now()
            }
        }
        
        block 
    }

}

pub struct Blockchain {

    // The blocks themselves
    pub blocks: Vec<Block> // TODO: make private
}

/// Mining
/// Will try arbitrary amount of nonces 
/// until H(nonce || prev_hash || content) < target   
fn mine(prev_hash: &Vec<u8>, contents: &str) {
    let contents_bytes = contents.as_bytes();

    for nonce in 0..u32::MAX {

        // Target hash [0, 0, 15, 0, ... 0]
        let mut target: Vec<u8> = vec![0; 32];
        target[2] = 0xFF;

        // Creating nonce and hasher
        let nonce_u8 = nonce.to_be_bytes().to_vec();
        let mut hasher = Sha256::new();
        
        // H(nonce || prev_hash || contents)
        hasher.update(nonce_u8);
        hasher.update(prev_hash);
        hasher.update(contents_bytes);
        let result_array = hasher.finalize();
        let result = &result_array[..];
        
        // check if result < target 

        // println!("nonce: {}", nonce); // TODO: Remove
        // println!("result: {:?}", result); // TODO: Remove
        // println!("target: {:?}", target); // TODO: Remove
        // println!("=============\n"); // TODO: Remove
        if result.to_vec() < target {
            println!("Nonce found: {}", nonce);
            break;
        }

    }
}

