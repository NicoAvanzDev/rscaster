use rayon::prelude::{IntoParallelRefMutIterator, ParallelIterator};
use sdl2::{
    event::Event, keyboard::KeyboardState, pixels::Color, render::Canvas, video::Window, Sdl,
};

use crate::graphics::drawable::Drawable;

use super::constants::{FRAMERATE, WINDOW_HEIGHT, WINDOW_WIDTH};

type DrawablePar = Box<dyn Drawable + Send + Sync>;

pub struct Engine {
    canvas: Canvas<Window>,
    context: Sdl,
    drawables: Vec<DrawablePar>,
}

impl Engine {
    pub fn initialize() -> Engine {
        let context = sdl2::init().unwrap();
        let video = context.video().unwrap();

        let window = video
            .window("rscaster", WINDOW_WIDTH * 2, WINDOW_HEIGHT)
            .build()
            .expect("unable to create window");

        let canvas = window
            .into_canvas()
            .build()
            .expect("unable to create canvas");

        Engine {
            canvas,
            context,
            drawables: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        // Setup game objects
        self.setup();

        let mut initial_ticks;
        let mut delta_time = 0;
        let mut ticks = self.context.timer().unwrap().ticks();

        let mut event_pump = self.context.event_pump().unwrap();

        // Main game loop
        'running: loop {
            initial_ticks = self.context.timer().unwrap().ticks();
            self.canvas.set_draw_color(Color::BLACK);
            self.canvas.clear();

            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'running,
                    _ => {}
                }
            }

            delta_time += initial_ticks - ticks;

            // Fixed update for physics
            if delta_time >= FRAMERATE {
                let kb_state = KeyboardState::new(&event_pump);
                self.drawables.par_iter_mut().for_each(|drawable| {
                    drawable.fixed_update(&kb_state);
                });
                delta_time = 0;
            }

            ticks = self.context.timer().unwrap().ticks();

            // Draw game objects
            self.render();

            self.canvas.present();
        }
    }

    pub fn add_drawable(&mut self, drawable: DrawablePar) {
        self.drawables.push(drawable);
    }

    fn render(&mut self) {
        for drawable in &mut self.drawables {
            drawable.draw(&mut self.canvas);
        }
    }

    fn setup(&mut self) {
        self.drawables.par_iter_mut().for_each(|drawable| {
            drawable.setup();
        });
    }
}
