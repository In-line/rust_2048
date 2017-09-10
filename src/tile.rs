use drawable::Drawable;
use drawable::prelude::*;

use assets_manager::AssetsManager;

//#[derive(Copy, Clone)]
pub struct Tile<'a> {
    pub x: f64,
    pub y: f64,
    pub number: Option<i32>,
    pub size: f64,
    pub assets_manager: &'a AssetsManager<'a>,
}


impl<'a> Tile<'a> {
    pub fn new(assets_manager: &'a AssetsManager<'a>) -> Tile<'a> {
        Tile {
            x: 0.0_f64,
            y: 0.0_f64,
            number: None,
            size: 0.0,
            assets_manager: assets_manager,
        }
    }
    pub fn x(mut self, x: f64) -> Tile<'a> {
        self.x = x;
        self
    }

    pub fn y(mut self, y: f64) -> Tile<'a> {
        self.y = y;
        self
    }

    pub fn number(mut self, number: Option<i32>) -> Tile<'a> {
        self.number = number;
        self
    }

    pub fn size(mut self, size: f64) -> Tile<'a> {
        self.size = size;
        self
    }
}


impl<'a> Drawable for Tile<'a> {
    fn render(&mut self, _: &RenderArgs, c: &Context, gl: &mut GlGraphics) {

        if self.size <= 0.0 {
            unreachable!();
        }

        use graphics::*;

//        let centered = rectangle::centered([
//            self.x + self.size / 2.0,
//            self.y - self.size / 2.0,
//            self.size / 2.0,
//            self.size / 2.0]);

        let square = rectangle::square(self.x, self.y, self.size);

        Rectangle::new([0.0, 1.0, 1.0, 1.0])
            .draw(square,
                  &DrawState::default(),
                  c.transform.trans(-self.size / 2.0, -self.size / 2.0),
                  gl
            );

        let number: String = match self.number {
            Some(ref num) => num.to_string(),
            None => "".to_string(),
        };

        use graphics::Transformed;
        use graphics::character::CharacterCache;

        let mut pf = self.assets_manager.get_primary_font();

        let mut text_len_x: f64 = 0.0;
        let mut text_len_y: f64 = 0.0;

        for ch in number.chars() {
            let character = pf.character(34, ch);

            text_len_x += character.width();

            let ch_len_y = character.top();
            if ch_len_y > text_len_y {
                text_len_y = ch_len_y;
            }
        }

        Text::new_color([0.0, 0.0, 1.0, 1.0], 34)
            .draw(&number,
                  &mut *pf,
                  &DrawState::default(),
                  c.trans(self.x - text_len_x / 2.0,
                          self.y + text_len_y / 2.0)
                      .transform,
                  gl);
    }
}