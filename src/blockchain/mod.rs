use crate::block::Block;

#[derive(Debug)]
pub struct Blockchain {
    blocks: Vec<Block>
}

impl Blockchain {
    pub fn new() -> Self {
        Self {
            blocks: vec![Block::new_genesis_block()]
        }
    }

    pub fn add_block(&mut self, data: &str) {
        let prev_block = self.blocks.last().unwrap();
        let new_block = Block::new(data, prev_block.get_hash());
        self.blocks.push(new_block);
    }

    pub fn traverse(&self) {
        for (i, b) in self.blocks.iter().enumerate() {
            println!("index {}", i);
            println!("Prev. hash:{:?}", b.pretty_prev_hash());
            println!("data:{:?}", b.pretty_data());
            println!("hash:{:?}", b.pretty_hash());
            println!();
        }
    }
}