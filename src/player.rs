mod crosshair;
mod cursor_lock;
mod grabbed_object;
mod grabbed_weapon;
mod movement_input;
mod spawner;

use bevy::prelude::*;

use crate::player::{
    crosshair::CrosshairPlugin, cursor_lock::CursorLockPlugin, grabbed_object::GrabbedObjectPlugin, grabbed_weapon::GrabbedWeaponPlugin, movement_input::PlayerMovementInputPlugin, spawner::PlayerSpawnerPlugin
};

const MOVEMENT_KEYBINDS: MovementKeybinds = MovementKeybinds {
    forward_key: KeyCode::KeyW,
    back_key: KeyCode::KeyS,
    left_key: KeyCode::KeyA,
    right_key: KeyCode::KeyD,
    jump_key: KeyCode::Space,
    run_key: KeyCode::ShiftLeft,
};

const WALKING_SPEED: f32 = 5.0;
const RUNNING_SPEED: f32 = 10.0;
const JUMP_FORCE: f32 = 20_000.0;

/// Mouse sensitivity calculated as: how many pixels the mouse needs to move in a direction to rotate by 1 radian in that direction.
/// - Higher value = less sensitive.
const PIXELS_PER_RADIAN: f32 = 600f32;

const MAX_GRAB_DISTANCE: f32 = 2.5;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PlayerSpawnerPlugin,
            PlayerMovementInputPlugin,
            CursorLockPlugin,
            CrosshairPlugin,
            GrabbedObjectPlugin,
            GrabbedWeaponPlugin,
        ));
    }
}

/// Marker component for the player. Only 1 player should be spawned.
#[derive(Component, Clone, Copy)]
pub struct Player;

/// Marker component for the player body mesh & collider.
#[derive(Component, Clone, Copy)]
pub struct PlayerBody;

/// Marker component for the player camera.
#[derive(Component, Clone, Copy)]
struct PlayerCamera;

#[derive(Copy, Clone)]
struct MovementKeybinds {
    pub forward_key: KeyCode,
    pub back_key: KeyCode,
    pub left_key: KeyCode,
    pub right_key: KeyCode,
    pub jump_key: KeyCode,
    pub run_key: KeyCode,
}
