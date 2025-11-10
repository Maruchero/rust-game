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