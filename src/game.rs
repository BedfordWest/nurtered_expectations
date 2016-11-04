use piston::event_loop::{
    EventLoop,
    Events,
    WindowEvents,
};

// Game struct - want to make sure it is generic over a backend, generic event, graphics, and window
pub struct Game<B, E, G, W>
    where B: Backend,
          E: GenericEvent,
          G: Graphics<Texture=B::Texture>,
          W: AdvancedWindow + Window,
{
    events: WindowEvents,
    window: W,
}

impl<B, E, G, W> Game<B, E, G, W>
    where B: Backend + 'static,
          E: GenericEvent,
          G: Graphics<Texture=B::Texture>,
          W: AdvancedWindow + Window,
{
    pub fn new(window: W) -> Self {

        // Set up the WindowEvents for the Game instance
        // in the future, add ups(config.ups) and max_fps(config.max_fps)
        let events = window.events();

        Game {
            events: events,
            window: window,
        }
    }

}

// Implementation of Game specific for opengl_graphics
impl<W> Game<GlBackend, Event<W::Event>, GlGraphics, W>
    where W: AdvancedWindow + Window,
          W::Event: GenericEvent,
{
    // This is the function to call to begin execution of the game loop
    pub fn run(&mut self, gl: &mut GlGraphics) {
        while let Some(e) = self.events.next(&mut self.window) {
            use piston::input::Event;

            // Determine which window event is next
            if let Some(args) = e.render_args() {
                // Event was a render, so let's draw stuff
                self.render(&args, &mut gl);
            }

            // TODO: Implement input handlers
            // if let Some(args) = e.press_args(&args) { }
            // if let Some(args) = e.release_args(&args) { }

            if let Some(args) = e.update_args() {
                self.update(&args);
            }           
        } 
    }
}
