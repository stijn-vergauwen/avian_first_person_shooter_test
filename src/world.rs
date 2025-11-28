pub mod character;
pub mod desired_movement;
pub mod desired_rotation;
pub mod grabbable_object;
pub mod weapons;
pub mod interaction_target;
pub mod grounded;

use avian3d::prelude::*;
use bevy::{camera::Viewport, color::palettes::tailwind::*, prelude::*};
use rand::Rng;

use crate::world::{
    character::CharacterPlugin, desired_movement::DesiredMovementPlugin, desired_rotation::DesiredRotationPlugin, grabbable_object::GrabbableObject, grounded::GroundedPlugin, interaction_target::InteractionTargetPlugin, weapons::WeaponsPlugin
};

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
        ))
        .add_systems(
            Startup,
            (
                spawn_static_entities,
                spawn_dynamic_entities,
                spawn_external_cam,
            ),
        );
    }
}

fn spawn_static_entities(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground plane
    let ground_shape = Cuboid::new(200.0, 1.0, 200.0);

    commands.spawn((
        Mesh3d(meshes.add(ground_shape)),
        MeshMaterial3d(materials.add(Color::srgb(0.5, 0.5, 0.5))),
        RigidBody::Static,
        Collider::from(ground_shape),
        Transform::from_xyz(0.0, -ground_shape.half_size.y, 0.0),
    ));

    // Wall
    let wall_shape = Cuboid::new(40.0, 5.0, 0.4);

    commands.spawn((
        Mesh3d(meshes.add(wall_shape)),
        MeshMaterial3d(materials.add(StandardMaterial::from_color(STONE_500))),
        RigidBody::Static,
        Collider::from(wall_shape),
        Transform {
            translation: Vec3::new(-10.0, wall_shape.half_size.y, 0.0),
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
    let cube_material = materials.add(StandardMaterial::from_color(AMBER_400));
    let spawn_count = 20;
    let spawn_position = Vec3::new(5.0, 5.0, -10.0);

    for index in 0..spawn_count {
        commands.spawn((
            Mesh3d(cube_mesh.clone()),
            MeshMaterial3d(cube_material.clone()),
            RigidBody::Dynamic,
            Collider::from(cube_shape),
            Transform::from_translation(spawn_position + Vec3::Y * index as f32 * 1.2),
        ));
    }

    // Pile of grabbable cubes
    let cube_shape = Cuboid::from_length(0.3);
    let cube_mesh = meshes.add(cube_shape);
    let cube_material = materials.add(StandardMaterial::from_color(PURPLE_700));
    let mut rng = rand::rng();
    let spawn_count = 40;
    let spawn_position = Vec3::new(0.0, 0.0, -6.0);
    let horizontal_spread = 0.3;

    for index in 0..spawn_count {
        let vertical_offset = Vec3::Y * index as f32 * cube_shape.half_size.y;
        let horizontal_offset = Vec3::new(
            rng.random_range(-horizontal_spread..horizontal_spread),
            0.0,
            rng.random_range(-horizontal_spread..horizontal_spread),
        );

        commands.spawn((
            GrabbableObject,
            Mesh3d(cube_mesh.clone()),
            MeshMaterial3d(cube_material.clone()),
            RigidBody::Dynamic,
            Collider::from(cube_shape),
            Restitution::new(0.8),
            Transform::from_translation(spawn_position + vertical_offset + horizontal_offset),
        ));
    }
}

fn spawn_external_cam(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Camera {
            viewport: Some(Viewport {
                physical_position: UVec2::ZERO,
                physical_size: UVec2::new(360, 300),
                ..default()
            }),
            order: 2,
            ..default()
        },
        Transform::from_translation(Vec3::new(-2.0, 2.5, -6.0)).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
