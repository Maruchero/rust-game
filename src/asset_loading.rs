//! This module is responsible for loading all the game's assets.
//! It runs during the `GameState::Loading` state.

use bevy::{asset::LoadState, prelude::*};

use crate::states::GameState;

pub struct AssetLoadingPlugin;

impl Plugin for AssetLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CaveAtlases>()
            .add_systems(OnEnter(GameState::Loading), load_assets)
            .add_systems(
                Update,
                check_assets_loaded.run_if(in_state(GameState::Loading)),
            );
    }
}

/// A temporary resource to hold the handle for the platform image while it's loading.
#[derive(Resource, Default)]
struct LoadingImageHandles {
    platform_image: Handle<Image>,
}

/// A resource to hold handles to the loaded texture atlases.
#[derive(Resource, Default)]
pub struct CaveAtlases {
    /// Handle for the platforms texture atlas layout.
    pub platform_atlas: Handle<TextureAtlasLayout>,
    // We also need to store the handle for the image itself
    pub platform_image: Handle<Image>,
}

/// Starts loading the platform image asset.
fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(LoadingImageHandles {
        platform_image: asset_server.load("textures/cave-structure/Cave - Platforms.png"),
    });
}

/// Checks if the platform image is loaded. If it is, it creates the texture atlas layout
/// and transitions to the MainMenu state.
fn check_assets_loaded(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    asset_server: Res<AssetServer>,
    loading_handles: Res<LoadingImageHandles>,
    images: Res<Assets<Image>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let load_state = asset_server.get_load_state(&loading_handles.platform_image);

    if load_state == Some(LoadState::Loaded) {
        // The image is loaded. Get its dimensions.
        let image = images.get(&loading_handles.platform_image).unwrap();
        let dimensions = image.size_f32();

        // Create a new texture atlas layout with the correct dimensions
        let mut layout = TextureAtlasLayout::new_empty(dimensions);
        // Add each platform sprite as a texture region
        // Rect::new(min_x, min_y, max_x, max_y)
        layout.add_texture(Rect::new(19.0, 24.0, 19.0 + 197.0, 24.0 + 191.0)); // small_square
        layout.add_texture(Rect::new(231.0, 20.0, 231.0 + 394.0, 20.0 + 194.0)); // small_rect_horizontal
        layout.add_texture(Rect::new(44.0, 641.0, 44.0 + 200.0, 641.0 + 396.0)); // small_rect_vertical
        layout.add_texture(Rect::new(666.0, 53.0, 666.0 + 316.0, 53.0 + 328.0)); // big_square
        layout.add_texture(Rect::new(25.0, 222.0, 25.0 + 567.0, 222.0 + 369.0)); // big_rect
        layout.add_texture(Rect::new(615.0, 397.0, 615.0 + 393.0, 397.0 + 368.0)); // big_big_square
        layout.add_texture(Rect::new(272.0, 644.0, 272.0 + 308.0, 644.0 + 64.0)); // ground_short
        layout.add_texture(Rect::new(627.0, 791.0, 627.0 + 358.0, 791.0 + 67.0)); // ground_long
        layout.add_texture(Rect::new(719.0, 887.0, 719.0 + 196.0, 887.0 + 107.0)); // rock_sm
        layout.add_texture(Rect::new(291.0, 720.0, 291.0 + 270.0, 720.0 + 161.0)); // rock_md
        layout.add_texture(Rect::new(265.0, 899.0, 265.0 + 395.0, 899.0 + 108.0)); // rock_lg

        // Add the layout to the asset collection
        let platform_atlas_layout_handle = texture_atlas_layouts.add(layout);

        // Insert the final CaveAtlases resource with the handles
        commands.insert_resource(CaveAtlases {
            platform_atlas: platform_atlas_layout_handle,
            platform_image: loading_handles.platform_image.clone(),
        });

        // Cleanup the temporary resource
        commands.remove_resource::<LoadingImageHandles>();

        // Transition to the main menu
        next_state.set(GameState::MainMenu);
    }
}
