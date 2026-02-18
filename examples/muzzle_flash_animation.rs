use std::time::Duration;

use avian_first_person_shooter_test::{
    utilities::{UtilitiesPlugin, system_sets::InputSystems},
    world::weapons::muzzle_flash::{MuzzleFlashAnimation, MuzzleFlashImages, MuzzleFlashPlugin},
};
use bevy::{color::palettes::css::GREY, prelude::*};

const ANIMATION_DURATION: Duration = Duration::from_millis(40);
const IMAGE_SIZE: f32 = 1.0;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, UtilitiesPlugin, MuzzleFlashPlugin))
        .add_systems(Startup, (setup_world, spawn_muzzle_flash_image).chain())
        .add_systems(
            Update,
            (
                start_animation_on_click.in_set(InputSystems),
                rotate_muzzle_flash_animation_on_arrow_keys,
                draw_gizmos,
            ),
        )
        .run();
}

fn setup_world(mut commands: Commands) {
    commands.spawn((Camera3d::default(), Transform::from_xyz(0.0, 0.0, 1.5)));
}

fn spawn_muzzle_flash_image(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    muzzle_flash_images: Res<MuzzleFlashImages>,
) {
    let mesh_handle = meshes.add(Rectangle::from_length(IMAGE_SIZE));

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(muzzle_flash_images.get_image_at_index(0)),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        cull_mode: None,
        ..default()
    });

    commands.spawn((
        MuzzleFlashAnimation::new(ANIMATION_DURATION),
        Mesh3d(mesh_handle.clone()),
        MeshMaterial3d(material_handle),
        Transform::default(),
        Visibility::Hidden,
    ));
}

fn start_animation_on_click(
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut muzzle_flash_animations: Query<&mut MuzzleFlashAnimation>,
    time: Res<Time>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        for mut muzzle_flash_animation in muzzle_flash_animations.iter_mut() {
            muzzle_flash_animation.start_animation(&time);
        }
    }
}

fn rotate_muzzle_flash_animation_on_arrow_keys(
    input: Res<ButtonInput<KeyCode>>,
    mut transforms: Query<&mut Transform, With<MuzzleFlashAnimation>>,
) {
    for pressed in input.get_pressed() {
        let y_rotation_degrees: f32 = match *pressed {
            KeyCode::ArrowLeft => -1.0,
            KeyCode::ArrowRight => 1.0,
            _ => continue,
        };

        for mut transform in transforms.iter_mut() {
            transform.rotate_axis(Dir3::Y, y_rotation_degrees.to_radians());
        }
    }
}

fn draw_gizmos(mut gizmos: Gizmos, transforms: Query<&Transform, With<MuzzleFlashAnimation>>) {
    for transform in transforms.iter() {
        gizmos.rect(
            transform.to_isometry(),
            Vec2::splat(IMAGE_SIZE),
            GREY.with_alpha(0.05),
        );
    }
}
