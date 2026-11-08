use std::{
    hash::Hash,
    sync::atomic::{AtomicUsize, Ordering},
};

use macroquad::{
    color::WHITE,
    math::{Vec2, vec2},
    texture::{Texture2D, draw_texture},
};

static SPRITE_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug)]
pub struct Sprite<'a> {
    id: usize,
    pub position: Vec2,
    pub texture: &'a Texture2D,
}

impl<'a> Sprite<'a> {
    pub fn new(x: f32, y: f32, texture: &'a Texture2D) -> Sprite<'a> {
        return Sprite {
            id: SPRITE_COUNTER.fetch_add(1, Ordering::SeqCst),
            position: vec2(x, y),
            texture: texture,
        };
    }

    pub fn render(&self) {
        draw_texture(self.texture, self.position.x, self.position.y, WHITE);
    }
}

impl Hash for Sprite<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for Sprite<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Sprite<'_> {}
