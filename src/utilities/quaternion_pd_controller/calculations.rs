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
    target_position: Quat,
    target_velocity: Vec3,
) -> Vec3 {
    (target_position
        * Quat::from_scaled_axis(target_velocity * config.target_velocity_multiplier)
        * values.position.inverse()
        * Quat::from_scaled_axis(values.velocity * config.velocity_multiplier).inverse())
    .to_scaled_axis()
        / config.acceleration_divisor
}
