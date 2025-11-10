pub mod character;
pub mod desired_movement;
mod desired_rotation;
pub mod weapons;

use avian3d::prelude::*;
use bevy::prelude::*;

use crate::world::{
    character::CharacterPlugin, desired_movement::DesiredMovementPlugin,
    desired_rotation::DesiredRotationPlugin, weapons::WeaponsPlugin,
};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            CharacterPlugin,
            DesiredMovementPlugin,
            DesiredRotationPlugin,
            WeaponsPlugin,
        ))
        .add_systems(Startup, (setup, spawn_external_cam));
    }
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground plane
    let ground_shape = Cuboid::new(100.0, 1.0, 100.0);

    commands.spawn((
        Mesh3d(meshes.add(ground_shape)),
        MeshMaterial3d(materials.add(Color::srgb(0.5, 0.5, 0.5))),
        RigidBody::Static,
        Collider::from(ground_shape),
        Transform::from_xyz(0.0, -ground_shape.half_size.y, 0.0),
    ));

    // Cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.2, 0.2))),
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 1.0),
        Transform::from_xyz(0.5, 8.0, 0.0),
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

fn spawn_external_cam(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Camera {
            viewport: Some(bevy::camera::Viewport {
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
