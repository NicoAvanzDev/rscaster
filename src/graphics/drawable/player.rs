use std::f32::consts::PI;

use sdl2::{
    keyboard::{KeyboardState, Scancode},
    pixels::Color,
    rect::Rect,
    render::Canvas,
    video::Window,
};

use crate::physics::physics::Vector2;

use super::{line::Line, Drawable};

const PLAYER_SIZE: f32 = 20.0;
const PLAYER_SIZE_HALF: f32 = PLAYER_SIZE / 2.0;

const RAY_LENGTH: f32 = 20.0;

const TURN_SPEED: f32 = 2.0;

pub struct Player {
    position: Vector2,
    direction: Vector2,
    rotation: f32,
    rect: Rect,
    ray: Line,
}

impl Player {
    pub fn new() -> Player {
        let pos = Vector2::new(150.0, 150.0);
        Player {
            position: pos,
            direction: Vector2::right(),
            rotation: 0.0,
            rect: Rect::new(
                (pos.x() - PLAYER_SIZE_HALF) as i32,
                (pos.y() - PLAYER_SIZE_HALF) as i32,
                PLAYER_SIZE as u32,
                PLAYER_SIZE as u32,
            ),
            ray: Line::new(
                pos.x() + PLAYER_SIZE_HALF,
                pos.y() + PLAYER_SIZE_HALF,
                pos.x() + PLAYER_SIZE_HALF * RAY_LENGTH,
                pos.y() + PLAYER_SIZE_HALF * RAY_LENGTH,
            ),
        }
    }

    fn handle_keyboard(&mut self, kb_state: &KeyboardState) {
        if kb_state.is_scancode_pressed(Scancode::W) {
            self.position = self.position.add(self.direction);
        }
        if kb_state.is_scancode_pressed(Scancode::A) {
            self.rotation -= 0.1;

            if self.rotation < 0.0 {
                self.rotation += PI * 2.0;
            }

            self.direction = Vector2::new(
                self.rotation.cos() * TURN_SPEED,
                self.rotation.sin() * TURN_SPEED,
            );
        }
        if kb_state.is_scancode_pressed(Scancode::S) {
            self.position = self.position.sub(self.direction);
        }
        if kb_state.is_scancode_pressed(Scancode::D) {
            self.rotation += 0.1;

            if self.rotation > PI * 2.0 {
                self.rotation -= PI * 2.0;
            }

            self.direction = Vector2::new(
                self.rotation.cos() * TURN_SPEED,
                self.rotation.sin() * TURN_SPEED,
            );
        }
    }
}

impl Drawable for Player {
    fn setup(&mut self) {}

    fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RED);

        canvas.fill_rect(self.rect).unwrap();

        self.ray.draw(canvas);
    }

    fn fixed_update(&mut self, kb_state: &KeyboardState) {
        self.handle_keyboard(kb_state);

        self.ray.set_pos(
            self.position.x() + PLAYER_SIZE_HALF,
            self.position.y() + PLAYER_SIZE_HALF,
            self.ray.x1() + self.direction.x() * RAY_LENGTH,
            self.ray.y1() + self.direction.y() * RAY_LENGTH,
        );

        self.rect.set_x((self.position.x()) as i32);
        self.rect.set_y((self.position.y()) as i32);
    }
}
