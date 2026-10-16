//! add_lantern_water_setdresssing（B 主基线；0x1422d2fa0，源码 22–102）。
//!
//! 证据：`as_inner_mut`（0x14067dd80）借 WaterRaster（+0x298 双缓冲选择）；
//! `is_corrupted`（0x140ae8320）门控；`DecoratorStorage::iter_terrain_decorators`
//! （0x140a1e280）遍历地形装饰物（宿主经 `TerrainDecoratorPositions` 供位）。每装饰物
//! `Circle2d::new`（0x140c97540）+ `init_rng`（0x140c96240，fastrand RNG 初始化）；
//! **内层 5 次**（`i/5.0`）：`fastrand::Rng::f32`×3 + `fmodf`（0x142923f10）沿圆周
//! jitter → `Circle2d::get_pos_at_u`（0x140c975a0）→ 水位采样 `bilinear_sample_at_world_xz`
//! （0x140a14df0）→ `0.0 < 采样`（入水才放）→ 归一化朝向 + 键
//! `calculate_hash<tuple<u32,usize,&str>>`（0x141cf7980，`lantern` @0x143042d28 len 7）→
//! `is_started`（0x14092e6c0）选偏移向量。
//! 直接写输出候选（0x68 stride）：`+0x0=5`、`+0x8=lantern`、`+0x18=2`、`+0x1c` 键、
//! `+0x30=5`、`+0x38` 位置、`+0x60` 缩放。

use bevy_ecs::prelude::*;
use country_core::resources::world_raster::WorldRaster;
use fastrand::Rng;
use glam::Vec3;

use utils::geometry::circle::Circle2d;

use super::{AutoClutterSpawn, SpawnShared, AutoClutterState};

/// 灯笼放置 RNG（fastrand；init_rng 0x140c96240 语义）。
#[derive(Resource, Debug)]
pub struct LanternRng(pub std::sync::Mutex<fastrand::Rng>);

impl Default for LanternRng {
    fn default() -> Self {
        Self(std::sync::Mutex::new(fastrand::Rng::with_seed(0x606f)))
    }
}

/// 地形装饰物位置表（宿主经 `DecoratorStorage::iter_terrain_decorators` +
/// `CachedDecoratorTransforms::transform` 填充）。
#[derive(Resource, Clone, Debug, Default)]
pub struct TerrainDecoratorPositions {
    pub entries: Vec<(u64, Vec3)>,
}

/// add_lantern_water_setdresssing（0x1422d2fa0）：地形装饰物周围 5 个圆周 jitter 落位，
/// 入水才放（type 5 灯笼）。
pub fn add_lantern_water_setdresssing(
    mut state: ResMut<AutoClutterState>,
    decorators: Res<TerrainDecoratorPositions>,
    water: Res<WorldRaster<512>>,
    mut rng: ResMut<LanternRng>,
    mut started: ResMut<IsStarted>,
) {
    let w = (*water).as_ref();
    let r = &mut rng.0.lock().unwrap();
    for (id, pos) in &decorators.entries {
        let circle = Circle2d::new(glam::Vec2::new(pos.x, pos.z), 1.0);
        for i in 0..5 {
            // 圆周 jitter（O：f32×3 + fmodf；i/5.0 基）。
            let u = (i as f32 / 5.0 + r.f32() * 0.1) % 1.0;
            let point = circle.get_pos_at_u(u);
            let s = w.bilinear_sample_at_world_xz(point.x, point.y, 130.0, 130.0);
            if !(s > 0.0) {
                continue; // 入水才放。
            }
            // is_started 门控选偏移向量（O 0x14092e6c0）。
            let offset = if started.0 { 0.3 } else { 0.6 };
            let out_pos = Vec3::new(point.x, offset, point.y);
            let key = hash_lantern_key(i, *id);
            state.spawn_candidates.push(AutoClutterSpawn {
                kind: 5,
                shared: SpawnShared::Prefab("lantern".to_string()),
                f18: 2,
                key,
                marker: 5,
                position: out_pos,
                host_id: *id,
                f48: key,
                f50: 0,
                f54: 0,
                f5c: 0,
                scale: 0.6,
            });
        }
    }
}

/// 放置键（`calculate_hash<tuple<u32,usize,&str>>` 0x141cf7980 语义）。
pub fn hash_lantern_key(i: usize, decorator_id: u64) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    (i as u32).hash(&mut h);
    decorator_id.hash(&mut h);
    "lantern".hash(&mut h);
    h.finish()
}

/// is_started 资源（O 0x14092e6c0：开始状态门控；宿主输入系统写）。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct IsStarted(pub bool);
