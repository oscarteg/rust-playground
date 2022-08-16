// Configure clippy for Bevy usage
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::enum_glob_use)]

use crate::states::GameState;
use animation::CharacterAnimationResource;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::ecs::archetype::Archetypes;
use bevy::ecs::component::Components;
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::window::close_on_esc;
use bevy::{app::AppExit, prelude::*, window::WindowMode};
use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    reflect::TypeUuid,
    utils::BoxedFuture,
};
use bevy_common_assets::ron::RonAssetPlugin;
use bevy_rapier2d::na::Vector2;
use bevy_rapier2d::prelude::*;
use rand::Rng;
use rapier2d::math::Vector;
use ron::de::from_bytes;
use serde::Deserialize;

mod animation;
mod gamestate;
mod menu;
mod player;
mod states;

#[cfg(debug_assertions)]
//use bevy_inspector_egui::WorldInspectorPlugin;
fn main() {
    let mut app = App::new();

    app.insert_resource(WindowDescriptor {
        width: 800.0,
        height: 600.0,
        title: "RustyJam".to_string(),
        mode: WindowMode::Windowed,
        resizable: true,
        ..Default::default()
    })
    .add_startup_system(setup)
    .add_plugins(DefaultPlugins)
    .add_plugin(LogDiagnosticsPlugin::default())
    .add_plugin(FrameTimeDiagnosticsPlugin::default())
    // .add_plugin(PlayerPlugin)
    .add_state(GameState::MainMenu)
    .add_system(close_on_esc);

    app.run();
}

pub struct PlayerPlugin;

pub const BACKGROUND_Z: f32 = 0.0;
pub const PLAYER_Z: f32 = 5.0;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugin(RapierDebugRenderPlugin::default())
            .add_plugin(RonAssetPlugin::<CharacterAnimationResource>::new(&["ron"]))
            // .add_system(print_on_load)
            .add_system(animation::basic_sprite_animation_system)
            .add_system(animation::animate_character_system.after("set_player_animation"))
            .add_system_set(
                SystemSet::on_enter(GameState::MainGame)
                    .with_system(setup.label("apartment_setup"))
                    .with_system(player::spawn_player.after("apartment_setup")),
            )
            .add_system_set(
                SystemSet::on_update(GameState::MainGame)
                    .with_system(player::player_movement_system.label("player_movement"))
                    .with_system(
                        player::set_player_animation_system
                            .after("player_movement")
                            .label("set_player_animation"),
                    ),
            )
            .add_system_set(
                SystemSet::on_enter(GameState::MainGame)
                    .with_system(setup.label("apartment_setup"))
                    .with_system(player::spawn_player.after("apartment_setup")),
            );
    }
}

/// Setup physics, camera, background, foreground, walls
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    // mut rapier_config: ResMut<RapierConfiguration>,
) {
    // setup rapier
    // rapier_config.gravity = Vec2 { x: 0.0, y: 0.0 };
    // rapier_config.scaled_shape_subdivision = 10;

    // Add player resource
    let handle: Handle<CharacterAnimationResource> = asset_server.load("data/player.ron");
    commands.insert_resource(handle);

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
