use std::f32::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct PdControllerConfig {
    pub velocity_multiplier: f32,
    pub acceleration_divisor: f32,
    pub target_velocity_multiplier: f32,
    /// Max velocity in m/s.
    pub max_velocity: f32,
    /// Max acceleration in m/s/s.
    pub max_acceleration: f32,
}

impl PdControllerConfig {
    /// Build a new PdControllerConfig from more intuitive parameters.
    ///
    /// ## Parameter explanation
    /// - `speed` describes how quickly the controller tries to reach the target position.
    ///     - `speed` equals the frequency of the controller in hertz, you can see this more clearly when `damping` is zero (gives a 'wave' motion).
    ///     - Practically, `speed` increases acceleration and reduces the effects of dampening (Not yet sure if this is a useful description).
    /// - `damping` describes how tightly the controller tries to follow the target velocity.
    ///     - `damping` increases the effect of `initial_response`, since it tries to follow the initial response more tightly.
    /// - `initial_response` describes how the controller reacts when the target changes.
    ///     - This is like a 'jump' towards or away from the target, set to 0.0 to disable.
    ///     
    /// ## Tuning
    /// It's easiest to start off with `damping` and `initial_response` both set to 0.0.  
    /// Set the speed to 1.0, adjust it until the reaction speed or force looks good, then adjust the others.
    ///
    /// Usual ranges:
    /// - `speed`: 0.5 to 2.0, always above 0.0.
    /// - `damping`: 0.0 to 1.0.
    ///     - Below 0.0 is unstable, above 1.0 can be slow, too high is also unstable.
    /// - `initial_response`: -1.0 to 1.0, usually 0.0 (disabled, smooth response).
    ///     - Positive values jump towards the target, negative values jump away from the target.
    ///
    pub fn from_parameters(speed: f32, damping: f32, initial_response: f32) -> Self {
        Self {
            velocity_multiplier: damping / (PI * speed),
            acceleration_divisor: 1.0 / (PI * 2.0 * speed).powf(2.0),
            target_velocity_multiplier: (initial_response * damping) / (PI * 2.0 * speed),
            max_velocity: f32::MAX,
            max_acceleration: f32::MAX,
        }
    }

    #[allow(unused)]
    pub fn with_max_velocity(mut self, max_velocity: f32) -> Self {
        self.max_velocity = max_velocity;
        self
    }

    #[allow(unused)]
    pub fn with_max_acceleration(mut self, max_acceleration: f32) -> Self {
        self.max_acceleration = max_acceleration;
        self
    }
}

impl Default for PdControllerConfig {
    fn default() -> Self {
        Self::from_parameters(1.0, 1.0, 0.0)
    }
}
