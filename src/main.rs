// Organize Nurtered Expectations as such:
// Main (module): responsible for initializing the game and passing control to the Game object
// Game (module, struct): handles the main game loop and logic, and implements traits necessary
//                        for this, including Render, Update, and Input
// Render (module, trait): handles behavior related to rendering the screen
// InputMouse (trait): handles behavior occuring when mouse input is received
// InputKeys (trait): handles behavior occuring when keyboard input is received
// TODO: InputTouch (trait): handles behavior occuring when touch input is received
// TODO: InputController (trait): handles behavior specific to controllers

extern crate piston;
extern crate piston_window;
extern crate graphics;
extern crate opengl_graphics;

use opengl_graphics::GlGraphics;

mod game;

use piston_window::*;

use game::Game;

const OPENGL_VERSION: OpenGL = OpenGL::V3_2;

fn main() {

    // TODO: Enter config loading & parsing here
    
    // Create the game window
    let mut window: PistonWindow =
        WindowSettings::new("Nurtered Expectations", [640, 480])
        .exit_on_esc(true).build().unwrap();

    // Initialize the graphics backend
    let mut gl = GlGraphics::new(OPENGL_VERSION);

    // Create the 'Game' instance
    let mut game = Game::new(window);
    // Run the game
    game.run(&mut gl);

}

