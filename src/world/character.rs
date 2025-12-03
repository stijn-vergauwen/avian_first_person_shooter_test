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

const MAX_MOVEMENT_STRENGTH: f32 = 8.0;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CharacterJumpPlugin).add_systems(
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

fn update_movement_force(
    mut characters_query: Query<(
        &Character,
        &GlobalTransform,
        Option<&DesiredMovement>,
        &mut ConstantForce,
    )>,
) {
    for (character, global_transform, desired_movement, mut force) in characters_query.iter_mut() {
        if !character.is_active {
            continue;
        }

        let desired_movement_force = match desired_movement {
            Some(desired_movement) => {
                let desired_movement_strength =
                    MAX_MOVEMENT_STRENGTH * desired_movement.fraction_of_max_strength.value();

                global_transform.rotation()
                    * (desired_movement.direction * desired_movement_strength)
            }
            None => Vec3::ZERO,
        };

        force.0 = desired_movement_force;
    }
}

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
