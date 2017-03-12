/// A game struct will represent the running game
use Direction;
use gameresources::GameResources;
use input::*;
use level::Level;
use piston::input::*;
use piston_window::*;
use player::{ Player, PlayerState };
use touch_visualizer::TouchVisualizer;
use view::View;

/// Keep track of the current game state
#[derive(Debug)]
enum GameState {
    Menu,
    Playing
}

/// Game struct
pub struct Game {
    
    // For debugging the mouse
    capture_cursor: bool,
    touch_visualizer: TouchVisualizer,
    
    // Track the game state
    game_state: GameState,

    // The game will have just one player for now
    player: Player,

    // The game needs a view to control rendering/display
    view: View,

    // Store the resources needed for the game's audio and display
    gameresources: GameResources,

    // Is the player holding a key?
    holding: Holding,

    // What is the last relevant key pressed?
    last_pressed: Key,

    // Keep a list of blocks for the game
    level: Level,
    
}

impl Game {
    
    /// Instantiate the game
    pub fn new(w: &PistonWindow) -> Self {
        let touch_visualizer = TouchVisualizer::new();
        let player = Player::new();
        let view = View::new();
        let gameresources = GameResources::new(&w);
        let holding = Holding::new();
        let level = Level::new();

        Game {
            capture_cursor: false,
            touch_visualizer: touch_visualizer,
            game_state: GameState::Menu,
            player: player,
            view: view,
            gameresources: gameresources,
            holding: holding,
            last_pressed: Key::D,
            level: level,
        }
    }

    /// Handle a mouse press event
    fn mouse_press(&mut self, button: MouseButton) {
        println!("Pressed mouse button '{:?}'", button);
    }

    /// Handle the release of a keyboard key
    fn release_key(&mut self, key: Key) {

        match key {

            Key::D => {
                self.holding.set_right(false);
                println!("Released keyboard key D");
            },

            Key::A => {
                self.holding.set_left(false);
                println!("Released keyboard key A");
            },            
            
            _ => println!("Released keyboard key '{:?}'", key),
        };        
    }

    /// Handle the release of a mouse button
    fn release_mouse(&mut self, button: MouseButton) {
        println!("Released mouse button '{:?}'", button);
    }

    /// Handle the release of a controller button
    fn release_controller_button(&mut self, button: ControllerButton) {
        println!("Released controller button '{:?}'", button);
    }        

    /// Handle a keyboard key press event
    fn key_press(&mut self, key: Key, w: &mut PistonWindow) {

        match key {
            Key::C => {
              println!("Toggled capture cursor");
              self.capture_cursor = !self.capture_cursor;
              w.set_capture_cursor(self.capture_cursor);                            
            },

            Key::D1 => {
              self.game_state = GameState::Menu;
              println!("Game state set to {:?}!", self.game_state);            
            },

            Key::D2 => {
              self.game_state = GameState::Playing;
              println!("Game state set to {:?}!", self.game_state);            
            },

            Key::F => {
              w.set_should_close(true);
              println!("Window will close!");            
            },

            Key::D => {
                self.holding.set_right(true);
                self.last_pressed = key;
                println!("Pressed keyboard key D");
            },

            Key::A => {
                self.holding.set_left(true);
                self.last_pressed = key;
                println!("Pressed keyboard key A");
            },

            Key::Space => {
                self.last_pressed = key;
                self.player.jump();
                println!("Pressed keyboard key Space");
            },            
            
            _ => println!("Pressed keyboard key '{:?}'", key),
        };
        
    }

    /// This is the function to call to begin execution of the game loop
    pub fn run(&mut self, mut window: PistonWindow) {

        let mut cursor = [0.0, 0.0];

        // Load textures
        self.view.load_player_sprite(&self.player, self.gameresources.get_char_texture_rc());
        self.view.load_block_sprite(self.gameresources.get_char_texture_rc());

        // Load the level
        self.level.load_blocks();

        // Begin the primary game loop by iterating through piston::event_loop::Events
        while let Some(e) = window.next() {
            self.touch_visualizer.event(window.size(), &e);

            // Event was a render, so let's draw stuff
            window.draw_2d(&e, |c, g| {
                match self.game_state {
                    GameState::Menu => {
                        clear([1.0; 4], g);                        
                        View::render_menu(&c, g, self.gameresources.get_menu_texture());
                        self.touch_visualizer.draw(&c, g);
                    },
                    GameState::Playing => {
                        clear([1.0; 4], g);
                        self.view.render_blocks(&c, g, self.level.get_blocks());
                        self.view.render_player(&c, g, &self.player);
                    }
                }
            });

            
            // Handle input

            // Mouse button was pressed
            if let Some(Button::Mouse(button)) = e.press_args() {
                self.mouse_press(button);

            };

            // Keyboard key was pressed
            if let Some(Button::Keyboard(key)) = e.press_args() {
                self.key_press(key, &mut window);

            };

            if let Some(button) = e.release_args() {
                match button {
                    Button::Keyboard(key) => self.release_key(key),
                    Button::Mouse(button) => self.release_mouse(button),
                    Button::Controller(button) => self.release_controller_button(button),
                }
            };

            e.mouse_cursor(|x,y| {
                cursor = [x, y];
                println!("Mouse moved '{} {}'", x, y);
            });

            e.mouse_scroll(|dx, dy| println!("Scrolled mouse '{}, {}'", dx, dy));
            e.mouse_relative(|dx, dy| println!("Relative mouse moved '{} {}'", dx, dy));
            e.text(|text| println!("Typed '{}'", text));
            e.resize(|w, h| println!("Resized '{}, {}'", w, h));

            if let Some(cursor) = e.cursor_args() {
                if cursor { println!("Mouse entered"); }
                else { println!("Mouse left"); }
            };

            if let Some(args) = e.update_args() {
                self.update(&args);
            }

        } 
    }

    /// Handle the update event
    fn update(&mut self, args: &UpdateArgs) {
        match self.game_state {
            GameState::Menu => {
            },
            GameState::Playing => {
                self.player.update_char(args.dt, &self.holding, &self.last_pressed);                
            }
        }        
    }

}

