/// Handle the game's input

/// Hold information related to input handling
pub struct InputHandler {
}

/// Structure to determine if a key is being held
pub struct Holding {
    right: bool,
    left: bool,
}

/// Implement input handling methods
impl InputHandler {
}

/// Implement holding methods
impl Holding {
    pub fn new() -> Self {
        Holding {
            right: false,
            left: false,
        }
    }

    pub fn get_right(&self) -> &bool {
        &self.right
    }

    pub fn get_left(&self) -> &bool {
        &self.left
    }    
    
    pub fn set_right(&mut self, b: bool) {
        self.right = b;
    }

    pub fn set_left(&mut self, b: bool) {
        self.left = b;
    }
    
}
