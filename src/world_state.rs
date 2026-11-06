use macroquad::{math::{Vec2, vec2}};

use crate::{assets::Assets, inputs::PlayerController, render::Renderer, sprite::Sprite};
#[derive(PartialEq, Eq)]
pub enum Orientation {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

pub struct PlayerState<'a> {
    pub orientation: Orientation,
    pub position: Vec2,
    pub sprite: Sprite<'a>
}

pub struct WorldState<'a> {
    player: PlayerState<'a>,
    player_controller: PlayerController<'a>
}

impl <'a> WorldState<'a> {
    pub fn new(assets: &'a Assets) -> Self {
        return Self {
            player: PlayerState {
                orientation: Orientation::DOWN,
                position: vec2(0.0, 0.0),
                sprite: Sprite::new(0.0, 0.0, &assets.player_front)
            },
            player_controller: PlayerController::new(assets)
        }
    }

    pub fn update(&mut self) {
        self.player_controller.process_inputs(&mut self.player);
    }

    pub fn render(&self, renderer: &Renderer) {
        renderer.render(&self.player.sprite);
    }
}