use bevy::{asset::Asset, reflect::Reflect};
use serde::{Deserialize, Serialize};

#[derive(Asset, Reflect, Debug, Deserialize, Serialize, Clone)]
pub struct WeaponConfig {
    pub path_to_model: String,
}