use macroquad::texture::{FilterMode, Texture2D, load_texture};

#[derive(Debug)]
pub struct Assets {
    pub player_front: Texture2D,
    pub player_back: Texture2D,
    pub world: Texture2D,
}

impl Assets {
    pub async fn new() -> Self {
        return Self {
            player_front: load("sprites/player/placeholder1.png").await,
            player_back: load("sprites/player/placeholder2.png").await,
            world: load("world/world-placeholder.png").await,
        };
    }
}

async fn load(path: &str) -> Texture2D {
    let texture = load_texture(format!("assets/textures/{}", path).as_str())
        .await
        .unwrap();
    texture.set_filter(FilterMode::Nearest);
    return texture;
}
