use avian3d::prelude::{
    Collider, Forces, LinearVelocity, Mass, RigidBody, RigidBodyForces
};
use bevy::{color::palettes::tailwind::YELLOW_700, prelude::*};

use super::{ShootWeapon, Weapon, bullet::SpawnBullet};

const WEAPON_RECOIL: f32 = 30.0;

pub struct WeaponShootingPlugin;

impl Plugin for WeaponShootingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_bullet_casing_assets)
            .add_observer(on_shoot_weapon)
            .add_observer(eject_casing);
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

fn on_shoot_weapon(
    shoot_weapon: On<ShootWeapon>,
    mut weapons_query: Query<(&GlobalTransform, Forces), With<Weapon>>,
    mut commands: Commands,
) {
    let (global_weapon_transform, mut weapon_forces) = weapons_query
        .get_mut(shoot_weapon.entity)
        .expect("ShootWeapon should always point to weapon entity.");

    weapon_forces.apply_linear_impulse(global_weapon_transform.back() * WEAPON_RECOIL);

    let origin = global_weapon_transform.translation() + global_weapon_transform.forward() * 0.35;
    let direction = global_weapon_transform.forward();

    commands.trigger(SpawnBullet {
        origin,
        direction,
        travel_speed: 300.0,
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
        Mesh3d(bullet_casing_assets.mesh.clone()),
        MeshMaterial3d(bullet_casing_assets.material.clone()),
        casing_transform,
        RigidBody::Dynamic,
        Collider::from(bullet_casing_assets.shape),
        Mass(0.2),
        LinearVelocity(weapon_velocity.0 + casing_transform.right() * 2.0),
    ));
}
