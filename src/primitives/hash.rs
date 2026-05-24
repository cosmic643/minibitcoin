use sha2::{Digest, Sha256};

pub type Hash256 = [u8; 32];

pub fn zero_hash() -> Hash256{
    [0u8; 32]
}

pub fn sha256(data: &[u8]) -> Hash256{
    let digest = Sha256::digest(data);

    let mut hash = [0u8; 32];
    hash.copy_from_slice(&digest);

    hash
}

pub fn hash_to_hex(hash: &Hash256) -> String{
    hex::encode(hash)
}