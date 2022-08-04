use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]

pub(crate) struct Velocity(Vec2);

struct Gravity(pub f32);

struct AffectedByGravity;

struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        // app.add_system(gravity_system);
    }
}

fn gravity_system(gravity: Res<Gravity>, time: Res<Time>, mut velocity: Mut<Velocity>) {
    velocity.0.y -= gravity.0 * time.delta_seconds();
}
