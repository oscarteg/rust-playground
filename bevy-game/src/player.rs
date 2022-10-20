use std::any::{Any, TypeId};
use std::borrow::Borrow;
use std::thread::spawn;

use bevy::prelude::*;
use bevy::render::texture::ImageSettings;
use bevy_common_assets::ron::RonAssetPlugin;
use bevy_rapier2d::prelude::*;
use rapier2d::prelude::{
    ColliderFlags, ColliderShape, RigidBodyActivation, RigidBodyCcd, RigidBodyMassProps,
    RigidBodyVelocity,
};

use crate::animation::*;
use crate::gamestate::AppState;

pub const PLAYER_Z: f32 = 5.0;
pub const PLAYER_SPRITE_WIDTH: f32 = 48.0;
pub const PLAYER_SPRITE_HEIGHT: f32 = 69.0;

/// Stores core attributes of player
#[derive(Debug, Component)]
pub struct Player {
    pub velocity: f32,
    pub speed: Velocity,
}

// Start of the plugin
pub struct PlayerPlugin;


impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ImageSettings::default_nearest());
        app.add_startup_system(setup);
        // app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugin(RapierDebugRenderPlugin::default());
        app.add_plugin(RonAssetPlugin::<CharacterAnimationResource>::new(&["ron"]));
        app.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(spawn_player));
        // app.add_system_set(SystemSet::on_update(AppState::InGame).with_system(spawn_player));
        app.add_system_set(
            SystemSet::on_update(AppState::InGame).with_system(player_movement_system),
        );
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut rapier_config: ResMut<RapierConfiguration>) {
    // Add player resource
    let handle: Handle<CharacterAnimationResource> = asset_server.load("data/player.ron");
    commands.insert_resource(handle);
    rapier_config.gravity = Vec2::ZERO;
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<(&Player, &mut RigidBodyVelocity)>
) {
    for (player, mut velocity) in players.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            velocity.linvel = Vec2::new(-player.speed, velocity.linvel.y).into();
        }
        if keyboard_input.pressed(KeyCode::Right) {
            velocity.linvel = Vec2::new(player.speed, velocity.linvel.y).into();
        }
    }
}

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {

    let texture_handle = asset_server.load("player.png");
    let texture_atlas = TextureAtlas::from_grid_with_padding(
        texture_handle,
        Vec2::new(PLAYER_SPRITE_WIDTH, PLAYER_SPRITE_HEIGHT),
        7,
        1,
        Vec2::ZERO,
        Vec2::new(50.0, 27.0),
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        // .insert_bundle(rigid_body)
        // .insert_bundle(collider)
        .spawn()
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_translation(Vec3::splat(1.0)),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Velocity::zero())
        .insert(Player {
            velocity: 100.0,
            speed: Velocity {
                linvel: Vec2::new(1.0, 2.0),
                angvel: 0.4,
            },
        })
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)));
}

/// Set the player's animation based on what the player is doing
pub fn set_player_animation(
    keyboard_input: Res<Input<KeyCode>>,
    character_animations: Res<CharacterAnimationResource>,
    mut player_query: Query<
        (
            &mut CharacterAnimationComponent,
            &mut TextureAtlasSprite,
            &mut Velocity,
        ),
        With<Player>,
    >,
) {
    for (mut character_animation, mut sprite, rb_vels) in player_query.iter_mut() {
        let mut restart_animation = false;

        // set to idle animation if velocity is 0 and key is released
        if rb_vels.linvel.x == 0.0 && rb_vels.linvel.y == 0.0 {
            if keyboard_input.just_released(KeyCode::A)
                || keyboard_input.just_released(KeyCode::Left)
            {
                character_animation.animation_type = CharacterAnimationType::LeftIdle;
                restart_animation = true;
            } else if keyboard_input.just_released(KeyCode::D)
                || keyboard_input.just_released(KeyCode::Right)
            {
                character_animation.animation_type = CharacterAnimationType::RightIdle;
                restart_animation = true;
            } else if keyboard_input.just_released(KeyCode::W)
                || keyboard_input.just_released(KeyCode::Up)
            {
                character_animation.animation_type = CharacterAnimationType::BackwardIdle;
                restart_animation = true;
            } else if keyboard_input.just_released(KeyCode::S)
                || keyboard_input.just_released(KeyCode::Down)
            {
                character_animation.animation_type = CharacterAnimationType::ForwardIdle;
                restart_animation = true;
            }
        }
        // set to move animation if key pressed
        if keyboard_input.just_pressed(KeyCode::A) || keyboard_input.just_pressed(KeyCode::Left) {
            character_animation.animation_type = CharacterAnimationType::LeftMove;
            restart_animation = true;
        } else if keyboard_input.just_pressed(KeyCode::D)
            || keyboard_input.just_pressed(KeyCode::Right)
        {
            character_animation.animation_type = CharacterAnimationType::RightMove;
            restart_animation = true;
        } else if keyboard_input.just_pressed(KeyCode::W)
            || keyboard_input.just_pressed(KeyCode::Up)
        {
            character_animation.animation_type = CharacterAnimationType::BackwardMove;
            restart_animation = true;
        } else if keyboard_input.just_pressed(KeyCode::S)
            || keyboard_input.just_pressed(KeyCode::Down)
        {
            character_animation.animation_type = CharacterAnimationType::ForwardMove;
            restart_animation = true;
        }

        // if animation changed restart the timer, sprite, and set animation type
        if restart_animation {
            let animation_data =
                character_animations.animations[&character_animation.animation_type];
            sprite.index = animation_data.0 as usize;
            character_animation.timer = AnimationTimer(Timer::from_seconds(animation_data.2, true));
        }
    }
}

