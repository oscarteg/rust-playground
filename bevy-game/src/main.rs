//! Renders an animated sprite by loading all animation frames from a single image (a sprite sheet)
//! into a texture atlas, and changing the displayed image periodically.

use bevy::{input::keyboard::KeyboardInput, prelude::*, render::texture::ImageSettings};

/// player component
#[derive(Component)]
struct Player {
    /// linear speed in meters per second
    movement_speed: f32,
}

#[derive(Default)]
struct CollisionEvent;

fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest()) // prevents blurry sprites
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_event::<CollisionEvent>()
        .add_system_set(
            SystemSet::new()
                .with_system(print_keyboard_event_system)
                .with_system(player_movement_system.before(check_for_collisions)),
        )
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("textures/rpg/chars/gabe/gabe-idle-run.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 7, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn_bundle(Camera2dBundle::default());

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..default()
        })
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
        .insert(Player {
            movement_speed: 500.0,
        });
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut player_transform = query.single_mut();
    let mut direction = 0.0;

    if keyboard_input.pressed(KeyCode::Left) {
        direction -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        direction += 1.0;
    }

    // Calculate the new horizontal paddle position based on player input
    let new_player_position = player_transform.translation.x + direction;

    // Update the paddle position,
    // making sure it doesn't cause the paddle to leave the arena
    // let left_bound = LEFT_WALL + WALL_THICKNESS / 2.0 + PADDLE_SIZE.x / 2.0 + PADDLE_PADDING;
    // let right_bound = RIGHT_WALL - WALL_THICKNESS / 2.0 - PADDLE_SIZE.x / 2.0 - PADDLE_PADDING;

    player_transform.translation.x = new_player_position
    // .clamp(left_bound, right_bound);
}

fn check_for_collisions(
    mut commands: Commands,
    // mut ball_query: Query<(&mut Velocity, &Transform), With<Ball>>,
    // collider_query: Query<(Entity, &Transform, Option<&Brick>), With<Collider>>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    // let (mut ball_velocity, ball_transform) = ball_query.single_mut();
    // let ball_size = ball_transform.scale.truncate();
    //
    // // check collision with walls
    // for (collider_entity, transform, maybe_brick) in &collider_query {
    //     let collision = collide(
    //         ball_transform.translation,
    //         ball_size,
    //         transform.translation,
    //         transform.scale.truncate(),
    //     );
    //     if let Some(collision) = collision {
    //         // Sends a collision event so that other systems can react to the collision
    //         collision_events.send_default();
    //
    //         // Bricks should be despawned and increment the scoreboard on collision
    //         if maybe_brick.is_some() {
    //             scoreboard.score += 1;
    //             commands.entity(collider_entity).despawn();
    //         }
    //
    //         // reflect the ball when it collides
    //         let mut reflect_x = false;
    //         let mut reflect_y = false;
    //
    //         // only reflect if the ball's velocity is going in the opposite direction of the
    //         // collision
    //         match collision {
    //             Collision::Left => reflect_x = ball_velocity.x > 0.0,
    //             Collision::Right => reflect_x = ball_velocity.x < 0.0,
    //             Collision::Top => reflect_y = ball_velocity.y < 0.0,
    //             Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
    //             Collision::Inside => { /* do nothing */ }
    //         }
    //
    //         // reflect velocity on the x-axis if we hit something on the x-axis
    //         if reflect_x {
    //             ball_velocity.x = -ball_velocity.x;
    //         }
    //
    //         // reflect velocity on the y-axis if we hit something on the y-axis
    //         if reflect_y {
    //             ball_velocity.y = -ball_velocity.y;
    //         }
    //     }
    // }
}

fn print_keyboard_event_system(mut keyboard_input_events: EventReader<KeyboardInput>) {
    keyboard_input_events
        .iter()
        .for_each(|event| info!("{:?}", event))
}
