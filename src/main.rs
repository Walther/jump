#![allow(clippy::type_complexity)] // Bevy has complex types

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;

mod game;
mod level;
mod menu;

// Enum that will be used as a global state for the game
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    MainMenu,
    Game,
    // PauseMenu,
    // GameOverMenu,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup)
        .add_state(GameState::MainMenu)
        .add_plugin(menu::MainMenuPlugin)
        .add_plugin(game::GamePlugin)
        // .add_plugin(game::PauseMenuPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    // UI camera
    commands.spawn_bundle(UiCameraBundle::default());
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in to_despawn.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
