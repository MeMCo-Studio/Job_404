mod assets;
mod inputs;
mod render;
mod sprite;
mod world_state;

use macroquad::prelude::*;
use render::Renderer;

use crate::{assets::Assets, world_state::WorldState};

#[macroquad::main("404 Job not found")]
async fn main() {
    let assets = Assets::new().await;
    let mut renderer = Renderer::new();
    let mut world_state = WorldState::new(&assets);
    // let mut player = &playerFront;
    loop {
        clear_background(BLACK);
        draw_text("404 NOT FOUND", 20.0, 20.0, 20.0, DARKBLUE);
        world_state.update();
        renderer.update();
        world_state.render(&renderer);
        next_frame().await
    }
}