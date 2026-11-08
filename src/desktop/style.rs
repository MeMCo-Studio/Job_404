use std::sync::Arc;

use egui::{Color32, Stroke, Visuals, style::WidgetVisuals};
use egui_macroquad::egui::{self, FontData};

use crate::desktop::palette::*;

pub fn configure_style(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();

    style.visuals = visuals();
    style.spacing.button_padding = egui::Vec2::new(12.0, 8.0);

    setup_custom_fonts(ctx);
    ctx.set_style(style);
}

fn visuals() -> Visuals {
    let mut visuals = Visuals::light();

    visuals.window_fill = SURFACE;
    visuals.window_stroke = Stroke::new(2.0_f32, BORDER);
    visuals.panel_fill = BACKGROUND;
    visuals.faint_bg_color = SURFACE_VARIANT; // groups, striped rows
    visuals.extreme_bg_color = INPUT; // text edit / scroll backgrounds
    visuals.code_bg_color = SURFACE_VARIANT;
    visuals.hyperlink_color = SECONDARY_DARK;
    visuals.warn_fg_color = WARNING;
    visuals.error_fg_color = ERROR;

    visuals.selection.bg_fill = SECONDARY;
    visuals.selection.stroke = Stroke::new(1.0_f32, BORDER);
    visuals.text_cursor.stroke = Stroke::new(2.0_f32, PRIMARY);

    let w = &mut visuals.widgets;

    w.noninteractive = widget(
        SURFACE,
        Stroke::new(1.0_f32, BORDER_MUTED),
        TEXT,
        w.noninteractive,
    );
    w.inactive = widget(BUTTON, Stroke::new(1.0_f32, BORDER), TEXT, w.inactive);
    w.hovered = widget(HIGHLIGHT, Stroke::new(1.0_f32, BORDER), TEXT, w.hovered);
    w.active = widget(
        PRIMARY,
        Stroke::new(1.0_f32, BORDER),
        TEXT_INVERSE,
        w.active,
    );
    w.open = widget(HIGHLIGHT, Stroke::new(1.0_f32, BORDER), TEXT, w.open);

    visuals
}

fn widget(fill: Color32, border: Stroke, text: Color32, base: WidgetVisuals) -> WidgetVisuals {
    WidgetVisuals {
        bg_fill: fill,
        weak_bg_fill: fill,
        bg_stroke: border,
        fg_stroke: Stroke::new(1.0_f32, text),
        ..base
    }
}

fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "Pixeloid".to_owned(),
        Arc::new(FontData::from_static(include_bytes!(
            "../../assets/fonts/PixeloidMono.otf"
        ))),
    );

    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "Pixeloid".to_owned());

    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("Pixeloid".to_owned());

    ctx.set_fonts(fonts);
}
