use avian3d::prelude::*;
use bevy::prelude::*;

use crate::world::{character::Character, grounded::Grounded};

pub struct CharacterJumpPlugin;

impl Plugin for CharacterJumpPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_attempt_jump);
    }
}

#[derive(EntityEvent, Clone, Copy)]
pub struct AttemptJump {
    pub entity: Entity,
    pub jump_force: f32,
}

fn on_attempt_jump(
    jump_attempt: On<AttemptJump>,
    mut character_force_query: Query<(&Grounded, Forces), With<Character>>,
) {
    let (grounded, mut character_force) = character_force_query
        .get_mut(jump_attempt.entity)
        .expect("AttemptJump should always point to existing entity with RigidBody component.");

    if grounded.is_grounded() {
        character_force.apply_force(Vec3::Y * jump_attempt.jump_force);
    }
}
