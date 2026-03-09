use bevy::{asset::Asset, math::Vec3, reflect::Reflect};
use serde::{Deserialize, Serialize};

#[derive(Asset, Reflect, Debug, Deserialize, Serialize, Clone)]
pub struct WeaponConfig {
    pub path_to_model: String,
    pub collider_size: Vec3,
    pub weight: f32,
    pub recoil: f32,
    pub bullet_speed: f32,
    pub bullet_impact_force: f32,
}
