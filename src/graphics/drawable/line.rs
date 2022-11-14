use sdl2::{pixels::Color, render::Canvas, video::Window};

use super::Drawable;

pub struct Line {
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
}

impl Line {
    pub fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Line {
        Line { x1, y1, x2, y2 }
    }

    pub fn set_pos(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) {
        self.x1 = x1;
        self.y1 = y1;
        self.x2 = x2;
        self.y2 = y2;
    }

    pub fn x1(&self) -> f32 {
        self.x1
    }

    pub fn y1(&self) -> f32 {
        self.y1
    }
}

impl Drawable for Line {
    fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::BLUE);
        canvas
            .draw_line(
                (self.x1 as i32, self.y1 as i32),
                (self.x2 as i32, self.y2 as i32),
            )
            .unwrap();
    }
}
