use macroquad::{input::{KeyCode, is_key_down}, math::vec2};

use crate::{assets::Assets, world_state::{Orientation, PlayerState}};

pub struct PlayerController<'a> {
    speed: f32,
    assets: &'a Assets
}

impl <'a> PlayerController <'a> {
    pub fn new(assets: &'a Assets) -> Self {
        return Self { 
            speed: 1.0,
            assets: assets
        }
    }

    pub fn process_inputs(&mut self, player_state: &mut PlayerState<'a>) {
        let is_up_pressed = is_key_down(KeyCode::Z);
        let is_down_pressed = is_key_down(KeyCode::S);
        let is_left_pressed = is_key_down(KeyCode::Q);
        let is_right_pressed = is_key_down(KeyCode::D);

        let mut direction = vec2(0.0, 0.0);
        if is_up_pressed && !is_down_pressed {
            direction.y = -1.0;
            player_state.sprite.texture = &self.assets.player_back;
            player_state.orientation = Orientation::UP;
        } else if is_down_pressed && !is_up_pressed {
            direction.y = 1.0;
            player_state.sprite.texture = &self.assets.player_front;
            player_state.orientation = Orientation::DOWN;
        }
        if is_right_pressed && !is_left_pressed {
            direction.x = 1.0;
            player_state.orientation = Orientation::LEFT;
        } else if is_left_pressed && !is_right_pressed {
            direction.x = -1.0;
            player_state.orientation = Orientation::RIGHT;
        }
        let direction = direction.try_normalize();
        if direction.is_some() {
            player_state.position += direction.unwrap() * self.speed;
            player_state.sprite.position.x = player_state.position.x;
            player_state.sprite.position.y = player_state.position.y;
        }
        
    }
}