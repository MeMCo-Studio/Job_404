use bevy::prelude::children;
use bevy::transform::components::Transform;
use bevy::{
    ecs::bundle::Bundle,
    prelude::{Color, Component, Vec2, Vec3},
    text::TextColor,
    ui::{
        AlignItems, BackgroundColor, BorderColor, BorderRadius, JustifyContent, Node, UiRect, px,
        widget::{Button, Text, TextShadow},
    },
    utils::default,
};
use std::default::Default;

#[derive(Component)]
pub struct UiButton {}

pub struct UiButtonBuilder {
    text: String,
    pos: Vec2,
    text_color: Color,
    background_color: Color,
}

impl Default for UiButtonBuilder {
    fn default() -> Self {
        UiButtonBuilder {
            text: String::from("Button"),
            pos: Vec2 { x: 0.0, y: 0.0 },
            text_color: Color::BLACK,
            background_color: Color::WHITE,
        }
    }
}

impl UiButtonBuilder {
    pub fn text(mut self, text: String) -> Self {
        self.text = text;
        self
    }

    pub fn position(mut self, pos: Vec2) -> Self {
        self.pos = pos;
        self
    }

    pub fn text_color(mut self, text_color: Color) -> Self {
        self.text_color = text_color;
        self
    }

    pub fn background_color(mut self, background_color: Color) -> Self {
        self.background_color = background_color;
        self
    }

    pub fn build(&self) -> impl Bundle {
        (
            Button,
            Node {
                width: px(150),
                height: px(65),
                border: UiRect::all(px(5)),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                border_radius: BorderRadius::MAX,
                ..default()
            },
            Transform::from_translation(Vec3 {
                x: self.pos.x,
                y: self.pos.y,
                z: 0.0,
            }),
            BorderColor::all(Color::WHITE),
            BackgroundColor(self.background_color),
            children![(
                Text::new(self.text.to_string()),
                TextColor(self.text_color),
                TextShadow::default(),
            )],
        )
    }
}
