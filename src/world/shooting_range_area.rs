use std::{f32::consts::PI, time::Duration};

use avian3d::prelude::*;
use bevy::prelude::*;
use rand::random_range;

use super::shooting_targets::{
    ResetAfterDuration, StandingTargetAssets, spawn_falling_standing_target,
    spawn_rotating_standing_target,
};

pub struct ShootingRangeAreaPlugin;

impl Plugin for ShootingRangeAreaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                spawn_static_entities,
                spawn_targets_in_lanes,
                spawn_target_props,
            ),
        );
    }
}

fn spawn_static_entities(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SceneRoot(asset_server.load("models/Shooting range area.glb#Scene0")),
        Transform {
            translation: Vec3::new(0.0, 0.0, -20.0),
            ..default()
        },
        RigidBody::Static,
        ColliderConstructorHierarchy::new(ColliderConstructor::TrimeshFromMesh),
    ));
}

fn spawn_targets_in_lanes(
    mut commands: Commands,
    standing_target_assets: Res<StandingTargetAssets>,
) {
    let start_position = Vec3::new(5.0, 0.0, -50.0);

    // Lane 1
    spawn_falling_standing_target(
        &mut commands,
        &standing_target_assets,
        Transform {
            translation: start_position + Vec3::new(0.0, 0.06, 20.0),
            rotation: Quat::from_axis_angle(Vec3::Y, PI),
            ..default()
        },
        Some(ResetAfterDuration::new(Duration::from_secs(2))),
    );

    spawn_falling_standing_target(
        &mut commands,
        &standing_target_assets,
        Transform {
            translation: start_position + Vec3::new(-2.0, 0.06, 17.0),
            rotation: Quat::from_axis_angle(Vec3::Y, PI),
            ..default()
        },
        Some(ResetAfterDuration::new(Duration::from_secs(2))),
    );

    spawn_falling_standing_target(
        &mut commands,
        &standing_target_assets,
        Transform {
            translation: start_position + Vec3::new(2.0, 0.06, 14.0),
            rotation: Quat::from_axis_angle(Vec3::Y, PI),
            ..default()
        },
        Some(ResetAfterDuration::new(Duration::from_secs(2))),
    );

    // Lane 2
    spawn_falling_standing_target(
        &mut commands,
        &standing_target_assets,
        Transform {
            translation: start_position + Vec3::new(10.0, 0.06, 0.0),
            rotation: Quat::from_axis_angle(Vec3::Y, PI),
            ..default()
        },
        Some(ResetAfterDuration::new(Duration::from_secs(2))),
    );

    spawn_falling_standing_target(
        &mut commands,
        &standing_target_assets,
        Transform {
            translation: start_position + Vec3::new(8.0, 0.06, -3.0),
            rotation: Quat::from_axis_angle(Vec3::Y, PI),
            ..default()
        },
        Some(ResetAfterDuration::new(Duration::from_secs(2))),
    );

    spawn_falling_standing_target(
        &mut commands,
        &standing_target_assets,
        Transform {
            translation: start_position + Vec3::new(12.0, 0.06, -6.0),
            rotation: Quat::from_axis_angle(Vec3::Y, PI),
            ..default()
        },
        Some(ResetAfterDuration::new(Duration::from_secs(2))),
    );

    // Lane 3
    spawn_rotating_standing_target(
        &mut commands,
        &standing_target_assets,
        Transform {
            translation: start_position + Vec3::new(20.0, 0.0, 0.0),
            rotation: Quat::from_axis_angle(Vec3::Y, PI),
            ..default()
        },
    );

    spawn_rotating_standing_target(
        &mut commands,
        &standing_target_assets,
        Transform {
            translation: start_position + Vec3::new(18.0, 0.0, 0.0),
            rotation: Quat::from_axis_angle(Vec3::Y, PI),
            ..default()
        },
    );

    spawn_rotating_standing_target(
        &mut commands,
        &standing_target_assets,
        Transform {
            translation: start_position + Vec3::new(22.0, 0.0, 0.0),
            rotation: Quat::from_axis_angle(Vec3::Y, PI),
            ..default()
        },
    );

    // Lane 4
    spawn_falling_standing_target(
        &mut commands,
        &standing_target_assets,
        Transform {
            translation: start_position + Vec3::new(30.0, 0.06, 0.0),
            rotation: Quat::from_axis_angle(Vec3::Y, PI),
            ..default()
        },
        Some(ResetAfterDuration::new(Duration::from_secs(2))),
    );

    spawn_falling_standing_target(
        &mut commands,
        &standing_target_assets,
        Transform {
            translation: start_position + Vec3::new(30.0, 0.06, 5.0),
            rotation: Quat::from_axis_angle(Vec3::Y, PI),
            ..default()
        },
        Some(ResetAfterDuration::new(Duration::from_secs(2))),
    );

    spawn_falling_standing_target(
        &mut commands,
        &standing_target_assets,
        Transform {
            translation: start_position + Vec3::new(30.0, 0.06, 10.0),
            rotation: Quat::from_axis_angle(Vec3::Y, PI),
            ..default()
        },
        Some(ResetAfterDuration::new(Duration::from_secs(2))),
    );

    spawn_falling_standing_target(
        &mut commands,
        &standing_target_assets,
        Transform {
            translation: start_position + Vec3::new(30.0, 0.06, 15.0),
            rotation: Quat::from_axis_angle(Vec3::Y, PI),
            ..default()
        },
        Some(ResetAfterDuration::new(Duration::from_secs(2))),
    );

    spawn_falling_standing_target(
        &mut commands,
        &standing_target_assets,
        Transform {
            translation: start_position + Vec3::new(30.0, 0.06, 20.0),
            rotation: Quat::from_axis_angle(Vec3::Y, PI),
            ..default()
        },
        Some(ResetAfterDuration::new(Duration::from_secs(2))),
    );
}

fn spawn_target_props(mut commands: Commands, asset_server: Res<AssetServer>) {
    let target_model = asset_server.load("models/Small shooting target.glb#Scene0");

    // line of small targets on 2nd shooting range wall
    for index in 0..10 {
        let position = Vec3::new(10.0, 1.06, -21.0) + index as f32 * Vec3::new(0.0, 0.0, -1.0);

        commands
            .spawn((
                SceneRoot(target_model.clone()),
                Transform::from_translation(position).looking_to(Dir3::Z, Dir3::Y),
                RigidBody::Dynamic,
                AngularDamping(0.5),
            ))
            .with_child((
                Collider::cylinder(0.2, 0.03),
                Mass(8.0),
                Transform::from_rotation(Quat::from_axis_angle(Vec3::X, 90f32.to_radians())),
                Visibility::default(),
            ));
    }

    // Stack of small targets on shooting range table
    for index in 0..6 {
        let position = Vec3::new(11.0, 2.0, -19.2)
            + index as f32 * Vec3::new(0.1, 0.05, 0.0)
            + random_range(-0.2..0.2) * Vec3::Z;

        commands
            .spawn((
                SceneRoot(target_model.clone()),
                Transform::from_translation(position).looking_to(Dir3::Y, Dir3::X),
                RigidBody::Dynamic,
                AngularDamping(0.5),
            ))
            .with_child((
                Collider::cylinder(0.2, 0.03),
                Mass(8.0),
                Transform::from_rotation(Quat::from_axis_angle(Vec3::X, 90f32.to_radians())),
                Visibility::default(),
            ));
    }

    // Targets on left wall of shooting range
    let target_model = asset_server.load("models/Shooting target.glb#Scene0");

    for index in 0..6 {
        let position = Vec3::new(-0.5, 1.0, -20.0) + index as f32 * Vec3::new(0.0, 0.05, -0.3);

        commands.spawn((
            SceneRoot(target_model.clone()),
            Transform::from_translation(position)
                .looking_to(Dir3::from_xyz(-1.0, 1.0, 0.0).unwrap(), Dir3::Y),
            RigidBody::Dynamic,
            ColliderConstructorHierarchy::default()
                .with_constructor_for_name(
                    // 'name' parameter should refer to the full name of the entity you want to target. Because Bevy uses the format `MeshName.MaterialName`, this means you need to target the material name even when in this case the mesh will be used instead of the material.
                    "Cube.005.Shooting target base color",
                    ColliderConstructor::TrimeshFromMesh,
                )
                .with_default_density(ColliderDensity(1600.0)),
        ));
    }
}
