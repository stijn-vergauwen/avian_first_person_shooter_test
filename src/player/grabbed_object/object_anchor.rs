use bevy::{color::palettes::tailwind::PURPLE_400, prelude::*};

use crate::{
    player::PlayerCamera,
    utilities::{
        DrawGizmos,
        system_sets::{DataSystems, DisplaySystems},
    },
    world::character::CharacterHead,
};

use super::GrabbedObject;

const ANCHOR_OFFSETS: AnchorOffsets = AnchorOffsets {
    inspecting: Vec3::new(0.0, 0.0, -1.2),
    default: Vec3::new(0.3, -0.3, -1.0),
    aim_down_sight: Vec3::new(0.01, -0.04, -0.25),
};

pub struct ObjectAnchorPlugin;

impl Plugin for ObjectAnchorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ANCHOR_OFFSETS)
            .add_systems(
                FixedUpdate,
                update_anchor_positions.in_set(DataSystems::PrepareData),
            )
            .add_systems(
                Update,
                draw_grabbed_object_anchor_position.in_set(DisplaySystems),
            );
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
pub struct CalculatedAnchorValues {
    default: Isometry3d,
    inspecting: Isometry3d,
    aim_down_sight: Isometry3d,
}

impl CalculatedAnchorValues {
    pub fn get_from_object_anchor(&self, object_anchor: ObjectAnchor) -> Isometry3d {
        match object_anchor {
            ObjectAnchor::Default => self.default,
            ObjectAnchor::Inspecting => self.inspecting,
            ObjectAnchor::AimDownSight => self.aim_down_sight,
        }
    }
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

// Gizmos

fn draw_grabbed_object_anchor_position(
    grabbed_object: Single<&GrabbedObject, With<DrawGizmos>>,
    mut gizmos: Gizmos,
) {
    gizmos.sphere(
        grabbed_object.current_anchor_value(),
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
