/// This module handles rendering for the game

use gfx_device_gl::{ Resources };
use piston_window::*;
use sprite::Sprite;

/// The view will hold textures, sprites, and rendering information
pub struct View
{
    position: (u32, u32),
    char_sprite: Option<Sprite<Texture<Resources>>>,
}

/// Implement rendering/display logic for the game
impl View
{
    pub fn new() -> Self {
        View {
            position: (0,0),
            char_sprite: None,
        }
    }    
    
    pub fn new_from_window(w: &PistonWindow) -> Self {
        View {
            position: (w.size().width, w.size().height),
            char_sprite: None,
        }
    }

}
