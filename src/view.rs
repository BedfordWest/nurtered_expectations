/// This module handles rendering for the game

use Direction;
use gfx_device_gl::{ Resources };
use graphics::types::SourceRectangle;
use piston_window::*;
use player::{ Player, PlayerState };
use sprite::Sprite;
use std::rc::Rc;

/// The view will hold textures, sprites, and rendering information
pub struct View
{
    position: (u32, u32),
    char_sprite: Option<Sprite<Texture<Resources>>>,
}

/// Implement rendering/display logic for the game
impl View
{
    /// Create a new view - set the initial view position
    pub fn new() -> Self {
        View {
            position: (0,0),
            char_sprite: None,
        }
    }    

    /// Render the main menu
    pub fn render_menu(c: &Context, g: &mut G2d, menu: &Texture<Resources>) {
        clear([1.0; 4], g);
        image(menu, c.transform, g);
    }

    /// Render the player's sprite
    pub fn render_player(&mut self, c: &Context, g: &mut G2d, player: &Player) {
        let mut sprite = self.char_sprite.as_mut().unwrap();
        sprite.set_position(player.get_position().0, player.get_position().1);
        match *player.get_state() {
            PlayerState::Walking(Direction::Right) => {
                let src_rect1: SourceRectangle = [0.0, 704.0, 64.0, 64.0];
                let src_rect2: SourceRectangle = [64.0, 704.0, 64.0, 64.0];
                let src_rect3: SourceRectangle = [128.0, 704.0, 64.0, 64.0];
                let src_rect4: SourceRectangle = [192.0, 704.0, 64.0, 64.0];
                let src_rect5: SourceRectangle = [256.0, 704.0, 64.0, 64.0];
                let src_rect6: SourceRectangle = [320.0, 704.0, 64.0, 64.0];
                let src_rect7: SourceRectangle = [384.0, 704.0, 64.0, 64.0];
                let src_rect8: SourceRectangle = [448.0, 704.0, 64.0, 64.0];
                let src_rect9: SourceRectangle = [512.0, 704.0, 64.0, 64.0];                
                match player.get_dt() {
                    0.0 ... 0.11 => sprite.set_src_rect(src_rect1),
                    0.11 ... 0.22 => sprite.set_src_rect(src_rect2),
                    0.22 ... 0.33 => sprite.set_src_rect(src_rect3),
                    0.33 ... 0.44 => sprite.set_src_rect(src_rect4),
                    0.44 ... 0.55 => sprite.set_src_rect(src_rect5),
                    0.55 ... 0.66 => sprite.set_src_rect(src_rect6),
                    0.66 ... 0.77 => sprite.set_src_rect(src_rect7),
                    0.77 ... 0.88 => sprite.set_src_rect(src_rect8),
                    0.88 ... 0.99 => sprite.set_src_rect(src_rect9),                    
                    _ => sprite.set_src_rect(src_rect9),
                }
            },

            PlayerState::Walking(Direction::Left) => {
                let src_rect1: SourceRectangle = [0.0, 576.0, 64.0, 64.0];
                let src_rect2: SourceRectangle = [64.0, 576.0, 64.0, 64.0];
                let src_rect3: SourceRectangle = [128.0, 576.0, 64.0, 64.0];
                let src_rect4: SourceRectangle = [192.0, 576.0, 64.0, 64.0];
                let src_rect5: SourceRectangle = [256.0, 576.0, 64.0, 64.0];
                let src_rect6: SourceRectangle = [320.0, 576.0, 64.0, 64.0];
                let src_rect7: SourceRectangle = [384.0, 576.0, 64.0, 64.0];
                let src_rect8: SourceRectangle = [448.0, 576.0, 64.0, 64.0];
                let src_rect9: SourceRectangle = [512.0, 576.0, 64.0, 64.0];                
                match player.get_dt() {
                    0.0 ... 0.11 => sprite.set_src_rect(src_rect1),
                    0.11 ... 0.22 => sprite.set_src_rect(src_rect2),
                    0.22 ... 0.33 => sprite.set_src_rect(src_rect3),
                    0.33 ... 0.44 => sprite.set_src_rect(src_rect4),
                    0.44 ... 0.55 => sprite.set_src_rect(src_rect5),
                    0.55 ... 0.66 => sprite.set_src_rect(src_rect6),
                    0.66 ... 0.77 => sprite.set_src_rect(src_rect7),
                    0.77 ... 0.88 => sprite.set_src_rect(src_rect8),
                    0.88 ... 0.99 => sprite.set_src_rect(src_rect9),                    
                    _ => sprite.set_src_rect(src_rect9),
                }
            },

            PlayerState::Jumping(Direction::Right) => {
                let src_rect1: SourceRectangle = [0.0, 192.0, 64.0, 64.0];
                let src_rect2: SourceRectangle = [64.0, 192.0, 64.0, 64.0];
                let src_rect3: SourceRectangle = [128.0, 192.0, 64.0, 64.0];
                let src_rect4: SourceRectangle = [192.0, 192.0, 64.0, 64.0];
                let src_rect5: SourceRectangle = [256.0, 192.0, 64.0, 64.0];
                let src_rect6: SourceRectangle = [320.0, 192.0, 64.0, 64.0];
                let src_rect7: SourceRectangle = [384.0, 192.0, 64.0, 64.0];
                match player.get_dt() {
                    0.0 ... 0.11 => sprite.set_src_rect(src_rect1),
                    0.11 ... 0.22 => sprite.set_src_rect(src_rect2),
                    0.22 ... 0.33 => sprite.set_src_rect(src_rect3),
                    0.33 ... 0.44 => sprite.set_src_rect(src_rect4),
                    0.44 ... 0.55 => sprite.set_src_rect(src_rect5),
                    0.55 ... 0.66 => sprite.set_src_rect(src_rect6),
                    0.66 ... 0.77 => sprite.set_src_rect(src_rect7),
                    _ => sprite.set_src_rect(src_rect7),                
                }
            },

            PlayerState::Jumping(Direction::Left) => {
                let src_rect1: SourceRectangle = [0.0, 64.0, 64.0, 64.0];
                let src_rect2: SourceRectangle = [64.0, 64.0, 64.0, 64.0];
                let src_rect3: SourceRectangle = [128.0, 64.0, 64.0, 64.0];
                let src_rect4: SourceRectangle = [192.0, 64.0, 64.0, 64.0];
                let src_rect5: SourceRectangle = [256.0, 64.0, 64.0, 64.0];
                let src_rect6: SourceRectangle = [320.0, 64.0, 64.0, 64.0];
                let src_rect7: SourceRectangle = [384.0, 64.0, 64.0, 64.0];
                match player.get_dt() {
                    0.0 ... 0.11 => sprite.set_src_rect(src_rect1),
                    0.11 ... 0.22 => sprite.set_src_rect(src_rect2),
                    0.22 ... 0.33 => sprite.set_src_rect(src_rect3),
                    0.33 ... 0.44 => sprite.set_src_rect(src_rect4),
                    0.44 ... 0.55 => sprite.set_src_rect(src_rect5),
                    0.55 ... 0.66 => sprite.set_src_rect(src_rect6),
                    0.66 ... 0.77 => sprite.set_src_rect(src_rect7),
                    _ => sprite.set_src_rect(src_rect7),
                }
            },

            PlayerState::Falling(Direction::Right) => {
                let src_rect: SourceRectangle = [320.0, 192.0, 64.0, 64.0];
                sprite.set_src_rect(src_rect);
            },

            PlayerState::Falling(Direction::Left) => {
                let src_rect: SourceRectangle = [320.0, 64.0, 64.0, 64.0];
                sprite.set_src_rect(src_rect);
            },            
            
            PlayerState::Standing(Direction::Right) => {
                let src_rect: SourceRectangle = [0.0, 704.0, 64.0, 64.0];
                sprite.set_src_rect(src_rect);
            },

            PlayerState::Standing(Direction::Left) => {
                let src_rect: SourceRectangle = [0.0, 576.0, 64.0, 64.0];
                sprite.set_src_rect(src_rect);
            },            
        }
            
        sprite.draw(c.transform, g);
    }

    /// Load the player's sprite from a texture
    pub fn load_player_sprite(&mut self, player: &Player, tex_rc: Rc<Texture<Resources>>) {
        self.char_sprite = Some(Sprite::from_texture(tex_rc));
        let mut sprite = self.char_sprite.as_mut().unwrap();
        sprite.set_position(300.0, 300.0);
    }

}
