#![feature(plugin)]
#![plugin(clippy)]

extern crate piston;
extern crate graphics;
extern crate opengl_graphics;
extern crate glutin_window;
extern crate rand;

use piston::window::WindowSettings;
use glutin_window::GlutinWindow;

mod game;
mod tile;
mod board;
mod drawable;
mod assets_manager;

use drawable::prelude::*;

fn main() {
    let (width, height) = (1000, 900);

    let mut window: GlutinWindow =
        WindowSettings::new("2048", [width, height])
            .exit_on_esc(true)
            .resizable(false)
            .build()
            .unwrap();

    let assets_manager = assets_manager::AssetsManager::load();

    let mut game = game::Game::new(&assets_manager);

    use opengl_graphics::{GlGraphics, OpenGL};
    let mut gl = GlGraphics::new(OpenGL::V3_2);

    use piston::event_loop::{Events, EventLoop, EventSettings};


    let mut events = Events::new(EventSettings::new().lazy(false));

    while let Some(e) = events.next(&mut window) {
        use drawable::Drawable;
        use piston::input::RenderEvent;

        if let Some(ref args) = e.render_args() {
            let c = &Context::new_abs(f64::from(args.width), f64::from(args.height));
            game.render(args, c, &mut gl);
        }
    }

    println!("Hello, world!");
}
