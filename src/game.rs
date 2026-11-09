use crate::{
    assets::Assets,
    desktop::Desktop,
    game_state::{GameScene, GameState},
    inputs::InputController,
    world_state::WorldState,
};

#[derive(Debug)]
pub struct Game<'a> {
    pub scene: GameScene,
    desktop: Desktop,
    world: WorldState<'a>,
    game_state: GameState,
}

impl<'a> Game<'a> {
    pub async fn new(assets: &'a Assets) -> Self {
        return Self {
            world: WorldState::new(assets),
            desktop: Desktop::new(),
            scene: GameScene::default(),
            game_state: GameState::default(),
        };
    }

    pub fn update(&mut self) {
        self.game_state.update();
        match self.scene {
            GameScene::World => self.world.update(),
            GameScene::Desktop => self.desktop.update(),
        }
        InputController::process_ui_inputs(self);
    }

    pub fn render(&mut self) {
        // Render ui + world
        egui_macroquad::ui(|ctx| match self.scene.clone() {
            GameScene::World => {} // HUD
            GameScene::Desktop => self.desktop.show(ctx, &mut self.game_state),
        });

        if self.scene == GameScene::World {
            self.world.render();
        }
    }
}
