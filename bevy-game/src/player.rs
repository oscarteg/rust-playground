use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_rapier2d::plugin::RapierPhysicsPlugin;
use bevy_rapier2d::prelude::*;
use ron::de::from_bytes;
use serde::Deserialize;

// pub const PLAYER_SPRITE_WIDTH: f32 = 48.0;
// pub const PLAYER_SPRITE_HEIGHT: f32 = 69.0;
pub const PLAYER_SPRITE_SHEET_OFF_SET: f32 = 27.0;
pub const PLAYER_SPRITE_SHEET_PADDING: f32 = 24.0;
pub const PLAYER_SPRITE_SCALE: f32 = 2.0;
pub const PLAYER_Z: f32 = 5.0;

// Plugin

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0));
        app.add_plugin(RapierDebugRenderPlugin::default());

        let file_bytes: &[u8; 274] = include_bytes!("../data/character_animations.ron");
        app.insert_resource(from_bytes::<CharacterAnimationResource>(file_bytes).unwrap());
        app.add_system_set(
            SystemSet::new()
                .with_system(spawn_player)
                .with_system(player_movement_system)
                .with_system(set_player_animation_system.label("set_player_animation")),
        );

        app.add_system(basic_sprite_animation_system);
        app.add_system(animate_character_system.after("set_player_animation"));
    }
}

/// This will be used to identify the main player entity
#[derive(Debug, Component)]
pub struct Player {
    pub speed: f32,
}

#[derive(Component)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn setup(
    // commands: Commands,
    asset_server: Res<AssetServer>,
    // mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    rapier_config.gravity = Vec2::ZERO;

    // commands
    //     .spawn()
    //     .insert(Direction::Up)
    //     .insert(Player { speed: 100.0 })
    //     .insert(RigidBody::Dynamic)
    //     .insert(Velocity::zero())
}

///
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
        .insert(Player { speed: 1.5 })
        .insert(CharacterAnimationComponent {
            timer: AnimationTimer(Timer::from_seconds(
                character_animations.animations[&character_starting_animation].2 as f32,
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
        .insert(Velocity::zero());
    // .insert_bundle(
    //     body_type: RigidBodyType::Dynamic,
    //     mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
    //     position: Vec2::new(10.0, 0.0).into(),
    //     ..Default::default()
    // })
    //         .insert_bundle((
    //             RigidBodyPositionSync::Discrete,
    //             Name::new("Player"),
    //             Player { speed: 1.5 },
    //         ));
}

// Set the player's animation based on what the player is doing
pub fn set_player_animation_system(
    keyboard_input: Res<Input<KeyCode>>,
    character_animations: Res<CharacterAnimationResource>,
    mut player_query: Query<
        (
            &mut CharacterAnimationComponent,
            &mut TextureAtlasSprite,
            &Velocity,
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
            character_animation.timer =
                AnimationTimer(Timer::from_seconds(animation_data.2 as f32, true));
        }
    }
}

// Move player by modifying velocity with input
pub fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    rapier_config: Res<RapierConfiguration>,
    mut player_info: Query<(&Player, &mut Velocity)>,
) {
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
            rb_vels.linvel.x = player.speed * (x_axis as f32);
        } else {
            rb_vels.linvel.x = 0.0;
        }

        // handle movement in y direction
        if y_axis != 0 {
            // accelerate to the player's maximum speed stat
            rb_vels.linvel.y = player.speed * (y_axis as f32);
        } else {
            rb_vels.linvel.y = 0.0;
        }
    }
}

// Animation

#[derive(Component, DerefMut, Deref)]
pub struct AnimationTimer(Timer);

#[derive(Component)]
pub struct BasicAnimationComponent;

#[derive(Deserialize, Hash, PartialEq, Eq, Clone)]
pub enum CharacterAnimationType {
    // Idle
    ForwardIdle,
    BackwardIdle,
    LeftIdle,
    RightIdle,
    // Move
    ForwardMove,
    BackwardMove,
    LeftMove,
    RightMove,
}

impl CharacterAnimationType {
    pub fn is_idle(&self) -> bool {
        matches!(
            self,
            Self::ForwardIdle | Self::BackwardIdle | Self::LeftIdle | Self::RightIdle
        )
    }
}

/// Stores data about character animations frames (data/character_animations.ron)
#[derive(Deserialize)]
pub struct CharacterAnimationResource {
    // start and end indexes of animations
    pub animations: HashMap<CharacterAnimationType, (f32, f32, f32)>,
}

/// Used for tracking animations of a character entity
#[derive(Component)]
pub struct CharacterAnimationComponent {
    pub timer: AnimationTimer,
    pub animation_type: CharacterAnimationType,
}

pub fn basic_sprite_animation_system(
    time: Res<Time>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut query: Query<
        (
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
            &Handle<TextureAtlas>,
        ),
        With<BasicAnimationComponent>,
    >,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index as usize + 1) % texture_atlas.textures.len();
        }
    }
}

pub fn animate_character_system(
    time: Res<Time>,
    character_animations: Res<CharacterAnimationResource>,
    animation_query: Query<(&mut CharacterAnimationComponent, &mut TextureAtlasSprite)>,
) {
    for (character_animation, sprite) in animation_query.into_iter() {
        character_animation.timer.clone().tick(time.delta());

        if character_animation.timer.finished() {
            // Get the indexed of the animation type
            let animation_idx =
                character_animations.animations[&character_animation.animation_type];

            if sprite.index == animation_idx.1 as usize {
                sprite.clone().index = animation_idx.0 as usize
            } else {
                sprite.clone().index += 1;
            }
        }
    }
}
