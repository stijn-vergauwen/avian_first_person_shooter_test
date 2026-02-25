use std::f32::consts::PI;

use avian3d::prelude::*;
use bevy::{color::palettes::tailwind::STONE_600, prelude::*};

use crate::world::{ArrayOfObjects, spawn_array_of_static_objects};

pub struct ShootingRangeAreaPlugin;

impl Plugin for ShootingRangeAreaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_static_entities, spawn_test_targets));
    }
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

fn spawn_test_targets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let target_model = asset_server.load("models/Small shooting target.glb#Scene0");

    commands
        .spawn((
            SceneRoot(target_model),
            Transform {
                translation: Vec3::new(0.0, 1.5, -2.0),
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

    let target_model = asset_server.load("models/Shooting target.glb#Scene0");

    commands.spawn((
        SceneRoot(target_model),
        Transform {
            translation: Vec3::new(-2.0, 1.5, -2.0),
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
}
