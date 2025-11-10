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

## Main Menu Implementation Plan

Here is a high-level outline of the steps to implement the game menu.

### 1. Define Game States
First, you'll need to manage the different states of your application. You'll introduce an `enum` to represent the distinct modes:
*   `MainMenu`: For when the menu is displayed.
*   `InGame`: For when the actual game is being played.
*   `LevelEditor`: For the level editing mode.

You will register this `enum` with the Bevy app and set `MainMenu` as the initial state.

### 2. Create the Menu UI
A "setup" system will run once when the application enters the `MainMenu` state. This system will create the menu's visual elements:
*   Spawn a root UI node to cover the screen.
*   Spawn text for the game's title.
*   Spawn three buttons: "Play", "Level Editor", and "Quit".

### 3. Implement Button Interactivity
Another system will run continuously while in the `MainMenu` state to:
*   Monitor the buttons for user clicks and provide visual feedback (e.g., hover color).
*   When "Play" is clicked, change the state to `InGame`.
*   When "Level Editor" is clicked, change the state to `LevelEditor`.
*   When "Quit" is clicked, send an event to close the application.

### 4. Clean Up the Menu
A "cleanup" system will run once when the application exits the `MainMenu` state. This system will despawn all menu UI elements so they don't appear in other states.

### Required Asset
You will need a font file (like a `.ttf` or `.otf`) in your `assets` directory to display the text.