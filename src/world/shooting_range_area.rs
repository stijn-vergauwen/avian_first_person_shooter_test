mod shooting_targets;

use std::f32::consts::PI;

use avian3d::prelude::*;
use bevy::{
    color::palettes::tailwind::NEUTRAL_700,
    prelude::*,
};
use shooting_targets::{
    ShootingTargetsPlugin, spawn_falling_standing_target, spawn_rotating_standing_target,
};


pub struct ShootingRangeAreaPlugin;

impl Plugin for ShootingRangeAreaPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ShootingTargetsPlugin)
            .add_systems(PreStartup, setup_assets)
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
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        SceneRoot(asset_server.load("models/Shooting range area.glb#Scene0")),
        Transform {
            translation: Vec3::new(0.0, 0.0, -20.0),
            ..default()
        },
        RigidBody::Static,
        ColliderConstructorHierarchy::new(ColliderConstructor::TrimeshFromMesh),
    ));
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
            translation: Vec3::new(2.0, 0.08, -14.0),
            rotation: Quat::from_axis_angle(Vec3::Y, PI),
            ..default()
        },
    );
}
