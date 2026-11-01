use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    let playerFront = load_texture("assets/textures/sprites/player/placeholder1.png").await.unwrap();
    let playerBack = load_texture("assets/textures/sprites/player/placeholder2.png").await.unwrap();
    let mut player = &playerFront;
    playerFront.set_filter(FilterMode::Nearest);
    playerBack.set_filter(FilterMode::Nearest);
    let mut position = vec2(0.0, 0.0);
    loop {
        clear_background(BLACK);
        draw_text("404 NOT FOUND", 20.0, 20.0, 20.0, DARKBLUE);
        
        draw_texture_ex(&player, position.x, position.y, WHITE, DrawTextureParams { 
            dest_size: Some(vec2(512.0, 512.0)), 
            ..Default::default() }
        );
        if is_key_down(KeyCode::Z) {
            player = &playerBack;
            position.y -= 1.0;
        } else if is_key_down(KeyCode::S) {
            player = &playerFront;
            position.y += 1.0;
        } else if is_key_down(KeyCode::Q) {
            position.x -= 1.0;
        } else if is_key_down(KeyCode::D) {
            position.x += 1.0;
        }

        next_frame().await
    }
}