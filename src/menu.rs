//! This module contains the `MenuPlugin` and all associated logic for the main menu.
use bevy::{app::AppExit, prelude::*};
use crate::states::GameState;

/// This plugin is responsible for building and managing the main menu.
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), setup_menu)
            .add_systems(
                Update,
                button_interaction_system.run_if(in_state(GameState::MainMenu)),
            )
            .add_systems(OnExit(GameState::MainMenu), cleanup_menu);
    }
}

// --- Components ---
#[derive(Component)]
struct MainMenuUI;
#[derive(Component)]
enum MenuButtonAction {
    Play,
    LevelEditor,
    Quit,
}

// --- Constants ---
const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

// --- Systems ---

/// Handles button interactions, changing their color and performing actions on click.
fn button_interaction_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color, menu_button_action) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                // Perform the action associated with the button
                match menu_button_action {
                    MenuButtonAction::Play => next_state.set(GameState::InGame),
                    MenuButtonAction::LevelEditor => next_state.set(GameState::LevelEditor),
                    MenuButtonAction::Quit => {
                        app_exit_events.send(AppExit);
                    }
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

/// Spawns all the UI entities for the main menu.
fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    // A 2D camera is required for UI to be rendered
    commands.spawn((Camera2dBundle::default(), MainMenuUI));

    let font = asset_server.load("fonts/DejaVuSans.ttf");

    // Root node that centers the menu
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            MainMenuUI,
        ))
        .with_children(|parent| {
            // Game title
            parent.spawn(TextBundle::from_section(
                "My Awesome Game",
                TextStyle {
                    font: font.clone(),
                    font_size: 80.0,
                    color: TEXT_COLOR,
                },
            )
            .with_style(Style {
                margin: UiRect::all(Val::Px(50.0)),
                ..default()
            }));

            // --- Buttons ---
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(250.0),
                            height: Val::Px(65.0),
                            margin: UiRect::all(Val::Px(10.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    MenuButtonAction::Play,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Play",
                        TextStyle {
                            font: font.clone(),
                            font_size: 40.0,
                            color: TEXT_COLOR,
                        },
                    ));
                });

            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(250.0),
                            height: Val::Px(65.0),
                            margin: UiRect::all(Val::Px(10.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    MenuButtonAction::LevelEditor,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Level Editor",
                        TextStyle {
                            font: font.clone(),
                            font_size: 40.0,
                            color: TEXT_COLOR,
                        },
                    ));
                });

            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(250.0),
                            height: Val::Px(65.0),
                            margin: UiRect::all(Val::Px(10.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    MenuButtonAction::Quit,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Quit",
                        TextStyle {
                            font: font.clone(),
                            font_size: 40.0,
                            color: TEXT_COLOR,
                        },
                    ));
                });
        });
}

/// Despawns all entities tagged with `MainMenuUI`.
fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<MainMenuUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}