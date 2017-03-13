/// Levels represent particle maps in the game

use block::Block;

pub struct Level {
    blocks: Vec<Block>,
}

impl Level {
    pub fn new() -> Self {
        let blocks = Vec::new();
        Level {
            blocks: blocks,
        }
    }

    pub fn get_blocks(&self) -> &Vec<Block> {
        &self.blocks
    }
    
    pub fn load_blocks(&mut self) {
        let mut blocks = Vec::new();
        for x in 1..10 {
            let mut block: Block = Block::new();
            let position: (f64, f64) = (x as f64 * 64.0, 364.0);
            block.set_position(position);
            blocks.push(block);
        }
        self.blocks = blocks;
    }
}


