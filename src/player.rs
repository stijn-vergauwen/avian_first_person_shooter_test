mod cursor_lock;
mod spawner;

use bevy::{color::palettes::tailwind::*, input::mouse::AccumulatedMouseMotion, prelude::*};

use crate::{
    player::{cursor_lock::CursorLockPlugin, spawner::PlayerSpawnerPlugin},
    utilities::{
        euler_angle::EulerAngle,
        fraction::Fraction,
        system_sets::{DisplaySystems, InputSystems},
    },
    world::{
        desired_movement::{DesiredMovement, SetDesiredMovement},
        desired_rotation::{DesiredRotation, RotationType, SetDesiredRotation},
        weapons::Weapon,
    },
};

const MOVEMENT_KEYBINDS: MovementKeybinds = MovementKeybinds {
    forward_key: KeyCode::KeyW,
    back_key: KeyCode::KeyS,
    left_key: KeyCode::KeyA,
    right_key: KeyCode::KeyD,
};

/// Upper threshold for delta mouse motion in a single update, this is to ignore motion spikes caused by input through Parsec.
const UPPER_MOUSE_MOTION_THRESHOLD: f32 = 1000.0;

/// Mouse sensitivity calculated as: how many pixels the mouse needs to move in a direction to rotate by 1 radian in that direction.
/// - Higher value = less sensitive.
const PIXELS_PER_RADIAN: f32 = 400f32;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((CursorLockPlugin, PlayerSpawnerPlugin))
            .add_systems(
                Update,
                (
                    (
                        handle_movement_input,
                        handle_rotation_input,
                        grab_test_weapon_on_keypress,
                    )
                        .in_set(InputSystems),
                    draw_player_gizmos.in_set(DisplaySystems),
                ),
            );
    }
}

/// Marker component for the player. Only 1 player should be spawned.
#[derive(Component, Clone, Copy)]
struct Player;

/// Marker component for the player camera.
#[derive(Component, Clone, Copy)]
struct PlayerCamera;

/// Marker component for the anchor that is used to position objects held by the player.
#[derive(Component, Clone, Copy)]
struct ToolAnchor;

#[derive(Copy, Clone)]
pub struct MovementKeybinds {
    pub forward_key: KeyCode,
    pub back_key: KeyCode,
    pub left_key: KeyCode,
    pub right_key: KeyCode,
}

fn handle_movement_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player_entity: Single<Entity, With<Player>>,
    mut previous_input: Local<Option<DesiredMovement>>,
    mut commands: Commands,
) {
    let move_direction = move_direction_from_input(MOVEMENT_KEYBINDS, &keyboard_input);

    let desired_movement = move_direction.map(|direction| DesiredMovement {
        direction,
        fraction_of_max_strength: Fraction::new_unchecked(1.0),
    });

    if desired_movement != *previous_input {
        *previous_input = desired_movement;

        commands.trigger(SetDesiredMovement {
            entity: *player_entity,
            desired_movement,
        });
    }
}

fn handle_rotation_input(
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    player_entity: Single<Entity, With<Player>>,
    mut commands: Commands,
) {
    let Some(desired_rotation) = calculate_desired_rotation(accumulated_mouse_motion.delta) else {
        return;
    };

    commands.trigger(SetDesiredRotation {
        entity: *player_entity,
        desired_rotation,
    });
}

fn calculate_desired_rotation(delta_motion: Vec2) -> Option<DesiredRotation> {
    if delta_motion.length() > UPPER_MOUSE_MOTION_THRESHOLD {
        println!("Mouse motion above threshold!");
    }

    (delta_motion.length() > 0.0 && delta_motion.length() < UPPER_MOUSE_MOTION_THRESHOLD).then(
        || {
            let delta_rotation = EulerAngle::from_radians(
                -delta_motion.y / PIXELS_PER_RADIAN,
                -delta_motion.x / PIXELS_PER_RADIAN,
                0.0,
                EulerRot::default(),
            );

            DesiredRotation {
                rotation: delta_rotation,
                rotation_type: RotationType::DeltaRotation,
            }
        },
    )
}

fn grab_test_weapon_on_keypress(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    tool_anchor_entity: Single<Entity, With<ToolAnchor>>,
    test_weapon_entity: Single<Entity, With<Weapon>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyE) {
        commands
            .entity(*test_weapon_entity)
            .insert((ChildOf(*tool_anchor_entity), Transform::default()));
    }
}

fn draw_player_gizmos(
    tool_anchor: Single<&GlobalTransform, With<ToolAnchor>>,
    player_camera: Single<&GlobalTransform, (With<PlayerCamera>, Without<ToolAnchor>)>,
    mut gizmos: Gizmos,
) {
    // Tool anchor
    gizmos.sphere(
        tool_anchor.compute_transform().to_isometry(),
        0.2,
        PURPLE_400,
    );

    // Player camera
    gizmos.ray(
        player_camera.translation(),
        player_camera.forward() * 10.0,
        LIME_400,
    );
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
