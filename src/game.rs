use tile::Tile;
use board::Board;
use assets_manager::AssetsManager;

pub const TILE_SIZE: f64 = 100.0;

pub struct Game<'a> {
    board: Board<'a>,
    _assets_manager: &'a AssetsManager<'a>,
}

impl<'a> Game<'a> {
    pub fn new(assets_manager: &'a AssetsManager<'a>) -> Game<'a> {

        let get_board = || {
            let mut board = Board::new(assets_manager);

            for _ in 0..2 {
                if let Some(pos) = board.generate_tile() {
                    board.insert_tile(
                        Tile::new(assets_manager)
                            .x(pos[0] + 50.0)
                            .y(pos[1] + 50.0)
                            .number(Some(2))
                            .size(TILE_SIZE)
                    );
                }
                else {
                    //panic!("Can't create!");
                }
            };

            board.insert_tile(
                Tile::new(assets_manager)
                    .x(50.0)
                    .y(50.0)
                    .number(Some(8))
                    .size(TILE_SIZE)
            );

            board
        };

        Game {
            board: get_board(),
            _assets_manager: assets_manager,
        }
    }
}

use drawable::Drawable;
use drawable::prelude::*;

impl<'a> Drawable for Game<'a> {
    fn render(&mut self, args: &RenderArgs, c: &Context, gl: &mut GlGraphics) {

        gl.draw(args.viewport(), |_, gl| {
            use graphics::clear;
            clear([1.0, 1.0, 1.0, 1.0], gl);

            self.board.render(args, c, gl);
        });
    }
}