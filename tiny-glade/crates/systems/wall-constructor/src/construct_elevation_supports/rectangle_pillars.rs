//! 矩形支柱（construct_elevation_supports/rectangle_pillars.rs）。
//!
//! `construct_rectangle_pillars`（0x14124fcb0）+ `find_stone_column_start_y`
//! （0x141250720）。支柱常量族（re-analyst §5.2：0.2/1.2/0.01/0.5/±0.2/2.1/
//! ±0.25/0.56/1.26）。
//!
//! 2026-10-17 restorer-remaining-wallconstructor：`find_stone_column_start_y`
//! 已按 fresh 机器码（105 条指令 + 跳表 `0x142c20af8` 17 项 + 常量逐字节）
//! 逐语句重建；`construct_rectangle_pillars` 仍为未实现（见函数注释分歧点）。
use country_core::resources::terrain_height::TerrainHeightsData;
use country_core::systems::collision::world::{
    RaycastSimpleResult, RaycastWorld, SubworldColliderHandle,
};
use glam::{Vec2, Vec3};
use utils::geometry::rectangle::Rectangle2d;
use utils::swizzlers::xvy;

/// x86 `MINSS a, b`：`if a < b { a } else { b }`；unordered（NaN）返回 **b**
/// （与 `f32::min` 的 NaN 传播不同，必须逐位保留）。
#[inline]
fn minss(a: f32, b: f32) -> f32 {
    if a < b { a } else { b }
}

/// B `find_stone_column_start_y`（0x141250720）：石柱起始高度。
///
/// fresh 机器码逐语句（`evidence/fsy-dis.txt`，105 条指令；跳表
/// `0x142c20af8` 17 项；常量 `0x1429ddd00`/`0x142925904`/`0x142925980`/
/// `0x1429cd030`/`0x14295b630`/`0x1429df16c`/`0x14295a0e4` 逐字节读取）：
///
/// 1. `RaycastWorld::raycast_simple_exclude_wall_id(position, (-0.0, -1.0, -0.0),
///    0x1202, excluded)`（`0x1408ccc50`；XMM3=max f32）。
/// 2. `code = hit.handle[0] as u8`：
///    - `0x11`（未命中）：`terrain.sample_at_world_xz(x, z)` 后 `xvy` 写回
///      hit 记录位置（其结果在本函数不再被读取；调用保留）。
///    - `0x01` → `None`。
///    - `0x02` → `Some(hit.position.y + (handle[0] bit8 ? -0.2 : -1.0))`
///      （`TEST byte[handle+1],1`；常量 `0x1429df16c=-0.2`、`0x14295a0e4=-1.0`）。
///    - `0x0d` → `Some(hit.position.y - 0.2)`（跳表 idx13 → `0x141250937`）。
///    - `0x00,0x06..=0x0b,0x0e` → 落入步骤 3。
///    - `0x03,0x04,0x05,0x0c,0x0f,0x10` → panic（跳表 panic 桩）。
/// 3. `axis = rect.is_some() ? rect.axis : (1.0, 0.0)`（`CMP byte[RDI],0`；
///    `RDI` 是 `&Option<Rectangle2d>`：tag u32@0 + payload 24B@4）。
///    `Rectangle2d::new(center=(position.x, position.z), size=(0.7, 0.7), axis)`，
///    `as_points2()` 取 4 角，逐点 `terrain.sample_at_world_xz`；
///    结果 = `minss(s0, 1000.0)` → 依次 `minss(s1, ·)`, `minss(s2, ·)`,
///    `minss(s3, ·)`（常量 `0x14295b630=1000.0`）。
/// 4. 返回 `Some(min)`（判别 `EAX=1`；`None` 为 `EAX=0`）。
pub fn find_stone_column_start_y(
    world: &RaycastWorld,
    position: Vec3,
    terrain: &TerrainHeightsData,
    rect: Option<Rectangle2d>,
    excluded: SubworldColliderHandle,
) -> Option<f32> {
    let hit: RaycastSimpleResult = world.raycast_simple_exclude_wall_id(
        position,
        Vec3::new(-0.0, -1.0, -0.0),
        0x1202,
        excluded,
    );
    let x = position.x;
    let z = position.z;
    let code = hit.handle[0] as u8;
    if code == 0x11 {
        // 未命中：地形采样 + xvy 写回 hit 位置（结果随后被轴选择覆盖）。
        let sampled = terrain.sample_at_world_xz(x, z);
        let _ = xvy(Vec2::new(x, z), sampled);
    } else {
        match code {
            0x00 | 0x06..=0x0b | 0x0e => {}
            0x01 => return None,
            0x02 => {
                let add = if hit.handle[0] & 0x100 != 0 {
                    -0.2
                } else {
                    -1.0
                };
                return Some(hit.position.y + add);
            }
            0x0d => return Some(hit.position.y - 0.2),
            // 跳表 panic 桩（unreachable）。
            0x03 | 0x04 | 0x05 | 0x0c | 0x0f | 0x10 => {
                panic!("find_stone_column_start_y: unreachable hit code")
            }
            _ => panic!("find_stone_column_start_y: hit code out of jump table"),
        }
    }
    let axis = match rect {
        None => Vec2::new(1.0, 0.0),
        Some(r) => r.axis,
    };
    let rectangle = Rectangle2d::new(Vec2::new(x, z), Vec2::splat(0.7), axis);
    let p = rectangle.as_points2();
    let s0 = terrain.sample_at_world_xz(p[0].x, p[0].y);
    let s1 = terrain.sample_at_world_xz(p[1].x, p[1].y);
    let s2 = terrain.sample_at_world_xz(p[2].x, p[2].y);
    let s3 = terrain.sample_at_world_xz(p[3].x, p[3].y);
    Some(minss(s3, minss(s2, minss(s1, minss(s0, 1000.0)))))
}

/// B `construct_rectangle_pillars`（0x14124fcb0）。**未实现（近似体已移除）**。
///
/// fresh 反编译已解出的主流程（`evidence/elev-0x14124fcb0.c`）：
/// - 门 `wall+0x298 == 0`；
/// - 对 `Rectangle2d` 4 角 `get_point` → 地形 `sample_at_world_xz` →
///   `terrain+0.05 <= wall+0x280` 门 → 逐墙 (stride 0x50) `PublicWalls::get`
///   （缺失 unwrap panic）→ `get_wall_shape` → `GladeBorder::is_pos_inside`；
/// - 通过后取角点两邻边 + `get_corner_normal`，3 个方向
///   `legacy_raycast_w_params`，命中码过滤 `0x6d84 >> (code&0x1f) & 1`，
///   求最小距离（NaN 感知），`>= 2.3` 才生成石柱；
/// - `find_stone_column_start_y` → smoothstep `((wall_h - y - 5.0)/5.0).clamp01`、
///   `(3-2t)t²` 后 `*1.5 + (1-t)`，沿角点法向 `-0.4` 外移；
/// - 逐记录 0x40B 提案 → `Rectangle3d::new` → 输出 Vec（0x20B 记录）；
/// - `WallStyle::is_halftimbered` 分派 `construct_brick_columns` /
///   `construct_wooden_pillar_w_stone_base`。
/// 逐语句重建仍缺：Rectangle2d/3d 调用实参组合、0x40/0x20 记录字段语义、
/// 常量 0x6d84 命中码全集（登记，未实现）。
pub fn construct_rectangle_pillars(_positions: &[Vec3], _height: f32, _out: &mut Vec<[u8; 96]>) {
    todo!(
        "未实现: construct_rectangle_pillars 0x14124fcb0（4 角采样/3 向 raycast/0x40 提案/Rectangle3d 输出链未逐语句）"
    )
}
