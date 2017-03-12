/// A terrain block is represented here

pub struct Block {
    position: (f64, f64),
}

impl Block {
    pub fn new() -> Self {
        Block {
            position: (0.0,0.0),
        }
    }

    pub fn get_position(&self) -> (f64, f64) {
        self.position
    }

    pub fn set_position(&mut self, position: (f64, f64)) {
        self.position = position;
    }
}
