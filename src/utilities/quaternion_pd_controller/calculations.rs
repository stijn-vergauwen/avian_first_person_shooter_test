use bevy::math::{Quat, Vec3};

use crate::utilities::{
    pd_controller::config::PdControllerConfig,
    quaternion_pd_controller::values::QuaternionPdControllerValues,
};

pub fn calculate_position(values: QuaternionPdControllerValues, delta_seconds: f32) -> Quat {
    values.position * Quat::from_scaled_axis(values.velocity * delta_seconds)
}

pub fn calculate_velocity(values: QuaternionPdControllerValues, delta_seconds: f32) -> Vec3 {
    values.velocity + values.acceleration * delta_seconds
}

pub fn calculate_acceleration(
    values: QuaternionPdControllerValues,
    config: PdControllerConfig,
    mut target_position: Quat,
    target_velocity: Vec3,
) -> Vec3 {
    // This check prevents a spike in acceleration when either the target_position (player rotation) or values.position (object rotation) flips sign, without needing to mutate those values themselves (target_position is only mutated within this fn).
    // A 'flip in sign' refers to a quaternion that points to the same angle but takes a different route, so it's inner values are different. This would be seen as a very big delta in the calculation below even though the resulting rotation is the same.
    // The target_velocity calculation also needed this same check to prevent that one from spiking, these checks together should keep the quaternion pd controller stable.
    if values.position.dot(target_position) < 0.0 {
        target_position = -target_position;
    }

    (target_position
        * Quat::from_scaled_axis(target_velocity * config.target_velocity_multiplier)
        * values.position.inverse()
        * Quat::from_scaled_axis(values.velocity * config.velocity_multiplier).inverse())
    .to_scaled_axis()
        / config.acceleration_divisor
}
