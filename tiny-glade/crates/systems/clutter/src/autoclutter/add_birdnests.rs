//! add_birdnests（B 主基线；0x1422c9c70，源码 37–141；closure$0 L84、closure$1 L105）。
//!
//! 证据：`Subscriber::take<DirtyTiles>`（0x141247c90）→ 复制 0x200 字节脏位集 → `new_clear`。
//! 遍历脏格行（`cell_from_world_space` 0x140960b20；`hash_one<&GlobalId>` 0x1405b0d70 ×2
//! 查已存在集）；未处理格调 `add_birdnests::closure_0`（0x1422ca8c0 ×2）+
//! `HashMap<u64,bool>::insert`（0x140eec230，cell→已处理去重）。
//! 直接写输出候选（state+0x48，0x68 stride）：`+0x0=4`（type）、`+0x8` Arc 强计数
//! （克隆共享字段）、位置（常量 0x3e19999a=0.15 偏移量）、`+0x60=1.0` 缩放。
//! 即每脏格 1 鸟巢候选。

use bevy_ecs::prelude::*;
use country_core::utils::dirty_tiles::DirtyTiles;
use hashbrown::HashMap;

use super::populate_autoclutter_regions::{cell_center_world, dirty_tiles_cells, AutoClutterDirtySubscriptions};
use super::{AutoClutterSpawn, SpawnShared, AutoClutterState};

/// add_birdnests（0x1422c9c70）：每脏格 1 鸟巢候选（type 4）。
pub fn add_birdnests(
    mut state: ResMut<AutoClutterState>,
    dirty: ResMut<super::populate_autoclutter_regions::DirtyTilesResource>,
    mut sub: ResMut<AutoClutterDirtySubscriptions>,
) {
    // 订阅冲刷（O：Subscriber::take → 复制 0x200B 位集 → new_clear 合入）。
    if let Some(reader) = sub.subscriber.lock().unwrap().as_mut().and_then(|s| s.take()) {
        let mut merged = DirtyTiles::new_clear();
        merged.extend(&reader);
        // 位集合入后由 populate 消费；此处按证据只取脏格行。
        let _ = &mut merged;
    }
    let mut processed: HashMap<u64, bool> = HashMap::new();
    for (x, y) in dirty_tiles_cells(&dirty.inner) {
        let cell_key = crate::autoclutter::populate_autoclutter_regions::hash_cell_key(x, y);
        if processed.contains_key(&cell_key) {
            continue;
        }
        processed.insert(cell_key, true);
        let (wx, wz) = cell_center_world(x, y);
        // 输出候选（O：+0x0=4、位置 +0.15 偏移、+0x60=1.0 缩放）。
        state.spawn_candidates.push(AutoClutterSpawn {
            kind: 4,
            // +0x08 = Arc 强计数（O：draft 明注"birdnest 为 Arc 强计数域"）。
            shared: SpawnShared::ArcStrong(cell_key),
            f18: 1,
            key: cell_key,
            marker: 5,
            position: glam::Vec3::new(wx, 0.15, wz),
            host_id: cell_key,
            f48: cell_key,
            f50: 0,
            f54: 0,
            f5c: 0,
            scale: 1.0,
        });
    }
}
