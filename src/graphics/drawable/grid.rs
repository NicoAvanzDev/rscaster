use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

use crate::graphics::drawable::Drawable;

const ROWS: usize = 10;
const COLS: usize = 10;

const GRID_CELL_SIZE: usize = 80;

const MAP: [[usize; ROWS]; COLS] = [
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 0, 0, 0, 0, 0, 1, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 1, 0, 0, 1],
    [1, 0, 0, 1, 1, 1, 1, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 1, 1, 1, 1, 1],
    [1, 0, 1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 1, 0, 0, 0, 0, 0, 0, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
];

pub struct Grid {}

impl Grid {
    pub fn new() -> Self {
        Grid {}
    }
}

impl Drawable for Grid {
    fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::WHITE);
        for row in 0..ROWS {
            for col in 0..COLS {
                let x = col * GRID_CELL_SIZE;
                let y = row * GRID_CELL_SIZE;
                let mut rect = Rect::new(
                    x as i32 + 1,
                    y as i32 + 1,
                    GRID_CELL_SIZE as u32 - 1,
                    GRID_CELL_SIZE as u32 - 1,
                );
                if MAP[row][col] == 1 {
                    canvas.fill_rect(rect).unwrap();
                } else {
                    rect.set_x(x as i32);
                    rect.set_y(y as i32);
                    rect.set_width(GRID_CELL_SIZE as u32 + 1);
                    rect.set_height(GRID_CELL_SIZE as u32 + 1);
                    canvas.draw_rect(rect).unwrap();
                }
            }
        }
    }
}
