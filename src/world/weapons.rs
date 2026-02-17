mod shooting;

use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{
    utilities::DrawGizmos,
    world::{
        grabbable_object::{GrabOrientation, GrabbableObject},
        weapons::shooting::WeaponShootingPlugin,
    },
};

pub struct WeaponsPlugin;

impl Plugin for WeaponsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WeaponShootingPlugin)
            .add_systems(Startup, spawn_test_weapon);
    }
}

/// Base component for weapons.
#[derive(Component, Clone, Copy)]
pub struct Weapon;

#[derive(EntityEvent, Clone, Copy)]
pub struct ShootWeapon {
    pub entity: Entity,
}

fn spawn_test_weapon(mut commands: Commands, asset_server: Res<AssetServer>) {
    let weapon_model = asset_server.load("models/Blocky assault rifle.glb#Scene0");

    commands.spawn((
        Weapon,
        GrabbableObject,
        GrabOrientation::IDENTITY,
        SceneRoot(weapon_model),
        Transform::from_xyz(0.0, 1.0, 0.0),
        RigidBody::Dynamic,
        Collider::cuboid(0.08, 0.16, 0.6),
        Mass(4.0),
        DrawGizmos,
        MaxAngularSpeed(40.0),
    ));
}
