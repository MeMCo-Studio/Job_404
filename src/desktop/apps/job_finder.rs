use egui::RichText;
use egui_macroquad::egui;

use crate::desktop::{apps::App, palette::TEXT};

pub struct JobFinder {
    search: String,
}

impl JobFinder {
    pub fn new() -> Self {
        Self {
            search: String::new(),
        }
    }
}

impl App for JobFinder {
    fn kind(&self) -> super::AppKind {
        super::AppKind::Jobs
    }
    fn ui(&mut self, ui: &mut egui::Ui, _game: &mut crate::game_state::GameState) {
        ui.heading("Find a job");
        ui.separator();

        ui.label("Search");

        ui.text_edit_singleline(&mut self.search);

        ui.add_space(12.0);

        ui.group(|ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                ui.label(RichText::new("Junior Vibe Coder").strong().color(TEXT));
                ui.label("Remote · Full time");
                ui.label("Salary: 35k-42k");
                ui.label("Requirement: 8 year of experience in Claude");
                if ui.button("Apply").clicked() {
                    println!("Applied")
                };
            });
        });

        ui.add_space(12.0);
    }
}
