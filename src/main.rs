use bevy::prelude::*;
use bevy::window::PresentMode;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 1600.,
            height: 900.,
            title: "Hangman!".to_string(),
            present_mode: PresentMode::Fifo,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .run();
}
