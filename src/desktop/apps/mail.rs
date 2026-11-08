use crate::desktop::{apps::App, palette::TEXT};
use egui_macroquad::egui::*;

pub struct Mail {
    not_read: usize,
}

impl Mail {
    pub fn new() -> Self {
        Self { not_read: 1 }
    }
}

impl App for Mail {
    fn kind(&self) -> super::AppKind {
        super::AppKind::Mail
    }
    fn title(&self) -> String {
        format!("{} ({})", self.kind().name(), self.not_read)
    }
    fn ui(&mut self, ui: &mut egui_macroquad::egui::Ui, _game: &mut crate::game_state::GameState) {
        ui.heading("Inbox");
        ui.group(|ui| {
            ui.set_width(ui.available_width());
            ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.label("Company <important@company.com>");
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.label(RichText::new("Today 14:32").color(TEXT));
                    })
                });
                ui.label(RichText::new("Very Important!").strong().color(TEXT));
            });
        });
    }
}
