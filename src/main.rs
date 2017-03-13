// Organize Nurtered Expectations as such:
// Main (module): responsible for initializing the game and passing control to the Game object
// Game (module, struct): handles the main game loop and logic, and implements traits necessary
//                        for this, including Render, Update, and Input
// Render (module, trait): handles behavior related to rendering the screen
// InputMouse (trait): handles behavior occuring when mouse input is received
// InputKeys (trait): handles behavior occuring when keyboard input is received
// TODO: InputTouch (trait): handles behavior occuring when touch input is received
// TODO: InputController (trait): handles behavior specific to controllers

extern crate find_folder;
extern crate gfx_device_gl;
extern crate glutin_window;
extern crate graphics;
extern crate ncollide;
extern crate opengl_graphics;
extern crate piston;
extern crate piston_window;
extern crate sprite;
extern crate touch_visualizer;

use game::Game;
use piston_window::*;

mod block;
mod game;
mod input;
mod level;
mod player;
mod view;
mod gameresources;

const OPENGL_VERSION: OpenGL = OpenGL::V3_2;

/// Pixels/second impact of gravity
pub const GRAVITY: f64 = 80.0;

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Left,
    Right,
}

fn main() {
    
    // TODO: Enter config loading & parsing here

    let (width, height) = (1920, 1080);    
    let mut window: PistonWindow<> =
        WindowSettings::new("Nurtured Expectations", (width, height))
        .opengl(OPENGL_VERSION)
        .resizable(true)
        .decorated(false)
        .exit_on_esc(true)
        .fullscreen(true)
        .build()
        .unwrap();    

    // Create the 'Game' instance
    let mut game = Game::new(&window);
    // Run the game
    game.run(window);

}

