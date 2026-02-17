use bevy::prelude::*;

use crate::{
    player::grabbed_object::GrabbedObject,
    utilities::system_sets::InputSystems,
    world::weapons::{ShootWeapon, Weapon},
};

pub struct GrabbedWeaponPlugin;

impl Plugin for GrabbedWeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (shoot_held_weapon, aim_down_sight).in_set(InputSystems),
        );
    }
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

fn aim_down_sight(
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut grabbed_object: Single<&mut GrabbedObject>,
    weapons_query: Query<&Weapon>,
) {
    let is_aiming = mouse_input.pressed(MouseButton::Right)
        && grabbed_object
            .entity
            .is_some_and(|grabbed_entity| weapons_query.contains(grabbed_entity));

    grabbed_object.is_aiming = is_aiming;
}
