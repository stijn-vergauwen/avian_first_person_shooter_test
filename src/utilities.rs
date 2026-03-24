pub mod angle;
mod app_state;
pub mod despawn_after_sleep;
pub mod euler_angle;
pub mod pd_controller;
pub mod quaternion_pd_controller;
pub mod system_sets;

use bevy::{
    asset::AssetPath,
    image::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor},
    prelude::*,
    window::{CursorIcon, PrimaryWindow},
};
use despawn_after_sleep::DespawnAfterSleepPlugin;

use crate::utilities::{app_state::AppStatePlugin, system_sets::SystemSetPlugin};

pub struct UtilitiesPlugin;

impl Plugin for UtilitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((AppStatePlugin, SystemSetPlugin, DespawnAfterSleepPlugin))
            .add_systems(Startup, add_cursor_icon_component_to_primary_window);
    }
}

fn add_cursor_icon_component_to_primary_window(
    mut commands: Commands,
    primary_window: Single<Entity, With<PrimaryWindow>>,
) {
    commands
        .entity(*primary_window)
        .insert(CursorIcon::default());
}

/// Marker component for entities whos gizmos should be drawn.
#[derive(Component, Clone, Copy)]
pub struct DrawGizmos;

/// Loads an image file at the given path using the AssetServer, with loader settings to repeat the image on both axes to be used as a tiling texture.
///
/// To scale this texture on your mesh, set the `uv_transform` property's scale on StandardMaterial, a higher scale value means the texture will be tiled more / smaller.
pub fn load_repeating_texture<'a>(
    asset_server: &AssetServer,
    path: impl Into<AssetPath<'a>>,
) -> Handle<Image> {
    asset_server.load_with_settings(path, |s: &mut _| {
        *s = ImageLoaderSettings {
            sampler: ImageSampler::Descriptor(ImageSamplerDescriptor {
                // rewriting mode to repeat image,
                address_mode_u: ImageAddressMode::Repeat,
                address_mode_v: ImageAddressMode::Repeat,
                ..default()
            }),
            ..default()
        }
    })
}
