use std::time::Duration;

use avian3d::prelude::*;
use bevy::{
    color::palettes::tailwind::AMBER_200,
    pbr::decal::{ForwardDecal, ForwardDecalMaterial, ForwardDecalMaterialExt},
    prelude::*,
};

use crate::utilities::system_sets::DataSystems;

const BULLET_LIFETIME: Duration = Duration::from_secs(5);

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_bullet_assets)
            .add_systems(
                FixedUpdate,
                (update_bullets, despawn_bullets_past_lifetime).in_set(DataSystems::UpdateEntities),
            )
            .add_observer(on_bullet_hit)
            .add_observer(on_spawn_bullet);
    }
}

#[derive(Component)]
struct Bullet {
    /// Bullet speed in the local Transform.forward() direction.
    travel_speed: f32,
    impact_force: f32,
    shot_at: Duration,
}

#[derive(Event)]
pub struct SpawnBullet {
    pub origin: Vec3,
    pub direction: Dir3,
    pub travel_speed: f32,
    pub impact_force: f32,
}

#[derive(EntityEvent, Clone, Copy)]
struct BulletHit {
    #[event_target]
    bullet_entity: Entity,
    hit_entity: Entity,
    /// The entity of the first RigidBody in the hierarchy, either the hit entity itself or the RigidBody this entity is a child of.
    rigid_body_entity: Entity,
    hit_position: Vec3,
    bullet_direction: Dir3,
    surface_normal: Dir3,
}

#[derive(Resource)]
struct BulletAssets {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

#[derive(Resource)]
struct BulletImpactAssets {
    material: Handle<ForwardDecalMaterial<StandardMaterial>>,
}

fn setup_bullet_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut decal_materials: ResMut<Assets<ForwardDecalMaterial<StandardMaterial>>>,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(BulletAssets {
        mesh: meshes.add(Cuboid::new(0.02, 0.02, 0.2)),
        material: materials.add(StandardMaterial {
            base_color: Color::from(AMBER_200),
            emissive: LinearRgba::from(AMBER_200),
            ..default()
        }),
    });

    commands.insert_resource(BulletImpactAssets {
        material: decal_materials.add(ForwardDecalMaterial {
            base: StandardMaterial {
                base_color_texture: Some(asset_server.load("textures/Bullet impact decal.png")),
                alpha_mode: AlphaMode::Mask(0.2),
                ..default()
            },
            extension: ForwardDecalMaterialExt {
                depth_fade_factor: 0.01,
            },
        }),
    });
}

fn on_spawn_bullet(
    event: On<SpawnBullet>,
    mut commands: Commands,
    bullet_assets: Res<BulletAssets>,
    time: Res<Time>,
) {
    commands.spawn((
        Bullet {
            travel_speed: event.travel_speed,
            impact_force: event.impact_force,
            shot_at: time.elapsed(),
        },
        Mesh3d(bullet_assets.mesh.clone()),
        MeshMaterial3d(bullet_assets.material.clone()),
        Transform::from_translation(event.origin).looking_to(event.direction, Dir3::Y),
    ));
}

fn update_bullets(
    mut bullets: Query<(Entity, &Bullet, &mut Transform)>,
    rigid_bodies: Query<&RigidBody>,
    parents: Query<&ChildOf>,
    spatial_query: SpatialQuery,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (bullet_entity, bullet, mut transform) in bullets.iter_mut() {
        let origin = transform.translation;
        let direction = transform.forward();
        let step_distance = bullet.travel_speed * time.delta_secs();

        if let Some(bullet_hit) = calculate_bullet_raycast(
            &spatial_query,
            rigid_bodies,
            parents,
            bullet_entity,
            origin,
            direction,
            step_distance,
        ) {
            commands.trigger(bullet_hit);
        }

        transform.translation += direction * step_distance;
    }
}

fn on_bullet_hit(
    bullet_hit: On<BulletHit>,
    bullets: Query<&Bullet>,
    objects: Query<(&Position, &Rotation)>,
    mut forces_query: Query<Forces>,
    mut commands: Commands,
    bullet_impact_assets: Res<BulletImpactAssets>,
) {
    let (global_position, global_rotation) = objects
        .get(bullet_hit.hit_entity)
        .expect("BulletHit hit_entity should always have position & rotation components");

    // This Transform calculation is quite confusing, this has to do with the decal needing the be positioned relative to the hit entity (because it gets parented),
    //      as well as the ForwardDecal creating a quad that faces towards Y instead of NEG_Z (which would be 'forward').
    //      All that makes it quite a mess to get the correct Transform, would be nice if I found ways to simplify.
    let impact_decal_position =
        global_rotation.0.inverse() * (bullet_hit.hit_position - global_position.0);

    let mut transform = Transform {
        translation: impact_decal_position,
        scale: Vec3::splat(0.05),
        ..default()
    }
    .looking_to(
        global_rotation.0.inverse() * -bullet_hit.surface_normal,
        Dir3::Y,
    );
    transform.rotate_local_x(90f32.to_radians());

    commands.spawn((
        ForwardDecal,
        MeshMaterial3d(bullet_impact_assets.material.clone()),
        transform,
        ChildOf(bullet_hit.hit_entity),
    ));

    if let Ok(mut forces) = forces_query.get_mut(bullet_hit.rigid_body_entity) {
        let impact_force = bullets.get(bullet_hit.bullet_entity).unwrap().impact_force;

        commands.queue(WakeBody(bullet_hit.rigid_body_entity));

        forces.apply_linear_impulse_at_point(
            bullet_hit.bullet_direction.as_vec3() * impact_force,
            bullet_hit.hit_position,
        );
    }

    commands.entity(bullet_hit.bullet_entity).despawn();
}

fn despawn_bullets_past_lifetime(
    bullets: Query<(Entity, &Bullet)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (bullet_entity, bullet) in bullets.iter() {
        if bullet.shot_at + BULLET_LIFETIME < time.elapsed() {
            commands.entity(bullet_entity).despawn();
        }
    }
}

// Utilities

fn calculate_bullet_raycast(
    spatial_query: &SpatialQuery,
    rigid_bodies: Query<&RigidBody>,
    parents: Query<&ChildOf>,
    bullet_entity: Entity,
    origin: Vec3,
    direction: Dir3,
    step_distance: f32,
) -> Option<BulletHit> {
    let hit_data = spatial_query.cast_ray(
        origin,
        direction,
        step_distance,
        true,
        &SpatialQueryFilter::default(),
    )?;

    let global_hit_point_position = origin + direction * hit_data.distance;

    let rigid_body_entity = find_closest_rigid_body_entity(rigid_bodies, parents, hit_data.entity);
    let surface_normal = Dir3::new_unchecked(hit_data.normal);

    Some(BulletHit {
        bullet_entity,
        hit_entity: hit_data.entity,
        rigid_body_entity,
        hit_position: global_hit_point_position,
        bullet_direction: direction,
        surface_normal,
    })
}

fn find_closest_rigid_body_entity(
    rigid_bodies: Query<&RigidBody>,
    parents: Query<&ChildOf>,
    entity: Entity,
) -> Entity {
    match rigid_bodies.contains(entity) {
        true => entity,
        false => parents
            .iter_ancestors(entity)
            .find(|parent| rigid_bodies.contains(*parent))
            .expect("BulletHit should always hit an entity that has a RigidBody in it's hierarchy"),
    }
}
