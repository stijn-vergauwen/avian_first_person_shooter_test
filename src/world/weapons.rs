mod bullet;
pub mod muzzle_flash;
pub mod shooting;
mod spawner;
pub mod weapon_config;
mod weapon_config_loader;
mod weapon_config_save;

use bevy::prelude::*;
use bullet::BulletPlugin;
use muzzle_flash::MuzzleFlashPlugin;
use shooting::WeaponShootingPlugin;
use spawner::WeaponSpawnerPlugin;
use weapon_config::WeaponConfig;
use weapon_config_loader::WeaponConfigLoader;
use weapon_config_save::WeaponConfigSavePlugin;

pub struct WeaponsPlugin;

impl Plugin for WeaponsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            WeaponShootingPlugin,
            MuzzleFlashPlugin,
            BulletPlugin,
            WeaponConfigSavePlugin,
            WeaponSpawnerPlugin,
        ))
        .init_asset::<WeaponConfig>()
        .init_asset_loader::<WeaponConfigLoader>();
    }
}

/// Base component for weapons.
#[derive(Component, Clone)]
pub struct Weapon {
    config: Handle<WeaponConfig>,
    trigger_is_pulled: bool,
}

impl Weapon {
    pub fn new(config: Handle<WeaponConfig>) -> Self {
        Self {
            config,
            trigger_is_pulled: false,
        }
    }

    pub fn config(&self) -> &Handle<WeaponConfig> {
        &self.config
    }
}
