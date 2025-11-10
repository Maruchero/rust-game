//! Defines the global `GameState` enum, which controls the application's flow.
use bevy::prelude::*;

#[derive(States, Default, Clone, Eq, PartialEq, Hash, Debug)]
pub enum GameState {
    #[default]
    MainMenu,
    InGame,
    LevelEditor,
}