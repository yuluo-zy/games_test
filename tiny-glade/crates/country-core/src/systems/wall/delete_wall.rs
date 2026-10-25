//! B 0x140aac1e0: publication has already removed the public geometry record;
//! this consumer removes the roof association and despawns wall/plaster entities.

use crate::resources::walls::public_walls::{
    OnWallDeleted, PublicWalls, WallEntities, WallPlasters,
};
use bevy_ecs::prelude::*;

pub fn delete_wall(
    mut deleted: EventReader<OnWallDeleted>,
    mut public: ResMut<PublicWalls>,
    mut plasters: ResMut<WallPlasters>,
    mut entities: ResMut<WallEntities>,
    mut commands: Commands,
) {
    for &OnWallDeleted(id) in deleted.read() {
        public.remove_roof(id);
        if let Some(entity) = entities.0.remove(&id) {
            commands.entity(entity).despawn();
        }
        if let Some(children) = plasters.0.remove(&id) {
            for entity in children.into_iter().flatten() {
                commands.entity(entity).despawn();
            }
        }
    }
}
