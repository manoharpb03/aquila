use bevy::prelude::*;
use bevy_aquila::{AquilaConfig, AquilaPlugin};

fn main() {
    let token = std::env::var("AQUILA_TOKEN").ok();
    let url = std::env::var("AQUILA_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());

    App::new()
        .add_plugins(AquilaPlugin::new(AquilaConfig {
            url,
            token,
            version: "v1.0".to_string(),
        }))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d::default());

    let image = asset_server.load("aquila://test.png");

    commands.spawn(Sprite { image, ..default() });
}
