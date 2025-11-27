use avian3d::prelude::*;
use bevy::{color::palettes::tailwind::CYAN_700, prelude::*};

use crate::{
    player::{ItemAnchor, MAX_GRAB_DISTANCE, Player, PlayerBody, PlayerCamera},
    world::{
        character::{Character, CharacterHead, CharacterNeck},
        desired_rotation::DesiredRotation,
        interaction_target::{
            CurrentInteractionTarget, InteractionTargetConfig, PlayerInteractionTarget,
        },
    },
};

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

    let body_capsule = Capsule3d::new(0.3, 1.0);

    let player_body_mesh_entity = commands
        .spawn((
            PlayerBody,
            Transform::from_translation(Vec3::Y * (body_capsule.half_length + body_capsule.radius)),
            Mesh3d(meshes.add(body_capsule)),
            MeshMaterial3d(materials.add(StandardMaterial::from_color(CYAN_700))),
            Collider::from(body_capsule),
            ChildOf(player_root_entity),
        ))
        .id();

    // Spawn neck root

    let player_neck_entity = commands
        .spawn((
            CharacterNeck,
            Transform::from_xyz(0.0, 1.5, 0.0),
            Visibility::Inherited,
            ChildOf(player_root_entity),
        ))
        .id();

    // Spawn head root

    let player_head_entity = commands
        .spawn((
            CharacterHead,
            Transform::from_xyz(0.0, 0.15, 0.0),
            Visibility::Inherited,
            ChildOf(player_neck_entity),
        ))
        .id();

    // Spawn head mesh
    let head_shape = Cuboid::from_length(0.35);

    let player_head_mesh_entity = commands
        .spawn((
            Transform::from_xyz(0.0, head_shape.half_size.y, 0.0),
            Mesh3d(meshes.add(head_shape)),
            MeshMaterial3d(materials.add(StandardMaterial::from_color(CYAN_700))),
            Collider::from(head_shape),
            ChildOf(player_head_entity),
        ))
        .id();

    // Spawn camera

    let player_camera_entity = commands
        .spawn((
            PlayerCamera,
            Camera3d::default(),
            IsDefaultUiCamera,
            CurrentInteractionTarget::from_config(InteractionTargetConfig {
                max_range: MAX_GRAB_DISTANCE,
                query_filter: SpatialQueryFilter::from_excluded_entities(vec![
                    player_body_mesh_entity,
                    player_head_mesh_entity,
                ]),
            }),
            Transform::from_xyz(0.0, head_shape.half_size.y, -head_shape.half_size.z),
            ChildOf(player_head_entity),
        ))
        .id();

    // Spawn Item anchor

    commands.spawn((
        ItemAnchor::default(),
        Transform::from_xyz(0.3, -0.3, -0.6),
        ChildOf(player_head_entity),
    ));

    // Spawn PlayerInteractionTarget resource

    commands.insert_resource(PlayerInteractionTarget::new(player_camera_entity));
}
