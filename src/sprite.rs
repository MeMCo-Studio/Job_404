use std::{hash::Hash, sync::atomic::{AtomicUsize, Ordering}};

use macroquad::{math::{Vec2, vec2}, texture::Texture2D};

static SPRITE_COUNTER: AtomicUsize = AtomicUsize::new(0);


pub struct Sprite<'a> {
    id: usize,
    pub position: Vec2,
    pub texture: &'a Texture2D
}

impl <'a> Sprite<'a> {
    pub fn new(x: f32, y: f32, texture: &'a Texture2D) -> Sprite<'a> {
        return Sprite {id: SPRITE_COUNTER.fetch_add(1, Ordering::SeqCst),
            position: vec2(x, y),
            texture: texture
        };
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