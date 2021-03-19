pub mod block;
pub mod blockchain;
pub mod proofofwork;
pub mod util;

pub use proofofwork::DEFAULT_DIFFICULTY;

pub use block::HASH_BYTE_SIZE;

pub type Sha256Hash = [u8; HASH_BYTE_SIZE];