//! This module contains all logic for the level editor.
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

use crate::states::GameState;

const GRID_SIZE: f32 = 32.0;

#[derive(Component)]
pub struct Tile;

#[derive(Component)]
pub struct LevelEditorCamera;

#[derive(Resource, Default, Debug)]
pub struct SnappedCoordinates(pub Vec2);

pub struct LevelEditorPlugin;

impl Plugin for LevelEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .init_resource::<SnappedCoordinates>()
            .add_systems(OnEnter(GameState::LevelEditor), setup_level_editor)
            .add_systems(
                Update,
                (editor_ui_system, cursor_system, tile_placement_system)
                    .run_if(in_state(GameState::LevelEditor)),
            )
            .add_systems(OnExit(GameState::LevelEditor), cleanup_level_editor);
    }
}

fn setup_level_editor(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), LevelEditorCamera));
}

fn cleanup_level_editor(
    mut commands: Commands,
    q_camera: Query<Entity, With<LevelEditorCamera>>,
    q_tiles: Query<Entity, With<Tile>>,
) {
    for entity in q_camera.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in q_tiles.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn editor_ui_system(
    mut contexts: EguiContexts,
    mut next_state: ResMut<NextState<GameState>>,
) {
    egui::Window::new("Level Editor").show(contexts.ctx_mut(), |ui| {
        ui.label("Welcome to the Level Editor!");
        ui.separator();

        // Action buttons
        if ui.button("Return to Main Menu").clicked() {
            next_state.set(GameState::MainMenu);
        }

        ui.separator();

        // Tile selection
        ui.label("Select a tile to place:");
        // TODO: Add tile selection buttons here

        ui.separator();

        // Tool selection
        ui.label("Tools:");
        // TODO: Add tool selection (e.g., place, erase)
    });
}

fn cursor_system(
    mut snapped_coords: ResMut<SnappedCoordinates>,
    q_window: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform), With<LevelEditorCamera>>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();

    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        let snapped_x = (world_position.x / GRID_SIZE).round() * GRID_SIZE;
        let snapped_y = (world_position.y / GRID_SIZE).round() * GRID_SIZE;

        snapped_coords.0 = Vec2::new(snapped_x, snapped_y);
    }
}

fn tile_placement_system(
    mut commands: Commands,
    snapped_coords: Res<SnappedCoordinates>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    q_tiles: Query<(Entity, &Transform), With<Tile>>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.5, 0.5, 0.5),
                    custom_size: Some(Vec2::new(GRID_SIZE, GRID_SIZE)),
                    ..default()
                },
                transform: Transform::from_translation(snapped_coords.0.extend(0.0)),
                ..default()
            },
            Tile,
        ));
    }

    if mouse_button_input.just_pressed(MouseButton::Right) {
        for (entity, transform) in q_tiles.iter() {
            if transform.translation.truncate() == snapped_coords.0 {
                commands.entity(entity).despawn();
            }
        }
    }
}
