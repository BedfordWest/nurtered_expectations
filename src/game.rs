/****************************** game.rs **********************************
** Control the main game loop, and implement traits necessary to do so **
*************************************************************************/

use config::*;
use gfx_device_gl::{ Resources };
use piston::event_loop::*;
use piston::input::*;
use piston_window::*;

const OPENGL_VERSION: OpenGL = OpenGL::V3_2;

// Game struct
pub struct Game 
{
    events: Events,
    menu_screen:Option<Texture<Resources>>
}

impl Game
{
    
    // Instantiate the game
    pub fn new() -> Self {
        let events = Events::new(EventSettings::new());

        Game {
            events: events,
            menu_screen: None
        }
    }

    // Handle the initial load of game resources
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
    

    // Handle the render event
    fn render(&mut self, c: &Context, g: &mut G2d) {
        clear([1.0; 4], g);
        let menu_image = self.menu_screen.as_ref().unwrap();
        image(menu_image, c.transform, g);
    } 

    // This is the function to call to begin execution of the game loop
    pub fn run(&mut self) {

        // Create the primary window for the game
        let mut window: PistonWindow =
            WindowSettings::new("Nurtered Expectations", [640, 480])
            .opengl(OPENGL_VERSION).exit_on_esc(true).build().unwrap();

        // Load up any resources necessary prior to beginning the game loop
        self.load_resources(&window);

        // Begin the primary game loop by iterating through piston::event_loop::Events
        while let Some(e) = self.events.next(&mut window) {

            if let Some(_) = e.render_args() {
                // Event was a render, so let's draw stuff
                window.draw_2d(&e, |c, g| {
                    self.render(&c, g);
                }); 
            }

            // TODO: Implement input handlers
            // if let Some(args) = e.press_args(&args) { }
            // if let Some(args) = e.release_args(&args) { }

            if let Some(args) = e.update_args() {
                self.update(&args);
            }           
        } 
    }

    // Handle the update event
    fn update(&mut self, args: &UpdateArgs) {
    }    
}
