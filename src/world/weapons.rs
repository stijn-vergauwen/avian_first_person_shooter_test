use avian3d::prelude::*;
use bevy::{
    color::palettes::tailwind::{RED_500, RED_900},
    prelude::*,
};

use crate::{
    utilities::system_sets::DisplaySystems,
    world::grabbable_object::GrabbableObject,
};

pub struct WeaponsPlugin;

impl Plugin for WeaponsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_test_weapon, setup_bullet_hit_point_assets))
            .add_systems(Update, draw_weapon_fire_direction.in_set(DisplaySystems))
            .add_observer(on_shoot_weapon);
    }
}

#[derive(Resource)]
pub struct BulletHitPointAssets {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

/// Base component for weapons.
#[derive(Component, Clone, Copy)]
pub struct Weapon;

#[derive(EntityEvent, Clone, Copy)]
pub struct ShootWeapon {
    pub entity: Entity,
}

fn setup_bullet_hit_point_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(BulletHitPointAssets {
        mesh: meshes.add(Sphere::new(0.05)),
        material: materials.add(StandardMaterial::from_color(RED_900)),
    });
}

fn spawn_test_weapon(mut commands: Commands, asset_server: Res<AssetServer>) {
    let weapon_model = asset_server.load("models/Blocky assault rifle.glb#Scene0");

    commands.spawn((
        Weapon,
        GrabbableObject,
        Collider::cuboid(0.1, 0.1, 0.6),
        Transform::from_xyz(0.0, 1.0, 0.0),
        SceneRoot(weapon_model),
    ));
}

fn on_shoot_weapon(
    shoot_weapon: On<ShootWeapon>,
    weapons_query: Query<(Entity, &GlobalTransform), With<Weapon>>,
    global_transform_query: Query<&GlobalTransform, Without<Weapon>>,
    spatial_query: SpatialQuery,
    mut commands: Commands,
    bullet_hit_point_assets: Res<BulletHitPointAssets>,
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
        let global_transform_of_entity_hit = global_transform_query
            .get(hit_data.entity)
            .expect("Raycast hit should always point to an entity with GlobalTransform component.");
        let global_position_of_entity_hit = global_transform_of_entity_hit.translation();

        commands.spawn((
            Mesh3d(bullet_hit_point_assets.mesh.clone()),
            MeshMaterial3d(bullet_hit_point_assets.material.clone()),
            Transform::from_translation(
                global_transform_of_entity_hit.rotation().inverse()
                    * (global_hit_point_position - global_position_of_entity_hit),
            ),
            ChildOf(hit_data.entity),
        ));
    }
}

fn draw_weapon_fire_direction(
    weapon_transform: Single<&GlobalTransform, With<Weapon>>,
    mut gizmos: Gizmos,
) {
    gizmos.ray(
        weapon_transform.translation(),
        weapon_transform.forward() * 100.0,
        RED_500,
    );
}
