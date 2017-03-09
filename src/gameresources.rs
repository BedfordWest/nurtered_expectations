/// Handle the resources required for the game - music, images, etc.

use find_folder::Search;
use gfx_device_gl::{ Resources };
use piston_window::*;
use std::rc::Rc;

pub struct GameResources {
    menu_texture: Texture<Resources>,
    char_texture: Rc<Texture<Resources>>,
}

impl GameResources {

    pub fn new(w: &PistonWindow) -> Self {

        let assets = Search::ParentsThenKids(3,3)
            .for_folder("assets").unwrap();

        // Load the main menu image
        let menu_path = assets.join("crystal-caves.jpg");
        let menu_texture = Texture::from_path(
            &mut w.factory.clone(),
            &menu_path,
            Flip::None,
            &TextureSettings::new()
        ).unwrap();

        // Load the character spritesheet texture
        let char_path = assets.join("char_example.png");
        let char_texture = Rc::new(Texture::from_path(
            &mut w.factory.clone(),
            &char_path,
            Flip::None,
            &TextureSettings::new()
        ).unwrap());
        
        GameResources {
            menu_texture: menu_texture,
            char_texture: char_texture,
        }
    }

    pub fn get_menu_texture(&self) -> &Texture<Resources> {
        &self.menu_texture
    }

    pub fn get_char_texture_rc(&self) -> Rc<Texture<Resources>> {
        self.char_texture.clone()
    }
    
}
