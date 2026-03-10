use bevy::prelude::*;

pub struct DesiredMovementPlugin;

impl Plugin for DesiredMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_set_desired_movement);
    }
}

/// Describes a desired movement velocity.
#[derive(Component, Copy, Clone, Debug, PartialEq, Default)]
pub struct DesiredMovement {
    pub velocity: Vec3,
}

#[derive(EntityEvent, Copy, Clone)]
pub struct SetDesiredMovement {
    pub entity: Entity,
    pub desired_movement: DesiredMovement,
}

fn on_set_desired_movement(
    set_desired_movement: On<SetDesiredMovement>,
    mut desired_movement_query: Query<&mut DesiredMovement>,
) {
    let mut desired_movement = desired_movement_query
        .get_mut(set_desired_movement.entity)
        .expect("SetDesiredMovement should always point to existing entity with DesiredMovement component.");

    *desired_movement = set_desired_movement.desired_movement;
}
