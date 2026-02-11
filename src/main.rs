#![allow(unused)]
use bevy::{
    DefaultPlugins,
    app::{App, Startup, Update},
    asset::AssetServer,
    camera::Camera2d,
    color::Color,
    ecs::{
        children,
        system::{Commands, ResMut},
    },
    math::Vec2,
    ui::{Node, percent, px},
    utils::default,
};

use crate::{
    ui::components::button::UiButtonBuilder,
    window_manager::{components::window::WindowBuilder, systems::window_resize::on_resize_system},
};

mod ui;
mod window_manager;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);
    app.add_systems(Startup, setup);
    app.add_systems(Update, on_resize_system);
    app.run();
}

fn setup(mut commands: Commands, assets: ResMut<AssetServer>) {
    let foo = commands
        .spawn((
            Node {
                width: percent(100),
                height: percent(100),
                left: px(30.0),
                display: bevy::ui::Display::Grid,
                ..default()
            },
            children![
                UiButtonBuilder::default()
                    .text("Start job search".into())
                    .position(Vec2::new(10., 42.))
                    .build(),
                UiButtonBuilder::default().text("Quit".into()).build()
            ],
        ))
        .id();

    commands.spawn(Camera2d);
    WindowBuilder::default()
        .background_color(Color::srgb(0.9, 0.9, 0.9))
        .build(&mut commands, &[foo]);
}
