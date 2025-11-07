use crate::utilities::fraction::Fraction;
use bevy::math::Dir3;
use bevy::prelude::*;

pub struct DesiredMovementPlugin;

impl Plugin for DesiredMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_set_desired_movement);
    }
}

/// Describes a desired movement direction and strength.
#[derive(Component, Copy, Clone, Debug, PartialEq)]
pub struct DesiredMovement {
    pub direction: Dir3,
    pub fraction_of_max_strength: Fraction,
}

impl Default for DesiredMovement {
    fn default() -> Self {
        Self {
            direction: Dir3::NEG_Z,
            fraction_of_max_strength: Fraction::new(0.0),
        }
    }
}

#[derive(EntityEvent, Copy, Clone)]
pub struct SetDesiredMovement {
    pub entity: Entity,
    pub desired_movement: Option<DesiredMovement>,
}

fn on_set_desired_movement(set_desired_movement: On<SetDesiredMovement>, mut commands: Commands) {
    let mut entity_commands = commands.entity(set_desired_movement.entity);

    match set_desired_movement.desired_movement {
        Some(desired_movement) => entity_commands.insert(desired_movement),
        None => entity_commands.remove::<DesiredMovement>(),
    };
}
