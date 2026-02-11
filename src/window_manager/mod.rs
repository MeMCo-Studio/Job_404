use bevy::app::{App, Plugin, Startup};
use components::*;
use systems::*;

pub mod components;
pub mod systems;

pub struct WindowManagerPlugin;

impl Plugin for WindowManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup::setup);
    }
}
