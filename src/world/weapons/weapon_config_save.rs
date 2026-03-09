use std::fs;

use bevy::prelude::*;
use ron::ser::PrettyConfig;

use super::weapon_config::WeaponConfig;

pub struct WeaponConfigSavePlugin;

impl Plugin for WeaponConfigSavePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_save_weapon_config);
    }
}

#[derive(Event)]
pub struct SaveWeaponConfig {
    pub weapon_config: WeaponConfig,
    pub file_name: String,
}

fn on_save_weapon_config(event: On<SaveWeaponConfig>) {
    if fs::exists("assets/weapons").is_ok_and(|exists| !exists) {
        fs::create_dir("assets/weapons").unwrap();
    }

    let serialized = ron::ser::to_string_pretty(
        &event.weapon_config,
        PrettyConfig::default().struct_names(true),
    )
    .unwrap();

    let path = format!("assets/weapons/{}.ron", event.file_name);

    fs::write(path, serialized).unwrap();
}
