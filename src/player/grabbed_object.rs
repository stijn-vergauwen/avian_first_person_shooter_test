mod inspector_mode;

use avian3d::prelude::{ComputedMass, Forces, RigidBodyForces, TransformInterpolation};
use bevy::{color::palettes::tailwind::PURPLE_400, prelude::*};

use crate::{
    player::{Player, PlayerCamera, grabbed_object::inspector_mode::InspectorModePlugin},
    utilities::{
        DrawGizmos,
        pd_controller::{PdController, config::PdControllerConfig},
        quaternion_pd_controller::QuaternionPdController,
        system_sets::{DataSystems, DisplaySystems, InputSystems},
    },
    world::{
        character::{Character, CharacterHead},
        grabbable_object::GrabOrientation,
        interaction_target::PlayerInteractionTarget,
    },
};

const ANCHOR_OFFSETS: AnchorOffsets = AnchorOffsets {
    inspecting: Vec3::new(0.0, 0.0, -1.2),
    default: Vec3::new(0.3, -0.3, -1.0),
    aim_down_sight: Vec3::new(0.01, -0.04, -0.25),
};

pub struct GrabbedObjectPlugin;

impl Plugin for GrabbedObjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InspectorModePlugin)
            .insert_resource(ANCHOR_OFFSETS)
            .add_systems(
                Update,
                (
                    grab_object_on_keypress.in_set(InputSystems),
                    draw_grabbed_object_anchor_position.in_set(DisplaySystems),
                ),
            )
            .add_systems(
                FixedUpdate,
                (
                    update_anchor_positions.in_set(DataSystems::PrepareData),
                    (
                        update_grabbed_object_position,
                        update_grabbed_object_rotation,
                    )
                        .in_set(DataSystems::UpdateEntities),
                ),
            )
            .add_observer(on_update_player_character_active)
            .add_observer(on_grab_object)
            .add_observer(on_drop_object);
    }
}

/// Holds data on the object held by the player.
#[derive(Component, Clone)]
pub struct GrabbedObject {
    pub entity: Option<Entity>,
    position_force_controller: PdController<Vec3>,
    rotation_force_controller: QuaternionPdController,
    anchor_values: CalculatedAnchorValues,
    pub current_object_anchor: ObjectAnchor,
}

impl GrabbedObject {
    pub fn new(
        position_force_controller_config: PdControllerConfig,
        rotation_force_controller_config: PdControllerConfig,
    ) -> Self {
        Self {
            entity: None,
            position_force_controller: PdController::new(position_force_controller_config),
            rotation_force_controller: QuaternionPdController::new(
                rotation_force_controller_config,
            ),
            anchor_values: CalculatedAnchorValues::default(),
            current_object_anchor: ObjectAnchor::Default,
        }
    }

    fn current_anchor_value(&self) -> Isometry3d {
        self.anchor_values
            .get_from_object_anchor(self.current_object_anchor)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ObjectAnchor {
    Default,
    Inspecting,
    AimDownSight,
}

#[derive(Resource, Clone, Copy)]
struct AnchorOffsets {
    default: Vec3,
    inspecting: Vec3,
    aim_down_sight: Vec3,
}

#[derive(Clone, Copy, Default)]
struct CalculatedAnchorValues {
    default: Isometry3d,
    inspecting: Isometry3d,
    aim_down_sight: Isometry3d,
}

impl CalculatedAnchorValues {
    fn get_from_object_anchor(&self, object_anchor: ObjectAnchor) -> Isometry3d {
        match object_anchor {
            ObjectAnchor::Default => self.default,
            ObjectAnchor::Inspecting => self.inspecting,
            ObjectAnchor::AimDownSight => self.aim_down_sight,
        }
    }
}

#[derive(EntityEvent, Clone, Copy)]
struct GrabObject {
    entity: Entity,
}

#[derive(EntityEvent, Clone, Copy)]
struct DropObject {
    entity: Entity,
}

#[derive(EntityEvent, Copy, Clone)]
pub struct UpdatePlayerCharacterActive {
    pub entity: Entity,
}

fn update_anchor_positions(
    mut grabbed_object: Single<&mut GrabbedObject>,
    offsets: Res<AnchorOffsets>,
    player_head: Single<&GlobalTransform, With<CharacterHead>>,
    player_camera: Single<&GlobalTransform, With<PlayerCamera>>,
) {
    grabbed_object.anchor_values.inspecting =
        calculate_anchor_position(&player_camera, offsets.inspecting);

    grabbed_object.anchor_values.default = calculate_anchor_position(&player_head, offsets.default);

    grabbed_object.anchor_values.aim_down_sight =
        calculate_anchor_position(&player_camera, offsets.aim_down_sight);
}

fn on_update_player_character_active(
    update_player_character_active: On<UpdatePlayerCharacterActive>,
    mut characters_query: Query<&mut Character>,
    grabbed_object: Single<&GrabbedObject>,
) {
    let Ok(mut character) = characters_query.get_mut(update_player_character_active.entity) else {
        return;
    };

    character.is_active = grabbed_object.current_object_anchor != ObjectAnchor::Inspecting;
}

fn grab_object_on_keypress(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    grabbed_object: Single<&GrabbedObject>,
    player_interaction_target: Res<PlayerInteractionTarget>,
    mut commands: Commands,
) {
    if keyboard_input.just_pressed(KeyCode::KeyE) {
        if let Some(entity) = grabbed_object.entity {
            commands.trigger(DropObject { entity });
        }

        if let Some(target) = player_interaction_target.current_target() {
            commands.trigger(GrabObject {
                entity: target.entity,
            });
        }
    }
}

fn on_grab_object(
    event: On<GrabObject>,
    mut grabbed_object: Single<&mut GrabbedObject>,
    grab_orientations: Query<&GrabOrientation>,
    mut commands: Commands,
) {
    let Ok(grab_orientation) = grab_orientations.get(event.entity) else {
        return;
    };

    grabbed_object.entity = Some(event.entity);

    // Add interpolation
    commands.entity(event.entity).insert(TransformInterpolation);

    // Set force controllers to new start values
    let target_isometry = grabbed_object.anchor_values.default;
    grabbed_object
        .position_force_controller
        .set_start_position(target_isometry.translation.into());
    grabbed_object
        .rotation_force_controller
        .set_start_position(target_isometry.rotation * grab_orientation.value());
}

fn on_drop_object(
    event: On<DropObject>,
    mut grabbed_object: Single<&mut GrabbedObject>,
    mut commands: Commands,
) {
    grabbed_object.entity = None;

    // Remove interpolation
    commands
        .entity(event.entity)
        .remove::<TransformInterpolation>();
}

fn update_grabbed_object_position(
    mut grabbed_object: Single<&mut GrabbedObject>,
    mut grabbable_objects: Query<(&GlobalTransform, Forces, &ComputedMass), Without<Player>>,
    mut player: Single<Forces, With<Player>>,
    time: Res<Time>,
) {
    let Some(grabbed_entity) = grabbed_object.entity else {
        return;
    };

    let (global_transform, mut forces, computed_mass) = grabbable_objects.get_mut(grabbed_entity).expect(
        "GrabbedObject should always point to existing entity with RigidBody component, or None.",
    );

    let target_position = grabbed_object.current_anchor_value().translation.to_vec3();

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

    grabbed_object
        .rotation_force_controller
        .set_target_position(player_rotation * grab_orientation.value());

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

// Gizmos

fn draw_grabbed_object_anchor_position(
    grabbed_object: Single<&GlobalTransform, (With<GrabbedObject>, With<DrawGizmos>)>,
    mut gizmos: Gizmos,
) {
    gizmos.sphere(
        grabbed_object.compute_transform().to_isometry(),
        0.2,
        PURPLE_400,
    );
}

// Utilities

fn calculate_anchor_position(
    global_transform: &GlobalTransform,
    grabbed_object_offset: Vec3,
) -> Isometry3d {
    Isometry3d {
        translation: (global_transform.translation()
            + global_transform.rotation() * grabbed_object_offset)
            .into(),
        rotation: global_transform.rotation(),
    }
}
