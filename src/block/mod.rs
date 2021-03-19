use chrono::prelude::*;
use crypto::digest::Digest;
use crypto::sha2::Sha256;

use crate::util;

const HASH_BYTE_SIZE: usize = 32;

pub type Sha256Hash = [u8; HASH_BYTE_SIZE];

#[derive(Debug)]
// #[derive(Default)]
pub struct Block {
    timestamp: i64,
    data: Vec<u8>,
    prev_block_hash: Sha256Hash,
    hash: Sha256Hash,
}

impl Block {
    pub fn pretty_hash(&self) -> String {
        hex::encode(self.hash)
    }

    pub fn pretty_data(&self) -> String {
        String::from_utf8(self.data.clone()).unwrap_or_else(|e| format!("not utf8 format data:{}", e))
    }

    pub fn get_hash(&self) -> [u8; 32] {
        self.hash.to_owned().into()
    }

    pub fn pretty_prev_hash(&self) -> String {
        hex::encode(self.prev_block_hash)
    }
    fn hash(&mut self) -> [u8; 32] {
        let mut vec = Vec::new();
        vec.extend_from_slice(&self.prev_block_hash);
        vec.extend_from_slice(&self.data);
        vec.extend_from_slice(&util::convert_u64_to_u8_array(self.timestamp as u64));
        let mut hasher = Sha256::new();
        hasher.input(&vec);
        let mut hash = Sha256Hash::default();
        hasher.result(&mut hash);
        hash
    }

    pub fn new(data: &str, prev_hash: Sha256Hash) -> Self {
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

    pub fn new_genesis_block() -> Self {
        Block::new("Genesis Block", Sha256Hash::default())
    }
}
