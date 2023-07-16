// Configure clippy for Bevy usage
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::enum_glob_use)]
#![allow(dead_code, unused_variables)]

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::render::render_resource::Texture;
use bevy::window::close_on_esc;

pub const BACKGROUND_Z: f32 = 1.0;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    MainMenu,
    InGame,
}

#[cfg(debug_assertions)]
fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins.set(
            // This sets image filtering to nearest
            // This is done to prevent textures with low resolution (e.g. pixel art) from being blurred
            // by linear filtering.
            ImagePlugin::default_nearest(),
        ),
        LogDiagnosticsPlugin::default(),
        FrameTimeDiagnosticsPlugin::default(),
        // Any plugin can register diagnostics. Uncomment this to add an entity count diagnostics:
        bevy::diagnostic::EntityCountDiagnosticsPlugin::default(),
        // Uncomment this to add an asset count diagnostics:
        // bevy::asset::diagnostic::AssetCountDiagnosticsPlugin::<Texture>::default(),
        // Uncomment this to add system info diagnostics:
        bevy::diagnostic::SystemInformationDiagnosticsPlugin::default(),
    ))
    .add_systems(Startup, setup)
    .add_systems(Update, close_on_esc);
    app.run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("level.png"),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, BACKGROUND_Z)),
            ..Default::default()
        })
        .insert(Name::new("Background"));
}
