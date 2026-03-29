use avian3d::prelude::*;
use bevy::prelude::*;

pub struct GymAreaPlugin;

impl Plugin for GymAreaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_static_entities);
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
}
