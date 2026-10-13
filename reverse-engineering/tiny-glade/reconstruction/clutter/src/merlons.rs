//! system-clutter：inter_roof_merlons（屋顶城垛）恢复草稿（B 主基线，反汇编级证据）
//!
//! 依据：evidence/multi-baseline/clutter-extra-analysis.md（2026-10-13 reva worker
//! `clutter-extra-owner` + radare2 RNE oracle 精读）。
//! 原模块：crates/systems/clutter/src/inter_roof_merlons/
//!   - detect_inter_roof_merlons.rs（B 0x141ce3a70，size 6751）
//!   - construct_inter_roof_merlons.rs（B 0x1422d4870，size 7070，源 L39–215+）
//!   - inter_roof_merlons_audio.rs（B 0x1422d4230，未精读）
//!
//! 语义：detect 在脏屋顶（平顶）的屋顶/墙形状交点处产洞（WallHoles/ShapeIntersectionHole）；
//! construct 沿屋顶形状曲线（circle/rectangle）等距 0.46 生成 BrickTransformSsbo 城垛砖。
//!
//! 依赖复用：bevy_ecs 0.16、glam 0.29、fastrand、hashbrown、utils 直接复用。
//! 字段为证据；未确认保留 unknown。不建孤立 Cargo（并入 reconstruction/clutter）。

use bevy_ecs::prelude::*;
use glam::{Vec2, Vec3};
use hashbrown::{HashMap, HashSet};
use std::sync::Arc;

use crate::autoclutter::WallId;

// ---- country_core 边界点（定义在 country-core 资源/组件模块）----

/// 墙体开洞集合（country_core::resources::walls::wall_holes::WallHoles）。
/// 证据：detect_inter_roof_merlons / detect_intra_shape_corners 收尾逐条 add（stride 0x28）。
pub struct WallHoles;

impl WallHoles {
    /// 证据：洞记录 stride 0x28；字段 +0x10/+0x18 语义 unknown。
    pub fn add(&mut self, _hole: WallHole) {
        // TODO(evidence): country-core 洞写入。
    }
}

/// 洞记录（stride 0x28；由 detect 产、WallHoles::add 消费）。
pub struct WallHole {
    /// +0x10（unknown 精确语义）。
    pub _f10: u64,
    /// +0x18（unknown）。
    pub _f18: u64,
}

/// 实例化砖缓冲（country_core::geometry::instanced_wall::BrickTransformSsbo）。
pub struct BrickTransformSsbo;

/// 实例化写入器（country_core::render::sparse_instance_buffer）。
pub struct SparseInstanceBufferWriter<B>(std::marker::PhantomData<B>);

impl<B> SparseInstanceBufferWriter<B> {
    /// 证据：construct 每城垛段 push 一次。
    pub fn push(&mut self, _brick: B) {
        // TODO(evidence): SparseInstanceBuffer 布局。
    }
    /// 证据：spawn_stitch_bricks 入口 clear 一次。
    pub fn clear(&mut self) {}
}

/// 屋顶（country_core::resources::roofs::roof_shape::Roof）。detect 门控方法。
pub struct Roof;

impl Roof {
    /// 证据：detect_inter_roof_merlons 对脏屋顶调 is_flat ×2（平顶门控）。
    pub fn is_flat(&self) -> bool {
        // TODO(evidence): 平顶判定。
        false
    }
    /// 证据：construct 平顶无沿门控（local_f1=1 条件之一）。
    pub fn is_flat_w_no_rim(&self) -> bool {
        false
    }
    /// 证据：detect `roof_shape_at_intersection_y`；`local_374 != 2` 才 expand+intersect。
    pub fn roof_shape_at_intersection_y(&self) -> f32 {
        0.0
    }
}

/// 墙样式（country_core::resources::walls::inner_wall_state::WallStyle）。
pub struct WallStyle;

impl WallStyle {
    /// 证据：construct 城垛门控之一（半木墙才生成）。
    pub fn is_halftimbered(&self) -> bool {
        false
    }
}

/// 屋顶形状元素（construct 宿主几何，stride 0x38）。
pub struct RoofShape {
    /// +0x10 i32：0=Circle2d，非 0=Rectangle2d。
    pub kind: ShapeKind,
    /// +0x14/+0x1c u64×2（形状基数据，Vec3/曲线数据）。
    pub base: [u64; 2],
    /// +0x24 u64。
    pub _f24: u64,
    /// +0x34 f32：宽度基准（城垛宽度 = 基准 + 0.23 + 抖动）。
    pub width_base: f32,
}

/// 形状种类（元素 +0x10 判别）。
pub enum ShapeKind {
    Circle,
    Rectangle,
}

/// 城垛断口描述（detect 的 retain 类型；布局 unknown）。
pub struct InterRoofMerlonsDescription;

/// 形状交点洞（与 detect_intra_shape_corners 共享；两系统 retain 同一 Vec）。
pub struct ShapeIntersectionHole;

/// 屋顶实体查询（country_core::resources::walls::public_walls::PublicWalls）。
pub struct PublicWalls;

impl PublicWalls {
    /// 证据：detect/construct 取屋顶实体（Entity 打包值，脏位集查询用）。
    pub fn roof_entity(&self) -> Option<Entity> {
        None
    }
}

/// 形状参数（country_core::systems::wall::valid_enclosure::ShapeParameters）。
pub struct ShapeParameters;

impl ShapeParameters {
    /// 证据：detect_inter_roof_merlons / detect_intra_shape_corners 交点处 expand ×2。
    pub fn expand(&mut self) {}
    /// 证据：detect_intra_shape_corners 形状包含测试。
    pub fn contains_shape(&self, _other: &ShapeParameters) -> bool {
        false
    }
    /// 证据：detect_intra_shape_corners 周长。
    pub fn perimeter(&self) -> f32 {
        0.0
    }
}

// ---- detect_inter_roof_merlons（0x141ce3a70）----

/// 脏墙集合：OnWallDeleted / OnWallChanged / RecalculateRoofCmd 三个事件链式汇入。
/// 证据：`WallAndRoofChanges::dirty_walls`（0x141cd16b0，源 L69）
/// `hashbrown::set::extend<Map<EventIterator<OnWallDeleted>,c0>> + Map<...OnWallChanged,c1> + Map<...RecalculateRoofCmd,c2>`。
pub fn dirty_walls(
    wall_deleted: EventReader<OnWallDeleted>,
    wall_changed: EventReader<OnWallChanged>,
    recalc_roof: EventReader<RecalculateRoofCmd>,
) -> HashSet<WallId> {
    wall_deleted
        .iter()
        .map(|e| e.0)
        .chain(wall_changed.iter().map(|e| e.0))
        .chain(recalc_roof.iter().map(|e| e.wall))
        .collect()
}

/// 城垛断口检测（源 L…）：脏屋顶（平顶）在屋顶/墙形状交点处产洞。
/// 证据：
/// - 入口 dirty_walls() + `Vec<InterRoofMerlonsDescription>::retain` +
///   `Vec<ShapeIntersectionHole>::retain`
/// - 主循环：PublicWalls::roof_entity() → 脏位集查墙 → Roof::is_flat() ×2 →
///   Roof::roof_shape_at_intersection_y() → `local_374 != 2` 时
///   ShapeParameters::expand() ×2 + intersect_shapes<closure_env$4>（0x140edd250）
/// - 收尾：stride 0x28 Vec 逐条 WallHoles::add()
pub fn detect_inter_roof_merlons(
    _walls: Res<PublicWalls>,
    _holes: ResMut<WallHoles>,
    _state: &mut InterRoofMerlonsState,
) {
    // TODO(evidence): 脏屋顶遍历 + 交点断口 → 洞。
    // hash 表 stride 0x2a8（GlobalId 集）；roof_entity 打包值→墙索引表 stride 0x14→位集。
    unknown();
}

/// 城垛断口状态（InterRoofMerlonsDescription 集 + ShapeIntersectionHole 集）。
pub struct InterRoofMerlonsState {
    pub descriptions: Vec<InterRoofMerlonsDescription>,
    pub holes: Vec<ShapeIntersectionHole>,
}

// ---- construct_inter_roof_merlons（0x1422d4870）----

/// 城垛砖生成：沿屋顶形状曲线等距 0.46 生成 BrickTransformSsbo 砖。
/// 证据（construct_inter_roof_merlons.c）：
/// - 宿主几何 Vec（stride 0x38）元素见 RoofShape
/// - FrozenMap<SsboId,Box<BufferState>>::get<BrickTransformSsbo> + Arc 克隆
/// - Circle2d::as_curve3 / Rectangle2d::as_curve3 → Curve::circular_slice<Vec3,2D>
/// - 门控 local_f1：WallStyle::is_halftimbered() 或
///   PublicWalls::roof_entity() + Roof::is_flat_w_no_rim()；随后 ^1 取反
/// - 数量 ceilf(长/0.46)（3eeb851f=0.46）；长*1000.0（447a0000）截整
/// - utils::random_splits_into() 切段；每段 fastrand::Rng::f32 抖动
///   ((r*.5-(1-r)*.5)*0.21*0.8+0.368)*.5+(宽基+0.23)
/// - Curve::get_coord_at_u ×2 → 插值 + 垂直归一（1/SQRT）→ x0y/xvy swizzle
/// - 每段 SparseInstanceBufferWriter::push<BrickTransformSsbo>()；记录含
///   段 u×曲线长、形状原点、(-100,-100) 常量（0xc2c80000）、砖种类 4、缩放
pub fn construct_inter_roof_merlons(
    _shapes: &[RoofShape],
    _buffer: &mut SparseInstanceBufferWriter<BrickTransformSsbo>,
    _walls: &PublicWalls,
) {
    for shape in _shapes {
        // 门控（local_f1；证据见上）。
        let halftimbered = style_of(shape).is_halftimbered();
        let flat_no_rim =
            _walls.roof_entity().is_some() && Roof::is_flat_w_no_rim(&Roof);
        let _is_merlon = halftimbered || flat_no_rim; // 随后 ^1 取反（local_f1 ^= 1）。

        let curve = match shape.kind {
            ShapeKind::Circle => circle_as_curve3(shape),
            ShapeKind::Rectangle => rectangle_as_curve3(shape),
        };
        let samples = curve.circular_slice(); // Vec3 stride 0xc
        let merlon_count = (samples.len() as f32 / 0.46).ceil() as usize;

        for seg in samples.segments() {
            let jitter = random_splits_jitter(); // ((r*.5-(1-r)*.5)*0.21*0.8+0.368)*.5
            let width = shape.width_base + 0.23 + jitter;
            let (pos, _dir) = seg.bilinear_pos_dir(); // get_coord_at_u + 归一 + swizzle
            _buffer.push(BrickTransformSsbo {
                // position/size/种类4/(-100,-100)/scale：country-core 布局 unknown。
            });
            debug_assert!(merlon_count > 0);
            let _ = width;
        }
    }
}

// ---- 占位（utils/geometry 复用，不还原实现）----
pub struct Curve<C> {
    _m: std::marker::PhantomData<C>,
}
impl<C> Curve<C> {
    pub fn circular_slice(&self) -> CurveSamples {
        CurveSamples
    }
}
pub struct CurveSamples;
impl CurveSamples {
    pub fn len(&self) -> usize {
        0
    }
    pub fn segments(&self) -> impl Iterator<Item = CurveSeg> {
        std::iter::empty()
    }
}
pub struct CurveSeg;
impl CurveSeg {
    pub fn bilinear_pos_dir(&self) -> (Vec3, Vec3) {
        (Vec3::ZERO, Vec3::ZERO)
    }
    pub fn u(&self) -> f32 {
        0.0
    }
}
pub struct CurveCoordSemantic2D;

fn circle_as_curve3(_s: &RoofShape) -> Curve<CurveCoordSemantic2D> {
    Curve { _m: std::marker::PhantomData }
}
fn rectangle_as_curve3(_s: &RoofShape) -> Curve<CurveCoordSemantic2D> {
    Curve { _m: std::marker::PhantomData }
}
fn random_splits_jitter() -> f32 {
    0.0
}
fn style_of(_s: &RoofShape) -> &WallStyle {
    // TODO(evidence): 形状→墙样式查询。
    &WallStyle
}
fn unknown() -> ! {
    unimplemented!("反汇编级证据：精确逻辑见 evidence/multi-baseline/clutter-extra-analysis.md")
}
