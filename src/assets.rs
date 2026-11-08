use macroquad::texture::{FilterMode, Texture2D, load_texture};

pub struct Assets {
    pub player_front: Texture2D,
    pub player_back: Texture2D,
}

impl Assets {
    pub async fn new() -> Self {
        let player_front = load_texture("assets/textures/sprites/player/placeholder1.png")
            .await
            .unwrap();
        let player_back = load_texture("assets/textures/sprites/player/placeholder2.png")
            .await
            .unwrap();
        player_front.set_filter(FilterMode::Nearest);
        player_back.set_filter(FilterMode::Nearest);
        return Self {
            player_front,
            player_back,
        };
    }
}
