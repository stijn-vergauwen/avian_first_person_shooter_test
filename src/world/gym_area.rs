use avian3d::prelude::*;
use bevy::{color::palettes::tailwind::PURPLE_500, prelude::*};

use crate::utilities::DrawGizmos;

use super::shooting_targets::{StandingTargetAssets, spawn_falling_standing_target};

pub struct GymAreaPlugin;

impl Plugin for GymAreaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (spawn_static_entities, spawn_parkour_course_targets),
        )
        .add_systems(Update, draw_point_light_gizmos);
    }
}

fn spawn_static_entities(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SceneRoot(asset_server.load("models/Gym area.glb#Scene0")),
        Transform {
            translation: Vec3::new(0.0, 0.0, 10.0),
            ..default()
        },
        RigidBody::Static,
        ColliderConstructorHierarchy::new(ColliderConstructor::TrimeshFromMesh),
    ));

    commands.spawn((
        SceneRoot(asset_server.load("models/Outdoor table.glb#Scene0")),
        Transform::from_xyz(44.0, 0.0, 69.5),
        RigidBody::Dynamic,
        ColliderConstructorHierarchy::new(ColliderConstructor::TrimeshFromMesh)
            .with_default_density(ColliderDensity(600.0)),
    ));

    let light_positions = [
        Vec3::new(0.0, 5.0, 22.0),
        Vec3::new(28.0, 2.8, 44.0),
        Vec3::new(32.0, 2.8, 53.0),
        Vec3::new(40.0, 2.8, 53.0),
        Vec3::new(25.0, 7.0, 63.0),
        Vec3::new(30.0, 7.0, 63.0),
        Vec3::new(35.0, 7.0, 63.0),
        Vec3::new(40.0, 7.0, 63.0),
    ];

    for position in light_positions {
        commands.spawn((
            PointLight {
                shadows_enabled: true,
                ..default()
            },
            Transform::from_translation(position),
        ));
    }
}

fn spawn_parkour_course_targets(
    mut commands: Commands,
    standing_target_assets: Res<StandingTargetAssets>,
) {
    let standing_targets_spawn_data: Vec<(Vec3, f32)> = vec![
        (Vec3::new(27.0, 0.06, 53.2), 0.0),    // in hallway
        (Vec3::new(46.0, 0.06, 51.5), 90.0),   // in hallway
        (Vec3::new(42.3, 0.06, 55.8), -90.0), // next to doorway into large room
        (Vec3::new(37.2, 0.06, 62.8), -110.0), // behind container, visible through window
        (Vec3::new(34.5, 0.06, 65.5), -60.0),  // behind container, visible through window
        (Vec3::new(33.0, 4.16, 69.0), -60.0),  // on catwalk
        (Vec3::new(23.5, 1.06, 66.5), -60.0),  // behind boxes with ramp
    ];

    for spawn_data in standing_targets_spawn_data {
        spawn_falling_standing_target(
            &mut commands,
            &standing_target_assets,
            Transform {
                translation: spawn_data.0,
                rotation: Quat::from_axis_angle(Vec3::Y, spawn_data.1.to_radians()),
                ..default()
            },
            None,
        );
    }
}

fn draw_point_light_gizmos(
    mut gizmos: Gizmos,
    query: Query<&GlobalTransform, (With<PointLight>, With<DrawGizmos>)>,
) {
    for transform in query.iter() {
        gizmos.sphere(transform.to_isometry(), 0.2, PURPLE_500);
    }
}
