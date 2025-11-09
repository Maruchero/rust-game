# Rust 2D Side-Scroller Game

This document outlines the plan for creating a 2D side-scrolling game with combat animations using Rust and the Bevy game engine, developed within a dev container.

## Project Overview

The game will be a 2D side-scrolling action game. The player will control a character that can move, jump, and attack enemies. The game will feature sprite-based animations for characters and effects. The existing assets in the `assets` directory will be used for the player, enemies, and environment.

## Core Features

*   **Player Control:** Responsive character movement (left, right, jump).
*   **Combat:** Player attack animations and collision detection.
*   **Enemies:** Basic enemy AI (e.g., patrolling).
*   **Scrolling Camera:** The camera will follow the player.
*   **Sprite Animations:** Animations for player and enemies (idle, run, attack).

## Technology Stack

*   **Language:** Rust
*   **Game Engine:** Bevy
*   **Development Environment:** Dev container (Docker)

## Development Plan

1.  **Environment Setup:**
    *   Configure the dev container for GUI applications (X11 forwarding).
    *   Set up a new Rust project using `cargo`.
    *   Add Bevy as a dependency.

2.  **Basic Scene:**
    *   Create a Bevy application that opens a window with a background color.
    *   Load and display a static sprite for the player.

3.  **Player Movement:**
    *   Implement keyboard controls for player movement.
    *   Add jumping mechanics.

4.  **World and Camera:**
    *   Build a simple level using the provided assets.
    *   Implement a 2D camera that follows the player's position.

5.  **Animation:**
    *   Create a texture atlas from the sprite sheets.
    *   Implement animations for the player character (e.g., idle, running).

6.  **Enemies and Combat:**
    *   Add enemy sprites to the level.
    *   Implement basic enemy patrol logic.
    *   Implement a player attack mechanic.
    *   Add collision detection to register hits on enemies.

7.  **UI and Polish:**
    *   Add a simple UI (e.g., health display).
    *   Incorporate sound effects and music.
    *   Refine gameplay and add more complex level elements.

## Devcontainer Usage

To run a GUI application from within the dev container, X11 forwarding needs to be configured. This involves:
1.  Installing X11 server on the host machine.
2.  Configuring the `Dockerfile` to install necessary X11 libraries.
3.  Setting the `DISPLAY` environment variable in `devcontainer.json`.

We will address the specific configuration details in the setup phase.
