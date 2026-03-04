use avian3d::prelude::*;
use bevy::prelude::*;

use crate::utilities::{
    pd_controller::config::PdControllerConfig,
    quaternion_pd_controller::QuaternionPdController,
    system_sets::{DataSystems, InputSystems},
};

use super::StandingTargetAssets;

pub struct ShootingTargetsPlugin;

impl Plugin for ShootingTargetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, enable_controllers_on_key.in_set(InputSystems))
            .add_systems(
                FixedUpdate,
                (update_controllers, disable_controllers_that_reached_target)
                    .in_set(DataSystems::UpdateEntities),
            );
    }
}

#[derive(Component)]
struct TargetResetController {
    controller: QuaternionPdController,
    is_enabled: bool,
}

pub fn spawn_rotating_standing_target(
    commands: &mut Commands,
    assets: &StandingTargetAssets,
    transform: Transform,
) {
    let stand = commands
        .spawn((
            Mesh3d(assets.stand_mesh.clone()),
            MeshMaterial3d(assets.stand_material.clone()),
            RigidBody::Static,
            Collider::from(assets.stand_shape),
            Transform {
                translation: transform.translation + Vec3::Y * assets.stand_shape.half_size.y,
                rotation: transform.rotation * Quat::from_axis_angle(Vec3::Y, 45f32.to_radians()),
                ..default()
            },
        ))
        .id();

    let target_position = transform.translation + Vec3::Y * (assets.stand_shape.size().y + 0.45);

    let shooting_target = commands
        .spawn((
            SceneRoot(assets.target_model.clone()),
            Transform {
                translation: target_position,
                rotation: transform.rotation,
                ..default()
            },
            RigidBody::Dynamic,
            ColliderConstructorHierarchy::default()
                .with_constructor_for_name(
                    // 'name' parameter should refer to the full name of the entity you want to target. Because Bevy uses the format `MeshName.MaterialName`, this means you need to target the material name even when in this case the mesh will be used instead of the material.
                    "Cube.005.Shooting target base color",
                    ColliderConstructor::TrimeshFromMesh,
                )
                .with_default_density(ColliderDensity(1000.0)),
            AngularDamping(0.3),
        ))
        .id();

    commands.spawn(
        RevoluteJoint::new(stand, shooting_target)
            .with_anchor(transform.translation + Vec3::Y * assets.stand_shape.size().y)
            .with_hinge_axis(Vec3::Y),
    );
}

pub fn spawn_falling_standing_target(
    commands: &mut Commands,
    assets: &StandingTargetAssets,
    transform: Transform,
) {
    let root = commands.spawn((transform, RigidBody::Static)).id();
    let pivot_point = commands
        .spawn((
            transform,
            Visibility::default(),
            RigidBody::Dynamic,
            ConstantLocalAngularAcceleration(Vec3::NEG_X * 50.0),
            TargetResetController {
                controller: QuaternionPdController::with_start_position(
                    PdControllerConfig::from_parameters(5.0, 5.0, 0.0),
                    transform.rotation,
                ),
                is_enabled: false,
            },
        ))
        .id();

    commands.spawn(
        RevoluteJoint::new(root, pivot_point)
            .with_hinge_axis(Vec3::X)
            .with_angle_limits(0.0, 90f32.to_radians()),
    );

    commands.spawn((
        Mesh3d(assets.stand_mesh.clone()),
        MeshMaterial3d(assets.stand_material.clone()),
        Collider::from(assets.stand_shape),
        Transform {
            translation: Vec3::Y * assets.stand_shape.half_size.y,
            rotation: Quat::from_axis_angle(Vec3::Y, 45f32.to_radians()),
            ..default()
        },
        ChildOf(pivot_point),
    ));

    let target_position = Vec3::Y * (assets.stand_shape.size().y + 0.45);

    commands.spawn((
        SceneRoot(assets.target_model.clone()),
        Transform::from_translation(target_position),
        ColliderConstructorHierarchy::default()
            .with_constructor_for_name(
                // 'name' parameter should refer to the full name of the entity you want to target. Because Bevy uses the format `MeshName.MaterialName`, this means you need to target the material name even when in this case the mesh will be used instead of the material.
                "Cube.005.Shooting target base color",
                ColliderConstructor::TrimeshFromMesh,
            )
            .with_default_density(ColliderDensity(1000.0)),
        ChildOf(pivot_point),
    ));
}

fn enable_controllers_on_key(
    input: Res<ButtonInput<KeyCode>>,
    mut controllers: Query<&mut TargetResetController>,
) {
    if input.just_pressed(KeyCode::KeyR) {
        for mut controller in controllers.iter_mut() {
            controller.is_enabled = !controller.is_enabled;
        }
    }
}

fn update_controllers(
    mut controllers: Query<(&mut TargetResetController, &Transform, Forces)>,
    time: Res<Time>,
) {
    for (mut controller, transform, mut forces) in controllers
        .iter_mut()
        .filter(|(controller, _, _)| controller.is_enabled)
    {
        let acceleration = controller.controller.update_from_physics_sim(
            transform.rotation,
            forces.angular_velocity(),
            time.delta_secs(),
        );

        forces.apply_angular_acceleration(acceleration);
    }
}

fn disable_controllers_that_reached_target(
    mut controllers: Query<(&mut TargetResetController, &Transform)>,
) {
    for (mut controller, transform) in controllers
        .iter_mut()
        .filter(|(controller, _)| controller.is_enabled)
    {
        let distance_to_target = transform
            .rotation
            .angle_between(controller.controller.target_position());

        if distance_to_target < 0.01 {
            controller.is_enabled = false;
        }
    }
}
