use sha2::{Sha256, Digest};

fn main() {
    let mut hasher = Sha256::new();
    let mut hasher2 = Sha256::new();

    hasher.update(b"test hash");
    hasher.update(b"test hash");

    hasher2.update("test hashtest hash");

    let result: &[u8] = &hasher.finalize()[..];
    let result2: &[u8] = &hasher2.finalize()[..];

    println!("{:?}", result == result2);
}
