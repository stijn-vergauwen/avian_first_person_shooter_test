use avian3d::prelude::{Forces, RigidBodyForces};
use bevy::prelude::*;

use super::{ShootWeapon, Weapon, bullet::SpawnBullet};

const WEAPON_RECOIL: f32 = 40.0;

pub struct WeaponShootingPlugin;

impl Plugin for WeaponShootingPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_shoot_weapon);
    }
}

fn on_shoot_weapon(
    shoot_weapon: On<ShootWeapon>,
    mut weapons_query: Query<(&GlobalTransform, Forces), With<Weapon>>,
    time: Res<Time<Fixed>>,
    mut commands: Commands,
) {
    let (global_weapon_transform, mut weapon_forces) = weapons_query
        .get_mut(shoot_weapon.entity)
        .expect("ShootWeapon should always point to weapon entity.");

    weapon_forces.apply_force(global_weapon_transform.back() * WEAPON_RECOIL / time.delta_secs());

    let origin = global_weapon_transform.translation() + global_weapon_transform.forward() * 0.35;
    let direction = global_weapon_transform.forward();

    commands.trigger(SpawnBullet {
        origin,
        direction,
        travel_speed: 300.0,
    });
}
