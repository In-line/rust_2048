
pub mod prelude {
    pub use piston::input::RenderArgs;
    pub use graphics::Context;
    pub use opengl_graphics::GlGraphics;
}

use self::prelude::*;

pub trait Drawable {
    fn render(&mut self, args: &RenderArgs, c: &Context, gl: &mut GlGraphics);
}

