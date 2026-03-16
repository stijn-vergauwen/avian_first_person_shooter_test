use bevy::prelude::*;

use crate::{
    player::grabbed_object::GrabbedObject,
    utilities::system_sets::InputSystems,
    world::weapons::{
        Weapon,
        shooting::{PullTrigger, ReleaseTrigger},
    },
};

use super::grabbed_object::object_anchor::ObjectAnchor;

pub struct GrabbedWeaponPlugin;

impl Plugin for GrabbedWeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (toggle_weapon_trigger, aim_down_sight).in_set(InputSystems),
        );
    }
}

fn toggle_weapon_trigger(
    mouse_input: Res<ButtonInput<MouseButton>>,
    grabbed_object: Single<&GrabbedObject>,
    weapons_query: Query<&Weapon>,
    mut commands: Commands,
) {
    if grabbed_object.current_object_anchor != ObjectAnchor::Inspecting
        && let Some(grabbed_entity) = grabbed_object.entity
        && weapons_query.contains(grabbed_entity)
    {
        if mouse_input.just_pressed(MouseButton::Left) {
            commands.trigger(PullTrigger {
                entity: grabbed_entity,
            });
        };

        if mouse_input.just_released(MouseButton::Left) {
            commands.trigger(ReleaseTrigger {
                entity: grabbed_entity,
            });
        };
    }
}

fn aim_down_sight(
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut grabbed_object: Single<&mut GrabbedObject>,
    weapons_query: Query<&Weapon>,
) {
    let set_is_aiming = mouse_input.pressed(MouseButton::Right)
        && grabbed_object
            .entity
            .is_some_and(|grabbed_entity| weapons_query.contains(grabbed_entity));

    grabbed_object.current_object_anchor =
        match (grabbed_object.current_object_anchor, set_is_aiming) {
            (ObjectAnchor::Default, true) => ObjectAnchor::AimDownSight,
            (ObjectAnchor::AimDownSight, false) => ObjectAnchor::Default,
            _ => return,
        };

    grabbed_object.switch_controller_config(set_is_aiming);
}
