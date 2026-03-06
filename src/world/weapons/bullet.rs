use std::time::Duration;

use avian3d::prelude::*;
use bevy::{
    color::palettes::tailwind::{AMBER_200, RED_900},
    prelude::*,
};

use crate::utilities::system_sets::DataSystems;

const BULLET_HIT_FORCE: f32 = 50.0;
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
    shot_at: Duration,
}

#[derive(Event)]
pub struct SpawnBullet {
    pub origin: Vec3,
    pub direction: Dir3,
    pub travel_speed: f32,
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
}

#[derive(Resource)]
struct BulletAssets {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

#[derive(Resource)]
struct BulletHitPointAssets {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

fn setup_bullet_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(BulletAssets {
        mesh: meshes.add(Cuboid::new(0.02, 0.02, 0.2)),
        material: materials.add(StandardMaterial {
            base_color: Color::from(AMBER_200),
            emissive: LinearRgba::from(AMBER_200),
            ..default()
        }),
    });

    commands.insert_resource(BulletHitPointAssets {
        mesh: meshes.add(Sphere::new(0.03)),
        material: materials.add(StandardMaterial::from_color(RED_900)),
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
    global_transforms: Query<&GlobalTransform>,
    mut forces_query: Query<Forces>,
    mut commands: Commands,
    bullet_hit_point_assets: Res<BulletHitPointAssets>,
) {
    let global_transform = global_transforms
        .get(bullet_hit.hit_entity)
        .expect("BulletHit hit_entity should always have GlobalTransform component");

    commands.spawn((
        Mesh3d(bullet_hit_point_assets.mesh.clone()),
        MeshMaterial3d(bullet_hit_point_assets.material.clone()),
        Transform::from_translation(
            global_transform.rotation().inverse()
                * (bullet_hit.hit_position - global_transform.translation()),
        ),
        ChildOf(bullet_hit.hit_entity),
    ));

    if let Ok(mut forces) = forces_query.get_mut(bullet_hit.rigid_body_entity) {
        commands.queue(WakeBody(bullet_hit.rigid_body_entity));

        forces.apply_linear_impulse_at_point(
            bullet_hit.bullet_direction.as_vec3() * BULLET_HIT_FORCE,
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

    Some(BulletHit {
        bullet_entity,
        hit_entity: hit_data.entity,
        rigid_body_entity,
        hit_position: global_hit_point_position,
        bullet_direction: direction,
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
