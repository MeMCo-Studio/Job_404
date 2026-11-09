use macroquad::time::get_frame_time;

use crate::game_state::time::GameClock;

mod time;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum GameScene {
    #[default]
    World,
    Desktop,
}

#[derive(Debug, Default)]
pub struct GameState {
    pub money: f32,
    pub time: GameClock,
}

impl GameState {
    pub fn update(&mut self) {
        self.time.tick(get_frame_time());
    }
}
