use std::{
    fmt::Debug,
    ops::{Add, Div, Mul, Sub},
};

use super::{config::PdControllerConfig, values::PdControllerValues};

pub fn calculate_position<T>(values: PdControllerValues<T>, delta_seconds: f32) -> T
where
    T: Copy + Default + Debug + Mul<f32, Output = T> + Add<T, Output = T>,
{
    values.position + values.velocity * delta_seconds
}

pub fn calculate_velocity<T>(values: PdControllerValues<T>, delta_seconds: f32) -> T
where
    T: Copy + Default + Debug + Mul<f32, Output = T> + Add<T, Output = T>,
{
    values.velocity + values.acceleration * delta_seconds
}

pub fn calculate_acceleration<T>(
    values: PdControllerValues<T>,
    config: PdControllerConfig,
    target_position: T,
    target_velocity: T,
) -> T
where
    T: Copy
        + Default
        + Debug
        + Mul<f32, Output = T>
        + Add<T, Output = T>
        + Sub<T, Output = T>
        + Div<f32, Output = T>,
{
    (target_position + target_velocity * config.target_velocity_multiplier
        - values.position
        - values.velocity * config.velocity_multiplier)
        / config.acceleration_divisor
}
