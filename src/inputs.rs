use macroquad::{
    input::{KeyCode, is_key_down, is_key_pressed},
    math::vec2,
};

use crate::{
    game::Game,
    game_state::GameScene,
    world_state::{Orientation, PlayerState},
};

pub struct InputController {}

impl InputController {
    pub fn process_player_inputs(player_state: &mut PlayerState) {
        let is_up_pressed = is_key_down(KeyCode::Z);
        let is_down_pressed = is_key_down(KeyCode::S);
        let is_left_pressed = is_key_down(KeyCode::Q);
        let is_right_pressed = is_key_down(KeyCode::D);

        let mut direction = vec2(0.0, 0.0);
        if is_up_pressed && !is_down_pressed {
            direction.y = -1.0;
            player_state.orientation = Orientation::UP;
        } else if is_down_pressed && !is_up_pressed {
            direction.y = 1.0;
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
            player_state.position += direction.unwrap();
            player_state.sprite.position.x = player_state.position.x;
            player_state.sprite.position.y = player_state.position.y;
        }
    }

    pub fn process_ui_inputs(state: &mut Game) {
        if is_key_pressed(KeyCode::Enter) {
            state.scene = GameScene::Desktop;
        } else if is_key_pressed(KeyCode::Escape) {
            state.scene = GameScene::World
        }
    }
}
