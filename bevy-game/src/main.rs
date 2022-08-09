#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Configure clippy for Bevy usage
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::enum_glob_use)]

use crate::states::GameState;
use animation::CharacterAnimationResource;
use bevy::prelude::*;
use bevy::window::close_on_esc;
use bevy::{app::AppExit, prelude::*, window::WindowMode};
use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    reflect::TypeUuid,
    utils::BoxedFuture,
};
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
        resizable: false,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .init_asset_loader::<CustomAssetLoader>()
    .add_plugin(ApartmentPlugin)
    .add_state(GameState::MainMenu)
    .add_system(close_on_esc);

    app.run();
}

pub struct ApartmentPlugin;

pub const BACKGROUND_Z: f32 = 0.0;
pub const HALLWAY_COVER_Z: f32 = 1.0;
pub const PLAYER_IN_BED_Z: f32 = 2.0;
pub const PIZZA_Z: f32 = 2.0;
pub const NPC_Z: f32 = 4.0;
pub const PLAYER_Z: f32 = 5.0;
pub const FOREGROUND_Z: f32 = 10.0;
pub const INTERACTABLE_ICON_Z: f32 = 11.0;
pub const LIGHTING_Z: f32 = 10.5;
pub const PEEPHOLE_Z: f32 = 10.2;

impl Plugin for ApartmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugin(RapierDebugRenderPlugin::default())
            // .add_system(print_on_load)
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
            );

        app.add_system(animation::basic_sprite_animation_system);
        app.add_system(animation::animate_character_system.after("set_player_animation"));
    }
}

#[derive(Default)]
pub struct CustomAssetLoader;

impl AssetLoader for CustomAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let custom_asset = ron::de::from_bytes::<CharacterAnimationResource>(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(custom_asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}

/// Setup physics, camera, background, foreground, walls
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    // setup rapier
    rapier_config.gravity = Vec2 { x: 0.0, y: 0.0 };

    rapier_config.scaled_shape_subdivision = 10;

    // create camera
    commands.spawn_bundle(Camera2dBundle::default());

    // create background
    //let texture_handle = asset_server.load("textures/apartment_background.png");
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            texture: asset_server.load("level.png"),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, BACKGROUND_Z)),
            ..Default::default()
        })
        .insert(Name::new("Background"));
}

// fn print_on_load(
//     mut state: ResMut<State>,
//     custom_assets: ResMut<Assets<CharacterAnimationResource>>,
// ) {
//     let custom_asset = custom_assets.get(&state.handle);
//     if state.printed || custom_asset.is_none() {
//         return;
//     }
//
//     info!("Custom asset loaded: {:?}", custom_asset.unwrap());
//     state.printed = true;
// }
