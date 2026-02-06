use std::fmt::Debug;

use bevy::math::{Quat, Vec3};

#[derive(Default, Debug, Clone, Copy)]
pub struct QuaternionPdControllerValues {
    pub position: Quat,
    pub velocity: Vec3,
    pub acceleration: Vec3,
}
