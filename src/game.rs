/// A game struct will represent the running game

use config::*;
use gfx_device_gl::{ Resources };
use piston::event_loop::*;
use piston::input::*;
use piston_window::*;
use player::Player;
use std::collections::HashSet;
use touch_visualizer::TouchVisualizer;

const OPENGL_VERSION: OpenGL = OpenGL::V3_2;

/// Keep track of the current game state
#[derive(Debug)]
enum GameState {
    Menu,
    Playing
}

/// Game struct
pub struct Game {
    events: Events,
    
    // Assets - move to a separate mod/struct later
    menu_screen: Option<Texture<Resources>>,

    // For debugging the mouse
    capture_cursor: bool,
    touch_visualizer: TouchVisualizer,
    
    // Track the game state
    game_state: GameState,

    // The game will have just one player for now
    player: Player
}

impl Game {
    
    /// Instantiate the game
    pub fn new() -> Self {
        let events = Events::new(EventSettings::new());
        let touch_visualizer = TouchVisualizer::new();
        let player = Player::new();

        Game {
            events: events,
            menu_screen: None,
            capture_cursor: false,
            touch_visualizer: touch_visualizer,
            game_state: GameState::Menu,
            player: player
        }
    }

    /// Handle the initial load of game resources
    // TODO: Move this to separate modules for display and audio
    fn load_resources(&mut self, w: &PistonWindow) {
        let root = root();
        let menu_path = root.join("./assets/crystal-caves.jpg");
        let menu_screen = Texture::from_path(
            &mut w.factory.clone(),
            &menu_path,
            Flip::None,
            &TextureSettings::new()
        ).unwrap();
        self.menu_screen = Some(menu_screen);
    }

    /// Handle a mouse press event
    fn mouse_press(&mut self, button: MouseButton) {
        println!("Pressed mouse button '{:?}'", button);
    }

    /// Handle the release of a keyboard key
    fn release_key(&mut self, key: Key) {
        println!("Released key '{:?}'", key);
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
              println!("Toggled captuppre cursor");
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
            
            _ => println!("Pressed keyboard key '{:?}'", key),
        };
        
    }

    /// Handle the render event - should be called from draw2d in the event loop
    fn render(&mut self, c: &Context, g: &mut G2d) {
        clear([1.0; 4], g);
        let menu_image = self.menu_screen.as_ref().unwrap();
        image(menu_image, c.transform, g);
        self.touch_visualizer.draw(c, g);
    } 

    /// This is the function to call to begin execution of the game loop
    pub fn run(&mut self) {

        // Create the primary window for the game
        let mut window: PistonWindow<> =
            WindowSettings::new("Nurtured Expectations", [1920, 1080])
            .opengl(OPENGL_VERSION)
            .resizable(true)
            .decorated(false)
            .exit_on_esc(true)
            .fullscreen(true)
            .build()
            .unwrap();

        // Load up any resources necessary prior to beginning the game loop
        self.load_resources(&window);

        let mut cursor = [0.0, 0.0];

        // Begin the primary game loop by iterating through piston::event_loop::Events
        while let Some(e) = self.events.next(&mut window) {
            self.touch_visualizer.event(window.size(), &e);

            // Event was a render, so let's draw stuff
            window.draw_2d(&e, |c, g| {
                self.render(&c, g);
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
    }    
}
