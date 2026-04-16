pub mod character;
pub mod desired_movement;
pub mod desired_rotation;
pub mod grabbable_object;
pub mod grounded;
mod gym_area;
pub mod interaction_target;
mod shooting_range_area;
pub mod shooting_targets;
pub mod skybox;
mod wall_mirror;
pub mod weapons;

use std::f32::consts::PI;

use avian3d::prelude::*;
use bevy::{
    color::palettes::tailwind::*,
    light::{CascadeShadowConfigBuilder, DirectionalLightShadowMap, NotShadowCaster},
    math::Affine2,
    prelude::*,
};
use grabbable_object::GrabbableObjectPlugin;
use rand::RngExt;
use shooting_targets::ShootingTargetsPlugin;
use skybox::SkyboxPlugin;

use crate::{
    utilities::load_repeating_texture,
    world::{
        character::CharacterPlugin,
        desired_movement::DesiredMovementPlugin,
        desired_rotation::DesiredRotationPlugin,
        grabbable_object::{DefaultGrabOrientation, GrabbableObject},
        grounded::GroundedPlugin,
        gym_area::GymAreaPlugin,
        interaction_target::InteractionTargetPlugin,
        shooting_range_area::ShootingRangeAreaPlugin,
        wall_mirror::WallMirrorPlugin,
        weapons::WeaponsPlugin,
    },
};

const TABLE_POSITION: Vec3 = Vec3::new(-9.3, 1.0, 0.0);

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            CharacterPlugin,
            DesiredMovementPlugin,
            DesiredRotationPlugin,
            WeaponsPlugin,
            GrabbableObjectPlugin,
            InteractionTargetPlugin,
            GroundedPlugin,
            WallMirrorPlugin,
            ShootingRangeAreaPlugin,
            GymAreaPlugin,
            SkyboxPlugin,
            ShootingTargetsPlugin,
        ))
        .insert_resource(GlobalAmbientLight {
            color: Color::from(BLUE_300),
            ..default()
        })
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .add_systems(PreStartup, setup_prototype_textures)
        .add_systems(
            Startup,
            (
                spawn_static_entities,
                spawn_dynamic_entities,
                spawn_radio,
                spawn_test_table,
            ),
        );
    }
}

#[derive(Resource)]
struct PrototypeTextures {
    grid: Handle<Image>,
    #[allow(unused)]
    checker: Handle<Image>,
}

fn setup_prototype_textures(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.insert_resource(PrototypeTextures {
        grid: load_repeating_texture(&asset_server, "textures/Grid texture.png"),
        checker: load_repeating_texture(&asset_server, "textures/Checker texture.png"),
    });
}

fn spawn_static_entities(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    prototype_textures: Res<PrototypeTextures>,
) {
    // Ground plane
    let ground_shape = Cuboid::new(600.0, 1.0, 600.0);
    let texture_scale = 2.0;

    commands.spawn((
        Mesh3d(meshes.add(ground_shape)),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::from(STONE_300),
            base_color_texture: Some(prototype_textures.grid.clone()),
            perceptual_roughness: 1.0,
            uv_transform: Affine2::from_scale(ground_shape.size().xz() / texture_scale),
            ..default()
        })),
        RigidBody::Static,
        Collider::from(ground_shape),
        Transform::from_xyz(0.0, -ground_shape.half_size.y, 0.0),
        NotShadowCaster,
    ));

    // Wall
    let wall_shape = Cuboid::new(40.0, 5.0, 0.4);

    commands.spawn((
        Mesh3d(meshes.add(wall_shape)),
        MeshMaterial3d(materials.add(Color::from(STONE_500))),
        RigidBody::Static,
        Collider::from(wall_shape),
        Transform {
            translation: Vec3::new(-10.0, wall_shape.half_size.y, 0.0),
            rotation: Quat::from_axis_angle(Vec3::Y, 90f32.to_radians()),
            ..default()
        },
    ));

    // Table
    let table_shape = Cuboid::new(10.0, 0.1, 1.0);

    commands.spawn((
        Mesh3d(meshes.add(table_shape)),
        MeshMaterial3d(materials.add(Color::from(STONE_400))),
        RigidBody::Static,
        Collider::from(table_shape),
        Transform {
            translation: TABLE_POSITION,
            rotation: Quat::from_axis_angle(Vec3::Y, 90f32.to_radians()),
            ..default()
        },
    ));

    // Light
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            illuminance: light_consts::lux::AMBIENT_DAYLIGHT,
            ..default()
        },
        CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 100.0,
            ..default()
        }
        .build(),
        Transform {
            translation: Vec3::new(0.0, 20.0, 0.0),
            rotation: Quat::from_euler(EulerRot::YXZ, 10f32.to_radians(), -50f32.to_radians(), 0.0),
            ..default()
        },
    ));
}

fn spawn_dynamic_entities(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    prototype_textures: Res<PrototypeTextures>,
) {
    // Tower of cubes
    let cube_shape = Cuboid::new(1.0, 1.0, 1.0);
    let cube_mesh = meshes.add(cube_shape);
    let cube_material = materials.add(StandardMaterial {
        base_color: Color::from(AMBER_400),
        base_color_texture: Some(prototype_textures.grid.clone()),
        ..default()
    });
    let spawn_count = 10;
    let spawn_position = Vec3::new(10.0, 1.0, -10.0);

    for index in 0..spawn_count {
        commands.spawn((
            Mesh3d(cube_mesh.clone()),
            MeshMaterial3d(cube_material.clone()),
            RigidBody::Dynamic,
            Collider::from(cube_shape),
            ColliderDensity(80.0),
            Transform::from_translation(spawn_position + Vec3::Y * index as f32 * 1.2),
        ));
    }

    // Pile of grabbable cubes
    let cube_shape = Cuboid::from_length(0.3);
    let cube_material = materials.add(StandardMaterial {
        base_color: Color::from(PURPLE_700),
        base_color_texture: Some(prototype_textures.grid.clone()),
        ..default()
    });
    let cube_mesh = meshes.add(cube_shape);
    let mut rng = rand::rng();
    let spawn_count = 100;
    let spawn_position = Vec3::new(3.0, 0.0, -6.0);
    let horizontal_spread = 0.3;

    for index in 0..spawn_count {
        let vertical_offset = Vec3::Y * index as f32 * cube_shape.half_size.y;
        let horizontal_offset = Vec3::new(
            rng.random_range(-horizontal_spread..horizontal_spread),
            0.0,
            rng.random_range(-horizontal_spread..horizontal_spread),
        );

        commands.spawn((
            GrabbableObject,
            Mesh3d(cube_mesh.clone()),
            MeshMaterial3d(cube_material.clone()),
            RigidBody::Dynamic,
            Collider::from(cube_shape),
            ColliderDensity(200.0),
            Restitution::new(0.8),
            Transform::from_translation(spawn_position + vertical_offset + horizontal_offset),
            MaxAngularSpeed(40.0),
            TranslationInterpolation,
        ));
    }
}

fn spawn_radio(mut commands: Commands, asset_server: Res<AssetServer>) {
    let radio_model = asset_server.load("models/Radio.glb#Scene0");

    commands.spawn((
        GrabbableObject,
        DefaultGrabOrientation(Quat::from_axis_angle(Vec3::Y, PI)),
        SceneRoot(radio_model),
        Transform {
            translation: TABLE_POSITION + Vec3::new(0.0, 0.5, 4.0),
            rotation: Quat::from_axis_angle(Vec3::Y, -90f32.to_radians()),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(0.4, 0.2, 0.15),
        Mass(5.0),
        MaxAngularSpeed(40.0),
    ));
}

fn spawn_test_table(mut commands: Commands, asset_server: Res<AssetServer>) {
    let table_model = asset_server.load("models/Outdoor table.glb#Scene0");

    commands.spawn((
        SceneRoot(table_model),
        Transform::from_xyz(-6.0, 0.0, -8.0),
        RigidBody::Dynamic,
        ColliderConstructorHierarchy::new(ColliderConstructor::TrimeshFromMesh)
            .with_default_density(ColliderDensity(600.0)),
    ));
}
