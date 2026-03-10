use avian3d::prelude::CollisionHooks;
use bevy::{ecs::system::SystemParam, prelude::*};

use crate::{
    player::{PlayerHeadMesh, grabbed_object::GrabbedObject},
    world::weapons::Weapon,
};

// Custom collision hook to prevent collision between player head and grabbed weapon

#[derive(SystemParam)]
pub struct PlayerWeaponCollisionHooks<'w, 's> {
    player_head_mesh: Single<'w, 's, Entity, With<PlayerHeadMesh>>,
    grabbed_object: Single<'w, 's, &'static GrabbedObject>,
    weapons_query: Query<'w, 's, &'static Weapon>,
}

impl CollisionHooks for PlayerWeaponCollisionHooks<'_, '_> {
    fn filter_pairs(&self, collider1: Entity, collider2: Entity, _: &mut Commands) -> bool {
        let colliders = [collider1, collider2];

        if colliders.contains(&self.player_head_mesh)
            && let Some(grabbed_entity) = self.grabbed_object.entity
            && self.weapons_query.contains(grabbed_entity)
        {
            return !colliders.contains(&grabbed_entity);
        }

        true
    }
}
