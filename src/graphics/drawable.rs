pub mod grid;
mod line;
pub mod player;

use sdl2::{keyboard::KeyboardState, render::Canvas, video::Window};

pub trait Drawable {
    fn draw(&self, _canvas: &mut Canvas<Window>) {}
    fn setup(&mut self) {}
    fn update(&self) {}
    fn fixed_update(&mut self, _kb_state: &KeyboardState) {}
}
