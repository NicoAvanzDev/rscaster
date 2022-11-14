use engine::engine::Engine;
use graphics::drawable::{grid::Grid, player::Player};

mod engine;
pub mod graphics;
mod physics;

fn main() -> Result<(), String> {
    let mut engine = Engine::initialize();

    engine.add_drawable(Box::new(Grid::new()));
    engine.add_drawable(Box::new(Player::new()));

    engine.run();

    Ok(())
}
