//! B `crates/systems/decorator/src/collision_detection.rs` — 装饰碰撞检测族
//! （D4 restorer tg_d4）。
//!
//! 证据（fresh RE-MCP，decomp-141fe5900.txt / decomp-141fe5c20.txt，O 结构）：
//! - `intersect_wall_decorators<F>` 0x141fe5900（merge_proposal 实例，
//!   closure = `propose_wall_decorator_merge_position_inner::closure$1`
//!   0x141fe5310）/ 0x141fe5c20（interaction 实例，closure =
//!   `calculate_decorator_wall_interaction_intent::closure$0` 0x141fe50e0 /
//!   0x141fe5720/0x141fe5530）：遍历 `Decorators` 内嵌 map（0x50B 条目，
//!   +0x30 数据 / +0x38 掩码 / +0x48 长）→ 每条目调闭包（装饰信息 + 候选
//!   参数）→ 命中 push 装饰 id 到出参 Vec（0x1428a7f20 grow，u32 载荷）。
//! - `intersect_wall_decorators_impl`（行 52）+ closure$0（行 61，O）：相交
//!   谓词 = `DerivedDecoratorInfo::dims_wall_space` + lib.rs
//!   `snapping_direction` + `utils::wall_space::WallSpaceRegion::intersects`
//!   （0x140c7xxx 族；bbox 重叠 + 方向一致）。
//! - `DecoratorWorldSpace::project_to_wall_space`（行 114，O）：把世界位置
//!   投影到墙空间（`utils::curve::CurveUBoundless::clamp` 0x140c904e0 等价，
//!   "assertion failed: (0.0..=1.0).contains(&t)" 0x142c236fd 来自
//!   curve_u.rs clamp）。
//!
//! 闭包实参（merge / interaction）归属 D1 merge_proposal_wall_decorators 与
//! D2 ui_systems/decorator_interaction_intent（各自模块声明；本文件只提供
//! 泛型宿主 + 公共谓词基元）。

use country_core::resources::walls::decorator::DecoratorInfo;
use country_core::resources::walls::decorator_storage::Decorators;
use country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo;
use glam::Vec3;
use utils::wall_space::WallSpaceRegion;

/// `intersect_wall_decorators`（0x141fe5900 / 0x141fe5c20，O 结构）：
/// 对 `Decorators` 内嵌 map 逐装饰调 `predicate`；命中 id 收集返回。
/// 闭包实参 = (装饰信息, 候选墙空间/尺寸) —— 由调用方（merge / interaction）
/// 提供；本泛型宿主只做遍历与收集。
pub fn intersect_wall_decorators<F>(
    decorators: &Decorators,
    mut predicate: F,
) -> Vec<u32>
where
    F: FnMut(&DecoratorInfo) -> bool,
{
    let mut hits = Vec::new();
    for (id, info) in decorators.decorators.iter() {
        if predicate(info) {
            hits.push(*id);
        }
    }
    hits
}

/// 相交谓词基元（closure$0 行 61，O 结构）：两装饰墙空间包围盒重叠
/// （`dims_wall_space`）+ 吸附方向一致（lib.rs `snapping_direction`）。
/// `a` 为候选（墙空间区域），`b` 为已存装饰（区域 + 尺寸）。
pub fn intersect_wall_decorators_predicate(
    a: WallSpaceRegion,
    b: WallSpaceRegion,
    derived_b: DerivedDecoratorInfo,
    snap_dir_a: i8,
    snap_dir_b: i8,
    space: &utils::wall_space::WallSpace,
) -> bool {
    // O：bbox 重叠（WallSpaceRegion::intersects）+ 方向一致性；尺寸来自
    // DerivedDecoratorInfo::dims_wall_space（0x140b26970 系）。
    let dims = derived_b.dims_wall_space();
    let expanded = WallSpaceRegion {
        center: b.center,
        half_extent: b.half_extent + glam::Vec2::new(dims.x, dims.y) * 0.5,
    };
    if !a.intersects(expanded, space) {
        return false;
    }
    // 吸附方向一致（I：原 snapping_direction 返回 ±1/0；0 视为可合并）。
    snap_dir_a == snap_dir_b || snap_dir_a == 0 || snap_dir_b == 0
}

/// `DecoratorWorldSpace::project_to_wall_space`（行 114，O）：世界位置 → 墙空间
/// 坐标（u 方向 + y；`CurveUBoundless::clamp` 0x140c904e0 断言 t∈[0,1]）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DecoratorWorldSpace {
    /// 墙空间 u 坐标（0..1 归一化）。
    pub u: f32,
    /// 墙空间 y（附着高）。
    pub y: f32,
}

impl DecoratorWorldSpace {
    /// 由墙曲线把世界位置投影到墙空间。
    /// `curve` = 归一化曲线采样（I：原 CurveUBoundless::clamp；本面以
    /// 最近点 u 投影承载，登记 RESTORE.md）。
    pub fn project_to_wall_space(world: Vec3, curve: &impl Fn(f32) -> Vec3) -> Self {
        // 线性最近点投影（I）：沿曲线粗采样取最小距离 u。
        let mut best = 0.0f32;
        let mut best_d = f32::MAX;
        for i in 0..=32 {
            let u = i as f32 / 32.0;
            let p = curve(u);
            let d = (p - world).length_squared();
            if d < best_d {
                best_d = d;
                best = u;
            }
        }
        debug_assert!((0.0..=1.0).contains(&best), "assertion failed: (0.0..=1.0).contains(&t)");
        DecoratorWorldSpace { u: best, y: world.y }
    }
}
