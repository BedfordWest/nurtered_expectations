/// Each player in the game will be represented by a player struct

/// The struct representing the player
pub struct Player {
    hp: i32
}

impl Player {
    pub fn new() -> Self {
        Player {
            hp: 100
        }
    }
}
