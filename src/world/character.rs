use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{utilities::system_sets::DataSystems, world::desired_movement::DesiredMovement};

const MAX_MOVEMENT_STRENGTH: f32 = 10.0;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            update_movement_force.in_set(DataSystems::UpdateEntities),
        );
    }
}

#[derive(Component, Copy, Clone, Debug)]
pub struct Character {
    /// If this Character is currently active / controllable.
    pub is_active: bool,
}

fn update_movement_force(
    mut characters_query: Query<(
        &Character,
        &GlobalTransform,
        Option<&DesiredMovement>,
        &mut ConstantForce,
    )>,
) {
    for (character, global_transform, desired_movement, mut force) in characters_query.iter_mut() {
        if !character.is_active {
            continue;
        }

        let desired_movement_force = match desired_movement {
            Some(desired_movement) => {
                let desired_movement_strength =
                    MAX_MOVEMENT_STRENGTH * desired_movement.fraction_of_max_strength.value();

                global_transform.rotation()
                    * (desired_movement.direction * desired_movement_strength)
            }
            None => Vec3::ZERO,
        };

        force.0 = desired_movement_force;
    }
}
