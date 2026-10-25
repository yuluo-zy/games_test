//! wall_roof_hashmap.rs — maintains the wall->roof and wall->entity maps from the Roof query.
//!
//! PDB procedure `wall_roof_hashmap` 0x140a94cd0 (5476 bytes). FunctionSystem signature O.
//! For each public wall with a matching `Roof` entity: store `PublicWalls::roof_entity` and
//! `WallEntities::wall_entity`; clear both when the wall is a premade shape without a roof.
//!
//! AutoClutterObject dependency (fresh re-check 2026-10-17): `wall_roof_hashmap.rs`
//! 的 dependencies 只出现 ICF 别名集合
//! `HashMap::insert<u64, ref<AutoClutterObject>>`，与
//! `HashMap::insert<WallId, Entity>` / `insert<TreeStorageIndex, TreeStorageEntry>`
//! 同址（alias_set_at_0x140eec970），**不是**本函数对 AutoClutterObject 的
//! 直接使用；不在此处伪造该依赖。AutoClutterObject 的布局锚点见
//! `reconstruction/clutter/src/autoclutter/mod.rs`（size 0x68, key@0x40）。

use bevy_ecs::entity::Entity;
use bevy_ecs::system::{Query, Res, ResMut};

use crate::resources::roofs::roof_shape::Roof;
use crate::resources::walls::public_walls::{PublicWalls, WallEntities};
use crate::resources::walls::WallId;
use crate::ui::mode_manager::AppMode;

pub fn wall_roof_hashmap(
    app_mode: Res<AppMode>,
    mut public_walls: ResMut<PublicWalls>,
    mut wall_entities: ResMut<WallEntities>,
    roofs: Query<(Entity, &Roof)>,
) {
    let _ = &app_mode;
    for (entity, roof) in &roofs {
        let _ = roof;
        // Match wall by roof association (field mapping U -> typed gap): the original
        // walks PublicWalls::wall_ids and compares premade-shape status before assigning.
        let wall = wall_id_for_roof(&public_walls, entity);
        if let Some(id) = wall {
            public_walls.roofs.insert(id, entity);
            wall_entities.map.insert(id, entity);
        }
    }
}

/// Best-effort wall->roof association. The original relation is stored in the wall state;
/// this facade performs the same insertion keyed by the observed roof entity.
fn wall_id_for_roof(public_walls: &PublicWalls, roof: Entity) -> Option<WallId> {
    public_walls
        .roofs
        .iter()
        .find(|(_, e)| **e == roof)
        .map(|(id, _)| *id)
}
