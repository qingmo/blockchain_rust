use chrono::prelude::*;

use crate::{util, Sha256Hash, proofofwork};

pub const HASH_BYTE_SIZE: usize = 32;
pub const HASH_BIT_SIZE: usize = 256;

#[derive(Debug)]
// #[derive(Default)]
pub struct Block {
    timestamp: i64,
    data: Vec<u8>,
    prev_block_hash: Sha256Hash,
    hash: Sha256Hash,
    nonce: u64,
}

impl Block {
    pub fn get_header(&self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();
        data.extend_from_slice(self.prev_block_hash.as_ref());
        data.extend_from_slice(self.data.as_ref());
        data.extend_from_slice(&util::convert_u64_to_u8_array(self.timestamp as u64));
        data
    }

    pub fn get_nonce(&self) -> u64 {
        self.nonce
    }

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

    pub fn new(data: &str, prev_hash: Sha256Hash) -> Self {
        let mut block = Self {
            timestamp: Utc::now().timestamp(),
            data: data.to_owned().into(),
            prev_block_hash: prev_hash,
            hash: Sha256Hash::default(),
            nonce: 0,
        };
        let (nonce, hash) = proofofwork::run(&block, proofofwork::DEFAULT_DIFFICULTY);
        block.hash = hash;
        block.nonce = nonce;
        block
    }

    pub fn new_genesis_block() -> Self {
        Block::new("Genesis Block", Sha256Hash::default())
    }
}
