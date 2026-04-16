use std::{f32::consts::PI, time::Duration};

use avian3d::prelude::*;
use bevy::{color::palettes::tailwind::NEUTRAL_700, prelude::*};

use crate::utilities::{
    pd_controller::config::PdControllerConfig,
    quaternion_pd_controller::QuaternionPdController,
    system_sets::{DataSystems, InputSystems},
};

const DISTANCE_TO_TARGET_THRESHOLD: f32 = 0.01;

pub struct ShootingTargetsPlugin;

impl Plugin for ShootingTargetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup_assets)
            .add_systems(Update, enable_controllers_on_key.in_set(InputSystems))
            .add_systems(
                FixedUpdate,
                (
                    update_controllers,
                    disable_controllers_that_reached_target,
                    update_reset_after_duration_components,
                )
                    .chain()
                    .in_set(DataSystems::UpdateEntities),
            )
            .add_observer(on_enable_reset_controller);
    }
}

#[derive(Resource)]
pub struct StandingTargetAssets {
    stand_shape: Cuboid,
    stand_mesh: Handle<Mesh>,
    stand_material: Handle<StandardMaterial>,
    target_model: Handle<Scene>,
}

fn setup_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let stand_shape = Cuboid::new(0.1, 0.8, 0.1);
    let stand_mesh = meshes.add(stand_shape);
    let stand_material = materials.add(Color::from(NEUTRAL_700));
    let target_model = asset_server.load("models/Shooting target.glb#Scene0");

    commands.insert_resource(StandingTargetAssets {
        stand_shape,
        stand_mesh,
        stand_material,
        target_model,
    });
}

#[derive(Component)]
pub struct TargetResetController {
    controller: QuaternionPdController,
    is_enabled: bool,
}

#[derive(Component)]
pub struct ResetAfterDuration {
    outside_threshold_since: Option<Duration>,
    reset_after: Duration,
}

impl ResetAfterDuration {
    pub fn new(reset_after: Duration) -> Self {
        Self {
            outside_threshold_since: None,
            reset_after,
        }
    }
}

#[derive(EntityEvent)]
struct EnableResetController {
    entity: Entity,
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
    reset_after_duration: Option<ResetAfterDuration>,
) {
    let root = commands.spawn((transform, RigidBody::Static)).id();
    let mut pivot_point_commands = commands.spawn((
        transform,
        Visibility::default(),
        RigidBody::Dynamic,
        ConstantLocalAngularAcceleration(Vec3::NEG_X * 30.0),
        TargetResetController {
            controller: QuaternionPdController::with_start_position(
                PdControllerConfig::from_parameters(1.2, 1.0, 0.0),
                transform.rotation,
            ),
            is_enabled: false,
        },
    ));

    if let Some(reset_after_duration) = reset_after_duration {
        pivot_point_commands.insert(reset_after_duration);
    }

    let pivot_point = pivot_point_commands.id();

    commands.spawn(
        RevoluteJoint::new(root, pivot_point)
            .with_hinge_axis(Vec3::X)
            .with_angle_limits(0.0, PI),
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
            .with_default_density(ColliderDensity(1600.0)),
        ChildOf(pivot_point),
    ));
}

fn enable_controllers_on_key(
    input: Res<ButtonInput<KeyCode>>,
    controllers: Query<Entity, With<TargetResetController>>,
    mut commands: Commands,
) {
    if input.just_pressed(KeyCode::KeyR) {
        for controller_entity in controllers.iter() {
            commands.trigger(EnableResetController {
                entity: controller_entity,
            });
        }
    }
}

fn update_controllers(
    mut controllers: Query<(&mut TargetResetController, &Transform, Forces)>,
    time: Res<Time>,
) {
    for (mut controller, transform, mut forces) in controllers.iter_mut() {
        let acceleration = controller.controller.update_from_physics_sim(
            transform.rotation,
            forces.angular_velocity(),
            time.delta_secs(),
        );

        if controller.is_enabled {
            forces.apply_angular_acceleration(acceleration * 50.0);
        }
    }
}

fn disable_controllers_that_reached_target(mut controllers: Query<&mut TargetResetController>) {
    for mut controller in controllers
        .iter_mut()
        .filter(|controller| controller.is_enabled)
    {
        if controller.controller.distance_to_target() < DISTANCE_TO_TARGET_THRESHOLD {
            controller.is_enabled = false;
        }
    }
}

fn update_reset_after_duration_components(
    mut components: Query<(Entity, &mut ResetAfterDuration, &TargetResetController)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (controller_entity, mut reset_after_duration, controller) in components.iter_mut() {
        let is_outside_threshold =
            controller.controller.distance_to_target() >= DISTANCE_TO_TARGET_THRESHOLD;

        if is_outside_threshold {
            if let Some(since) = reset_after_duration.outside_threshold_since {
                if since + reset_after_duration.reset_after < time.elapsed() {
                    commands.trigger(EnableResetController {
                        entity: controller_entity,
                    });
                }
            } else {
                reset_after_duration.outside_threshold_since = Some(time.elapsed());
            }
        } else if reset_after_duration.outside_threshold_since.is_some() {
            reset_after_duration.outside_threshold_since = None;
        }
    }
}

fn on_enable_reset_controller(
    event: On<EnableResetController>,
    mut controllers: Query<&mut TargetResetController>,
) {
    let mut controller = controllers
        .get_mut(event.entity)
        .expect("EntityEvent should always point to existing entity.");
    controller.is_enabled = true;
}
