pub mod angle;
mod app_state;
pub mod euler_angle;
pub mod fraction;
pub mod pd_controller;
pub mod quaternion_pd_controller;
pub mod system_sets;

use bevy::{
    prelude::*,
    window::{CursorIcon, PrimaryWindow},
};

use crate::utilities::{app_state::AppStatePlugin, system_sets::SystemSetPlugin};

pub struct UtilitiesPlugin;

impl Plugin for UtilitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((AppStatePlugin, SystemSetPlugin))
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
