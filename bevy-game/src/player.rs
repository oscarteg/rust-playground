use std::borrow::{Borrow, BorrowMut};
use std::collections::HashMap;
use bevy::prelude::*;
use bevy::render::texture::ImageSettings;
use bevy_common_assets::ron::RonAssetPlugin;
use bevy_rapier2d::prelude::*;
use rapier2d::prelude::RigidBodyType;
use crate::animation::*;
use crate::gamestate::AppState;

const PLAYER_SPRITE_SCALE: f32 = 2.0;
const PLAYER_Z: f32 = 5.0;

/// Stores core attributes of player
#[derive(Debug, Component)]
pub struct PlayerComponent {
    pub speed: f32,
}

// Start of the plugin
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
        app.insert_resource(ImageSettings::default_nearest());
        app.add_plugin(RonAssetPlugin::<CharacterAnimationResource>::new(&["ron"]));
        app.add_system(spawn_player);

        // app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        //     .add_plugin(RapierDebugRenderPlugin::default());
        // app.add_system_set(
        //     SystemSet::new()
        //         .with_system(basic_sprite_animation_system)
        //         .with_system(animate_character_system.after("set_player_animation")),
        // );
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Add player resource
    let handle: Handle<CharacterAnimationResource> = asset_server.load("data/player.ron");
    commands.insert_resource(handle);

    let texture_handle: Handle<Image> = asset_server.load("player.png");
    commands.insert_resource(texture_handle);
}

/// Spawns a player
pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    texture_handle: Res<Handle<CharacterAnimationResource>>,
    character_animations: Res<Handle<CharacterAnimationResource>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // spawn player
    let character_starting_animation = CharacterAnimationType::ForwardIdle;
    // let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 46.0), 6, 8);
    // let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let sprite_transform = Transform {
        translation: Vec3::new(0.0, 0.0, PLAYER_Z),
        scale: Vec3::new(PLAYER_SPRITE_SCALE, PLAYER_SPRITE_SCALE, 0.0),
        ..Default::default()
    };

    commands
        .spawn()
        .insert(PlayerComponent { speed: 1.5 })
        // .insert(CharacterAnimationComponent {
        //     timer: AnimationTimer(Timer::from_seconds(
        //         character_animations.animations[&character_starting_animation].2,
        //         true,
        //     )),
        //     animation_type: character_starting_animation.clone(),
        // })
        // .insert_bundle(SpriteSheetBundle {
        //     texture_atlas: texture_atlas_handle,
        //     transform: sprite_transform,
        //     sprite: TextureAtlasSprite {
        //         index: character_animations.animations[&character_starting_animation].0 as usize,
        //         ..Default::default()
        //     },
        //     ..Default::default()
        // })
        .insert(RigidBody::Dynamic)
        .insert_bundle(TransformBundle::from(Transform::from_xyz(10.0, 0.0, 0.0)));
}

/// Set the player's animation based on what the player is doing
pub fn set_player_animation_system(
    keyboard_input: Res<Input<KeyCode>>,
    character_animations: Res<CharacterAnimationResource>,
    mut player_query: Query<
        (
            &mut CharacterAnimationComponent,
            &mut TextureAtlasSprite,
            &mut Velocity,
        ),
        With<PlayerComponent>,
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

// Move player by modifying velocity with input
pub fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    // rapier_config: Res<RapierConfiguration>,
    mut player_info: Query<(&PlayerComponent, &mut Velocity)>,
    app_state: Res<State<AppState>>,
) {
    // if we are not playing the game prevent the player from moving
    if app_state.current() != &AppState::InGame {
        return;
    }

    for (player, mut rb_vels) in player_info.iter_mut() {
        // get key presses
        let up = keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up);
        let down = keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down);
        let left = keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left);
        let right = keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right);

        // convert to axis multipliers
        let x_axis = -(left as i8) + right as i8;
        let y_axis = -(down as i8) + up as i8;

        // handle movement in x direction
        if x_axis != 0 {
            // accelerate to the player's maximum speed stat
            // rb_vels.linvel.x =
            // player.speed * (x_axis as f32) * rapier_config.scaled_shape_subdivision as f32;
        } else {
            rb_vels.linvel.x = 0.0;
        }

        // handle movement in y direction
        if y_axis != 0 {
            // accelerate to the player's maximum speed stat
            // rb_vels.linvel.y =
            //     player.speed * (y_axis as f32) * rapier_config.scaled_shape_subdivision as f32;
        } else {
            rb_vels.linvel.y = 0.0;
        }
    }
}
