use bevy::prelude::*;

pub struct WeaponsPlugin;

impl Plugin for WeaponsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_test_weapon);
    }
}

/// Marker component for weapons.
#[derive(Component, Clone, Copy)]
struct Weapon;

fn spawn_test_weapon(mut commands: Commands, asset_server: Res<AssetServer>) {
    let weapon_model = asset_server.load("models/Blocky assault rifle.glb#Scene0");

    commands.spawn((
        Weapon,
        Transform::from_xyz(0.0, 1.0, 0.0),
        SceneRoot(weapon_model),
    ));
}
