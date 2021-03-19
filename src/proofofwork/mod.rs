use num::{One, BigUint};
use crate::block::{Block, HASH_BIT_SIZE};
use crate::Sha256Hash;
use crypto::sha2::Sha256;
use crypto::digest::Digest;
use crate::util;

pub const DEFAULT_DIFFICULTY: usize = 4;

const MAX_NONCE: u64 = u64::MAX;

fn prepare_date(block: &Block, target_bits: u64, nonce: u64) -> Vec<u8> {
    let mut data: Vec<u8> = block.get_header();
    data.extend_from_slice(&util::convert_u64_to_u8_array(target_bits));
    data.extend_from_slice(&util::convert_u64_to_u8_array(nonce));
    data
}

pub fn run(block: &Block, difficulty: usize) -> (u64, Sha256Hash) {
    let mut hash= Sha256Hash::default();
    let mut nonce  = 0 as u64;
    let target = BigUint::one() << (HASH_BIT_SIZE - difficulty * 4);
    println!("Mining the block containing {:?}", block.pretty_data());
    while nonce < MAX_NONCE {
        let data = prepare_date(block, (difficulty * 4) as u64, nonce);
        let mut hasher = Sha256::new();
        hasher.input(&data);
        hash= Sha256Hash::default();
        hasher.result(&mut hash);
        let hash_int: BigUint = BigUint::from_bytes_be(hash.as_ref());
        if hash_int.lt(&target) {
            println!("{:?}", hex::encode(hash));
            break
        } else {
            nonce = nonce + 1;
        }
    }

    return (nonce, hash)
}

pub fn validate(block: &Block, difficulty: usize)->bool {

    let data = prepare_date(block, (difficulty * 4) as u64, block.get_nonce());
    let mut hasher = Sha256::new();
    hasher.input(&data);
    let mut hash= Sha256Hash::default();
    hasher.result(&mut hash);
    let hash_int: BigUint = BigUint::from_bytes_be(hash.as_ref());
    let target = BigUint::one() << (HASH_BIT_SIZE - difficulty * 4);
    hash_int.lt(&target)
}

#[cfg(test)]
mod tests {
    use num::{BigInt, One};
    use crypto::sha2::Sha256;
    use crypto::digest::Digest;
    use crate::Sha256Hash;

    #[test]
    fn test_big_int_hash() {
        let data1 = "I like donuts".as_bytes();
        let data2 = "I like donutsca07ca".as_bytes();
        let difficulty = 24 as usize;
        let mut target = BigInt::one();
        target = target << (256 - difficulty);
        let mut hasher = Sha256::new();
        hasher.input(&data1);
        let mut hash= Sha256Hash::default();
        hasher.result(&mut hash);
        print!("{:?}\n", hex::encode(hash));
        print!("{:?}\n", format!("{:64x}", target));

        let mut hasher = Sha256::new();
        hasher.input(&data2);
        let mut hash= Sha256Hash::default();
        hasher.result(&mut hash);
        print!("{:?}\n", hex::encode(hash));

    }
}
