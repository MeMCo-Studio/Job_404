use crate::desktop::apps::App;

pub struct Browser {}

impl Browser {
    pub fn new() -> Self {
        Self {}
    }
}

impl App for Browser {
    fn kind(&self) -> super::AppKind {
        super::AppKind::Browser
    }

    fn ui(&mut self, ui: &mut egui_macroquad::egui::Ui, _game: &mut crate::game_state::GameState) {
        ui.label("Faster than Internet Explorer");
    }
}
