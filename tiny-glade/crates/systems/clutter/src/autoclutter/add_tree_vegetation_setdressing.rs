//! add_tree_vegetation_setdressing（B 主基线；0x1422d3800，源码 19–100）。
//!
//! 证据：遍历树（`PlacementDataSsbo::position` 0x140ad51a0 ×2 取位）；`is_dragging`
//! （0x14092e6a0）、`is_started`×4、`is_corrupted`×4 多重门控（拖拽/拖拽开始/存档损坏）。
//! 每树 `Circle2d::new`+`init_rng`；**内层 12 次**（`i/12.0`）：f32+fmodf 圆周 jitter →
//! `get_pos_at_u` → 水位采样 → 入水 → 键 `calculate_hash<tuple<TreeStorageIndex,usize,&str>>`
//! （0x141cf7600，`tree` @0x143042d91 len 4）→ 写输出候选（0x68 stride，`+0x0=5`、
//! `+0x30=5`）。与 lantern 同构（type 5），count 12 vs 5，宿主 = 树 vs 地形装饰物。

use bevy_ecs::prelude::*;
use country_core::resources::world_raster::WorldRaster;
use fastrand::Rng;
use glam::Vec3;

use utils::geometry::circle::Circle2d;

use super::{AutoClutterSpawn, SpawnShared, AutoClutterState};

/// 树位置表（宿主经 `PlacementDataSsbo::position` 填充；TreeStorageIndex 键）。
#[derive(Resource, Clone, Debug, Default)]
pub struct TreePositions {
    pub entries: Vec<(u64, Vec3)>,
}

/// 树放置 RNG（fastrand；init_rng 语义）。
#[derive(Resource, Debug)]
pub struct TreeRng(pub std::sync::Mutex<fastrand::Rng>);

impl Default for TreeRng {
    fn default() -> Self {
        Self(std::sync::Mutex::new(fastrand::Rng::with_seed(0x606f)))
    }
}

/// add_tree_setdressing（0x1422d3800）：每树 12 个圆周 jitter 落位，入水才放（type 5）。
pub fn add_tree_setdressing(
    mut state: ResMut<AutoClutterState>,
    trees: Res<TreePositions>,
    water: Res<WorldRaster<512>>,
    mut rng: ResMut<TreeRng>,
    mut started: ResMut<crate::autoclutter::add_lantern_water_setdresssing::IsStarted>,
    mut dragging: ResMut<IsDragging>,
) {
    let w = (*water).as_ref();
    let r = &mut rng.0.lock().unwrap();
    // 门控（O：is_dragging / is_started×4 / is_corrupted×4）。
    if dragging.0 || !started.0 {
        return;
    }
    for (index, pos) in &trees.entries {
        let circle = Circle2d::new(glam::Vec2::new(pos.x, pos.z), 1.5);
        for i in 0..12 {
            let u = (i as f32 / 12.0 + r.f32() * 0.08) % 1.0;
            let point = circle.get_pos_at_u(u);
            let s = w.bilinear_sample_at_world_xz(point.x, point.y, 130.0, 130.0);
            if !(s > 0.0) {
                continue;
            }
            let out_pos = Vec3::new(point.x, 0.3, point.y);
            let key = hash_tree_key(*index, i);
            state.spawn_candidates.push(AutoClutterSpawn {
                kind: 5,
                shared: SpawnShared::Prefab("tree".to_string()),
                f18: 2,
                key,
                marker: 5,
                position: out_pos,
                host_id: *index,
                f48: key,
                f50: 0,
                f54: 0,
                f5c: 0,
                scale: 0.6,
            });
        }
    }
}

/// 放置键（`calculate_hash<tuple<TreeStorageIndex,usize,&str>>` 0x141cf7600 语义）。
pub fn hash_tree_key(tree_index: u64, i: usize) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    tree_index.hash(&mut h);
    i.hash(&mut h);
    "tree".hash(&mut h);
    h.finish()
}

/// 拖拽状态（O 0x14092e6a0 is_dragging；宿主输入系统写）。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct IsDragging(pub bool);
