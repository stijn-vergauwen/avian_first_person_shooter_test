use std::ops::Range;

use crate::utilities::{angle::Angle, euler_angle::EulerAngle};
use bevy::prelude::*;

const VERTICAL_ROTATION_RANGE: Range<Angle> = Range {
    start: Angle::from_degrees(-80.0),
    end: Angle::from_degrees(80.0),
};

pub struct DesiredRotationPlugin;

impl Plugin for DesiredRotationPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_set_desired_rotation);
    }
}

#[derive(Component, Copy, Clone, Debug, Default, PartialEq)]
pub struct DesiredRotation {
    pub rotation: EulerAngle,
    pub rotation_type: RotationType,
}

#[derive(EntityEvent, Copy, Clone)]
pub struct SetDesiredRotation {
    pub entity: Entity,
    pub desired_rotation: DesiredRotation,
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub enum RotationType {
    /// Update the rotation value to the given rotation.
    #[expect(unused)]
    AbsoluteRotation,
    /// Add the given rotation to the current rotation value.
    #[default]
    DeltaRotation,
}

fn on_set_desired_rotation(
    set_desired_rotation: On<SetDesiredRotation>,
    mut desired_rotation_query: Query<&mut DesiredRotation>,
) {
    let z_rotation = set_desired_rotation.desired_rotation.rotation.z.radians();

    assert_eq!(
        0.0, z_rotation,
        "DesiredRotation is not allowed to contain roll (Z axis). Given z value: {}",
        z_rotation
    );

    let mut desired_rotation = desired_rotation_query
        .get_mut(set_desired_rotation.entity)
        .expect("SetDesiredRotation should always point to existing entity with DesiredRotation component.");

    let mut new_rotation = match set_desired_rotation.desired_rotation.rotation_type {
        RotationType::AbsoluteRotation => set_desired_rotation.desired_rotation.rotation,
        RotationType::DeltaRotation => {
            desired_rotation.rotation + set_desired_rotation.desired_rotation.rotation
        }
    };

    new_rotation.x = new_rotation.x.clamp(&VERTICAL_ROTATION_RANGE);

    desired_rotation.rotation = new_rotation;
}
