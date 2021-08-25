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
    pub fn new(prev_block_opt: Option<Block>, contents: String) -> Self {
        // Sha-256 hasher
        let mut hasher = Sha256::new();
        let block;
        
        // Hashes content and stores in vector
        hasher.update(contents.as_bytes());
        let hash: Vec<u8> = hasher.finalize()[..].iter().cloned().collect();

        
        // Some(prev_block) => Not the genesis block
        if let Some(prev_block) = prev_block_opt {
            block = Block {
                contents,
                prev_hash: prev_block.hash,
                hash,
                timestamp: SystemTime::now()
            }
        } else {
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
    blocks: Vec<Block>
}

/// Mining
/// Will try arbitrary amount of nonces until H(nonce || prev_hash || content) < target   
fn mine(prev_hash: &Vec<u8>, contents: &str) {
    let nonce: u32;

    // Target hash
    let mut target: Vec<u8> = vec![0; 32];
    target[2] = 0x0F;

    for nonce in 0..u32::MAX {
        // Concatenate nonce + prev_hash + contents
        // Could rewrite this to be more performant (remove copying)
        // Or keep so mining is even more difficult
        let mut nonce_u8 = nonce.to_be_bytes().to_vec();
        // let mut contents_u8 = contents.as_bytes().to_vec();
        // let mut prev_hash_u8 = prev_hash.to_owned();

        let mut hasher = Sha256::new();
        
        hasher.update(nonce_u8);
        hasher.update(contents.as_bytes());
        hasher.update(prev_hash);

        let result = &hasher.finalize()[..];

    }
}

