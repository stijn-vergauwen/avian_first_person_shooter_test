use std::time::Duration;

use bevy::{asset::Asset, math::Vec3, reflect::Reflect};
use serde::{Deserialize, Serialize};

use crate::utilities::euler_angle::EulerAngle;

#[derive(Asset, Reflect, Debug, Deserialize, Serialize, Clone)]
pub struct WeaponConfig {
    pub path_to_model: String,
    pub collider_size: Vec3,
    pub weight: f32,
    pub recoil: f32,
    pub bullet_speed: f32,
    pub bullet_impact_force: f32,
    pub firing_type: FiringType,
    /// The position relative to the weapon where shots should be fired from.
    pub shot_origin: Vec3,
    /// How the weapon should be held relative to the player camera when aiming down sight.
    pub ads_position: Vec3,
    /// The position relative to the weapon where shells should be ejected from.
    pub shell_ejection_position: Vec3,
    /// The rotation relative to the weapon where shells should be ejected from.
    pub shell_ejection_rotation: EulerAngle,
    /// The spin (Angular velocity) along each axis that ejected shells should have.
    pub shell_ejection_force: f32,
    /// The spin (Angular velocity) along each axis that ejected shells should have.
    pub shell_ejection_spin: Vec3,
}

#[derive(Reflect, Debug, Deserialize, Serialize, Clone, Copy)]
pub enum FiringType {
    SemiAutomatic,
    Automatic(SecondsBetweenShots),
}

#[derive(Reflect, Debug, Deserialize, Serialize, Clone, Copy)]
pub struct SecondsBetweenShots(pub f32);

impl SecondsBetweenShots {
    pub fn as_duration(&self) -> Duration {
        Duration::from_secs_f32(self.0)
    }
}
