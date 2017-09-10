use std::env;
use opengl_graphics::glyph_cache::GlyphCache;
use opengl_graphics::TextureSettings;

use std::cell::{RefCell, RefMut};

pub struct AssetsManager<'a> {
    primary_font: RefCell<GlyphCache<'a>>,
}

impl<'a> AssetsManager<'a> {

    pub fn load() -> AssetsManager<'a> {
        let exe_directory = env::current_exe().unwrap().parent().unwrap().to_owned();

        let exe_directory = &exe_directory.join("assets/FiraSans-Regular.ttf");

        AssetsManager {
            primary_font: RefCell::new(GlyphCache::new(exe_directory, TextureSettings::new()).unwrap()),
        }
    }

    pub fn get_primary_font(&self) -> RefMut<GlyphCache<'a>> {
        self.primary_font.borrow_mut()
    }
}