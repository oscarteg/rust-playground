use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rapier2d::prelude::RigidBodyType;

use crate::animation::{
    AnimationTimer, CharacterAnimationComponent, CharacterAnimationResource, CharacterAnimationType,
};

use crate::{states::GameState, PLAYER_Z};

pub const PLAYER_SPRITE_SCALE: f32 = 2.0;

/// Stores core attributes of player
#[derive(Debug, Component)]
pub struct PlayerComponent {
    pub speed: f32,
}

/// Spawns a player
pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    character_animations: Res<CharacterAnimationResource>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // spawn player
    let character_starting_animation = CharacterAnimationType::ForwardIdle;
    let texture_handle = asset_server.load("player.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 46.0), 6, 8);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let sprite_transform = Transform {
        translation: Vec3::new(0.0, 0.0, PLAYER_Z),
        scale: Vec3::new(PLAYER_SPRITE_SCALE, PLAYER_SPRITE_SCALE, 0.0),
        ..Default::default()
    };

    commands
        .spawn()
        .insert(PlayerComponent { speed: 1.5 })
        .insert(CharacterAnimationComponent {
            timer: AnimationTimer(Timer::from_seconds(
                character_animations.animations[&character_starting_animation].2,
                true,
            )),
            animation_type: character_starting_animation.clone(),
        })
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: sprite_transform,
            sprite: TextureAtlasSprite {
                index: character_animations.animations[&character_starting_animation].0 as usize,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert_bundle(TransformBundle::from(Transform::from_xyz(10.0, 0.0, 0.0)));
    // .insert_bundle()
    // .insert_bundle(RigidBody::Dynamic {
    //     body_type: RigidBodyType::Dynamic.into(),
    //     mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
    //     position: Vec2::new(10.0, 0.0).into(),
    //     ..Default::default()
    // })
    // .insert_bundle((
    //     RigidBodyPositionSync::Discrete,
    //     Name::new("Player"),
    //     PlayerComponent { speed: 1.5 },
    // ))
    // .with_children(|parent| {
    //     parent.spawn().insert_bundle(ColliderBundle {
    //         shape: ColliderShape::cuboid(3.0, 1.0).into(),
    //         position: Vec2::new(0.0, -3.8).into(),
    //         material: ColliderMaterial {
    //             friction: 0.0,
    //             restitution: 0.0,
    //             ..Default::default()
    //         }
    //         .into(),
    //         ..Default::default()
    //     });
    // });
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
    rapier_config: Res<RapierConfiguration>,
    mut player_info: Query<(&PlayerComponent, &mut Velocity)>,
    app_state: Res<State<GameState>>,
) {
    // if we are not playing the game prevent the player from moving
    if app_state.current() != &GameState::MainGame {
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
            rb_vels.linvel.x =
                player.speed * (x_axis as f32) * rapier_config.scaled_shape_subdivision as f32;
        } else {
            rb_vels.linvel.x = 0.0;
        }

        // handle movement in y direction
        if y_axis != 0 {
            // accelerate to the player's maximum speed stat
            rb_vels.linvel.y =
                player.speed * (y_axis as f32) * rapier_config.scaled_shape_subdivision as f32;
        } else {
            rb_vels.linvel.y = 0.0;
        }
    }
}