use tile::Tile;
use assets_manager::AssetsManager;

use std::vec::Vec;

use game::TILE_SIZE;

use rand;

pub struct Board<'a> {
    contents: Vec<Tile<'a>>,
    assets_manager: &'a AssetsManager<'a>,
}

impl<'a> Board<'a> {
    pub fn new(assets_manager: &'a AssetsManager<'a>) -> Board<'a> {

//        use std::iter;
//        let contents =
//            iter::repeat(
//                iter::repeat(Tile::new(assets_manager))
//                .take(4)
//                .collect()
//            )
//            .take(4)
//            .collect();

        Board {
            contents: vec![],
            assets_manager: assets_manager,
        }
    }

    pub fn generate_tile(&self) -> Option<[f64; 2]> {
        if self.contents.len() >= (4 * 4) {
            return None;
        }

        let mut rg = rand::thread_rng();
        let range = rand::distributions::Range::new(0, 10000);

        use rand::distributions::IndependentSample;

        for _ in 0..100 {
            let x = f64::from(range.ind_sample(&mut rg)) % 4.0 * TILE_SIZE;
            let y = f64::from(range.ind_sample(&mut rg)) % 4.0 * TILE_SIZE;


            if self.check_availability(x, y) {
                return Some([x, y]);
            }
        }

        None
    }

    pub fn check_availability(&self, x: f64, y: f64) -> bool {
        !self.contents.iter().any(|tile| {
            // https://stackoverflow.com/questions/13390333/two-rectangles-intersection

            (tile.x - x).abs() < TILE_SIZE && (tile.y - y).abs() < TILE_SIZE
        })
    }

    pub fn insert_tile(&mut self, tile: Tile<'a>) {
        self.contents.push(tile);
    }
}

use drawable::Drawable;
use drawable::prelude::*;

impl<'a> Drawable for Board<'a> {
    fn render(&mut self, args: &RenderArgs, c: &Context, gl: &mut GlGraphics) {
        for tile in &mut self.contents {
            tile.render(args, c, gl);
        }
    }
}