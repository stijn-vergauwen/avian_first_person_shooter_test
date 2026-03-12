use avian3d::prelude::{Collider, Mass};
use bevy::prelude::*;

use super::{
    Weapon,
    shooting::AutomaticFire,
    weapon_config::{FiringType, WeaponConfig},
};

pub struct WeaponConfigModifiedPlugin;

impl Plugin for WeaponConfigModifiedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_weapon_when_asset_modified)
            .add_observer(on_weapon_config_modified);
    }
}

/// Describes when a WeaponConfig asset was modified, either because it was mutably referenced or because the source file was hot-reloaded.
#[derive(Event)]
pub struct WeaponConfigModified {
    pub asset_id: AssetId<WeaponConfig>,
    /// All weapon entities that point to this asset.
    pub weapon_entities: Vec<Entity>,
    pub new_data: WeaponConfig,
}

fn update_weapon_when_asset_modified(
    mut reader: MessageReader<AssetEvent<WeaponConfig>>,
    weapon_configs: Res<Assets<WeaponConfig>>,
    weapons: Query<(Entity, &Weapon)>,
    mut commands: Commands,
) {
    for message in reader.read() {
        let AssetEvent::Modified { id } = message else {
            continue;
        };

        let weapon_config = weapon_configs
            .get(*id)
            .expect("WeaponConfig modified message should always point to existing asset");

        let weapon_entities = weapons
            .iter()
            .filter_map(|(entity, weapon)| (weapon.config.id() == *id).then_some(entity))
            .collect();

        commands.trigger(WeaponConfigModified {
            asset_id: *id,
            weapon_entities,
            new_data: weapon_config.clone(),
        });
    }
}

fn on_weapon_config_modified(
    weapon_config_modified: On<WeaponConfigModified>,
    mut weapons: Query<(&mut Mass, &mut Collider, Option<&mut AutomaticFire>), With<Weapon>>,
    mut commands: Commands,
) {
    let weapon_config = &weapon_config_modified.new_data;

    for weapon_entity in weapon_config_modified.weapon_entities.clone() {
        let (mut mass, mut collider, automatic_fire) = weapons.get_mut(weapon_entity).unwrap();

        mass.0 = weapon_config.weight;
        *collider = Collider::from(Cuboid::from_size(weapon_config.collider_size));

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
}
