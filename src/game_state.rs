#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum GameScene {
    #[default]
    World,
    Desktop,
}

#[derive(Debug, Default)]
pub struct GameState {
    pub scene: GameScene,
}
