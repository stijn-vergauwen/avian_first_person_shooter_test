use avian3d::prelude::{Forces, RigidBodyForces};
use bevy::{color::palettes::tailwind::PURPLE_400, prelude::*};

use crate::{
    player::Player,
    utilities::{
        DrawGizmos,
        pd_controller::{PdController, config::PdControllerConfig},
        system_sets::{DataSystems, DisplaySystems, InputSystems},
    },
    world::{
        grabbable_object::GrabbableObject,
        interaction_target::PlayerInteractionTarget,
        weapons::{ShootWeapon, Weapon},
    },
};

pub struct GrabbedObjectPlugin;

impl Plugin for GrabbedObjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                (grab_object_on_keypress, shoot_held_weapon).in_set(InputSystems),
                draw_grabbed_object_anchor_position.in_set(DisplaySystems),
            ),
        )
        .add_systems(
            FixedUpdate,
            update_grabbed_object_position.in_set(DataSystems::UpdateEntities),
        );
    }
}

/// Holds data on the object held by the player.
#[derive(Component, Clone)]
pub struct GrabbedObject {
    pub entity: Option<Entity>,
    position_force_controller: PdController<Vec3>,
}

impl GrabbedObject {
    pub fn new(position_force_controller_config: PdControllerConfig) -> Self {
        Self {
            entity: None,
            position_force_controller: PdController::new(position_force_controller_config),
        }
    }
}

fn grab_object_on_keypress(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut grabbed_object: Single<&mut GrabbedObject>,
    player_interaction_target: Res<PlayerInteractionTarget>,
    grabbable_query: Query<&GrabbableObject>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyE) {
        grabbed_object.entity = None;

        if let Some(target) = player_interaction_target.current_target()
            && grabbable_query.contains(target.entity)
        {
            grabbed_object.entity = Some(target.entity);
        }
    }
}

#[allow(clippy::type_complexity)]
fn update_grabbed_object_position(
    mut grabbed_object: Single<(&mut GrabbedObject, &GlobalTransform)>,
    mut player: Single<Forces, With<Player>>,
    mut target_item_query: Query<(&GlobalTransform, Forces), (Without<GrabbedObject>, Without<Player>)>,
    time: Res<Time>,
) {
    let Some(target_item_entity) = grabbed_object.0.entity else {
        return;
    };

    let mut target_item = target_item_query.get_mut(target_item_entity).expect(
        "GrabbedObject should always point to existing entity with RigidBody component, or None.",
    );

    let target_position = grabbed_object.1.translation();
    let position_controller = &mut grabbed_object.0.position_force_controller;

    position_controller.set_target_position(target_position);
    position_controller.set_position(target_item.0.translation());
    position_controller.set_velocity(target_item.1.linear_velocity());
    position_controller.update(time.delta_secs());

    // Apply position force to grabbed object
    target_item
        .1
        .apply_force(position_controller.acceleration());

    // Apply opposite position force to player
    player.apply_force(-position_controller.acceleration());
}

fn shoot_held_weapon(
    mouse_input: Res<ButtonInput<MouseButton>>,
    grabbed_object: Single<&GrabbedObject>,
    weapons_query: Query<&Weapon>,
    mut commands: Commands,
) {
    if mouse_input.just_pressed(MouseButton::Left)
        && let Some(grabbed_entity) = grabbed_object.entity
        && weapons_query.contains(grabbed_entity)
    {
        commands.trigger(ShootWeapon {
            entity: grabbed_entity,
        });
    };
}

fn draw_grabbed_object_anchor_position(
    tool_anchor: Single<&GlobalTransform, (With<GrabbedObject>, With<DrawGizmos>)>,
    mut gizmos: Gizmos,
) {
    gizmos.sphere(
        tool_anchor.compute_transform().to_isometry(),
        0.2,
        PURPLE_400,
    );
}
