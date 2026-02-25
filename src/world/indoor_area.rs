use avian3d::prelude::*;
use bevy::{color::palettes::tailwind::STONE_300, prelude::*};

pub struct IndoorAreaPlugin;

impl Plugin for IndoorAreaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_static_entities);
    }
}

fn spawn_static_entities(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Indoor area
    let quarter_turn: Quat = Quat::from_axis_angle(Vec3::Y, 90f32.to_radians());
    let wall_shape = Cuboid::new(4.0, 2.5, 0.2);
    let mesh_handle = meshes.add(wall_shape);
    let material_handle = materials.add(Color::from(STONE_300));

    commands.spawn((
        Mesh3d(mesh_handle.clone()),
        MeshMaterial3d(material_handle.clone()),
        RigidBody::Static,
        Collider::from(wall_shape),
        Transform {
            translation: Vec3::new(-12.0, wall_shape.half_size.y, 13.0),
            rotation: quarter_turn,
            ..default()
        },
    ));

    commands.spawn((
        Mesh3d(mesh_handle.clone()),
        MeshMaterial3d(material_handle.clone()),
        RigidBody::Static,
        Collider::from(wall_shape),
        Transform {
            translation: Vec3::new(-14.0, wall_shape.half_size.y, 15.0),
            ..default()
        },
    ));

    commands.spawn((
        Mesh3d(mesh_handle.clone()),
        MeshMaterial3d(material_handle.clone()),
        RigidBody::Static,
        Collider::from(wall_shape),
        Transform {
            translation: Vec3::new(-20.0, wall_shape.half_size.y, 15.0),
            ..default()
        },
    ));

    commands.spawn((
        Mesh3d(mesh_handle.clone()),
        MeshMaterial3d(material_handle.clone()),
        RigidBody::Static,
        Collider::from(wall_shape),
        Transform {
            translation: Vec3::new(-22.0, wall_shape.half_size.y, 13.0),
            rotation: quarter_turn,
            ..default()
        },
    ));

    commands.spawn((
        Mesh3d(mesh_handle.clone()),
        MeshMaterial3d(material_handle.clone()),
        RigidBody::Static,
        Collider::from(wall_shape),
        Transform {
            translation: Vec3::new(-24.0, wall_shape.half_size.y, 11.0),
            ..default()
        },
    ));
}
