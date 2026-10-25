//! 墙↔屋顶关联事件（systems/wall/wall_state/wall_roof_hashmap.rs）。
//!
//! 原 B 模块路径（skeleton 富化/PDB）：`country_core::systems::wall::
//! wall_state::wall_roof_hashmap`，宿主 `RoofIsInvalid`/
//! `RecalculateGableRoofWallHeights`/`RecalculateWallBricks` 等事件。
//! 重建把事件归入 `resources::roofs::commands`；本模块按原路径再导出，
//! 供 system-color 等原路径消费方接线。
pub use crate::resources::roofs::commands::{
    RecalculateWallBricks, RecalculateWallHoles, RoofIsInvalid, WallGableChanged,
};

use crate::systems::ui_clutter::autoclutter_state::AutoClutterObject;

/// B 0x140a94cd0 `wall_roof_hashmap` core: one roof per wall association.
///
/// fresh 2026-10-17:
/// - the object record is `AutoClutterObject` (0x68 B); the map key is its
///   `+0x40` qword (from_iter 0x142612aa0 `IMUL 0x68` + `[obj + 0x40]`);
/// - repeated insertion of the same key panics with
///   `"Only one roof should be associated with a wall id"`
///   (string @0x142ada570).
///
/// The native dense-table/generation probes and the replay-mode gate
/// (`"Tried deleting roof {} during Replay"`) belong to the host resource
/// tables; this owner takes the already-resolved roof-bearing objects and
/// performs the anchored association.
pub fn build_wall_roof_hashmap<'a>(
    objects: impl IntoIterator<Item = &'a AutoClutterObject>,
) -> std::collections::HashMap<u64, &'a AutoClutterObject> {
    let mut map = std::collections::HashMap::new();
    for object in objects {
        let key = object.key();
        if map.insert(key, object).is_some() {
            panic!("Only one roof should be associated with a wall id");
        }
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn associates_each_object_by_its_0x40_key() {
        let a = AutoClutterObject {
            key: 7,
            ..Default::default()
        };
        let b = AutoClutterObject {
            key: 9,
            ..Default::default()
        };
        let map = build_wall_roof_hashmap([&a, &b]);
        assert_eq!(map.len(), 2);
        assert_eq!(map[&7].key(), 7);
        assert_eq!(map[&9].key(), 9);
    }

    #[test]
    #[should_panic(expected = "Only one roof should be associated with a wall id")]
    fn duplicate_key_panics_like_the_native_insert_check() {
        let a = AutoClutterObject {
            key: 7,
            ..Default::default()
        };
        let b = AutoClutterObject {
            key: 7,
            ..Default::default()
        };
        let _ = build_wall_roof_hashmap([&a, &b]);
    }
}
