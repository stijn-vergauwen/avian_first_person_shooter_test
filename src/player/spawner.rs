use bevy::{color::palettes::tailwind::CYAN_700, prelude::*};
use avian3d::prelude::*;

use crate::{player::{Player, PlayerCamera, ToolAnchor}, world::{character::{Character, CharacterHead}, desired_rotation::DesiredRotation}};

pub struct PlayerSpawnerPlugin;

impl Plugin for PlayerSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let start_position = Vec3::ZERO;

    // Spawn root

    let player_root_entity = commands
        .spawn((
            Player,
            Character { is_active: true },
            Visibility::Inherited,
            Transform::from_translation(start_position),
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            ConstantForce::default(),
            DesiredRotation::default(),
        ))
        .id();

    // Spawn body

    let body_capsule = Capsule3d::new(0.4, 1.0);

    commands.spawn((
        Transform::from_translation(Vec3::Y * (body_capsule.half_length + body_capsule.radius)),
        Mesh3d(meshes.add(body_capsule)),
        MeshMaterial3d(materials.add(StandardMaterial::from_color(CYAN_700))),
        Collider::from(body_capsule),
        ChildOf(player_root_entity),
    ));

    // Spawn head

    let player_head_entity = commands.spawn((
        CharacterHead,
        Transform::from_xyz(0.0, 1.7, 0.0),
        ChildOf(player_root_entity),
    )).id();

    // Spawn camera

    commands.spawn((
        PlayerCamera,
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, -0.0),
        ChildOf(player_head_entity),
    ));

    // Spawn tool anchor

    commands.spawn((
        ToolAnchor,
        Transform::from_xyz(0.3, -0.3, -0.6),
        ChildOf(player_head_entity),
    ));
}