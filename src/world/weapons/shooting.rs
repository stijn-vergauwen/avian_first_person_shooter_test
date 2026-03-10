use std::time::Duration;

use avian3d::prelude::{
    Collider, Forces, LinearDamping, LinearVelocity, Mass, RigidBody, RigidBodyForces,
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
    mut weapons_query: Query<(Entity, &GlobalTransform, Forces, &Weapon)>,
    weapon_configs: Res<Assets<WeaponConfig>>,
    mut commands: Commands,
) {
    let (weapon_entity, global_weapon_transform, mut weapon_forces, weapon) = weapons_query
        .get_mut(shoot_weapon.entity)
        .expect("ShootWeapon should always point to weapon entity.");
    let weapon_config = weapon_configs.get(&weapon.config).unwrap();

    weapon_forces.apply_linear_impulse(global_weapon_transform.back() * weapon_config.recoil);

    let origin = global_weapon_transform.translation() + global_weapon_transform.forward() * 0.2;
    let direction = global_weapon_transform.forward();

    commands.trigger(SpawnBullet {
        shot_by: weapon_entity,
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
    mut commands: Commands,
) {
    let (_weapon, weapon_transform, weapon_velocity) = weapons
        .get(shoot_weapon.entity)
        .expect("ShootWeapon should always point to weapon entity.");

    let ejection_offset = Vec3::new(0.1, 0.0, 0.0);
    let casing_transform = Transform {
        translation: weapon_transform.translation() + weapon_transform.rotation() * ejection_offset,
        rotation: weapon_transform.rotation()
            * Quat::from_axis_angle(Vec3::NEG_Z, -20f32.to_radians()),
        ..default()
    };

    commands.spawn((
        GrabbableObject,
        Mesh3d(bullet_casing_assets.mesh.clone()),
        MeshMaterial3d(bullet_casing_assets.material.clone()),
        casing_transform,
        RigidBody::Dynamic,
        Collider::from(bullet_casing_assets.shape),
        Mass(0.4),
        LinearVelocity(weapon_velocity.0 + casing_transform.right() * 2.0),
        LinearDamping(0.05),
    ));
}
