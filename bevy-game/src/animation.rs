use std::collections::HashMap;

use bevy::{prelude::*, reflect::TypeUuid};
use serde::Deserialize;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

/// Stores data about character animations frames (data/player.ron)
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

