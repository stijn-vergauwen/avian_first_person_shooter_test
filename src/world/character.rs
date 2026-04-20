pub mod jump;

use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{
    utilities::system_sets::DataSystems,
    world::{
        character::jump::CharacterJumpPlugin, desired_movement::DesiredMovement,
        desired_rotation::DesiredRotation,
    },
};

const MOVEMENT_STRENGTH: f32 = 230.0;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CharacterJumpPlugin)
            .add_observer(on_set_character_active)
            .add_systems(
                FixedUpdate,
                (update_movement_force, update_rotation).in_set(DataSystems::UpdateEntities),
            );
    }
}

#[derive(Component, Copy, Clone, Debug)]
pub struct Character {
    /// If this Character is currently active / controllable.
    pub is_active: bool,
}

/// Marker component.
#[derive(Component, Clone, Copy)]
pub struct CharacterHead;

/// Marker component.
#[derive(Component, Clone, Copy)]
pub struct CharacterNeck;

#[derive(EntityEvent, Clone, Copy)]
pub struct SetCharacterActive {
    pub entity: Entity,
    pub set_active: bool,
}

fn on_set_character_active(
    event: On<SetCharacterActive>,
    mut character_query: Query<&mut Character>,
) {
    let mut character = character_query
        .get_mut(event.entity)
        .expect("SetCharacterActive should always point to valid Character entity.");
    character.is_active = event.set_active;
}

fn update_movement_force(
    mut characters_query: Query<(
        &Character,
        &GlobalTransform,
        &DesiredMovement,
        &mut ConstantForce,
        &LinearVelocity,
    )>,
) {
    for (character, global_transform, desired_movement, mut force, linear_velocity) in
        characters_query.iter_mut()
    {
        if !character.is_active {
            force.0 = Vec3::ZERO;
            continue;
        }

        let movement_force = calculate_movement_force(
            desired_movement.velocity,
            **linear_velocity,
            global_transform.rotation(),
        );

        force.0 = movement_force;
    }
}

#[allow(clippy::type_complexity)]
fn update_rotation(
    mut character: Single<(&mut Transform, &Character, &DesiredRotation)>,
    mut character_neck: Single<&mut Transform, (With<CharacterNeck>, Without<Character>)>,
    mut character_head: Single<
        &mut Transform,
        (
            With<CharacterHead>,
            (Without<Character>, Without<CharacterNeck>),
        ),
    >,
) {
    if !character.1.is_active {
        return;
    }

    let desired_rotation = character.2;

    character.0.rotation = Quat::from_axis_angle(Vec3::Y, desired_rotation.rotation.y.radians());

    // My idea here is to 'spread' the head rotation over the length of the neck, to get rotation & movement closer to how your real neck moves.
    let half_vertical_rotation =
        Quat::from_axis_angle(Vec3::X, desired_rotation.rotation.x.radians() / 2.0);
    character_neck.rotation = half_vertical_rotation;
    character_head.rotation = half_vertical_rotation;
}

fn calculate_movement_force(
    target_velocity: Vec3,
    current_velocity: Vec3,
    character_rotation: Quat,
) -> Vec3 {
    let delta = (character_rotation * target_velocity - current_velocity).with_y(0.0);

    delta * MOVEMENT_STRENGTH
}
