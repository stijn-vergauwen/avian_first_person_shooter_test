use avian3d::prelude::{ComputedMass, Forces, RigidBodyForces};
use bevy::prelude::*;

use crate::{
    player::{Player, PlayerCamera, grabbed_object::PRIMARY_HAND_OFFSET},
    utilities::system_sets::DataSystems,
    world::{
        character::CharacterHead,
        grabbable_object::GrabOrientation,
        weapons::{Weapon, weapon_config::WeaponConfig},
    },
};

use super::{GrabbedObject, HoldPosition, INSPECTING_OFFSET};

pub struct GrabbedObjectMovementPlugin;

impl Plugin for GrabbedObjectMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                update_grabbed_object_position,
                update_grabbed_object_rotation,
            )
                .in_set(DataSystems::UpdateEntities),
        );
    }
}

#[allow(clippy::too_many_arguments)]
fn update_grabbed_object_position(
    mut grabbed_object: ResMut<GrabbedObject>,
    hold_position: Res<HoldPosition>,
    mut grabbable_objects: Query<(&GlobalTransform, Forces, &ComputedMass), Without<Player>>,
    mut player: Single<Forces, With<Player>>,
    player_camera: Single<&GlobalTransform, With<PlayerCamera>>,
    player_head: Single<&GlobalTransform, With<CharacterHead>>,
    weapons: Query<&Weapon>,
    weapon_configs: Res<Assets<WeaponConfig>>,
    time: Res<Time>,
) {
    let Some(grabbed_entity) = grabbed_object.entity else {
        return;
    };

    let (global_transform, mut forces, computed_mass) = grabbable_objects.get_mut(grabbed_entity).expect(
        "GrabbedObject should always point to existing entity with RigidBody component, or None.",
    );

    let target_position = match *hold_position {
        HoldPosition::PrimaryHand => player_head.transform_point(PRIMARY_HAND_OFFSET),
        HoldPosition::Inspecting => player_camera.transform_point(INSPECTING_OFFSET),
        HoldPosition::AimDownSight => {
            if let Ok(weapon) = weapons.get(grabbed_entity)
                && let Some(weapon_config) = weapon_configs.get(weapon.config())
            {
                player_camera.transform_point(-weapon_config.ads_position)
            } else {
                return;
            }
        }
    };

    grabbed_object
        .position_force_controller
        .set_target_position(target_position);

    let new_acceleration = grabbed_object
        .position_force_controller
        .update_from_physics_sim(
            global_transform.translation(),
            forces.linear_velocity(),
            time.delta_secs(),
        );

    // Adjust strength based on mass of grabbed object, this prevents light objects from glitching out
    let adjusted_acceleration = new_acceleration * (0.5 + computed_mass.value() * 0.6).min(5.0);

    // Apply position force to grabbed object
    forces.apply_force(adjusted_acceleration);

    // Apply opposite position force to player
    player.apply_force(-adjusted_acceleration);
}

fn update_grabbed_object_rotation(
    mut grabbed_object: ResMut<GrabbedObject>,
    hold_position: Res<HoldPosition>,
    mut grabbable_objects: Query<(&GlobalTransform, Forces, &GrabOrientation, &ComputedMass)>,
    player_head: Single<&GlobalTransform, With<CharacterHead>>,
    time: Res<Time>,
) {
    let Some(grabbed_entity) = grabbed_object.entity else {
        return;
    };

    let (grobal_transform, mut forces, grab_orientation, computed_mass) = grabbable_objects.get_mut(grabbed_entity).expect(
        "GrabbedObject should always point to existing entity with RigidBody component, or None.",
    );

    let player_rotation = player_head.rotation();

    let target_rotation = match *hold_position {
        HoldPosition::AimDownSight => {
            let z_rotation = grab_orientation.0.to_euler(EulerRot::YXZ).2;
            player_rotation * Quat::from_axis_angle(Vec3::Z, z_rotation)
        }
        _ => player_rotation * grab_orientation.value(),
    };

    grabbed_object
        .rotation_force_controller
        .set_target_position(target_rotation);

    let new_acceleration = grabbed_object
        .rotation_force_controller
        .update_from_physics_sim(
            grobal_transform.rotation(),
            forces.angular_velocity(),
            time.delta_secs(),
        );

    // Adjust strength based on mass of grabbed object, this prevents light objects from glitching out
    let adjusted_acceleration = new_acceleration * (0.5 + computed_mass.value() * 0.6).min(3.0);

    forces.apply_angular_acceleration(adjusted_acceleration);
}
