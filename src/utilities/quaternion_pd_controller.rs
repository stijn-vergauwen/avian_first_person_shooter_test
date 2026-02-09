mod calculations;
mod values;

use std::fmt::Debug;

use bevy::math::{Quat, Vec3};

use crate::utilities::{
    pd_controller::config::PdControllerConfig,
    quaternion_pd_controller::{
        calculations::{calculate_acceleration, calculate_position, calculate_velocity},
        values::QuaternionPdControllerValues,
    },
};

#[derive(Default, Debug, Clone)]
pub struct QuaternionPdController {
    config: PdControllerConfig,
    values: QuaternionPdControllerValues,
    target_position: Quat,
    prev_target_position: Quat,
}

#[allow(unused)]
impl QuaternionPdController {
    pub fn new(config: PdControllerConfig) -> Self {
        Self {
            config,
            ..Default::default()
        }
    }

    pub fn with_start_position(config: PdControllerConfig, start_position: Quat) -> Self {
        let values = QuaternionPdControllerValues {
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

    /// Returns the controllers current 'position' value.
    pub fn position(&self) -> Quat {
        self.values.position
    }

    /// Returns the controllers current 'velocity' value.
    pub fn velocity(&self) -> Vec3 {
        self.values.velocity
    }

    /// Returns the controllers current 'acceleration' value.
    pub fn acceleration(&self) -> Vec3 {
        self.values.acceleration
    }

    /// Returns the controllers current target position.
    pub fn target_position(&self) -> Quat {
        self.target_position
    }

    /// Sets the controllers current 'position' value.
    ///
    /// Use this before each controller update call when the in-game 'position' value might deviate from what the controller calculated last update cycle.
    pub fn set_position(&mut self, position: Quat) {
        self.values.position = position;
    }

    /// Sets the controllers current 'velocity' value.
    ///
    /// Use this before each controller update call when the in-game 'velocity' value might deviate from what the controller calculated last update cycle.
    pub fn set_velocity(&mut self, velocity: Vec3) {
        self.values.velocity = velocity;
    }

    /// Sets a new target position for the controller to move towards.
    ///
    /// If you want the controller to take the shortest path towards the target, use the `set_shortest_target_position` method instead.
    pub fn set_target_position(&mut self, mut target_position: Quat) {
        self.target_position = target_position;
    }

    /// Sets a new start position for the controller. This resets the controller to a new position without it reacting to a target velocity spike.
    pub fn set_start_position(&mut self, start_position: Quat) {
        self.target_position = start_position;
        self.prev_target_position = start_position;
    }

    /// Updates the values of this controller.
    pub fn update(&mut self, delta_seconds: f32) {
        let target_velocity = self.calculate_target_velocity(delta_seconds);
        self.update_previous_values();

        self.update_acceleration(delta_seconds, target_velocity);
        self.update_velocity(delta_seconds);
        self.update_position(delta_seconds);
    }

    /// Updates the position & velocity values of this controller to the current physics simulation state and returns the new acceleration value.
    pub fn update_from_physics_sim(
        &mut self,
        position: Quat,
        velocity: Vec3,
        delta_seconds: f32,
    ) -> Vec3 {
        self.set_position(position);
        self.set_velocity(velocity);

        let mut target_velocity = self.calculate_target_velocity(delta_seconds);
        self.update_previous_values();

        self.update_acceleration(delta_seconds, target_velocity);

        self.values.acceleration
    }

    /// Updates the prev_target_position field of this controller.
    pub fn update_previous_values(&mut self) {
        self.prev_target_position = self.target_position;
    }

    /// Updates the acceleration value of this controller.
    pub fn update_acceleration(&mut self, delta_seconds: f32, target_velocity: Vec3) {
        validate_delta_seconds(delta_seconds);

        self.values.acceleration = calculate_acceleration(
            self.values,
            self.config,
            self.target_position,
            target_velocity,
        );
    }

    /// Updates the velocity value of this controller.
    pub fn update_velocity(&mut self, delta_seconds: f32) {
        validate_delta_seconds(delta_seconds);

        self.values.velocity = calculate_velocity(self.values, delta_seconds);
    }

    /// Updates the position value of this controller.
    pub fn update_position(&mut self, delta_seconds: f32) {
        validate_delta_seconds(delta_seconds);

        self.values.position = calculate_position(self.values, delta_seconds);
    }

    fn calculate_target_velocity(&self, delta_seconds: f32) -> Vec3 {
        // This check prevents a spike in target_velocity when the target_position (player rotation) flips sign, without needing to mutate the player rotation itself.
        // A 'flip in sign' refers to a quaternion that points to the same angle but takes a different route, so it's inner values are different. This would be seen as a very big delta in the calculation below even though the resulting rotation is the same.
        // The acceleration calculation also needed this same check to prevent that one from spiking, these checks together should keep the quaternion pd controller stable.
        let target_position = match self.prev_target_position.dot(self.target_position) > 0.0 {
            true => self.target_position,
            false => -self.target_position,
        };

        (target_position * self.prev_target_position.inverse()).to_scaled_axis() / delta_seconds
    }
}

fn validate_delta_seconds(delta_seconds: f32) {
    assert!(
        delta_seconds > 0.0,
        "Delta seconds must be higher than zero!"
    );
}
