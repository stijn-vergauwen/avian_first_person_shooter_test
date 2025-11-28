use avian3d::prelude::*;
use bevy::prelude::*;

use crate::utilities::system_sets::DataSystems;

pub struct InteractionTargetPlugin;

impl Plugin for InteractionTargetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                update_current_interaction_target,
                update_player_interaction_target,
            )
                .chain()
                .in_set(DataSystems::PrepareData),
        );
    }
}

/// Easy access to the InteractionTarget of the player when there is only 1 player.
#[derive(Resource, Clone, Copy)]
pub struct PlayerInteractionTarget {
    /// Should point to the entity from which you want player interaction raycasts to happen. For example the player camera entity.
    player_interaction_entity: Entity,
    /// A copy of the latest InteractionTarget data in the CurrentInteractionTarget component.
    current_target: Option<InteractionTarget>,
}

impl PlayerInteractionTarget {
    pub fn new(player_interaction_entity: Entity) -> Self {
        Self {
            player_interaction_entity,
            current_target: None,
        }
    }

    pub fn current_target(&self) -> Option<InteractionTarget> {
        self.current_target
    }
}

/// Component that tracks current InteractionTarget using raycast.
#[derive(Component, Clone)]
pub struct CurrentInteractionTarget {
    target: Option<InteractionTarget>,
    config: InteractionTargetConfig,
}

impl CurrentInteractionTarget {
    pub fn from_config(config: InteractionTargetConfig) -> Self {
        Self {
            target: None,
            config,
        }
    }
}

#[derive(Clone, Copy)]
pub struct InteractionTarget {
    pub entity: Entity,
}

#[derive(Clone)]
pub struct InteractionTargetConfig {
    pub max_distance: f32,
    pub query_filter: SpatialQueryFilter,
}

fn update_current_interaction_target(
    mut components_query: Query<(&mut CurrentInteractionTarget, &GlobalTransform)>,
    spatial_query: SpatialQuery,
) {
    for (mut current_interaction_target, global_transform) in components_query.iter_mut() {
        let origin = global_transform.translation();
        let direction = global_transform.forward();

        current_interaction_target.target = query_target(
            &spatial_query,
            origin,
            direction,
            &current_interaction_target.config,
        );
    }
}

fn update_player_interaction_target(
    player_interaction_target: Option<ResMut<PlayerInteractionTarget>>,
    current_interaction_targets_query: Query<&CurrentInteractionTarget>,
) {
    if let Some(mut player_interaction_target) = player_interaction_target {
        player_interaction_target.current_target = current_interaction_targets_query
            .get(player_interaction_target.player_interaction_entity)
            .expect("Entity reference should always be valid.")
            .target;
    }
}

// Utilities

fn query_target(
    spatial_query: &SpatialQuery,
    origin: Vec3,
    direction: Dir3,
    config: &InteractionTargetConfig,
) -> Option<InteractionTarget> {
    let hit_data = spatial_query.cast_ray(
        origin,
        direction,
        config.max_distance,
        false,
        &config.query_filter,
    )?;

    let target = InteractionTarget {
        entity: hit_data.entity,
    };

    Some(target)
}
