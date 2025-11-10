//! The main entry point of the application.
use bevy::{prelude::*, render::texture::ImageSampler};
use bevy::render::texture::ImageSamplerDescriptor;

mod states;
mod menu;
mod asset_loading;
mod level_editor;

use states::GameState;
use menu::MenuPlugin;
use asset_loading::AssetLoadingPlugin;
use level_editor::LevelEditorPlugin;

fn main() {
    App::new()
        // Add Bevy's default plugins
        .add_plugins(DefaultPlugins.set(ImagePlugin {
            default_sampler: ImageSamplerDescriptor::nearest(),
        }))
        // Initialize the global state machine
        .init_state::<GameState>()
        // Add our custom plugins
        .add_plugins((MenuPlugin, AssetLoadingPlugin, LevelEditorPlugin))
        // Add temporary systems for prototyping
        .add_systems(OnEnter(GameState::InGame), || info!("Entered InGame state"))
        .run();
}
