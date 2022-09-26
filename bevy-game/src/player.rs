use bevy::prelude::*;
use bevy::render::texture::ImageSettings;
use bevy_common_assets::ron::RonAssetPlugin;
use bevy_rapier2d::prelude::*;

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
        app.insert_resource(ImageSettings::default_nearest());
        app.add_startup_system(setup);

        // app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        //     .add_plugin(RapierDebugRenderPlugin::default());
        app.add_plugin(RonAssetPlugin::<CharacterAnimationResource>::new(&["ron"]));
        app.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(spawn_player));
        // app.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(spawn_player));
        app.add_system(animate_sprite);

        // app.add_system_set(
        //     SystemSet::new()
        //         .with_system(basic_sprite_animation_system)
        //         .with_system(animate_character_system.after("set_player_animation")),
        // );
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    // Add player resource
    let handle: Handle<CharacterAnimationResource> = asset_server.load("data/player.ron");
    commands.insert_resource(handle);

    let texture_handle = asset_server.load("player.png");
    let texture_atlas = TextureAtlas::from_grid_with_padding(texture_handle, Vec2::new(48.0, 69.0), 7, 1, Vec2::ZERO, Vec2::new(50.0, 27.0));
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // commands
    //     .spawn()
    //     .insert(PlayerComponent { speed: 1.5 })
    //     .insert(RigidBody::Dynamic);
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            // transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
            transform: Transform::from_translation(Vec3::splat(1.0)),
            ..default()
        })
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)));
}

/// Spawns a player
pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    character_animations: Res<Handle<CharacterAnimationResource>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // spawn player
    let texture_handle = asset_server.load("player.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 46.0), 6, 8);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
}

fn animate_sprite(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
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
