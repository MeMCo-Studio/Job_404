use egui_macroquad::egui::{
    Align, Button, Context, Frame, Layout, Margin, RichText, Stroke, TopBottomPanel,
};

use crate::{
    desktop::{AppWindow, palette},
    game_state::GameState,
};

pub fn paint_taskbar(ctx: &Context, windows: &mut Vec<AppWindow>, game_state: &mut GameState) {
    TopBottomPanel::bottom("taskbar")
        .exact_height(42.0)
        .frame(
            Frame::NONE
                .fill(palette::SURFACE_INVERSE)
                .stroke(Stroke::new(2.0_f32, palette::PRIMARY))
                .inner_margin(Margin::symmetric(12, 4)),
        )
        .show(ctx, |ui| {
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                ui.label(RichText::new("Start").strong().color(palette::HIGHLIGHT));
                ui.separator();
                for window in windows {
                    if !window.state.is_opened() {
                        continue;
                    }
                    let (fill, text) = if window.state.is_minimized() {
                        (
                            palette::SURFACE_INVERSE_VARIANT,
                            palette::TEXT_INVERSE_MUTED,
                        )
                    } else {
                        (palette::PRIMARY, palette::TEXT_INVERSE)
                    };
                    if ui
                        .add(
                            Button::new(RichText::new(window.app.kind().name()).color(text))
                                .fill(fill),
                        )
                        .clicked()
                    {
                        window.state.toggle_minimize();
                    };
                }
                let now = game_state.time.now();
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label(
                        RichText::new(now.format("%A %d %B %H:%M").to_string())
                            .color(palette::TEXT_INVERSE),
                    );
                });
            });
        });
}
