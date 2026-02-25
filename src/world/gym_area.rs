use avian3d::prelude::*;
use bevy::{
    color::palettes::tailwind::{NEUTRAL_200, NEUTRAL_400},
    prelude::*,
};

use crate::world::{ArrayOfObjects, spawn_array_of_static_objects};

pub struct GymAreaPlugin;

impl Plugin for GymAreaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_static_entities);
    }
}

fn spawn_static_entities(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Parkour area
    let block_shape = Cuboid::from_length(4.0);
    let mut array_of_objects = ArrayOfObjects {
        center_position: Vec3::new(0.0, block_shape.half_size.y, 40.0),
        count: 3,
        distance_between: Vec3::new(-8.0, 0.0, 0.0),
        shape: block_shape,
        color: Color::from(NEUTRAL_400),
    };

    spawn_array_of_static_objects(&mut commands, &mut meshes, &mut materials, array_of_objects);
    array_of_objects.center_position.z += 8.0;
    spawn_array_of_static_objects(&mut commands, &mut meshes, &mut materials, array_of_objects);

    let ramp_shape = Cuboid::new(2.0, 0.1, 14.0);
    commands.spawn((
        Mesh3d(meshes.add(ramp_shape)),
        MeshMaterial3d(materials.add(Color::from(NEUTRAL_200))),
        RigidBody::Static,
        Collider::from(ramp_shape),
        Transform {
            translation: Vec3::new(11.0, 2.0, 35.0),
            rotation: Quat::from_axis_angle(Vec3::X, -20f32.to_radians()),
            ..default()
        },
    ));
}
