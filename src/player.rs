/// Each player in the game will be represented by a player struct

use Direction;
use input::Holding;
use piston::input::keyboard::Key;

/// Different player animations will need different animation reset deltas
/// These deltas are assumed to be in seconds
const WALK_RESET: f64 = 1.0;
const JUMP_RESET: f64 = 0.6;

/// Store the player's state as an enum
pub enum PlayerState {
    Walking(Direction),
    Jumping(Direction),
    Standing(Direction),
    Falling(Direction),
}

/// The struct representing the player
pub struct Player {
    hp: i32,
    dt: f64,
    state: PlayerState,
    position: (f64, f64),
    facing: Direction,
    velocity: (f64, f64),
    jump_speed: f64,
}

impl Player {
    pub fn new() -> Self {
        Player {
            hp: 100,
            dt: 0.0,
            state: PlayerState::Standing(Direction::Right),
            position: (300.0, 300.0),
            facing: Direction::Right,
            velocity: (0.0, 0.0),
            jump_speed: 2.0,
        }
    }

    pub fn get_dt(&self) -> f64 {
        self.dt
    }

    fn get_facing(&self) -> Direction {
        self.facing.clone()
    }

    pub fn get_position(&self) -> (f64, f64) {
        self.position
    }

    pub fn get_state(&self) -> &PlayerState {
        &self.state
    }

    pub fn jump(&mut self) {
        self.velocity.1 -= self.jump_speed;
    }
    
    /// Reset the player's delta timer after `reset` seconds
    fn reset_dt(&mut self, reset: f64) {
        let dt_mod = (self.dt * 100.0) as i64;
        let dt_rem = dt_mod % (reset * 100.0) as i64;
        self.dt = dt_rem as f64/100.0;
    }

    /// Determine the appropriate state for the player to be in
    fn resolve_state(&mut self, holding: &Holding, last_pressed: &Key) {
        if (*last_pressed == Key::Space) && (self.velocity.1 < 0.0) {
            self.state = PlayerState::Jumping(self.get_facing());
        }
        
        if *holding.get_right() && (*last_pressed == Key::D) {
            self.state = PlayerState::Walking(Direction::Right);
            self.facing = Direction::Right;
        }

        else if *holding.get_left() && (*last_pressed == Key::A) {
            self.state = PlayerState::Walking(Direction::Left);
            self.facing = Direction::Left;
        }

        else if self.velocity.1 > 0.0 {
            self.state = PlayerState::Falling(self.get_facing());
        }

        else {
            self.state = PlayerState::Standing(self.get_facing());
        }
    }

    pub fn set_state(&mut self, state: PlayerState) {
        self.state = state;
    }

    /// Update the character - move this to player later
    pub fn update_char(&mut self, dt: f64, holding: &Holding, last_pressed: &Key) {
        self.resolve_state(holding, last_pressed);
        self.dt += dt;
        match self.state {
            PlayerState::Walking(Direction::Right) => {
                if self.dt > WALK_RESET { self.reset_dt(WALK_RESET); }
                self.velocity.0 = 1.0;
            },

            PlayerState::Walking(Direction::Left) => {
                if self.dt > WALK_RESET { self.reset_dt(WALK_RESET); }
                self.velocity.0 = -1.0;
            },

            PlayerState::Jumping(Direction::Right) => {
                if self.dt > JUMP_RESET { self.reset_dt(JUMP_RESET); }
                self.velocity.1 += ::GRAVITY * dt;
            },

            PlayerState::Jumping(Direction::Left) => {
                if self.dt > JUMP_RESET { self.reset_dt(JUMP_RESET); }
                self.velocity.1 += ::GRAVITY * dt;
            },

            PlayerState::Falling(Direction::Left) => {
                self.velocity.1 += ::GRAVITY * dt;                
            },

            PlayerState::Falling(Direction::Right) => {
                self.velocity.1 += ::GRAVITY * dt;                
            },            
            
            _ => {
                self.dt = 0.0;
                self.velocity = (0.0, 0.0);
            },

        }

        self.position.0 += self.velocity.0 * dt;
        self.position.1 += self.velocity.1 * dt;        
    }
    
}
