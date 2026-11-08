use macroquad::prelude::*;

use crate::sprite::Sprite;

const VIEWPORT_SIZE: i32 = 256;
pub struct Renderer {
    scale: f32,
    size: f32,
    offset: Vec2,
}

impl Renderer {
    pub fn new() -> Renderer {
        let mut new_renderer = Renderer {
            scale: 0.0,
            size: 0.0,
            offset: vec2(0.0, 0.0),
        };
        new_renderer.update();
        return new_renderer;
    }

    pub fn update(&mut self) {
        let width = screen_width();
        let height = screen_height();
        self.size = f32::min(width, height);
        self.scale = self.size / VIEWPORT_SIZE as f32;
        self.offset.x = (width - self.size) / 2.0;
        self.offset.y = (height - self.size) / 2.0;
        draw_rectangle(self.offset.x, self.offset.y, self.size, self.size, DARKGRAY);
    }

    pub fn render(&self, sprite: &Sprite) {
        let draw_parameters = DrawTextureParams {
            dest_size: Some(vec2(
                sprite.texture.width() * self.scale,
                sprite.texture.height() * self.scale,
            )),
            ..Default::default()
        };
        draw_texture_ex(
            &sprite.texture,
            sprite.position.x * self.scale + self.offset.x,
            sprite.position.y * self.scale + self.offset.y,
            WHITE,
            draw_parameters,
        );
    }
}
