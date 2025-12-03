use avian3d::prelude::LinearVelocity;
use bevy::{color::palettes::tailwind::PURPLE_400, prelude::*};

use crate::{
    utilities::{
        DrawGizmos,
        system_sets::{DataSystems, DisplaySystems, InputSystems},
    },
    world::{grabbable_object::GrabbableObject, interaction_target::PlayerInteractionTarget},
};

pub struct GrabbedObjectPlugin;

impl Plugin for GrabbedObjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                grab_object_on_keypress.in_set(InputSystems),
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
#[derive(Component, Clone, Copy, Default)]
pub struct GrabbedObject {
    pub entity: Option<Entity>,
}

fn grab_object_on_keypress(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut item_anchor: Single<&mut GrabbedObject>,
    player_interaction_target: Res<PlayerInteractionTarget>,
    grabbable_query: Query<&GrabbableObject>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyE) {
        item_anchor.entity = None;

        if let Some(target) = player_interaction_target.current_target()
            && grabbable_query.contains(target.entity)
        {
            item_anchor.entity = Some(target.entity)
        }
    }
}

fn update_grabbed_object_position(
    item_anchor: Single<(&GrabbedObject, &GlobalTransform)>,
    mut target_item_query: Query<
        (&mut Transform, Option<&mut LinearVelocity>),
        Without<GrabbedObject>,
    >,
) {
    let Some(target_item_entity) = item_anchor.0.entity else {
        return;
    };

    let mut target_item = target_item_query
        .get_mut(target_item_entity)
        .expect("ItemAnchor should always point to existing entity or None.");

    target_item.0.translation = item_anchor.1.translation();
    target_item.0.rotation = item_anchor.1.rotation();

    // Reset linear velocity as temp fix for rigidbody movement issue
    if let Some(mut velocity) = target_item.1 {
        velocity.0 = Vec3::ZERO;
    }
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
