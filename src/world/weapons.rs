mod bullet;
pub mod muzzle_flash;
mod shooting;
mod weapon_config;
mod weapon_config_loader;
mod weapon_config_saver;

use std::time::Duration;

use avian3d::prelude::*;
use bevy::{asset::{AssetPath, saver::AssetSaver}, light::NotShadowCaster, prelude::*};
use weapon_config::WeaponConfig;
use weapon_config_loader::WeaponConfigLoader;
use weapon_config_saver::WeaponConfigSaver;

use crate::{
    utilities::DrawGizmos,
    world::{
        TABLE_POSITION,
        grabbable_object::GrabbableObject,
        weapons::{
            bullet::BulletPlugin,
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
        app.add_plugins((WeaponShootingPlugin, MuzzleFlashPlugin, BulletPlugin))
            .init_asset::<WeaponConfig>()
            .init_asset_loader::<WeaponConfigLoader>()
            .add_systems(Startup, (spawn_test_weapon, save_test_weapon_config));
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

fn save_test_weapon_config(
    mut weapon_configs: ResMut<Assets<WeaponConfig>>,
    asset_server: Res<AssetServer>,
) {
    // Code from Grok, doesn't work, it tried to use an 'AssetIo' resource which doesn't exist
    // let path = "assets/my_weapon.ron";  // Relative to your asset folder
    // let weapon_config = weapon_configs.add(WeaponConfig {path_to_model: String::from("models/Blocky assault rifle.glb#Scene0")});



    // let mut writer = futures_lite::future::block_on(asset_io.create_writer(path)).unwrap();  // Async, but block for simplicity in sync system
    // let saver = WeaponConfigSaver::default();
    // let saved_asset = SavedAsset::from(weapon_config);  // For simple assets without sub-handles

    // futures_lite::future::block_on(saver.save(&mut writer, saved_asset, &())).unwrap();  // Block on async save
}