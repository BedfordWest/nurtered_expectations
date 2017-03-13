/// A terrain block is represented here

use graphics::types::SourceRectangle;

pub struct Block {
    position: (f64, f64),
    bounding: (f64, f64),
}

impl Block {
    pub fn new() -> Self {
        Block {
            position: (0.0, 0.0),
            bounding: (32.0, 32.0),
        }
    }

    /// Get the bounding box for the box
    pub fn get_bounding(&self) -> SourceRectangle {
        [self.position.0 - self.bounding.0, self.position.1 - self.bounding.1,
         self.bounding.0 * 2.0, self.bounding.1 * 2.0]
    }
    
    pub fn get_position(&self) -> (f64, f64) {
        self.position
    }

    pub fn set_position(&mut self, position: (f64, f64)) {
        self.position = position;
    }
}
