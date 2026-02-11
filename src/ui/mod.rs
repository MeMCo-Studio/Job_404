use bevy::app::{App, Plugin};

pub mod components;
pub mod systems;

pub struct CustomUIPlugin;

impl Plugin for CustomUIPlugin {
    fn build(&self, app: &mut App) {
        // add things to your app here
    }
}
