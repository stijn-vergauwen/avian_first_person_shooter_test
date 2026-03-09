mod bullet;
pub mod muzzle_flash;
pub mod shooting;
mod weapon_config;
mod weapon_config_loader;
mod weapon_config_save;

use std::{fs, time::Duration};

use avian3d::prelude::*;
use bevy::{light::NotShadowCaster, prelude::*};
use shooting::AutomaticFire;
use weapon_config::{FiringType, SecondsBetweenShots, WeaponConfig};
use weapon_config_loader::WeaponConfigLoader;
use weapon_config_save::{SaveWeaponConfig, WeaponConfigSavePlugin};

use crate::world::{
    TABLE_POSITION,
    grabbable_object::GrabbableObject,
    weapons::{
        bullet::BulletPlugin,
        muzzle_flash::{MuzzleFlashAnimation, MuzzleFlashImages, MuzzleFlashPlugin},
        shooting::WeaponShootingPlugin,
    },
};

const MUZZLE_FLASH_DURATION: Duration = Duration::from_millis(40);
const MUZZLE_FLASH_SIZE: f32 = 0.8;

pub struct WeaponsPlugin;

impl Plugin for WeaponsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            WeaponShootingPlugin,
            MuzzleFlashPlugin,
            BulletPlugin,
            WeaponConfigSavePlugin,
        ))
        .init_asset::<WeaponConfig>()
        .init_asset_loader::<WeaponConfigLoader>()
        .add_systems(Startup, load_weapon_configs)
        .add_systems(Update, spawn_weapon_when_config_loaded)
        .add_observer(on_spawn_weapon);
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
}

#[derive(Resource)]
struct WeaponsToSpawn {
    weapon_config_handles: Vec<Handle<WeaponConfig>>,
}

#[derive(Event)]
struct SpawnWeapon {
    config: Handle<WeaponConfig>,
    transform: Transform,
}

fn load_weapon_configs(asset_server: Res<AssetServer>, mut commands: Commands) {
    let mut weapon_config_handles = Vec::new();

    if let Ok(read_dir) = fs::read_dir("assets/weapons") {
        for entry in read_dir.filter_map(|entry| entry.ok()) {
            let file_name = entry
                .file_name()
                .into_string()
                .expect("OsString of file name should always be convertable to String.");

            println!("Now loading: {}", file_name);

            let path = format!("weapons/{}", file_name);
            let weapon_config_handle = asset_server.load::<WeaponConfig>(path);

            weapon_config_handles.push(weapon_config_handle);
        }
    }

    commands.insert_resource(WeaponsToSpawn {
        weapon_config_handles,
    });
}

fn spawn_weapon_when_config_loaded(
    mut reader: MessageReader<AssetEvent<WeaponConfig>>,
    mut weapons_to_spawn: ResMut<WeaponsToSpawn>,
    mut commands: Commands,
) {
    for message in reader.read() {
        let AssetEvent::LoadedWithDependencies { id } = message else {
            continue;
        };

        let Some(index) = weapons_to_spawn
            .weapon_config_handles
            .iter()
            .position(|handle| handle.id() == *id)
        else {
            continue;
        };

        let config_handle = weapons_to_spawn.weapon_config_handles.swap_remove(index);
        let position_offset = Vec3::NEG_Z * weapons_to_spawn.weapon_config_handles.len() as f32;

        commands.trigger(SpawnWeapon {
            config: config_handle,
            transform: Transform {
                translation: TABLE_POSITION + Vec3::new(0.0, 0.5, 3.0) + position_offset,
                rotation: Quat::from_euler(
                    EulerRot::YXZ,
                    20f32.to_radians(),
                    0.0,
                    90f32.to_radians(),
                ),
                ..default()
            },
        });
    }
}

fn on_spawn_weapon(
    event: On<SpawnWeapon>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    weapon_configs: Res<Assets<WeaponConfig>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    muzzle_flash_images: Res<MuzzleFlashImages>,
) {
    let weapon_config = weapon_configs.get(&event.config).unwrap();
    let weapon_model = asset_server.load(&weapon_config.path_to_model);
    let collider_shape = Cuboid::from_size(weapon_config.collider_size);

    let muzzle_flash_mesh_handle = meshes.add(Rectangle::from_length(MUZZLE_FLASH_SIZE));
    let muzzle_flash_material_handle = materials.add(StandardMaterial {
        base_color: Color::WHITE.with_alpha(0.5),
        base_color_texture: Some(muzzle_flash_images.get_image_at_index(0)),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        cull_mode: None,
        ..default()
    });

    let mut weapon_commands = commands.spawn((
        Weapon::new(event.config.clone()),
        GrabbableObject,
        SceneRoot(weapon_model),
        event.transform,
        RigidBody::Dynamic,
        Collider::from(collider_shape),
        Mass(weapon_config.weight),
        MaxAngularSpeed(40.0),
    ));

    weapon_commands.with_child((
        MuzzleFlashAnimation::new(MUZZLE_FLASH_DURATION),
        Mesh3d(muzzle_flash_mesh_handle.clone()),
        MeshMaterial3d(muzzle_flash_material_handle),
        Transform::from_xyz(0.0, 0.0, -0.6),
        Visibility::Hidden,
        NotShadowCaster,
    ));

    if let FiringType::Automatic(seconds_between_shots) = weapon_config.firing_type {
        weapon_commands.insert(AutomaticFire::new(seconds_between_shots.as_duration()));
    }
}

#[allow(unused)]
fn save_test_weapon_config(mut commands: Commands) {
    let weapon_config = WeaponConfig {
        path_to_model: String::from("models/Blocky assault rifle.glb#Scene0"),
        collider_size: Vec3::new(0.08, 0.14, 0.6),
        weight: 4.0,
        recoil: 30.0,
        bullet_speed: 300.0,
        bullet_impact_force: 50.0,
        firing_type: FiringType::Automatic(SecondsBetweenShots(0.15)),
    };

    commands.trigger(SaveWeaponConfig {
        weapon_config,
        file_name: String::from("test_weapon"),
    });
}
