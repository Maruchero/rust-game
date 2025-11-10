//! This module contains all logic for the level editor.
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy::input::mouse::MouseWheel;

use crate::asset_loading::CaveAtlases;
use crate::states::GameState;

#[derive(Component)]
pub struct Tile;

#[derive(Component)]
pub struct LevelEditorCamera;

#[derive(Resource, Default, Debug)]
pub struct WorldCoordinates(pub Vec2);

#[derive(Resource, Default, Debug)]
pub struct SelectedTile {
    pub index: usize,
}

#[derive(Resource, Default)]
pub struct CameraDragState {
    pub is_dragging: bool,
    pub last_mouse_position: Option<Vec2>,
}

pub struct LevelEditorPlugin;

impl Plugin for LevelEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .init_resource::<WorldCoordinates>()
            .init_resource::<SelectedTile>()
            .init_resource::<CameraDragState>() // Initialize the new resource
            .add_systems(OnEnter(GameState::LevelEditor), setup_level_editor)
            .add_systems(
                Update,
                (
                    editor_ui_system,
                    cursor_system,
                    tile_placement_system,
                    camera_drag_start_system,
                    camera_drag_system,
                    camera_drag_end_system,
                    camera_zoom_system, // New system for zooming
                )
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
    mut selected_tile: ResMut<SelectedTile>,
    cave_atlases: Res<CaveAtlases>,
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
        
        ui.horizontal_wrapped(|ui| {
            for (i, name) in cave_atlases.tile_names.iter().enumerate() {
                if ui.button(name).clicked() {
                    selected_tile.index = i;
                }
            }
        });

        ui.separator();

        // Tool selection
        ui.label("Tools:");
        // TODO: Add tool selection (e.g., place, erase)
    });
}

fn cursor_system(
    mut world_coords: ResMut<WorldCoordinates>,
    q_window: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform), With<LevelEditorCamera>>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();

    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        world_coords.0 = world_position;
    }
}

fn tile_placement_system(
    mut commands: Commands,
    mut contexts: EguiContexts,
    world_coords: Res<WorldCoordinates>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    q_tiles: Query<(Entity, &Transform, &Sprite), With<Tile>>,
    cave_atlases: Res<CaveAtlases>,
    selected_tile: Res<SelectedTile>,
    texture_atlas_layouts: Res<Assets<TextureAtlasLayout>>,
) {
    // If egui is interacting with the pointer, don't place tiles
    if contexts.ctx_mut().wants_pointer_input() {
        return;
    }
    if mouse_button_input.just_pressed(MouseButton::Left) {
        // Check if a tile already exists at the clicked location
        for (_entity, transform, sprite) in q_tiles.iter() {
            let size = sprite.custom_size.unwrap_or(Vec2::new(1.0, 1.0));
            let tile_rect = Rect::from_center_size(transform.translation.truncate(), size);

            if tile_rect.contains(world_coords.0) {
                // A tile already exists here, so don't place a new one
                return;
            }
        }

        let layout = texture_atlas_layouts.get(&cave_atlases.platform_atlas).unwrap();
        let tile_rect = layout.textures[selected_tile.index];
        let tile_size = Vec2::new(tile_rect.width(), tile_rect.height());

        let scale_factor = 0.2;
        let scaled_size = tile_size * scale_factor;

        commands.spawn((
            SpriteSheetBundle {
                texture: cave_atlases.platform_image.clone(),
                atlas: TextureAtlas {
                    layout: cave_atlases.platform_atlas.clone(),
                    index: selected_tile.index,
                },
                sprite: Sprite {
                    custom_size: Some(scaled_size),
                    ..default()
                },
                transform: Transform::from_translation(world_coords.0.extend(0.0)),
                ..default()
            },
            Tile,
        ));
    }

    if mouse_button_input.just_pressed(MouseButton::Right) {
        for (entity, transform, sprite) in q_tiles.iter() {
            let size = sprite.custom_size.unwrap_or(Vec2::new(1.0, 1.0));
            let tile_rect = Rect::from_center_size(transform.translation.truncate(), size);

            if tile_rect.contains(world_coords.0) {
                commands.entity(entity).despawn();
            }
        }
    }
}

fn camera_drag_start_system(
    mut camera_drag_state: ResMut<CameraDragState>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    q_window: Query<&Window>,
    mut contexts: EguiContexts,
) {
    if contexts.ctx_mut().wants_pointer_input() {
        return;
    }

    if mouse_button_input.just_pressed(MouseButton::Middle) {
        if let Some(position) = q_window.single().cursor_position() {
            camera_drag_state.is_dragging = true;
            camera_drag_state.last_mouse_position = Some(position);
        }
    }
}

fn camera_drag_system(
    mut camera_drag_state: ResMut<CameraDragState>,
    q_window: Query<&Window>,
    mut q_camera: Query<&mut Transform, With<LevelEditorCamera>>,
    mut contexts: EguiContexts,
) {
    if contexts.ctx_mut().wants_pointer_input() {
        return;
    }

    if camera_drag_state.is_dragging {
        if let Some(current_mouse_position) = q_window.single().cursor_position() {
            if let Some(last_mouse_position) = camera_drag_state.last_mouse_position {
                let delta = current_mouse_position - last_mouse_position;

                let mut camera_transform = q_camera.single_mut();
                // Scale the delta by the camera's scale to make movement consistent regardless of zoom
                let scaled_delta = delta * camera_transform.scale.x;

                // Invert the delta because dragging the mouse right should move the camera left
                camera_transform.translation.x -= scaled_delta.x;
                camera_transform.translation.y += scaled_delta.y; // Y-axis is usually inverted for screen coords vs world coords

                camera_drag_state.last_mouse_position = Some(current_mouse_position);
            }
        }
    }
}

fn camera_drag_end_system(
    mut camera_drag_state: ResMut<CameraDragState>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut contexts: EguiContexts,
) {
    if contexts.ctx_mut().wants_pointer_input() {
        return;
    }

    if mouse_button_input.just_released(MouseButton::Middle) {
        camera_drag_state.is_dragging = false;
        camera_drag_state.last_mouse_position = None;
    }
}

fn camera_zoom_system(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut q_camera: Query<&mut Transform, With<LevelEditorCamera>>,
    mut contexts: EguiContexts,
    world_coords: Res<WorldCoordinates>, // Add WorldCoordinates resource
) {
    if contexts.ctx_mut().wants_pointer_input() {
        return;
    }

    let mut camera_transform = q_camera.single_mut();
    let mut zoom_amount = 0.0;
    for event in mouse_wheel_events.read() {
        zoom_amount += event.y;
    }

    if zoom_amount != 0.0 {
        let zoom_speed = 0.1; // Adjust this value to change zoom speed
        let old_scale = camera_transform.scale.x;
        let new_scale_factor = 1.0 - (zoom_amount * zoom_speed);
        let new_scale = (old_scale * new_scale_factor).clamp(0.1, 2.0); // Clamp the new scale

        // Calculate the mouse's world position before zoom
        let mouse_world_pos_before_zoom = world_coords.0;

        // Calculate the camera's new position to zoom towards the mouse
        let camera_center_before_zoom = camera_transform.translation.truncate();
        let offset_from_center = mouse_world_pos_before_zoom - camera_center_before_zoom;
        let new_offset_from_center = offset_from_center * (new_scale / old_scale);
        let new_camera_center = mouse_world_pos_before_zoom - new_offset_from_center;

        camera_transform.translation = new_camera_center.extend(camera_transform.translation.z);
        camera_transform.scale = Vec3::splat(new_scale);
    }
}
