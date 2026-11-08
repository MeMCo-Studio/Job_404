mod assets;
mod desktop;
mod game_state;
mod inputs;
mod render;
mod sprite;
mod world_state;

use macroquad::prelude::*;
use render::Renderer;

use crate::{
    assets::Assets,
    desktop::Desktop,
    game_state::{GameScene, GameState},
    world_state::WorldState,
};

#[macroquad::main("404 Job not found")]
async fn main() {
    let assets = Assets::new().await;
    let mut renderer = Renderer::new();
    let mut world_state = WorldState::new(&assets);
    let mut desktop = Desktop::new();
    let mut game = GameState::default();

    egui_macroquad::cfg(desktop::style::configure_style);

    loop {
        clear_background(BLACK);
        draw_text("404 NOT FOUND", 20.0, 20.0, 20.0, DARKBLUE);

        renderer.update();

        // Update
        match game.scene {
            GameScene::World => world_state.update(&mut game),
            GameScene::Desktop => desktop.update(&mut game),
        }

        // Render ui + world
        egui_macroquad::ui(|ctx| match game.scene {
            GameScene::World => {} // HUD
            GameScene::Desktop => desktop.show(ctx, &mut game),
        });

        if game.scene == GameScene::World {
            world_state.render(&renderer);
        }

        // draw gui on top
        egui_macroquad::draw();

        next_frame().await
    }
}
