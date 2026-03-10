use std::{fs, time::Duration};

use avian3d::prelude::{Collider, Mass, MaxAngularSpeed, RigidBody};
use bevy::prelude::*;

use crate::world::{TABLE_POSITION, grabbable_object::GrabbableObject};

use super::{
    Weapon,
    muzzle_flash::{MuzzleFlashAnimation, MuzzleFlashAssets},
    shooting::AutomaticFire,
    weapon_config::{FiringType, SecondsBetweenShots, WeaponConfig},
    weapon_config_save::SaveWeaponConfig,
};

const MUZZLE_FLASH_DURATION: Duration = Duration::from_millis(40);

pub struct WeaponSpawnerPlugin;

impl Plugin for WeaponSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_weapon_configs)
            .add_systems(
                Update,
                (
                    spawn_weapon_when_config_loaded,
                    update_weapon_when_asset_modified,
                ),
            )
            .add_observer(on_spawn_weapon);
    }
}

#[derive(Resource)]
struct WeaponsToSpawn {
    weapon_config_handles: Vec<Handle<WeaponConfig>>,
}

impl WeaponsToSpawn {
    fn take_handle_with_id(&mut self, id: AssetId<WeaponConfig>) -> Option<Handle<WeaponConfig>> {
        self.weapon_config_handles
            .iter()
            .position(|handle| handle.id() == id)
            .map(|index| self.weapon_config_handles.swap_remove(index))
    }
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

        let Some(config_handle) = weapons_to_spawn.take_handle_with_id(*id) else {
            continue;
        };

        commands.trigger(SpawnWeapon {
            config: config_handle,
            transform: calculate_weapon_spawn_transform(&weapons_to_spawn),
        });
    }
}

fn on_spawn_weapon(
    event: On<SpawnWeapon>,
    weapon_configs: Res<Assets<WeaponConfig>>,
    asset_server: Res<AssetServer>,
    muzzle_flash_assets: Res<MuzzleFlashAssets>,
    mut commands: Commands,
) {
    let weapon_config = weapon_configs.get(&event.config).unwrap();
    let weapon_model = asset_server.load(&weapon_config.path_to_model);

    let mut weapon_commands = commands.spawn((
        Weapon::new(event.config.clone()),
        GrabbableObject,
        SceneRoot(weapon_model),
        event.transform,
        RigidBody::Dynamic,
        cuboid_collider_from_size(weapon_config.collider_size),
        Mass(weapon_config.weight),
        MaxAngularSpeed(40.0),
    ));

    weapon_commands.with_child((
        MuzzleFlashAnimation::new(MUZZLE_FLASH_DURATION),
        Mesh3d(muzzle_flash_assets.mesh.clone()),
        MeshMaterial3d(muzzle_flash_assets.material.clone()),
        Transform::from_xyz(0.0, 0.0, -0.6),
    ));

    if let FiringType::Automatic(seconds_between_shots) = weapon_config.firing_type {
        weapon_commands.insert(AutomaticFire::new(seconds_between_shots.as_duration()));
    }
}

fn update_weapon_when_asset_modified(
    mut reader: MessageReader<AssetEvent<WeaponConfig>>,
    mut weapons: Query<(
        Entity,
        &Weapon,
        &mut Mass,
        &mut Collider,
        Option<&mut AutomaticFire>,
    )>,
    weapon_configs: Res<Assets<WeaponConfig>>,
    mut commands: Commands,
) {
    for message in reader.read() {
        let AssetEvent::Modified { id } = message else {
            continue;
        };

        let weapon_config = weapon_configs
            .get(*id)
            .expect("WeaponConfig modified message should always point to existing asset");

        for (weapon_entity, _, mass, collider, automatic_fire) in weapons
            .iter_mut()
            .filter(|(_, weapon, _, _, _)| weapon.config.id() == *id)
        {
            update_weapon_properties(
                weapon_config,
                weapon_entity,
                mass,
                collider,
                automatic_fire,
                &mut commands,
            );
        }
    }
}

fn update_weapon_properties(
    weapon_config: &WeaponConfig,
    weapon_entity: Entity,
    mut mass: Mut<Mass>,
    mut collider: Mut<Collider>,
    automatic_fire: Option<Mut<AutomaticFire>>,
    commands: &mut Commands,
) {
    mass.0 = weapon_config.weight;
    *collider = cuboid_collider_from_size(weapon_config.collider_size);

    match (weapon_config.firing_type, automatic_fire) {
        (FiringType::SemiAutomatic, None) => (),
        (FiringType::SemiAutomatic, Some(_)) => {
            commands.entity(weapon_entity).remove::<AutomaticFire>();
        }
        (FiringType::Automatic(seconds_between_shots), None) => {
            commands
                .entity(weapon_entity)
                .insert(AutomaticFire::new(seconds_between_shots.as_duration()));
        }
        (FiringType::Automatic(seconds_between_shots), Some(mut automatic_fire)) => {
            automatic_fire.time_between_shots = seconds_between_shots.as_duration();
        }
    }
}

fn cuboid_collider_from_size(size: Vec3) -> Collider {
    Collider::from(Cuboid::from_size(size))
}

fn calculate_weapon_spawn_transform(weapons_to_spawn: &WeaponsToSpawn) -> Transform {
    let position_offset = Vec3::NEG_Z * weapons_to_spawn.weapon_config_handles.len() as f32;
    Transform {
        translation: TABLE_POSITION + Vec3::new(0.0, 0.5, 3.0) + position_offset,
        rotation: Quat::from_euler(EulerRot::YXZ, 20f32.to_radians(), 0.0, 90f32.to_radians()),
        ..default()
    }
}

#[allow(unused)]
/// Save a test WeaponConfig to file to see how the config file should be formatted.
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
