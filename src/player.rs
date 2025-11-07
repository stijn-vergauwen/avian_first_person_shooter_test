use avian3d::prelude::*;
use bevy::{color::palettes::tailwind::*, prelude::*};

use crate::{
    utilities::{fraction::Fraction, system_sets::InputSystems},
    world::{
        character::Character,
        desired_movement::{DesiredMovement, SetDesiredMovement},
    },
};

const MOVEMENT_KEYBINDS: MovementKeybinds = MovementKeybinds {
    forward_key: KeyCode::KeyW,
    back_key: KeyCode::KeyS,
    left_key: KeyCode::KeyA,
    right_key: KeyCode::KeyD,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, read_player_movement_input.in_set(InputSystems))
            .add_observer(|_: On<Add, DesiredMovement>| {
                println!("DesiredMovement component added!")
            });
    }
}

/// Marker component for the player. Only 1 player should be spawned.
#[derive(Component, Clone, Copy)]
struct Player;

#[derive(Copy, Clone)]
pub struct MovementKeybinds {
    pub forward_key: KeyCode,
    pub back_key: KeyCode,
    pub left_key: KeyCode,
    pub right_key: KeyCode,
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
        ))
        .id();

    // Spawn body

    let body_capsule = Capsule3d::new(0.3, 1.0);

    commands.spawn((
        Transform::from_translation(Vec3::Y * (body_capsule.half_length + body_capsule.radius)),
        Mesh3d(meshes.add(body_capsule)),
        MeshMaterial3d(materials.add(StandardMaterial::from_color(CYAN_700))),
        Collider::from(body_capsule),
        ChildOf(player_root_entity),
    ));
}

fn read_player_movement_input(
    player_entity: Single<Entity, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
) {
    let move_direction = move_direction_from_input(MOVEMENT_KEYBINDS, &keyboard_input);

    let desired_movement = move_direction.map(|direction| DesiredMovement {
        direction,
        fraction_of_max_strength: Fraction::new_unchecked(1.0),
    });

    commands.trigger(SetDesiredMovement {
        entity: *player_entity,
        desired_movement,
    });
}

// Utilities

fn move_direction_from_input(
    keybinds: MovementKeybinds,
    input: &ButtonInput<KeyCode>,
) -> Option<Dir3> {
    let mut direction = Vec3::ZERO;

    if input.pressed(keybinds.forward_key) {
        direction.z -= 1.0;
    }

    if input.pressed(keybinds.back_key) {
        direction.z += 1.0;
    }

    if input.pressed(keybinds.left_key) {
        direction.x -= 1.0;
    }

    if input.pressed(keybinds.right_key) {
        direction.x += 1.0;
    }

    Dir3::new(direction).ok()
}
