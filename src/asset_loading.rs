//! This module is responsible for loading all the game's assets.
//! It runs during the `GameState::Loading` state.

use bevy::{asset::{LoadState, AsyncReadExt}, prelude::*};

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
struct LoadingHandles {
    platform_image: Handle<Image>,
    platform_data: Handle<TextAsset>,
}

/// A resource to hold handles to the loaded texture atlases.
#[derive(Resource, Default)]
pub struct CaveAtlases {
    /// Handle for the platforms texture atlas layout.
    pub platform_atlas: Handle<TextureAtlasLayout>,
    // We also need to store the handle for the image itself
    pub platform_image: Handle<Image>,
    /// The names of the tiles in the atlas.
    pub tile_names: Vec<String>,
}

/// A custom asset type for the platform data file.
#[derive(Asset, TypePath, Debug)]
pub struct TextAsset(pub String);

#[derive(Default)]
pub struct TextAssetLoader;

impl bevy::asset::AssetLoader for TextAssetLoader {
    type Asset = TextAsset;
    type Settings = ();
    type Error = std::io::Error;

    fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader,
        _settings: &'a Self::Settings,
        _load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let text = String::from_utf8(bytes).unwrap();
            Ok(TextAsset(text))
        })
    }

    fn extensions(&self) -> &[&str] {
        &["txt"]
    }
}


/// Starts loading the platform image asset.
fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(LoadingHandles {
        platform_image: asset_server.load("textures/cave-structure/Cave - Platforms.png"),
        platform_data: asset_server.load("textures/cave-structure/Cave - Platforms.txt"),
    });
}

/// Checks if the platform image is loaded. If it is, it creates the texture atlas layout
/// and transitions to the MainMenu state.
fn check_assets_loaded(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    asset_server: Res<AssetServer>,
    loading_handles: Res<LoadingHandles>,
    images: Res<Assets<Image>>,
    text_assets: Res<Assets<TextAsset>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let image_load_state = asset_server.get_load_state(&loading_handles.platform_image);
    let data_load_state = asset_server.get_load_state(&loading_handles.platform_data);

    if image_load_state == Some(LoadState::Loaded) && data_load_state == Some(LoadState::Loaded) {
        // The image is loaded. Get its dimensions.
        let image = images.get(&loading_handles.platform_image).unwrap();
        let dimensions = image.size_f32();

        // The data file is loaded. Parse it.
        let data = text_assets.get(&loading_handles.platform_data).unwrap();
        let mut tile_names = Vec::new();

        // Create a new texture atlas layout with the correct dimensions
        let mut layout = TextureAtlasLayout::new_empty(dimensions);
        
        // Parse the CSV data
        for line in data.0.lines().skip(1) { // Skip header
            let parts: Vec<&str> = line.split(',').collect();
            let name = parts[0].to_string();
            let x: f32 = parts[1].trim().parse().unwrap();
            let y: f32 = parts[2].trim().parse().unwrap();
            let width: f32 = parts[3].trim().parse().unwrap();
            let height: f32 = parts[4].trim().parse().unwrap();

            tile_names.push(name);
            layout.add_texture(Rect::new(x, y, x + width, y + height));
        }

        // Add the layout to the asset collection
        let platform_atlas_layout_handle = texture_atlas_layouts.add(layout);

        // Insert the final CaveAtlases resource with the handles
        commands.insert_resource(CaveAtlases {
            platform_atlas: platform_atlas_layout_handle,
            platform_image: loading_handles.platform_image.clone(),
            tile_names,
        });

        // Cleanup the temporary resource
        commands.remove_resource::<LoadingHandles>();

        // Transition to the main menu
        next_state.set(GameState::MainMenu);
    }
}
