use std::f32::consts::PI;

use avian3d::prelude::*;
use bevy::{
    color::palettes::tailwind::{NEUTRAL_700, STONE_600},
    prelude::*,
};

use crate::world::{ArrayOfObjects, spawn_array_of_static_objects};

pub struct ShootingRangeAreaPlugin;

impl Plugin for ShootingRangeAreaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup_assets)
            .add_systems(Startup, (spawn_static_entities, spawn_test_targets));
    }
}

#[derive(Resource)]
struct StandingTargetAssets {
    stand_shape: Cuboid,
    stand_mesh: Handle<Mesh>,
    stand_material: Handle<StandardMaterial>,
    target_model: Handle<Scene>,
}

fn setup_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let stand_shape = Cuboid::new(0.1, 0.8, 0.1);
    let stand_mesh = meshes.add(stand_shape);
    let stand_material = materials.add(Color::from(NEUTRAL_700));
    let target_model = asset_server.load("models/Shooting target.glb#Scene0");

    commands.insert_resource(StandingTargetAssets {
        stand_shape,
        stand_mesh,
        stand_material,
        target_model,
    });
}

fn spawn_static_entities(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let wall_shape = Cuboid::new(0.5, 0.8, 40.0);
    spawn_array_of_static_objects(
        &mut commands,
        &mut meshes,
        &mut materials,
        ArrayOfObjects {
            center_position: Vec3::new(40.0, wall_shape.half_size.y, -60.0),
            count: 7,
            distance_between: Vec3::new(10.0, 0.0, 0.0),
            shape: wall_shape,
            color: Color::from(STONE_600),
        },
    );
}

fn spawn_test_targets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    standing_target_assets: Res<StandingTargetAssets>,
) {
    // Small test target
    let target_model = asset_server.load("models/Small shooting target.glb#Scene0");

    commands
        .spawn((
            SceneRoot(target_model),
            Transform {
                translation: Vec3::new(0.0, 1.5, -14.0),
                rotation: Quat::from_axis_angle(Vec3::Y, PI),
                ..default()
            },
            RigidBody::Static,
        ))
        .with_child((
            Collider::cylinder(0.2, 0.03),
            Transform::from_rotation(Quat::from_axis_angle(Vec3::X, 90f32.to_radians())),
            Visibility::default(),
        ));

    // Test target
    let target_model = asset_server.load("models/Shooting target.glb#Scene0");

    commands.spawn((
        SceneRoot(target_model),
        Transform {
            translation: Vec3::new(-2.0, 1.5, -14.0),
            rotation: Quat::from_axis_angle(Vec3::Y, PI),
            ..default()
        },
        RigidBody::Static,
        ColliderConstructorHierarchy::default().with_constructor_for_name(
            // 'name' parameter should refer to the full name of the entity you want to target. Because Bevy uses the format `MeshName.MaterialName`, this means you need to target the material name even when in this case the mesh will be used instead of the material.
            "Cube.005.Shooting target base color",
            ColliderConstructor::TrimeshFromMesh,
        ),
    ));

    // Standing target
    spawn_rotating_standing_target(
        &mut commands,
        &standing_target_assets,
        Transform {
            translation: Vec3::new(4.0, 0.0, -14.0),
            rotation: Quat::from_axis_angle(Vec3::Y, PI),
            ..default()
        },
    );

    spawn_falling_standing_target(
        &mut commands,
        &standing_target_assets,
        Transform {
            translation: Vec3::new(2.0, 0.0, -14.0),
            rotation: Quat::from_axis_angle(Vec3::Y, PI),
            ..default()
        },
    );
}

fn spawn_rotating_standing_target(
    commands: &mut Commands,
    assets: &StandingTargetAssets,
    transform: Transform,
) {
    let stand = commands
        .spawn((
            Mesh3d(assets.stand_mesh.clone()),
            MeshMaterial3d(assets.stand_material.clone()),
            RigidBody::Static,
            Collider::from(assets.stand_shape),
            Transform {
                translation: transform.translation + Vec3::Y * assets.stand_shape.half_size.y,
                rotation: transform.rotation * Quat::from_axis_angle(Vec3::Y, 45f32.to_radians()),
                ..default()
            },
        ))
        .id();

    let target_position = transform.translation + Vec3::Y * (assets.stand_shape.size().y + 0.45);

    let shooting_target = commands
        .spawn((
            SceneRoot(assets.target_model.clone()),
            Transform {
                translation: target_position,
                rotation: transform.rotation,
                ..default()
            },
            RigidBody::Dynamic,
            ColliderConstructorHierarchy::default()
                .with_constructor_for_name(
                    // 'name' parameter should refer to the full name of the entity you want to target. Because Bevy uses the format `MeshName.MaterialName`, this means you need to target the material name even when in this case the mesh will be used instead of the material.
                    "Cube.005.Shooting target base color",
                    ColliderConstructor::TrimeshFromMesh,
                )
                .with_default_density(ColliderDensity(1000.0)),
            AngularDamping(0.3),
        ))
        .id();

    commands.spawn(
        RevoluteJoint::new(stand, shooting_target)
            .with_anchor(transform.translation + Vec3::Y * assets.stand_shape.size().y)
            .with_hinge_axis(Vec3::Y),
    );
}

fn spawn_falling_standing_target(
    commands: &mut Commands,
    assets: &StandingTargetAssets,
    transform: Transform,
) {
    let root = commands
        .spawn((RigidBody::Dynamic, transform, Visibility::default()))
        .id();
    let pivot_point = commands.spawn((RigidBody::Static, transform)).id();

    commands.spawn(RevoluteJoint::new(root, pivot_point).with_hinge_axis(Vec3::X));

    commands.spawn((
        Mesh3d(assets.stand_mesh.clone()),
        MeshMaterial3d(assets.stand_material.clone()),
        Collider::from(assets.stand_shape),
        Transform {
            translation: Vec3::Y * assets.stand_shape.half_size.y,
            rotation: Quat::from_axis_angle(Vec3::Y, 45f32.to_radians()),
            ..default()
        },
        ChildOf(root),
    ));

    let target_position = Vec3::Y * (assets.stand_shape.size().y + 0.45);

    commands.spawn((
        SceneRoot(assets.target_model.clone()),
        Transform::from_translation(target_position),
        ColliderConstructorHierarchy::default()
            .with_constructor_for_name(
                // 'name' parameter should refer to the full name of the entity you want to target. Because Bevy uses the format `MeshName.MaterialName`, this means you need to target the material name even when in this case the mesh will be used instead of the material.
                "Cube.005.Shooting target base color",
                ColliderConstructor::TrimeshFromMesh,
            )
            .with_default_density(ColliderDensity(1000.0)),
        ChildOf(root),
    ));
}
