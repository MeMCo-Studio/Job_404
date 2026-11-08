use egui_macroquad::egui::{Ui, Vec2, vec2};

use crate::{
    desktop::apps::{browser::Browser, job_finder::JobFinder, mail::Mail},
    game_state::GameState,
};

mod browser;
mod job_finder;
mod mail;

pub trait App {
    fn kind(&self) -> AppKind;
    fn title(&self) -> String {
        self.kind().name().to_string()
    }
    fn default_size(&self) -> Vec2 {
        vec2(500.0, 600.0)
    }
    fn ui(&mut self, ui: &mut Ui, game: &mut GameState);
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppKind {
    Jobs,
    Mail,
    Browser,
}

impl AppKind {
    pub const ALL: [AppKind; 3] = [Self::Jobs, Self::Mail, Self::Browser];

    pub fn name(&self) -> &'static str {
        match self {
            Self::Jobs => "Job Finder",
            Self::Mail => "Mail",
            Self::Browser => "Browser",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Self::Jobs => "J",
            Self::Mail => "M",
            Self::Browser => "B",
        }
    }

    pub fn create(&self) -> Option<Box<dyn App>> {
        match self {
            Self::Jobs => Some(Box::new(JobFinder::new())),
            Self::Mail => Some(Box::new(Mail::new())),
            Self::Browser => Some(Box::new(Browser::new())),
        }
    }
}
