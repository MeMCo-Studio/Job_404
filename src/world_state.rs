use macroquad::{
    color::WHITE,
    math::{Vec2, vec2},
    texture::draw_texture,
};

use crate::{assets::Assets, inputs::InputController, sprite::Sprite};

pub const WORLD_SIZE: f32 = 256.0;

#[derive(PartialEq, Eq, Debug, Default)]
pub enum Orientation {
    #[default]
    UP,
    DOWN,
    LEFT,
    RIGHT,
}
#[derive(Debug)]
pub struct PlayerState<'a> {
    pub orientation: Orientation,
    pub position: Vec2,
    pub sprite: Sprite<'a>,
    assets: &'a Assets,
}

impl<'a> PlayerState<'a> {
    pub fn render(&self) {
        self.sprite.render();
    }

    pub fn update(&mut self) {
        match self.orientation {
            Orientation::UP => self.sprite.texture = &self.assets.player_back,
            Orientation::DOWN => self.sprite.texture = &self.assets.player_front,
            Orientation::LEFT => {}
            Orientation::RIGHT => {}
        };
    }
}

#[derive(Debug)]
pub struct WorldState<'a> {
    player: PlayerState<'a>,
    assets: &'a Assets,
}

impl<'a> WorldState<'a> {
    pub fn new(assets: &'a Assets) -> Self {
        return Self {
            player: PlayerState {
                orientation: Orientation::DOWN,
                position: vec2(0.0, 0.0),
                sprite: Sprite::new(0.0, 0.0, &assets.player_front),
                assets: assets,
            },
            assets: assets,
        };
    }

    pub fn update(&mut self) {
        InputController::process_player_inputs(&mut self.player);
        self.player.update();
    }

    pub fn render(&self) {
        draw_texture(&self.assets.world, 0.0, 0.0, WHITE);
        self.player.render();
    }
}
