use avian3d::prelude::LinearVelocity;
use bevy::prelude::*;

use crate::utilities::system_sets::DataSystems;

pub struct ItemAnchorPlugin;

impl Plugin for ItemAnchorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            update_target_item_position.in_set(DataSystems::UpdateEntities),
        );
    }
}

/// Marker component for the anchor that is used to position objects held by the player.
#[derive(Component, Clone, Copy, Default)]
pub struct ItemAnchor {
    pub target_item_entity: Option<Entity>,
}

fn update_target_item_position(
    item_anchor: Single<(&ItemAnchor, &GlobalTransform)>,
    mut target_item_query: Query<(&mut Transform, Option<&mut LinearVelocity>), Without<ItemAnchor>>,
) {
    let Some(target_item_entity) = item_anchor.0.target_item_entity else {
        return;
    };

    let mut target_item = target_item_query
        .get_mut(target_item_entity)
        .expect("ItemAnchor should always point to existing entity or None.");

    target_item.0.translation = item_anchor.1.translation();
    target_item.0.rotation = item_anchor.1.rotation();

    // Reset linear velocity as temp fix for rigidbody movement issue
    if let Some(mut velocity) = target_item.1 {
        velocity.0 = Vec3::ZERO;
    }
}
