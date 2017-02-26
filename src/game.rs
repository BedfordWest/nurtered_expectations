/****************************** game.rs **********************************
** Control the main game loop, and implement traits necessary to do so **
*************************************************************************/

use config::*;
// use glutin_window::GlutinWindow as Window;
use graphics::Graphics;
use piston::event_loop::*;
use piston::input::*;
use piston_window::*;



// Game struct - want to make sure it is generic over a backend, generic event, graphics, and window
pub struct Game 
{
    events: Events,
    window: PistonWindow
}

impl Game
{

    // Load game texture
/*    fn load_assets(&mut self) {

        // Determine the root directory string
        let root = root();

        // Define the location for each asset


        // Create the texture for each asset


    } */


    
    // Instantiate the game
    pub fn new() -> Self {

        const OPENGL_VERSION: OpenGL = OpenGL::V3_2;
        let mut events = Events::new(EventSettings::new());
        let mut window: PistonWindow =
            WindowSettings::new("Nurtered Expectations", [640, 480])
            .opengl(OPENGL_VERSION).exit_on_esc(true).build().unwrap();
        
        Game {
            events: events,
            window: window
        }
    }

    // Handle the render event
/*    fn render(&mut self, c: &Context, g: &mut Graphics) {
        clear([1.0; 4], g);
        let root = root();
        let menu_path = root.join("./assets/crystal-caves.jpg");
        let menu_screen = Texture::from_path(
            &mut self.window.factory,
            &menu_path,
            Flip::None,
            &TextureSettings::new()
        ).unwrap();
        image(&menu_screen, c.transform, g);
} */

    fn render<E>(&mut self, e: &E, ren: RenderArgs) where E: GenericEvent {

        let root = root();
        let menu_path = root.join("./assets/crystal-caves.jpg");
        let menu_screen = Texture::from_path(
            &mut self.window.factory,
            &menu_path,
            Flip::None,
            &TextureSettings::new()
        ).unwrap();
        
        self.window.draw_2d(e, |c, g| {
                    //       self.render(&c, g);
            clear([1.0; 4], g);
            image(&menu_screen, c.transform, g);
        });
    } 
    

    // This is the function to call to begin execution of the game loop
    pub fn run(&mut self) {

        

        
        while let Some(e) = self.events.next(&mut self.window) {
           //  use piston::input::Event;

            // Determine which window event is next
            if let Some(args) = e.render_args() {
                // Event was a render, so let's draw stuff
                self.render(&e, args);
/*                self.window.draw_2d(&e, |c, g| {
                    self.render(&c, g);
                    //clear([1.0; 4], g);
                    // image(&menu_screen, c.transform, g); 
                    
                }); */
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
