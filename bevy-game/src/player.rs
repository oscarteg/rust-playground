use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use bevy::render::texture::ImageSettings;
use bevy_rapier2d::plugin::RapierPhysicsPlugin;
use bevy_rapier2d::prelude::*;

pub const PLAYER_SPRITE_WIDTH: f32 = 48.0;
pub const PLAYER_SPRITE_HEIGHT: f32 = 69.0;
const PLAYER_SPRITE_SHEET_OFF_SET: f32 = 27.0;
const PLAYER_SPRITE_SHEET_PADDING: f32 = 24.0;

/// Stores core attributes of player
#[derive(Debug, Component)]
pub struct Player(f32);

#[derive(Component, DerefMut, Deref)]
pub struct AnimationTimer(Timer);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // app.insert_resource(ImageSettings::default_nearest()); // Prevents blurry sprites

        app.add_startup_system(setup);
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0));
        app.add_plugin(RapierDebugRenderPlugin::default());
        app.add_system(animate_player);
        app.add_system(player_movement);
    }
}


#[derive(Component)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    let texture_handle = asset_server.load("player.png");
    let texture_atlas = TextureAtlas::from_grid_with_padding(texture_handle, Vec2::new(PLAYER_SPRITE_WIDTH, PLAYER_SPRITE_HEIGHT), 4, 3, Vec2::new(0.0, PLAYER_SPRITE_SHEET_OFF_SET), Vec2::new(0.0, 24.0));
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_xyz(0.0, 0.0, 100.0),
            ..default()
        })
        .insert(Direction::Up)
        .insert(AnimationTimer(Timer::from_seconds(0.4, true)))
        .insert(RigidBody::Dynamic)
        .insert(Velocity::zero())
        // .insert(Collider::ball(sprite_size / 2.0))
        .insert(Player(100.0));
}

pub fn animate_player(
    time: Res<Time>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
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

// fn player_movement(
//     keyboard_input: Res<Input<KeyCode>>,
//     mut player_info: Query<(&Player, &mut Velocity)>,
// ) {
//     for (player, mut rb_vels) in player_info.iter_mut() {
//         let up = keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up);
//         let down = keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down);
//         let left = keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left);
//         let right = keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right);
//
//         let x_axis = -(left as i8) + right as i8;
//         let y_axis = -(down as i8) + up as i8;
//
//         let mut move_delta = Vec2::new(x_axis as f32, y_axis as f32);
//         if move_delta != Vec2::ZERO {
//             move_delta /= move_delta.length();
//         }
//
//         // Update the velocity on the rigid_body_component,
//         // the bevy_rapier plugin will update the Sprite transform.
//         rb_vels.linvel = move_delta * player.0;
//     }
// }

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_info: Query<(&Player, &mut Velocity)>,
) {
    for (player, mut rb_vels) in player_info.iter_mut() {
        let up = keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up);
        let down = keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down);
        let left = keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left);
        let right = keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right);

        let x_axis = -(left as i8) + right as i8;
        let y_axis = -(down as i8) + up as i8;

        let mut move_delta = Vec2::new(x_axis as f32, y_axis as f32);
        if move_delta != Vec2::ZERO {
            move_delta /= move_delta.length();
        }

        // Update the velocity on the rigid_body_component,
        // the bevy_rapier plugin will update the Sprite transform.
        rb_vels.linvel = move_delta * player.0;
    }
}

/// The sprite is animated by changing its translation depending on the time that has passed since
/// the last frame.
fn sprite_movement(keyboard_input: Res<Input<KeyCode>>, time: Res<Time>, mut sprite_position: Query<(&mut Direction, &mut Transform)>) {
    for (mut logo, mut transform) in &mut sprite_position {
        if keyboard_input.pressed(KeyCode::Down) {
            transform.translation.y -= 150. * time.delta_seconds()
        }

        if keyboard_input.pressed(KeyCode::Up) {
            transform.translation.y += 150. * time.delta_seconds()
        }

        if keyboard_input.pressed(KeyCode::Left) {
            transform.translation.x -= 150. * time.delta_seconds()
        }

        if keyboard_input.pressed(KeyCode::Right) {
            transform.translation.x += 150. * time.delta_seconds()
        }

        // match *logo {
        //     Direction::Down => transform.translation.y -= 150. * time.delta_seconds(),
        //     Direction::Left => transform.translation.y -= 150. * time.delta_seconds(),
        //     Direction::Right => transform.translation.y -= 150. * time.delta_seconds(),
        // }

        // if transform.translation.y > 200. {
        //     *logo = Direction::Down;
        // } else if transform.translation.y < -200. {
        //     *logo = Direction::Up;
        // }
    }
}