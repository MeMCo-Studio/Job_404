use bevy::ecs::{entity::Entity, hierarchy::ChildOf};
use std::default::Default;

use bevy::{
    color::Color,
    ecs::{
        bundle::Bundle,
        children,
        component::Component,
        event::EntityEvent,
        observer::On,
        system::{Commands, Query},
    },
    math::{Vec2, Vec3},
    picking::events::{Drag, Pointer},
    text::TextColor,
    transform::components::Transform,
    ui::{
        AlignItems, BackgroundColor, BorderColor, FlexDirection, JustifyContent, Node, UiRect,
        UiTransform, Val, Val2, percent, px, widget::Text,
    },
    utils::default,
};

#[derive(Component)]
pub struct Window;
pub struct WindowBuilder {
    title: String,
    pos: Vec2,
    background_color: Color,
    maximized: bool,
}

impl Default for WindowBuilder {
    fn default() -> Self {
        WindowBuilder {
            title: String::from("Window"),
            pos: Vec2 { x: 0.0, y: 0.0 },
            background_color: Color::WHITE,
            maximized: false,
        }
    }
}

impl WindowBuilder {
    pub fn title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    pub fn position(mut self, pos: Vec2) -> Self {
        self.pos = pos;
        self
    }

    pub fn background_color(mut self, background_color: Color) -> Self {
        self.background_color = background_color;
        self
    }

    pub fn build(&self, commands: &mut Commands<'_, '_>, children: &[Entity]) {
        let titlebar = self.build_titlebar();

        let parent = (
            Window,
            Node {
                width: px(300),
                height: px(250),
                border: UiRect::all(px(5)),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Start,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            Transform::from_translation(Vec3 {
                x: self.pos.x,
                y: self.pos.y,
                z: 0.0,
            }),
            BorderColor::all(Color::NONE),
            BackgroundColor(Color::NONE),
            children![],
        );

        let parent = commands.spawn(parent).id();
        let titlebar = commands.spawn(titlebar).observe(drag_handler).id();
        let content = commands.spawn(self.build_content()).id();
        commands.entity(content).add_children(children);
        commands.entity(parent).add_children(&[titlebar, content]);
    }

    fn build_titlebar(&self) -> impl Bundle {
        (
            Node {
                width: percent(100),
                height: px(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.23, 0.21, 0.97)),
            children![(Text::new(self.title.to_string()), TextColor(Color::WHITE),)],
        )
    }

    fn build_content(&self) -> impl Bundle {
        (
            Node {
                width: percent(100),
                height: percent(100),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(self.background_color),
        )
    }
}

fn drag_handler(
    on_drag: On<Pointer<Drag>>,
    children: Query<&ChildOf>,
    mut transforms: Query<&mut UiTransform>,
) {
    if let Ok(titlebar) = children.get(on_drag.event_target()) {
        let mut window_transform = transforms.get_mut(titlebar.parent()).unwrap();
        let (Val::Px(x), Val::Px(y)) = (
            window_transform.translation.x,
            window_transform.translation.y,
        ) else {
            return;
        };

        window_transform.translation = Val2::px(on_drag.delta.x + x, on_drag.delta.y + y);
    }
}
