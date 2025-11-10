use bevy::prelude::*;

mod states;
mod menu;

use states::GameState;
use menu::MenuPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_plugins(MenuPlugin)
        .add_systems(OnEnter(GameState::InGame), || info!("Entered InGame state"))
        .add_systems(OnEnter(GameState::LevelEditor), || {
            info!("Entered LevelEditor state")
        })
        .run();
}