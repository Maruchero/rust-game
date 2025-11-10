//! The main entry point of the application.
use bevy::prelude::*;

mod states;
mod menu;
mod asset_loading;

use states::GameState;
use menu::MenuPlugin;
use asset_loading::AssetLoadingPlugin;

fn main() {
    App::new()
        // Add Bevy's default plugins
        .add_plugins(DefaultPlugins)
        // Initialize the global state machine
        .init_state::<GameState>()
        // Add our custom plugins
        .add_plugins((MenuPlugin, AssetLoadingPlugin))
        // Add temporary systems for prototyping
        .add_systems(OnEnter(GameState::InGame), || info!("Entered InGame state"))
        .add_systems(OnEnter(GameState::LevelEditor), || {
            info!("Entered LevelEditor state")
        })
        .run();
}
