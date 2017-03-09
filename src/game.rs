/// A game struct will represent the running game

use gameresources::GameResources;
use gfx_device_gl::{ Resources };
use graphics::types::SourceRectangle;
use piston::input::*;
use piston_window::*;
use player::Player;
use sprite::*;
use touch_visualizer::TouchVisualizer;
use view::View;

/// Keep track of the current game state
#[derive(Debug)]
enum GameState {
    Menu,
    Playing
}

/// Game struct
pub struct Game
{
    
    // Assets - move to a separate mod/struct later
    menu_screen: Option<Texture<Resources>>,

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
}

impl Game
{
    
    /// Instantiate the game
    pub fn new(w: &PistonWindow) -> Self {
        let touch_visualizer = TouchVisualizer::new();
        let player = Player::new();
        let view = View::new();
        let gameresources = GameResources::new(&w);

        Game {
            menu_screen: None,
            capture_cursor: false,
            touch_visualizer: touch_visualizer,
            game_state: GameState::Menu,
            player: player,
            view: view,
            gameresources: gameresources,
        }
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
        let menu_texture = self.gameresources.get_menu_texture();
        image(menu_texture, c.transform, g);
        self.touch_visualizer.draw(c, g);
    } 

    /// This is the function to call to begin execution of the game loop
    pub fn run(&mut self, mut window: PistonWindow) {

        let mut cursor = [0.0, 0.0];
        //      let mut scene = Scene::new();
        let sample_char_tex = self.gameresources.get_char_texture_rc();
        let mut sample_char = Sprite::from_texture(sample_char_tex);
        sample_char.set_position(300.0, 300.0);
//        let id = scene.add_child(sample_char);

        // Begin the primary game loop by iterating through piston::event_loop::Events
        while let Some(e) = window.next() {
//            scene.event(&e);
            self.touch_visualizer.event(window.size(), &e);

            // Event was a render, so let's draw stuff
            window.draw_2d(&e, |c, g| {
                self.render(&c, g);
                sample_char.draw(c.transform, g);
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
                let src_rect1: SourceRectangle = [0.0, 704.0, 64.0, 64.0];
                let src_rect2: SourceRectangle = [64.0, 704.0, 64.0, 64.0];
                let src_rect3: SourceRectangle = [128.0, 704.0, 64.0, 64.0];
                let src_rect4: SourceRectangle = [192.0, 704.0, 64.0, 64.0];
                let src_rect5: SourceRectangle = [256.0, 704.0, 64.0, 64.0];
                let src_rect6: SourceRectangle = [320.0, 704.0, 64.0, 64.0];
                let src_rect7: SourceRectangle = [384.0, 704.0, 64.0, 64.0];
                let src_rect8: SourceRectangle = [448.0, 704.0, 64.0, 64.0];
                let src_rect9: SourceRectangle = [512.0, 704.0, 64.0, 64.0];                
                match self.player.get_dt() {
                    0.0 ... 0.11 => sample_char.set_src_rect(src_rect1),
                    0.11 ... 0.22 => sample_char.set_src_rect(src_rect2),
                    0.22 ... 0.33 => sample_char.set_src_rect(src_rect3),
                    0.33 ... 0.44 => sample_char.set_src_rect(src_rect4),
                    0.44 ... 0.55 => sample_char.set_src_rect(src_rect5),
                    0.55 ... 0.66 => sample_char.set_src_rect(src_rect6),
                    0.66 ... 0.77 => sample_char.set_src_rect(src_rect7),
                    0.77 ... 0.88 => sample_char.set_src_rect(src_rect8),
                    0.88 ... 0.99 => sample_char.set_src_rect(src_rect9),                    
                    _ => sample_char.set_src_rect(src_rect9),
                }
            }

        } 
    }

    /// Handle the update event
    fn update(&mut self, args: &UpdateArgs) {
        self.player.update_char(args.dt);
        if self.player.get_dt() > 1.0 { self.player.reset_dt(); }
    }


}
