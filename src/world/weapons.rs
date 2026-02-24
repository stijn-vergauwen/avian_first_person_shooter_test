pub mod muzzle_flash;
mod shooting;

use std::time::Duration;

use avian3d::prelude::*;
use bevy::{light::NotShadowCaster, prelude::*};

use crate::{
    utilities::DrawGizmos,
    world::{
        TABLE_POSITION,
        grabbable_object::{GrabOrientation, GrabbableObject},
        weapons::{
            muzzle_flash::{MuzzleFlashAnimation, MuzzleFlashImages, MuzzleFlashPlugin},
            shooting::WeaponShootingPlugin,
        },
    },
};

const MUZZLE_FLASH_DURATION: Duration = Duration::from_millis(40);
const MUZZLE_FLASH_SIZE: f32 = 0.8;

pub struct WeaponsPlugin;

impl Plugin for WeaponsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((WeaponShootingPlugin, MuzzleFlashPlugin))
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

fn spawn_test_weapon(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    muzzle_flash_images: Res<MuzzleFlashImages>,
) {
    let weapon_model = asset_server.load("models/Blocky assault rifle.glb#Scene0");

    let muzzle_flash_mesh_handle = meshes.add(Rectangle::from_length(MUZZLE_FLASH_SIZE));
    let muzzle_flash_material_handle = materials.add(StandardMaterial {
        base_color: Color::WHITE.with_alpha(0.5),
        base_color_texture: Some(muzzle_flash_images.get_image_at_index(0)),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        cull_mode: None,
        ..default()
    });

    commands
        .spawn((
            Weapon,
            GrabbableObject,
            GrabOrientation::IDENTITY,
            SceneRoot(weapon_model),
            Transform {
                translation: TABLE_POSITION + Vec3::new(0.0, 0.5, 3.0),
                rotation: Quat::from_euler(
                    EulerRot::YXZ,
                    10f32.to_radians(),
                    0.0,
                    90f32.to_radians(),
                ),
                ..default()
            },
            RigidBody::Dynamic,
            Collider::cuboid(0.08, 0.14, 0.6),
            Mass(4.0),
            DrawGizmos,
            MaxAngularSpeed(40.0),
        ))
        .with_child((
            MuzzleFlashAnimation::new(MUZZLE_FLASH_DURATION),
            Mesh3d(muzzle_flash_mesh_handle.clone()),
            MeshMaterial3d(muzzle_flash_material_handle),
            Transform::from_xyz(0.0, 0.0, -0.6),
            Visibility::Hidden,
            NotShadowCaster,
        ));
}
