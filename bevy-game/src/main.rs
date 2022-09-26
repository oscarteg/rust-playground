// Configure clippy for Bevy usage
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::enum_glob_use)]

use bevy::{prelude::*, window::WindowMode};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::ecs::archetype::Archetypes;
use bevy::ecs::component::Components;
use bevy::window::close_on_esc;

use gamestate::StatePlugin;
use player::PlayerPlugin;

mod animation;
mod gamestate;
mod player;

#[cfg(debug_assertions)]
fn main() {
    let mut app = App::new();

    app.insert_resource(WindowDescriptor {
        width: 1000.0,
        height: 1000.0,
        title: "RustyJam".to_string(),
        mode: WindowMode::Windowed,
        resizable: true,
        ..Default::default()
    })
        .add_startup_system(setup)
        .add_startup_system(print_resources)
        .add_plugins(DefaultPlugins)
        // Helper
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // Game
        .add_plugin(PlayerPlugin)
        .add_plugin(StatePlugin)
        .add_system(close_on_esc);

    app.run();
}

pub const BACKGROUND_Z: f32 = 0.0;
pub const PLAYER_SPRITE_WIDTH: f32 = 48.0;
pub const PLAYER_SPRITE_HEIGHT: f32 = 69.0;

/// Setup physics, camera, background, foreground, walls
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // create camera
    commands.spawn_bundle(Camera2dBundle::default());

    // create background
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            texture: asset_server.load("level.png"),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, BACKGROUND_Z)),
            ..Default::default()
        })
        .insert(Name::new("Background"));
}

// Debugging
fn print_resources(archetypes: &Archetypes, components: &Components) {
    let mut r: Vec<_> = archetypes
        .resource()
        .components()
        .map(|id| components.get_info(id).unwrap())
        .map(|info| info.name())
        .collect();

    // sort list alphebetically
    r.sort();
    r.iter().for_each(|name| println!("{}", name));
}
