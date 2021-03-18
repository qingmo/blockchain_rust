use chrono::prelude::*;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use hex::ToHex;

const HASH_BYTE_SIZE: usize = 32;

pub type Sha256Hash = [u8; HASH_BYTE_SIZE];

#[derive(Debug)]
struct Block {
    timestamp: i64,
    data: Vec<u8>,
    prev_block_hash: Sha256Hash,
    hash: Sha256Hash,
}

impl Block {
    fn hash(&mut self) -> [u8; 32] {
        let mut vec = Vec::new();
        vec.extend_from_slice(&self.prev_block_hash);
        vec.extend_from_slice(&self.data);
        vec.extend_from_slice(&convert_u64_to_u8_array(self.timestamp as u64));
        let mut hasher = Sha256::new();
        hasher.input(&vec);
        let mut hash = Sha256Hash::default();
        hasher.result(&mut hash);
        hash
    }

    fn new(data: &str, prev_hash: Sha256Hash) -> Self {
        let mut block = Self {
            timestamp: Utc::now().timestamp(),
            data: data.to_owned().into(),
            prev_block_hash: prev_hash,
            hash: Sha256Hash::default(),
        };
        let hash = block.hash();
        block.hash = hash;
        block
    }

    fn new_genesis_block() -> Self {
        Block::new("Genesis Block", Sha256Hash::default())
    }
}

#[derive(Debug)]
pub struct Blockchain {
    blocks: Vec<Block>
}

impl Blockchain {
    fn new() -> Self {
        Self {
            blocks: vec![Block::new_genesis_block()]
        }
    }

    fn add_block(&mut self, data: &str) {
        let prev_block = self.blocks.last().unwrap();
        let mut new_block = Block::new(data, prev_block.hash);
        self.blocks.push(new_block);
    }
}

fn convert_u64_to_u8_array(data: u64) -> [u8; 8] {
    return [
        data as u8,
        (data >> 8) as u8,
        (data >> 16) as u8,
        (data >> 24) as u8,
        (data >> 32) as u8,
        (data >> 40) as u8,
        (data >> 48) as u8,
        (data >> 56) as u8,
    ];
}

fn main() {
    let mut bc = Blockchain::new();

    bc.add_block("Send 1 BTC to Ivan");
    bc.add_block("Send 2 more BTC to Ivan");

    for (i, b) in bc.blocks.iter().enumerate() {
        println!("index {}", i);
        println!("Prev. hash:{:?}", hex::encode(b.prev_block_hash));
        println!("data:{:?}", String::from_utf8(b.data.clone())
            .unwrap_or_else(|e| format!("not utf8 format data:{}", e)));
        println!("hash:{:?}", hex::encode(b.hash));
        println!();
    }
}
