//! add_lantern_water_setdresssing（B 主基线；0x1422d2fa0，size 2096）。
//!
//! 2026-10-17 restorer-clutter-remaining：按 fresh 反编译 + 逐指令反汇编重建
//! （`evidence/decomp/0x1422d2fa0_*.c`、`evidence/disasm5_2.txt`、常量实读
//! `0x142a7fdf4=0.65`、`0x1429ddce8=-0.05`、`0x142a7fa50=-0.8`、
//! `0x1429eb6f8=-0.1`、`0x1429259b0=1.3`、`0x1429cd128=1.2`、`0x142925984=0.1`、
//! `0x142925940=(0.25,0.25)`、`0x142925988=5.0`、`0x142a16cb8=0.8`）。
//!
//! 控制流（O）：
//! 1. `TweakerView::as_inner_mut` 借 WaterRaster（+0x298 双缓冲选择）；
//!    `BackupSaveFileKind::is_corrupted` → 早退。
//! 2. `DecoratorStorage::iter_terrain_decorators`：每装饰物
//!    `CachedDecoratorTransforms::transform` 取位 → `Circle2d::new(center.xz, 0.65)` →
//!    `init_rng(装饰物 id)`。
//! 3. 内层 **5 次**：`u = rem_euclid(f32*0.1 + i/5, 1)` → `get_pos_at_u(u)`；
//!    两次 `f32` 抖动 ×0.25 加到位（0x142925940=(0.25,0.25)）；4 角 bilinear（130.0）
//!    采样；**`0.0 < sample` 才放**（入水门）。
//! 4. 入水后：两次 `f32` 归一化（1/SQRT）得方向；键
//!    `calculate_hash<tuple<u32,usize,&str>>`（i、装饰物 id、"lantern" len 7）。
//! 5. 偏移/缩放三路（O）：
//!    - is_started → y = i/5 × -0.05 + base，factor 0.8；
//!    - 非 started 且键偶 → y = i/5 × -0.1 + base，factor 1.3；
//!    - 否则 y = -0.8，factor 0.8。
//!    base 每轮被更新（累加）。
//! 6. `f32` 抽取 → 缩放 `factor × (r×1.2 + (1-r)×0.8)`；写输出候选 stride 0x68
//!    （+0=5、+8 prefab、+0x18=2、+0x1c 键、+0x30=5、+0x38 位置、+0x60 缩放）。
//!
//! U 登记：base（+0x38 宿主表）来自 walls/roofs 三张索引表（+0x100/+0x108/+0x110），
//! 本 crate 无该宿主查询 → 以地形装饰物 `pos.y` 作 base（合同分歧）。输出候选
//! 精确 0x68 偏移在本 crate 以语义字段承载（`AutoClutterSpawn` 合同 U）。

use bevy_ecs::prelude::*;
use country_core::resources::world_raster::WorldRaster;
use glam::{Vec2, Vec3};

use utils::geometry::circle::Circle2d;

use super::{AutoClutterSpawn, AutoClutterState, SpawnShared};

/// 灯笼放置 RNG（fastrand；init_rng 0x140c96240 语义——每装饰物重播种子）。
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

/// 圆半径（O，0x142a7fdf4 = 0.65）。
pub const LANTERN_CIRCLE_RADIUS: f32 = 0.65;
/// 内层迭代数（O，0x142925988 = 5.0）。
pub const LANTERN_ITERATIONS: usize = 5;

/// add_lantern_water_setdresssing（0x1422d2fa0）：地形装饰物周围 5 个圆周 jitter 落位，
/// **入水才放**（type 5 灯笼）。
pub fn add_lantern_water_setdresssing(
    mut state: ResMut<AutoClutterState>,
    decorators: Res<TerrainDecoratorPositions>,
    water: Res<country_core::resources::world_raster_defs::WaterRaster>,
    mut started: ResMut<IsStarted>,
) {
    let w = (*water).as_ref();
    for (id, pos) in &decorators.entries {
        let circle = Circle2d::new(Vec2::new(pos.x, pos.z), LANTERN_CIRCLE_RADIUS);
        // O：每装饰物 init_rng（0x140c96240）。
        let mut rng = utils::rng::rng_from_seed_u64(utils::rng::init_rng_u64(*id));
        // O：base 为宿主三张表（+0x100/+0x108/+0x110）选中值；本处以装饰物 y 承载（U）。
        let mut base_y = pos.y;
        for i in 0..LANTERN_ITERATIONS {
            let base = i as f32 / LANTERN_ITERATIONS as f32;
            // O：fmodf(...,1.0) 且负值 +1.0 = rem_euclid。
            let u = (rng.f32() * 0.1 + base).rem_euclid(1.0);
            let point = circle.get_pos_at_u(u);
            // O：两次 f32 抖动 ×0.25 加到位（0x142925940=(0.25,0.25)）。
            let jitter = Vec2::new(rng.f32(), rng.f32()) * 0.25;
            let point = point + jitter;
            // O：4 角 bilinear（130.0）采样，`0.0 < sample` 才放。
            let sample = w.bilinear_sample_at_world_xz(point.x, point.y, 130.0, 130.0);
            if !(0.0 < sample) {
                continue;
            }
            // O：两次 f32 → 1/SQRT 归一化方向。
            let nx = rng.f32();
            let nz = rng.f32();
            let inv = 1.0 / (nx * nx + nz * nz).sqrt();
            let _dir = Vec2::new(nx * inv, nz * inv);
            // O：键 tuple<u32,usize,&str>（i、装饰物 id、"lantern"）。
            let key = hash_lantern_key(i, *id);
            // O：三路偏移/缩放（is_started → -0.05 斜率；非 started 且键偶 → -0.1 斜率；
            //    否则常量 -0.8）。
            let started_now = started.0;
            let (y, factor) = if started_now {
                (base_y + base * -0.05, 0.8f32)
            } else if key & 1 == 0 {
                (base_y + base * -0.1, 1.3f32)
            } else {
                (-0.8, 0.8f32)
            };
            // O：base 每轮更新（累加）。
            base_y = y;
            // O：缩放 `factor*(r*1.2+(1-r)*0.8)`。
            let r = rng.f32();
            let scale = factor * (r * 1.2 + (1.0 - r) * 0.8);
            state.spawn_candidates.push(AutoClutterSpawn {
                kind: 5,
                shared: SpawnShared::Prefab("lantern".to_string()),
                f18: 2,
                key,
                marker: 5,
                position: Vec3::new(point.x, y, point.y),
                host_id: *id,
                f48: key,
                f50: 0,
                f54: 0,
                f58: 0,
                f5c: 0,
                scale,
            });
        }
    }
}

/// 放置键（`calculate_hash<tuple<u32,usize,&str>>` 0x141cf7980 语义）。
pub fn hash_lantern_key(i: usize, decorator_id: u64) -> u64 {
    utils::calculate_hash(&((i as u32), decorator_id, "lantern"))
}

/// is_started 资源（O 0x14092e6c0：开始状态门控；宿主输入系统写）。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct IsStarted(pub bool);
