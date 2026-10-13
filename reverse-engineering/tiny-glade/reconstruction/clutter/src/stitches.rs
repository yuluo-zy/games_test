//! system-clutter：inter_shape_stitches（形状缝合）恢复草稿（B 主基线，反汇编级证据）
//!
//! 依据：evidence/multi-baseline/clutter-extra-analysis.md（2026-10-13 reva worker
//! `clutter-extra-owner` + radare2 RNE oracle 精读）。
//! 原模块：crates/systems/clutter/src/inter_shape_stitches/
//!   - detect_intra_shape_corners.rs（B 0x141cd1a90，size 8769，源 L94–~750；
//!     含 dirty_walls L69 / intersect_shapes L478 / add_hole_at_shape_intersection L725）
//!   - stitch_bricks.rs（B spawn_stitches 0x141ce65b0 size 3025 源 L28；
//!     spawn_stitch_bricks 0x141ce7190 size 2961 源 L92；on_exit_game_state L124；
//!     impl$3::default L141；InterShapeBrickStitches::clear L148）
//!
//! 语义：detect 在墙对形状交线处产角点/洞（Corner / ShapeIntersectionHole / WallHoles）；
//! spawn_stitches 把短段楼梯空隙缝合成 StairsFloatingBrickParams；
//! spawn_stitch_bricks 把缝合砖烘焙进 BrickTransformSsbo。
//!
//! 依赖复用：bevy_ecs 0.16、glam 0.29、fastrand、hashbrown、utils 直接复用。
//! 字段为证据；未确认保留 unknown。不建孤立 Cargo。

use bevy_ecs::prelude::*;
use glam::{Vec2, Vec3};
use hashbrown::HashMap;
use std::sync::Arc;

use crate::autoclutter::WallId;
use crate::merlons::{BrickTransformSsbo, SparseInstanceBufferWriter, WallHoles};

// ---- country_core 边界点 ----

/// 墙状态（country_core::resources::walls::public_wall_state::PublicWallState）。
pub struct PublicWallState;
impl PublicWallState {
    /// 证据：detect_intra_shape_corners 取墙顶高度（角点高 = max_y + flag*(-0.67)）。
    pub fn max_y(&self) -> f32 {
        0.0
    }
    /// 证据：add_hole_at_shape_intersection 的墙空间换算。
    pub fn wall_space(&self) -> WallSpace {
        WallSpace
    }
}

/// 墙空间坐标（utils::wall_space）。
pub struct WallSpace;
impl WallSpace {
    pub fn subtract(&self, _o: WallSpace) -> Vec2 {
        Vec2::ZERO
    }
    pub fn project_clamp(&self) -> Vec2 {
        Vec2::ZERO
    }
}

/// 墙空间坐标换算（utils::wall_space::WallSpaceCoord）。
pub struct WallSpaceCoord;
impl WallSpaceCoord {
    pub fn from_world_space(&self) -> WallSpaceCoord {
        WallSpaceCoord
    }
}

/// 角点（detect 的 retain 类型；布局 unknown）。
pub struct Corner;

/// 形状交点洞（与 inter_roof_merlons 共享；retain + WallHoles::add 消费）。
pub struct ShapeIntersectionHole;

/// 屋顶类型查询（country_core::resources::roofs::roof_shape::Roof::ty）。
pub fn roof_ty(_entity: Entity) -> u8 {
    // 证据：detect_intra_shape_corners 对每墙对判 roof ty。
    0
}

/// 形状交点洞写入：在世界坐标 x/z 处把墙空间投影钳制后写洞记录（stride 0x14）。
/// 证据（add_hole_at_shape_intersection，0x141cd3ce0，源 L725）：
/// - WallSpaceCoord::from_world_space ×2 + wall_space + subtract + project_clamp
/// - 洞记录 stride 0x14：+0 世界x、+4 世界y+偏移、+8 高度差、+0xc Δ、+0x10 kind byte（5/1）
pub fn add_hole_at_shape_intersection(
    _out: &mut WallHole,
    _wall: &PublicWallState,
    _pos: Vec2,
) {
    unknown();
}

/// 形状交点通用遍历（intersect_shapes<closure_env$4>，0x140edbf60，源 L478）：
/// Circle2d::intersect_circle / Rectangle2d::as_transformation_and_size /
/// as_points2_centered_aligned / get_closest_u_from_pos / get_normal_at_u /
/// get_point / Circle2d::intersect_ray —— 在两形状交点处调闭包 env$4。
pub fn intersect_shapes<F>(_a: &Shape2d, _b: &Shape2d, _on_intersection: F)
where
    F: FnMut(Vec2),
{
    unknown();
}

/// 形状（判别 +0=0 圆，否则矩形）。
pub enum Shape2d {
    Circle(Circle2d),
    Rectangle(Rectangle2d),
}

pub struct Circle2d {
    pub _pos: Vec2,
    pub _r: f32,
}
pub struct Rectangle2d;

// ---- detect_intra_shape_corners（0x141cd1a90）----

/// 墙对形状角点检测（源 L94–~750）。
/// 证据：
/// - dirty_walls() 脏墙集；retain Corner / HashMap<[WallId;2],Vec<Vec2>> / ShapeIntersectionHole
/// - 主循环：脏墙对（hash 表 stride 0x2a8）→ Roof::ty() → PublicWallState::max_y() →
///   角点高 = max_y + flag*(-0.67)（__real_bf2b851f）→ Roof::is_flat 门控 →
///   ShapeParameters::expand()×2 + contains_shape() + perimeter() +
///   intersect_shapes<closure_env$4>（0x140edbf60）→ add_hole_at_shape_intersection
/// - 收尾：WallHoles::add()（stride 0x28）
pub fn detect_intra_shape_corners(
    _walls: Res<PublicWallState>,
    _holes: ResMut<WallHoles>,
    _state: &mut CornerState,
) {
    unknown();
}

pub struct CornerState {
    pub corners: Vec<Corner>,
    pub wall_pair_points: HashMap<[WallId; 2], Vec<Vec2>>,
    pub holes: Vec<ShapeIntersectionHole>,
}

// ---- stitch_bricks ----

/// 缝合描述（spawn_stitches 宿主，stride 0x40）。
pub struct StitchSpan {
    /// +0x00 i32 种类（必须 ==1 才处理）。
    pub kind: u32,
    /// +0x04 f32（>=0.2 跳过）。
    pub _f04: f32,
    /// +0x14 f32 段起点。
    pub start: f32,
    /// +0x18 f32 段长。
    pub length: f32,
}

/// 悬浮砖参数（StairsFloatingBrickParams，stride 0x40）。
/// 证据（spawn_stitches 写块）：+0 4 float 位置/朝向、+0x10 方向 qword、
/// +0x18 参数 u32、+0x1c 抖动 f32、+0x20 Δ f32、+0x24 0.3、+0x28 Vec2、+0x38 u64。
pub struct StairsFloatingBrickParams {
    pub _pos_dir: [f32; 4],
    pub _f10: u64,
    pub _f18: u32,
    pub _f1c: f32,
    pub _f20: f32,
    pub _f24: f32,
    pub _f28: Vec2,
    pub _f38: u64,
}

/// 缝合砖生成：把短段（≤2 单位）楼梯空隙缝合成悬浮砖参数。
/// 证据（spawn_stitches，0x141ce65b0，源 L28）：
/// - 遍历 Vec<stride 0x40> 缝合描述：kind==1 且 +0x04<0.2，段长 u=min(1000,2*|长|)≤2
/// - utils::random_splits() 细分 + 1/SQRT 法向 + x0y swizzle
/// - 每细分点 fastrand::Rng::f32 抖动 (r*.5-(1-r)*.5)*0.1+0.4；Δ+(-0.1)
/// - RawVec::grow_one<StairsFloatingBrickParams> + 写 0x40 记录
pub fn spawn_stitches(
    _spans: &[StitchSpan],
    _out: &mut Vec<StairsFloatingBrickParams>,
) {
    for span in _spans {
        if span.kind != 1 || span._f04 >= 0.2 {
            continue;
        }
        let u = (2.0 * span.length.abs()).min(1000.0).trunc();
        if u > 2.0 {
            continue; // 只缝短段
        }
        unknown(); // random_splits + 逐点写 0x40 记录
    }
}

/// 缝合砖烘焙：清 BrickTransformSsbo 后把缝合砖逐条 push。
/// 证据（spawn_stitch_bricks，0x141ce7190，源 L92）：
/// - FrozenMap<SsboId,Box<BufferState>>::get<BrickTransformSsbo> + Arc 克隆
/// - SparseInstanceBufferWriter::clear<BrickTransformSsbo>()
/// - 遍历 Vec<stride 0x40>：render::impl$1::from() + calculate_hash<([WallId;2],usize)> +
///   geometry::instanced_wall::impl$1::from() + SparseInstanceBufferWriter::push<BrickTransformSsbo>
pub fn spawn_stitch_bricks(
    _bricks: &[StairsFloatingBrickParams],
    _buffer: &mut SparseInstanceBufferWriter<BrickTransformSsbo>,
) {
    _buffer.clear();
    for _b in _bricks {
        // render from + hash([WallId;2],usize) + instanced_wall from → push。
        unknown();
    }
}

/// 缝合砖状态（InterShapeBrickStitches；clear L148）。
pub struct InterShapeBrickStitches;

fn unknown() -> ! {
    unimplemented!("反汇编级证据：精确逻辑见 evidence/multi-baseline/clutter-extra-analysis.md")
}
