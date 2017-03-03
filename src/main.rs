// Organize Nurtered Expectations as such:
// Main (module): responsible for initializing the game and passing control to the Game object
// Game (module, struct): handles the main game loop and logic, and implements traits necessary
//                        for this, including Render, Update, and Input
// Render (module, trait): handles behavior related to rendering the screen
// InputMouse (trait): handles behavior occuring when mouse input is received
// InputKeys (trait): handles behavior occuring when keyboard input is received
// TODO: InputTouch (trait): handles behavior occuring when touch input is received
// TODO: InputController (trait): handles behavior specific to controllers


extern crate gfx_device_gl;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate piston_window;
extern crate touch_visualizer;

use game::Game;

mod config;
mod game;
mod player;

fn main() {

    // TODO: Enter config loading & parsing here


    // Create the 'Game' instance
    let mut game = Game::new();
    // Run the game
    game.run();

}

