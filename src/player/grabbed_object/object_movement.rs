use avian3d::prelude::{ComputedMass, Forces, RigidBodyForces};
use bevy::prelude::*;

use crate::{
    player::{Player, PlayerCamera},
    utilities::system_sets::DataSystems,
    world::{
        grabbable_object::GrabOrientation,
        weapons::{Weapon, weapon_config::WeaponConfig},
    },
};

use super::{GrabbedObject, object_anchor::ObjectAnchor};

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

fn update_grabbed_object_position(
    mut grabbed_object: Single<&mut GrabbedObject>,
    mut grabbable_objects: Query<(&GlobalTransform, Forces, &ComputedMass), Without<Player>>,
    mut player: Single<Forces, With<Player>>,
    player_camera: Single<&GlobalTransform, With<PlayerCamera>>,
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

    let mut target_position = grabbed_object.current_anchor_value().translation.to_vec3();

    // Override for calculating ads position
    // TODO: remove this workaround once object anchors have been reworked
    if grabbed_object.current_object_anchor == ObjectAnchor::AimDownSight {
        let Ok(weapon) = weapons.get(grabbed_entity) else {
            return;
        };

        let weapon_config = weapon_configs.get(weapon.config()).unwrap();
        let camera_transform = *player_camera;

        target_position = camera_transform.translation()
            - camera_transform.rotation() * weapon_config.ads_position;
    }

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
    mut grabbed_object: Single<&mut GrabbedObject>,
    mut grabbable_objects: Query<(&GlobalTransform, Forces, &GrabOrientation, &ComputedMass)>,
    time: Res<Time>,
) {
    let Some(grabbed_entity) = grabbed_object.entity else {
        return;
    };

    let (grobal_transform, mut forces, grab_orientation, computed_mass) = grabbable_objects.get_mut(grabbed_entity).expect(
        "GrabbedObject should always point to existing entity with RigidBody component, or None.",
    );

    let player_rotation = grabbed_object.current_anchor_value().rotation;

    let mut target_rotation = player_rotation * grab_orientation.value();

    // Override for calculating ads rotation
    // TODO: remove this workaround once object anchors have been reworked
    if grabbed_object.current_object_anchor == ObjectAnchor::AimDownSight {
        let z_rotation = grab_orientation.0.to_euler(EulerRot::YXZ).2;
        target_rotation = player_rotation * Quat::from_axis_angle(Vec3::Z, z_rotation);
    }

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
