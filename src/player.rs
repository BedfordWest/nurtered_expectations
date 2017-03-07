/// Each player in the game will be represented by a player struct

/// The struct representing the player

use piston::input::UpdateArgs;

pub struct Player {
    hp: i32,
    dt: f64
}

impl Player {
    pub fn new() -> Self {
        Player {
            hp: 100,
            dt: 0.0
        }
    }

    /// Update the character - move this to player later
    pub fn update_char(&mut self, dt: f64) {
        self.dt += dt;
    }

    pub fn reset_dt(&mut self) {
        let dt_mod = (self.dt * 100.0) as i64;
        let dt_rem = dt_mod % 100;
        self.dt = dt_rem as f64/100.0;
    }

    pub fn get_dt(&self) -> f64 {
        self.dt
    }
    
}
