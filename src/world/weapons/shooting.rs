use avian3d::prelude::{Forces, RigidBodyForces, SpatialQuery, SpatialQueryFilter};
use bevy::{
    color::palettes::tailwind::{RED_500, RED_900},
    prelude::*,
};

use crate::{
    utilities::{DrawGizmos, system_sets::DisplaySystems},
    world::weapons::{ShootWeapon, Weapon},
};

const BULLET_HIT_FORCE: f32 = 6_000.0;

pub struct WeaponShootingPlugin;

impl Plugin for WeaponShootingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_bullet_hit_point_assets)
            .add_systems(Update, draw_weapon_fire_direction.in_set(DisplaySystems))
            .add_observer(on_shoot_weapon)
            .add_observer(on_weapon_hit);
    }
}

#[derive(Resource)]
struct BulletHitPointAssets {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

#[derive(Event, Clone, Copy)]
struct WeaponHit {
    pub hit_entity: Entity,
    pub hit_position: Vec3,
    pub bullet_direction: Dir3,
}

fn setup_bullet_hit_point_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(BulletHitPointAssets {
        mesh: meshes.add(Sphere::new(0.03)),
        material: materials.add(StandardMaterial::from_color(RED_900)),
    });
}

fn on_shoot_weapon(
    shoot_weapon: On<ShootWeapon>,
    weapons_query: Query<(Entity, &GlobalTransform), With<Weapon>>,
    spatial_query: SpatialQuery,
    mut commands: Commands,
) {
    let (weapon_entity, global_weapon_transform) = weapons_query
        .get(shoot_weapon.entity)
        .expect("ShootWeapon should always point to weapon entity.");

    let origin = global_weapon_transform.translation(); // TODO: start raycast in front of weapon instead of inside it.
    let direction = global_weapon_transform.forward();

    if let Some(hit_data) = spatial_query.cast_ray(
        origin,
        direction,
        100.0,
        false,
        &SpatialQueryFilter::from_excluded_entities([weapon_entity]),
    ) {
        let global_hit_point_position = origin + direction * hit_data.distance;

        commands.trigger(WeaponHit {
            hit_entity: hit_data.entity,
            hit_position: global_hit_point_position,
            bullet_direction: direction,
        });
    }
}

fn on_weapon_hit(
    weapon_hit: On<WeaponHit>,
    mut hit_object_query: Query<(&GlobalTransform, Option<Forces>)>,
    mut commands: Commands,
    bullet_hit_point_assets: Res<BulletHitPointAssets>,
) {
    let (global_transform, forces) = hit_object_query
        .get_mut(weapon_hit.hit_entity)
        .expect("WeaponHit hit_entity should always point to existing entity");

    if let Some(mut forces) = forces {
        forces.apply_force_at_point(
            weapon_hit.bullet_direction.as_vec3() * BULLET_HIT_FORCE,
            weapon_hit.hit_position,
        );
    }

    commands.spawn((
        Mesh3d(bullet_hit_point_assets.mesh.clone()),
        MeshMaterial3d(bullet_hit_point_assets.material.clone()),
        Transform::from_translation(
            global_transform.rotation().inverse()
                * (weapon_hit.hit_position - global_transform.translation()),
        ),
        ChildOf(weapon_hit.hit_entity),
    ));
}

fn draw_weapon_fire_direction(
    weapon_transform: Single<&GlobalTransform, (With<Weapon>, With<DrawGizmos>)>,
    mut gizmos: Gizmos,
) {
    gizmos.ray(
        weapon_transform.translation(),
        weapon_transform.forward() * 100.0,
        RED_500,
    );
}
