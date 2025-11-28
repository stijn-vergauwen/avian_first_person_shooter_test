use avian3d::prelude::*;
use bevy::prelude::*;

use crate::utilities::system_sets::DataSystems;

pub struct GroundedPlugin;

impl Plugin for GroundedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            update_grounded_components.in_set(DataSystems::PrepareData),
        );
    }
}

/// Component that tracks if entity is grounded using raycast.
#[derive(Component, Clone)]
pub struct Grounded {
    /// The normal direction of the ground if grounded.
    ground_normal: Option<Dir3>,
    config: GroundedConfig,
}

impl Grounded {
    pub fn from_config(config: GroundedConfig) -> Self {
        Self {
            ground_normal: None,
            config,
        }
    }

    pub fn is_grounded(&self) -> bool {
        self.ground_normal.is_some()
    }
}

#[derive(Clone)]
pub struct GroundedConfig {
    /// Position offset for raycasting. If clipping is an issue, setting this to a value like 0.1 might help.
    pub raycast_height_offset: f32,
    pub max_distance: f32,
    pub query_filter: SpatialQueryFilter,
}

impl Default for GroundedConfig {
    fn default() -> Self {
        Self {
            raycast_height_offset: 0.1,
            max_distance: 0.5,
            query_filter: Default::default(),
        }
    }
}

fn update_grounded_components(
    mut components_query: Query<(&mut Grounded, &GlobalTransform)>,
    spatial_query: SpatialQuery,
) {
    for (mut grounded, global_transform) in components_query.iter_mut() {
        update_grounded_component(global_transform, &mut grounded, &spatial_query);
    }
}

fn update_grounded_component(
    global_transform: &GlobalTransform,
    grounded: &mut Grounded,
    spatial_query: &SpatialQuery,
) {
    let origin = global_transform.translation() + Vec3::Y * grounded.config.raycast_height_offset;
    let direction = global_transform.down();

    grounded.ground_normal = query_grounded(spatial_query, origin, direction, &grounded.config);
}

fn query_grounded(
    spatial_query: &SpatialQuery,
    origin: Vec3,
    direction: Dir3,
    config: &GroundedConfig,
) -> Option<Dir3> {
    let hit_data = spatial_query.cast_ray(
        origin,
        direction,
        config.max_distance,
        false,
        &config.query_filter,
    )?;

    Some(Dir3::new(hit_data.normal).unwrap())
}
