pub mod character;
pub mod desired_movement;
pub mod desired_rotation;
pub mod grabbable_object;
pub mod grounded;
pub mod interaction_target;
mod wall_mirror;
pub mod weapons;

use std::f32::consts::PI;

use avian3d::prelude::*;
use bevy::{color::palettes::tailwind::*, prelude::*};
use rand::Rng;

use crate::world::{
    character::CharacterPlugin,
    desired_movement::DesiredMovementPlugin,
    desired_rotation::DesiredRotationPlugin,
    grabbable_object::{GrabOrientation, GrabbableObject},
    grounded::GroundedPlugin,
    interaction_target::InteractionTargetPlugin,
    wall_mirror::WallMirrorPlugin,
    weapons::WeaponsPlugin,
};

const TABLE_POSITION: Vec3 = Vec3::new(-9.3, 1.0, 0.0);

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            CharacterPlugin,
            DesiredMovementPlugin,
            DesiredRotationPlugin,
            WeaponsPlugin,
            InteractionTargetPlugin,
            GroundedPlugin,
            WallMirrorPlugin,
        ))
        .add_systems(
            Startup,
            (
                spawn_static_entities,
                spawn_dynamic_entities,
                spawn_radio,
            ),
        );
    }
}

// TODO: reusable fn for spawning static object (takes in mesh & material assets, shape, color, transform)

fn spawn_static_entities(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground plane
    let ground_shape = Cuboid::new(200.0, 1.0, 200.0);

    commands.spawn((
        Mesh3d(meshes.add(ground_shape)),
        MeshMaterial3d(materials.add(Color::from(STONE_200))),
        RigidBody::Static,
        Collider::from(ground_shape),
        Transform::from_xyz(0.0, -ground_shape.half_size.y, 0.0),
    ));

    // Wall
    let wall_shape = Cuboid::new(40.0, 5.0, 0.4);

    commands.spawn((
        Mesh3d(meshes.add(wall_shape)),
        MeshMaterial3d(materials.add(Color::from(STONE_500))),
        RigidBody::Static,
        Collider::from(wall_shape),
        Transform {
            translation: Vec3::new(-10.0, wall_shape.half_size.y, 0.0),
            rotation: Quat::from_axis_angle(Vec3::Y, 90f32.to_radians()),
            ..default()
        },
    ));

    // Table
    let table_shape = Cuboid::new(10.0, 0.1, 1.0);

    commands.spawn((
        Mesh3d(meshes.add(table_shape)),
        MeshMaterial3d(materials.add(Color::from(STONE_400))),
        RigidBody::Static,
        Collider::from(table_shape),
        Transform {
            translation: TABLE_POSITION,
            rotation: Quat::from_axis_angle(Vec3::Y, 90f32.to_radians()),
            ..default()
        },
    ));

    // Light
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 20.0, 0.0),
            rotation: Quat::from_euler(EulerRot::YXZ, 10f32.to_radians(), -50f32.to_radians(), 0.0),
            ..default()
        },
    ));
}

fn spawn_dynamic_entities(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Tower of cubes
    let cube_shape = Cuboid::new(1.0, 1.0, 1.0);
    let cube_mesh = meshes.add(cube_shape);
    let cube_material = materials.add(Color::from(AMBER_400));
    let spawn_count = 20;
    let spawn_position = Vec3::new(5.0, 5.0, -10.0);

    for index in 0..spawn_count {
        commands.spawn((
            Mesh3d(cube_mesh.clone()),
            MeshMaterial3d(cube_material.clone()),
            RigidBody::Dynamic,
            Collider::from(cube_shape),
            ColliderDensity(100.0),
            Transform::from_translation(spawn_position + Vec3::Y * index as f32 * 1.2),
            SleepingDisabled,
        ));
    }

    // Pile of grabbable cubes
    let cube_shape = Cuboid::from_length(0.3);
    let cube_color = Color::from(PURPLE_700);
    let cube_mesh = meshes.add(cube_shape);
    let mut rng = rand::rng();
    let spawn_count = 40;
    let spawn_position = Vec3::new(0.0, 0.0, -6.0);
    let horizontal_spread = 0.3;

    for index in 0..spawn_count {
        let cube_material = materials.add(cube_color);

        let vertical_offset = Vec3::Y * index as f32 * cube_shape.half_size.y;
        let horizontal_offset = Vec3::new(
            rng.random_range(-horizontal_spread..horizontal_spread),
            0.0,
            rng.random_range(-horizontal_spread..horizontal_spread),
        );

        commands.spawn((
            GrabbableObject,
            GrabOrientation::IDENTITY,
            Mesh3d(cube_mesh.clone()),
            MeshMaterial3d(cube_material),
            RigidBody::Dynamic,
            Collider::from(cube_shape),
            ColliderDensity(300.0),
            Restitution::new(0.8),
            Transform::from_translation(spawn_position + vertical_offset + horizontal_offset),
            SleepingDisabled,
            MaxAngularSpeed(40.0),
        ));
    }
}

fn spawn_radio(mut commands: Commands, asset_server: Res<AssetServer>) {
    let radio_model = asset_server.load("models/Radio.glb#Scene0");

    commands.spawn((
        GrabbableObject,
        GrabOrientation::with_default_orientation(Quat::from_axis_angle(Vec3::Y, PI)),
        SceneRoot(radio_model),
        Transform {
            translation: TABLE_POSITION + Vec3::new(0.0, 0.5, 4.0),
            rotation: Quat::from_axis_angle(Vec3::Y, -90f32.to_radians()),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(0.4, 0.2, 0.15),
        Mass(5.0),
        SleepingDisabled,
        MaxAngularSpeed(40.0),
    ));
}