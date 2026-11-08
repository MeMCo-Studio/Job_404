pub mod apps;
pub mod palette;
pub mod style;
pub mod taskbar;
pub mod widgets;

use crate::{
    desktop::{
        apps::{App, AppKind},
        taskbar::paint_taskbar,
        widgets::window::{WindowState, game_window},
    },
    game_state::{GameScene, GameState},
};

use egui_macroquad::egui::{self, Id};

use egui::{Context, RichText, Vec2};
use macroquad::input::{KeyCode, is_key_pressed};

pub struct Desktop {
    windows: Vec<AppWindow>,
}

pub struct AppWindow {
    pub id: egui::Id,
    pub state: WindowState,
    pub app: Box<dyn App>,
}

impl Desktop {
    pub fn new() -> Self {
        Self {
            windows: Vec::new(),
        }
    }

    pub fn update(&mut self, state: &mut GameState) {
        if is_key_pressed(KeyCode::Escape) {
            state.scene = GameScene::World
        }
    }

    pub fn show(&mut self, ctx: &Context, game: &mut GameState) {
        self.draw_background(ctx);
        if let Some(kind) = self.draw_icons(ctx) {
            self.open_app(ctx, kind);
        }
        self.draw_taskbar(ctx);
        for window in self.windows.iter_mut() {
            if !window.state.is_opened() || window.state.is_minimized() {
                continue;
            }
            game_window(
                &ctx,
                window.id,
                &mut window.state,
                &window.app.title(),
                |ui| window.app.ui(ui, game),
            );
        }
    }

    fn draw_background(&self, ctx: &Context) {
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(palette::BACKGROUND))
            .show(ctx, |_ui| {});
    }

    fn draw_icons(&mut self, ctx: &Context) -> Option<AppKind> {
        let mut clicked = None;
        egui::Area::new("desktop_icons".into())
            .fixed_pos(egui::pos2(20.0, 30.0))
            .show(ctx, |ui| {
                ui.set_width(80.0);

                for kind in AppKind::ALL {
                    let response = ui
                        .vertical_centered(|ui| {
                            let button = egui::Button::new(
                                RichText::new(kind.icon()).size(24.0).color(palette::TEXT),
                            )
                            .fill(palette::SURFACE)
                            .stroke(egui::Stroke::new(2.0_f32, palette::BORDER))
                            .min_size(Vec2::splat(48.0));

                            ui.add(button)
                        })
                        .inner;

                    ui.vertical_centered(|ui| {
                        ui.label(RichText::new(kind.name()).color(palette::TEXT_INVERSE));
                    });
                    if response.clicked() {
                        clicked = Some(kind)
                    }

                    ui.add_space(12.0);
                }
            });

        clicked
    }

    fn open_app(&mut self, ctx: &Context, kind: AppKind) {
        // Already exists
        if let Some(window) = self.windows.iter_mut().find(|w| w.app.kind() == kind) {
            window.state.open();
            ctx.move_to_top(egui::LayerId::new(egui::Order::Middle, window.id));
            return;
        }

        let Some(app) = kind.create() else { return };

        let n = self.windows.len() as f32;
        let pos = egui::pos2(140.0 + n * 24.0, 40.0 + n * 24.0);

        let state = WindowState::new(pos, app.default_size());

        self.windows.push(AppWindow {
            id: Id::new(kind),
            state,
            app,
        });
    }

    fn draw_taskbar(&mut self, ctx: &Context) {
        paint_taskbar(ctx, &mut self.windows);
    }
}
