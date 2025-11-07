use crate::utilities::euler_angle::EulerAngle;
use bevy::prelude::*;

pub struct DesiredRotationPlugin;

impl Plugin for DesiredRotationPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_set_desired_rotation);
    }
}

#[derive(Component, Copy, Clone, Debug, Default)]
pub struct DesiredRotation {
    pub rotation: EulerAngle,
    pub rotation_type: RotationType,
}

#[derive(EntityEvent, Copy, Clone)]
pub struct SetDesiredRotation {
    pub entity: Entity,
    pub desired_rotation: Option<DesiredRotation>,
}

#[derive(Debug, Copy, Clone, Default)]
pub enum RotationType {
    /// Update the rotation value to the given rotation.
    #[expect(unused)]
    AbsoluteRotation,
    /// Add the given rotation to the current rotation value.
    #[default]
    DeltaRotation,
}

fn on_set_desired_rotation(set_desired_rotation: On<SetDesiredRotation>, mut commands: Commands) {
    let z_rotation = set_desired_rotation
        .desired_rotation
        .map(|desired_rotation| desired_rotation.rotation.z.radians())
        .unwrap_or(0.0);

    assert_eq!(
        0.0, z_rotation,
        "DesiredRotation is not allowed to contain roll (Z axis). Given z value: {}",
        z_rotation
    );

    let mut entity_commands = commands.entity(set_desired_rotation.entity);

    match set_desired_rotation.desired_rotation {
        Some(desired_rotation) => entity_commands.insert(desired_rotation),
        None => entity_commands.remove::<DesiredRotation>(),
    };
}
