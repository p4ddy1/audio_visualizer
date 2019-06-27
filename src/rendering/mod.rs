extern crate sdl2;

use self::sdl2::render::Canvas;
use self::sdl2::video::Window;

pub mod bar_renderer;

pub trait Renderer {
    fn new() -> Self;
    fn render_data(&mut self, data: &[f64], canvas: &mut Canvas<Window>);
}
