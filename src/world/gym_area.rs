use avian3d::prelude::*;
use bevy::{color::palettes::tailwind::PURPLE_500, prelude::*};

use crate::utilities::DrawGizmos;

pub struct GymAreaPlugin;

impl Plugin for GymAreaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_static_entities)
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
            DrawGizmos,
        ));
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
