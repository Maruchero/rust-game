# Project Overview

This is a 2D side-scrolling game being developed in Rust using the Bevy game engine. The goal is to create a game with a player character that can move, jump, and attack enemies, with sprite-based animations for characters and effects. The project is set up to be developed within a dev container.

# Building and Running

To build and run the project, use the following Cargo command:

```sh
cargo run
```

*TODO: Add instructions for running the game with GUI support from the dev container, as this requires X11 forwarding.*

# Development Conventions

*   **Language:** Rust
*   **Game Engine:** Bevy
*   **Code Style:** Follow standard Rust conventions and formatting (e.g., using `cargo fmt`).
*   **Testing:** *TODO: Define a testing strategy for the game.*

## Main Menu Architecture

The main menu is implemented using Bevy's state machine and built-in UI tools. This section describes the architecture.

### 1. Game States
The application's flow is managed by a `GameState` enum with three states:
*   `MainMenu`: For when the menu is displayed.
*   `InGame`: For when the actual game is being played.
*   `LevelEditor`: For the level editing mode.

The app is initialized into the `MainMenu` state.

### 2. UI Construction (`OnEnter`)
A `setup_menu` system runs once when the application enters the `MainMenu` state. It is responsible for spawning all the menu's UI entities, including:
*   A 2D Camera for the UI.
*   A root `NodeBundle` to structure the layout.
*   A `TextBundle` for the game's title.
*   Three `ButtonBundle` entities for "Play", "Level Editor", and "Quit".

All entities created in this system are tagged with a `MainMenuUI` component.

### 3. Button Interactivity (`Update`)
A system runs every frame during the `MainMenu` state to handle button interactivity:
*   It queries for buttons and changes their `BackgroundColor` based on the `Interaction` state (Hovered, Pressed, or None).
*   When a button is `Pressed`, it checks a `MenuButtonAction` component on the button to determine the action:
    *   **Play**: Changes the state to `InGame`.
    *   **Level Editor**: Changes the state to `LevelEditor`.
    *   **Quit**: Sends an `AppExit` event to close the application.

### 4. UI Cleanup (`OnExit`)
A `cleanup_menu` system runs once when the application exits the `MainMenu` state. It queries for all entities with the `MainMenuUI` tag and despawns them recursively, ensuring the menu does not appear over other game states.

## Texture Atlas Loading Plan

This plan outlines the steps to load sprite sheets as `TextureAtlas` assets, making them available throughout the application for rendering tiles and sprites. This will focus on the `cave-structure` assets first.

### 1. Create a `Loading` State
To ensure assets are loaded before the game starts, a new `Loading` state will be added to the `GameState` enum. The application will be configured to start in this state.

### 2. Create a Central Asset Resource
A new resource, `CaveAtlases`, will be created to store the handles for our loaded texture atlases. This will make them easily accessible from any system that needs to spawn tiles or sprites.

```rust
// Example Structure
#[derive(Resource)]
struct CaveAtlases {
    pub platforms: Handle<TextureAtlas>,
    pub floor: Handle<TextureAtlas>,
    // ... other handles
}
```

### 3. Implement Asset Loading Logic
A new module, `src/asset_loading.rs`, will be created to contain the loading logic. A system within this module will run once upon entering the `Loading` state. It will:
1.  Load the raw `.png` image files from `assets/textures/cave-structure/`.
2.  For each image that is a sprite sheet, create a `TextureAtlasLayout` using the dimensions you provide (tile size, columns, rows).
3.  Create a `TextureAtlas` from the image and its layout.
4.  Store the handle to the final `TextureAtlas` in the `CaveAtlases` resource.

### 4. Implement State Transition
A second system will run every frame during the `Loading` state. It will monitor the load status of all the assets we're loading. Once all assets are confirmed to be fully loaded, it will change the `GameState` from `Loading` to `MainMenu`.

### 5. Information Required
To implement this, I will need the layout information for each file in `assets/textures/cave-structure/` that you want to use as a sprite sheet. For each one, please provide:
1.  The **file path**.
2.  The **pixel dimensions** of a single sprite/tile (width and height).
3.  The **number of columns and rows** in the sheet.

## Level Editor Implementation Plan

This plan outlines the steps to create a basic level editor for the game.

### 1. Create a `LevelEditorPlugin`
All logic for the editor will be encapsulated in its own plugin. This plugin will contain all the systems and resources that run only when the application is in the `LevelEditor` state.

### 2. Set Up the Editor UI with `bevy_egui`
A robust UI is needed for an editor. The `bevy_egui` library is the standard choice for this in the Bevy ecosystem.
*   Add the `bevy_egui` crate to the project.
*   Create a system to draw an `egui` window that will serve as the editor's control panel.
*   This panel will contain UI for selecting tiles, tools (e.g., place, erase), and buttons for actions like "Save" and "Return to Main Menu".

### 3. Implement Mouse and Grid Systems
To place objects in the world, we need to know where the mouse is.
*   Create a system to convert the mouse's screen position to world coordinates.
*   Implement a grid system to snap the world coordinates, ensuring tiles are placed in an organized way.

### 4. Implement Tile Placement and Deletion
This is the core interactive feature of the editor.
*   A system will use the snapped grid coordinates from the mouse position.
*   On **left-click**, it will spawn the tile currently selected in the `egui` UI panel.
*   On **right-click**, it will despawn any tile at that grid location.

### 5. Implement Level Saving and Loading (`serde`)
To make the editor useful, levels must be saved and loaded. The `serde` library is the standard for this in Rust.
*   **Define Level Structure:** Create a serializable Rust `struct` to define the layout of a level (e.g., a list of tile types and their positions).
*   **Saving:** An event triggered by the "Save" button will cause a system to run. This system will query all placed tiles, build the level `struct`, and use `serde` to write it to a file (e.g., `assets/levels/my_level.json`).
*   **Loading:** The `InGame` state will have a corresponding system to read a level file, deserialize it, and spawn the entities into the game world.

## Development Log

### Progress - 2025-11-10
*   **Dev Environment:** Diagnosed and fixed critical graphics and audio issues within the dev container. Stabilized the `.devcontainer.json` configuration for a reliable X11-based workflow.
*   **Asset Organization:** Refactored the `assets` directory to be organized by asset type (`textures`, `fonts`, etc.) for better scalability.
*   **Main Menu:** Designed and implemented a complete, functional main menu with interactive buttons, including planning, coding, and debugging.
*   **Code Architecture:** Refactored the entire application from a single `main.rs` file into a modular, plugin-based architecture (`MenuPlugin`, `states` module), which is the standard for Bevy projects.
*   **Documentation:** Documented the new code structure and created high-level implementation plans for the next major features (Texture Atlases, Level Editor).

### Next Steps
1.  **Implement Texture Atlas Loading:** The immediate next step is to provide the layout information (tile size, columns, rows) for the `cave-structure` sprite sheets so I can implement the asset loading system.
2.  **Implement the Level Editor:** Follow the documented plan to build the level editor using `bevy_egui` and `serde`.
3.  **Implement the `InGame` State:** Begin development on the core gameplay mechanics, such as player character, controls, and physics.
