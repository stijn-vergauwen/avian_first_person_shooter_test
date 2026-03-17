use bevy::prelude::*;

use crate::{
    player::grabbed_object::GrabbedObject,
    utilities::system_sets::InputSystems,
    world::weapons::{
        Weapon,
        shooting::{PullTrigger, ReleaseTrigger},
    },
};

use super::grabbed_object::HoldPosition;

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
    grabbed_object: Res<GrabbedObject>,
    hold_position: Res<HoldPosition>,
    weapons_query: Query<&Weapon>,
    mut commands: Commands,
) {
    if *hold_position != HoldPosition::Inspecting
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
    weapons_query: Query<&Weapon>,
    mut grabbed_object: ResMut<GrabbedObject>,
    mut hold_position: ResMut<HoldPosition>,
) {
    let set_is_aiming = mouse_input.pressed(MouseButton::Right)
        && grabbed_object
            .entity
            .is_some_and(|grabbed_entity| weapons_query.contains(grabbed_entity));

    *hold_position = match (*hold_position, set_is_aiming) {
        (HoldPosition::PrimaryHand, true) => HoldPosition::AimDownSight,
        (HoldPosition::AimDownSight, false) => HoldPosition::PrimaryHand,
        _ => return,
    };

    grabbed_object.switch_controller_config(set_is_aiming);
}
