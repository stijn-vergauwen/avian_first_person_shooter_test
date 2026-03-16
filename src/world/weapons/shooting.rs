use std::time::Duration;

use avian3d::prelude::{
    AngularVelocity, Collider, Forces, LinearDamping, LinearVelocity, Mass, RigidBody, RigidBodyForces
};
use bevy::{color::palettes::tailwind::YELLOW_700, prelude::*};

use crate::{utilities::system_sets::DataSystems, world::grabbable_object::GrabbableObject};

use super::{Weapon, bullet::SpawnBullet, weapon_config::WeaponConfig};

pub struct WeaponShootingPlugin;

impl Plugin for WeaponShootingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_bullet_casing_assets)
            .add_systems(
                FixedUpdate,
                update_automatic_fire.in_set(DataSystems::UpdateEntities),
            )
            .add_observer(on_pull_trigger)
            .add_observer(on_release_trigger)
            .add_observer(on_shoot_weapon)
            .add_observer(eject_casing);
    }
}

#[derive(EntityEvent, Clone, Copy)]
pub struct PullTrigger {
    pub entity: Entity,
}

#[derive(EntityEvent, Clone, Copy)]
pub struct ReleaseTrigger {
    pub entity: Entity,
}

#[derive(EntityEvent, Clone, Copy)]
pub struct ShootWeapon {
    pub entity: Entity,
}

#[derive(Component, Clone, Copy)]
pub struct AutomaticFire {
    last_shot_at: Option<Duration>,
    pub time_between_shots: Duration,
}

impl AutomaticFire {
    pub fn new(time_between_shots: Duration) -> Self {
        Self {
            last_shot_at: None,
            time_between_shots,
        }
    }
}

#[derive(Resource)]
struct BulletCasingAssets {
    shape: Cuboid,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

fn setup_bullet_casing_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let shape = Cuboid::new(0.02, 0.02, 0.06);

    commands.insert_resource(BulletCasingAssets {
        shape,
        mesh: meshes.add(shape),
        material: materials.add(StandardMaterial::from_color(YELLOW_700)),
    });
}

fn on_pull_trigger(
    event: On<PullTrigger>,
    mut weapons: Query<(&mut Weapon, Option<&AutomaticFire>)>,
    mut commands: Commands,
) {
    let (mut weapon, automatic_fire) = weapons
        .get_mut(event.entity)
        .expect("PullTrigger should always point to weapon.");
    weapon.trigger_is_pulled = true;

    if automatic_fire.is_none() {
        commands.trigger(ShootWeapon {
            entity: event.entity,
        });
    }
}

fn on_release_trigger(event: On<ReleaseTrigger>, mut weapons: Query<&mut Weapon>) {
    let mut weapon = weapons
        .get_mut(event.entity)
        .expect("PullTrigger should always point to weapon.");
    weapon.trigger_is_pulled = false;
}

fn update_automatic_fire(
    mut weapons: Query<(Entity, &mut AutomaticFire, &Weapon)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (weapon_entity, mut automatic_fire, weapon) in weapons.iter_mut() {
        if !weapon.trigger_is_pulled {
            continue;
        }

        if automatic_fire.last_shot_at.is_none_or(|last_shot_at| {
            last_shot_at + automatic_fire.time_between_shots < time.elapsed()
        }) {
            commands.trigger(ShootWeapon {
                entity: weapon_entity,
            });

            automatic_fire.last_shot_at = Some(time.elapsed());
        }
    }
}

fn on_shoot_weapon(
    shoot_weapon: On<ShootWeapon>,
    mut weapons: Query<(&GlobalTransform, Forces, &Weapon)>,
    weapon_configs: Res<Assets<WeaponConfig>>,
    mut commands: Commands,
) {
    let (global_transform, mut weapon_forces, weapon) = weapons
        .get_mut(shoot_weapon.entity)
        .expect("ShootWeapon should always point to weapon entity.");
    let weapon_config = weapon_configs.get(&weapon.config).unwrap();

    let origin = global_transform.transform_point(weapon_config.shot_origin);
    let direction = global_transform.forward();

    weapon_forces
        .apply_linear_impulse_at_point(global_transform.back() * weapon_config.recoil, origin);

    commands.trigger(SpawnBullet {
        shot_by: shoot_weapon.entity,
        origin,
        direction,
        travel_speed: weapon_config.bullet_speed,
        impact_force: weapon_config.bullet_impact_force,
    });
}

fn eject_casing(
    shoot_weapon: On<ShootWeapon>,
    weapons: Query<(&Weapon, &GlobalTransform, &LinearVelocity)>,
    bullet_casing_assets: Res<BulletCasingAssets>,
    weapon_configs: Res<Assets<WeaponConfig>>,
    mut commands: Commands,
) {
    let (weapon, weapon_transform, weapon_velocity) = weapons
        .get(shoot_weapon.entity)
        .expect("ShootWeapon should always point to weapon entity.");

    let weapon_config = weapon_configs.get(weapon.config()).unwrap();
    let weapon_rotation = weapon_transform.rotation();

    let ejection_offset = weapon_config.shell_ejection_position;
    let casing_transform = Transform {
        translation: weapon_transform.translation() + weapon_rotation * ejection_offset,
        rotation: weapon_rotation,
        ..default()
    };

    let shell_direction =
        weapon_rotation * weapon_config.shell_ejection_rotation.to_quat() * Dir3::NEG_Z;

    commands.spawn((
        GrabbableObject,
        Mesh3d(bullet_casing_assets.mesh.clone()),
        MeshMaterial3d(bullet_casing_assets.material.clone()),
        casing_transform,
        RigidBody::Dynamic,
        Collider::from(bullet_casing_assets.shape),
        Mass(0.4),
        LinearVelocity(weapon_velocity.0 + shell_direction * weapon_config.shell_ejection_force),
        LinearDamping(0.05),
        AngularVelocity(weapon_rotation * weapon_config.shell_ejection_spin),
    ));
}
