use egui_macroquad::egui::*;

use crate::desktop::palette;

const TITLE_H: f32 = 40.0;

#[derive(Debug)]
pub struct WindowState {
    pub pos: Pos2,
    pub size: Vec2,
    open: bool,
    minimized: bool,
}

impl WindowState {
    pub fn new(pos: Pos2, size: Vec2) -> Self {
        Self {
            minimized: false,
            open: true,
            pos,
            size,
        }
    }

    pub fn is_opened(&self) -> bool {
        self.open
    }
    pub fn is_minimized(&self) -> bool {
        self.minimized
    }

    pub fn open(&mut self) {
        self.open = true;
        self.minimized = false;
    }
    pub fn close(&mut self) {
        self.open = false
    }
    pub fn toggle_minimize(&mut self) {
        self.minimized = !self.minimized;
    }
    pub fn minimize(&mut self) {
        self.minimized = true
    }
}

pub fn game_window(
    ctx: &Context,
    id: Id,
    state: &mut WindowState,
    title: &str,
    add_contents: impl FnOnce(&mut Ui),
) {
    Window::new(title)
        .id(id)
        .title_bar(false)
        .movable(false)
        .current_pos(state.pos)
        .fixed_size(state.size)
        .resizable(false)
        .frame(
            Frame::new()
                .shadow(Shadow::NONE)
                .fill(palette::SURFACE)
                .inner_margin(Margin::same(2))
                .stroke(Stroke::new(6_f32, palette::BORDER)),
        )
        .show(ctx, |ui| {
            let title_rect = ui.allocate_space(vec2(ui.available_width(), TITLE_H)).1;

            ui.painter().rect_filled(title_rect, 0.0, palette::PRIMARY);

            ui.painter().hline(
                title_rect.x_range(),
                title_rect.bottom(),
                Stroke::new(2.0_f32, palette::PRIMARY_DARK),
            );
            ui.put(
                title_rect,
                Label::new(RichText::new(title).color(palette::TEXT_INVERSE).strong()),
            );
            let drag = ui.interact(title_rect, id.with("title"), Sense::all());
            if drag.dragged() {
                state.pos += drag.drag_delta();
            }
            let close_rect = Rect::from_min_size(
                title_rect.right_top() - vec2(TITLE_H, 0.0),
                Vec2::splat(TITLE_H),
            );
            if ui.put(close_rect, Button::new("X")).clicked() {
                state.close();
            }

            let minimize_rect = Rect::from_min_size(
                title_rect.right_top() - vec2(2_f32 * TITLE_H, 0.0),
                Vec2::splat(TITLE_H),
            );
            if ui.put(minimize_rect, Button::new("-")).clicked() {
                state.minimize();
            }

            ScrollArea::vertical()
                .auto_shrink(false)
                .show(ui, add_contents);
        });
}
