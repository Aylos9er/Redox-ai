// This submodule contians render related stuff.

use orbgl::Canvas;

use crate::theme::Theme;

pub use self::shapes::*;
pub use self::structs::*;
pub use self::renderer::Renderer;

mod shapes;
mod structs;
mod renderer;

/// Provides data for rendering like renderer and theme.
pub struct RenderContext<'a> {
    pub renderer: &'a mut Canvas,
    pub theme: &'a Theme,
}