mod assets;
mod desktop;
mod game_state;
mod inputs;
mod sprite;
mod world_state;

use macroquad::prelude::*;

use crate::{assets::Assets, game_state::Game, world_state::WORLD_SIZE};
#[macroquad::main("404 Job not found")]
async fn main() {
    let assets = Assets::new().await;
    let mut game = Game::new(&assets).await;

    let render_target = render_target(WORLD_SIZE as u32, WORLD_SIZE as u32);
    render_target.texture.set_filter(FilterMode::Nearest);
    let mut render_target_cam =
        Camera2D::from_display_rect(Rect::new(0., 0., WORLD_SIZE, WORLD_SIZE));
    render_target_cam.render_target = Some(render_target.clone());
    egui_macroquad::cfg(desktop::style::configure_style);

    loop {
        let scale = f32::min(screen_width() / WORLD_SIZE, screen_height() / WORLD_SIZE);

        set_camera(&render_target_cam);

        // Update
        game.update();
        game.render();

        set_default_camera();
        clear_background(BLACK);

        draw_texture_ex(
            &render_target.texture,
            (screen_width() - (WORLD_SIZE as f32 * scale)) * 0.5,
            (screen_height() - (WORLD_SIZE as f32 * scale)) * 0.5,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(WORLD_SIZE as f32 * scale, WORLD_SIZE as f32 * scale)),
                flip_y: true, // Must flip y otherwise 'render_target' will be upside down
                ..Default::default()
            },
        );

        // draw gui on top
        egui_macroquad::draw();

        draw_fps();
        next_frame().await
    }
}
