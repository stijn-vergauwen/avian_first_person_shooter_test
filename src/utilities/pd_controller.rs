mod calculations;
pub mod config;
mod constrain_value;
mod values;

use std::{
    fmt::Debug,
    marker::Copy,
    ops::{Add, Div, Mul, Sub},
};

use calculations::{calculate_acceleration, calculate_position, calculate_velocity};
use config::PdControllerConfig;
use constrain_value::ConstrainValue;
use values::PdControllerValues;

// PD controller code taken from 2D IK test project.

#[derive(Default, Debug, Clone)]
pub struct PdController<T>
where
    T: Copy + Default + Debug,
{
    config: PdControllerConfig,
    values: PdControllerValues<T>,
    target_position: T,
    prev_target_position: T,
}

    #[allow(unused)]
impl<T> PdController<T>
where
    T: Copy
        + Default
        + Debug
        + Mul<f32, Output = T>
        + Add<T, Output = T>
        + Sub<T, Output = T>
        + Div<f32, Output = T>
        + ConstrainValue,
{
    pub fn new(config: PdControllerConfig) -> Self {
        Self {
            config,
            ..Default::default()
        }
    }

    pub fn with_start_position(config: PdControllerConfig, start_position: T) -> Self {
        let values = PdControllerValues::<T> {
            position: start_position,
            ..Default::default()
        };

        Self {
            config,
            values,
            target_position: start_position,
            prev_target_position: start_position,
        }
    }

    /// Sets a new target position for the controller to move towards
    pub fn set_target_position(&mut self, target_position: T) {
        self.target_position = target_position;
    }

    /// Returns the controllers current 'position' value.
    #[expect(unused)]
    pub fn position(&self) -> T {
        self.values.position
    }

    /// Returns the controllers current 'velocity' value.
    pub fn velocity(&self) -> T {
        self.values.velocity
    }

    /// Sets the controllers current 'position' value.
    ///
    /// Use this before each controller update call when the in-game 'position' value might deviate from what the controller calculated last update cycle.
    pub fn set_position(&mut self, position: T) {
        self.values.position = position;
    }

    /// Sets the controllers current 'velocity' value.
    ///
    /// Use this before each controller update call when the in-game 'velocity' value might deviate from what the controller calculated last update cycle.
    pub fn set_velocity(&mut self, velocity: T) {
        self.values.velocity = velocity;
    }

    /// Updates the values of this controller.
    pub fn update(&mut self, delta_seconds: f32) {
        let target_velocity = self.calculate_target_velocity(delta_seconds);
        self.update_previous_values();

        self.update_acceleration(delta_seconds, target_velocity);
        self.update_velocity(delta_seconds);
        self.update_position(delta_seconds);
    }

    /// Updates the prev_target_position field of this controller.
    pub fn update_previous_values(&mut self) {
        self.prev_target_position = self.target_position;
    }

    /// Updates the acceleration value of this controller.
    pub fn update_acceleration(&mut self, delta_seconds: f32, target_velocity: T) {
        validate_delta_seconds(delta_seconds);

        self.values.acceleration = calculate_acceleration(
            self.values,
            self.config,
            self.target_position,
            target_velocity,
        )
        .constrain(self.config.max_acceleration);
    }

    /// Updates the velocity value of this controller.
    pub fn update_velocity(&mut self, delta_seconds: f32) {
        validate_delta_seconds(delta_seconds);

        self.values.velocity =
            calculate_velocity(self.values, delta_seconds).constrain(self.config.max_velocity);
    }

    /// Updates the position value of this controller.
    pub fn update_position(&mut self, delta_seconds: f32) {
        validate_delta_seconds(delta_seconds);

        self.values.position = calculate_position(self.values, delta_seconds);
    }

    fn calculate_target_velocity(&self, delta_seconds: f32) -> T {
        (self.target_position - self.prev_target_position) / delta_seconds
    }
}

fn validate_delta_seconds(delta_seconds: f32) {
    assert!(
        delta_seconds > 0.0,
        "Delta seconds must be higher than zero!"
    );
}
