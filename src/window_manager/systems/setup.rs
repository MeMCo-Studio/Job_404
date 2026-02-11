use bevy::prelude::*;

pub fn setup(asset_server: Res<AssetServer>) {
    let texture_handle: Handle<Image> = asset_server.load("assets/textures/iu/screen.png");
}
