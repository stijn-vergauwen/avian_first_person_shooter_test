use bevy::{
    camera::{ImageRenderTarget, RenderTarget},
    core_pipeline::prepass::DepthPrepass,
    prelude::*,
    render::render_resource::{Face, TextureFormat},
};

use crate::world::TABLE_POSITION;

pub struct WallMirrorPlugin;

impl Plugin for WallMirrorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_wall_mirror);
    }
}

fn spawn_wall_mirror(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let position = TABLE_POSITION + Vec3::new(-0.49, 1.5, 0.0);

    let mirror_shape = Plane3d::new(Vec3::Z, Vec2::new(4.0, 1.5));

    let mirror_image = images.add(Image::new_target_texture(
        1600,
        600,
        TextureFormat::Rgba8UnormSrgb,
        Some(TextureFormat::Rgba8UnormSrgb),
    ));

    commands.spawn((
        Mesh3d(meshes.add(mirror_shape)),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(mirror_image.clone()),
            cull_mode: Some(Face::Front),
            unlit: true,
            ..default()
        })),
        Transform {
            translation: position,
            rotation: Quat::from_axis_angle(Vec3::Y, -90f32.to_radians()),
            ..default()
        },
    ));

    commands.spawn((
        Camera3d::default(),
        RenderTarget::Image(ImageRenderTarget::from(mirror_image)),
        DepthPrepass,
        Transform {
            translation: position,
            rotation: Quat::from_axis_angle(Vec3::Y, -90f32.to_radians()),
            ..default()
        },
    ));
}
