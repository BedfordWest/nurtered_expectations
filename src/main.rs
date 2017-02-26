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
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate piston_window;

use game::Game;

mod game;
mod config;

fn main() {

    // TODO: Enter config loading & parsing here

    // Create the 'Game' instance
    let mut game = Game::new();
    // Run the game
    game.run();

}

