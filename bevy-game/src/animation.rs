use std::collections::HashMap;

use bevy::{prelude::*, reflect::TypeUuid};
use serde::Deserialize;

/// Tag for basic (1 row) animation
#[derive(Component)]
pub struct BasicAnimationComponent;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

/// Animate basic (1 row) animations
pub fn basic_sprite_animation_system(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
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

/// Animate a characters (people, includes player)
pub fn animate_character_system(
    time: Res<Time>,
    character_animations: Res<CharacterAnimationResource>,
    mut animation_query: Query<(&mut CharacterAnimationComponent, &mut TextureAtlasSprite)>,
) {
    for (mut character_animation, mut sprite) in animation_query.iter_mut() {
        character_animation.timer.tick(time.delta());

        if character_animation.timer.finished() {
            let animation_idxs =
                character_animations.animations[&character_animation.animation_type];
            if sprite.index == animation_idxs.1 as usize {
                sprite.index = animation_idxs.0 as usize;
            } else {
                sprite.index += 1;
            }
        }
    }
}

/// Stores data about character animations frames (data/character_animations.ron)
#[derive(Default, Debug, Deserialize, TypeUuid)]
#[uuid = "39cadc56-aa9c-4543-8640-a018b74b5052"]
pub struct CharacterAnimationResource {
    // start and end indexes of animations
    pub animations: HashMap<CharacterAnimationType, (u32, u32, f32)>,
}

/// Used for tracking animations of a character entity
#[derive(Component)]
pub struct CharacterAnimationComponent {
    pub timer: AnimationTimer,
    pub animation_type: CharacterAnimationType,
}

/// Types of character animations
#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
pub enum CharacterAnimationType {
    ForwardIdle,
    ForwardMove,
    LeftIdle,
    LeftMove,
    RightIdle,
    RightMove,
    BackwardIdle,
    BackwardMove,
}

impl CharacterAnimationType {
    fn is_idle(&self) -> bool {
        matches!(
            self,
            CharacterAnimationType::ForwardIdle
                | CharacterAnimationType::BackwardIdle
                | CharacterAnimationType::LeftIdle
                | CharacterAnimationType::RightIdle
        )
    }
}

