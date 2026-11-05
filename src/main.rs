mod render;
mod sprite;

use macroquad::prelude::*;
use render::Renderer;

use crate::sprite::Sprite;

#[macroquad::main("404 Job not found")]
async fn main() {
    let player_front = load_texture("assets/textures/sprites/player/placeholder1.png").await.unwrap();
    let player_back = load_texture("assets/textures/sprites/player/placeholder2.png").await.unwrap();
    let mut renderer = Renderer::new();
    let mut player = Sprite::new(0.0, 0.0, &player_front);
    // let mut player = &playerFront;
    player_front.set_filter(FilterMode::Nearest);
    player_back.set_filter(FilterMode::Nearest);
    loop {
        clear_background(BLACK);
        draw_text("404 NOT FOUND", 20.0, 20.0, 20.0, DARKBLUE);
        if is_key_down(KeyCode::Z) {
            player.texture = &player_back;
            player.position.y -= 1.0;
        } else if is_key_down(KeyCode::S) {
            player.texture = &player_front;
            player.position.y += 1.0;
        } else if is_key_down(KeyCode::Q) {
            player.position.x -= 1.0;
        } else if is_key_down(KeyCode::D) {
            player.position.x += 1.0;
        }
        renderer.update();
        renderer.render(&player);
        next_frame().await
    }
}