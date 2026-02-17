use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};

use crate::{
    player::{JUMP_FORCE, MOVEMENT_KEYBINDS, MovementKeybinds, PIXELS_PER_RADIAN, Player, RUNNING_SPEED, WALKING_SPEED}, utilities::{
        euler_angle::EulerAngle,
        system_sets::InputSystems,
    }, world::{
        character::{Character, jump::AttemptJump},
        desired_movement::{DesiredMovement, SetDesiredMovement},
        desired_rotation::{DesiredRotation, RotationType, SetDesiredRotation},
    }
};

/// Upper threshold for delta mouse motion in a single update, this is to ignore motion spikes caused by input through Parsec.
const UPPER_MOUSE_MOTION_THRESHOLD: f32 = 1000.0;

pub struct PlayerMovementInputPlugin;

impl Plugin for PlayerMovementInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_movement_input,
                handle_rotation_input,
                handle_jump_input,
            )
                .in_set(InputSystems),
        );
    }
}

fn handle_movement_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player_entity: Single<Entity, With<Player>>,
    mut previous_input: Local<DesiredMovement>,
    mut commands: Commands,
) {
    let move_direction = move_direction_from_input(MOVEMENT_KEYBINDS, &keyboard_input);
    let move_speed = match keyboard_input.pressed(MOVEMENT_KEYBINDS.run_key) {
        true => RUNNING_SPEED,
        false => WALKING_SPEED,
    };

    let desired_movement = DesiredMovement {
        velocity: move_direction.map_or(Vec3::ZERO, |direction| direction * move_speed),
    };

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
    player: Single<(Entity, &Character), With<Player>>,
    mut commands: Commands,
) {
    if !player.1.is_active {
        return;
    }

    if let Some(desired_rotation) = calculate_desired_rotation(accumulated_mouse_motion.delta) {
        commands.trigger(SetDesiredRotation {
            entity: player.0,
            desired_rotation,
        });
    }
}

fn handle_jump_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player_entity: Single<Entity, With<Player>>,
    mut commands: Commands,
) {
    if keyboard_input.just_pressed(MOVEMENT_KEYBINDS.jump_key) {
        commands.trigger(AttemptJump {
            entity: *player_entity,
            jump_force: JUMP_FORCE,
        });
    }
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

fn calculate_desired_rotation(delta_motion: Vec2) -> Option<DesiredRotation> {
    if delta_motion.length() > UPPER_MOUSE_MOTION_THRESHOLD {
        println!("Mouse motion above threshold!");
    }

    (delta_motion.length() > 0.0 && delta_motion.length() < UPPER_MOUSE_MOTION_THRESHOLD).then(
        || DesiredRotation {
            rotation: delta_rotation_from_mouse_motion(delta_motion),
            rotation_type: RotationType::DeltaRotation,
        },
    )
}

fn delta_rotation_from_mouse_motion(delta_motion: Vec2) -> EulerAngle {
    EulerAngle::from_radians(
        -delta_motion.y / PIXELS_PER_RADIAN,
        -delta_motion.x / PIXELS_PER_RADIAN,
        0.0,
        EulerRot::default(),
    )
}
