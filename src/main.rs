use bevy::{
    DefaultPlugins,
    app::{App, Startup},
    asset::AssetServer,
    camera::Camera2d,
    ecs::{
        children,
        system::{Commands, ResMut},
    },
    math::Vec2,
    ui::{Node, percent},
    utils::default,
};

use crate::components::ui::button::UiButtonBuilder;

mod components;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);
    app.add_systems(Startup, setup);

    app.run();
}

fn setup(mut commands: Commands, assets: ResMut<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn((
        Node {
            width: percent(100),
            height: percent(100),
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
    ));
}
