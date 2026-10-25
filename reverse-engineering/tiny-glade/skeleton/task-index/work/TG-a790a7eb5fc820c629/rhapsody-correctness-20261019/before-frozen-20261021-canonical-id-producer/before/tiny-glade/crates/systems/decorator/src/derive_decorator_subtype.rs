//! `derive_decorator_subtype.rs` — 装饰子类型派生
//! （derive_decorator_subtypes 0x140812b20 / derive_decorator_subtype
//! 0x140813d00 / is_half_dormer 0x140814470）。
//!
//! 机器码证据：
//! - `groupD/decomp/decomp-0x140813d00-derive_decorator_subtype.txt`（跳表被
//!   Ghidra 打散；仅 case 2 的墙/屋顶/角检查与 is_half_dormer 调用面可读）。
//! - `building-placement/140814470-original-disasm.txt`（is_half_dormer 全
//!   指令：墙形 tag、flat/gable 过滤、gable 边排除、阈值化最近段、0x160600
//!   族掩码、碰撞表查询、`top > wall_height` 严格比较）。
//! - `building-placement/half-dormer-findings.md`：B 侧完整 CPU consumer 已由
//!   `reconstruction/building/decorator_subtype.rs` 以 1,326 个原版执行案例
//!   验证（0 失败）。本文件按该已验证实现移植到本 crate 的类型面。
//!
//! 记号：O = 机器码/反编译直接观察；I = 由调用面或已验证移植推断；U = 证据
//! 不足，显式拒绝或登记，不以默认值冒充。
//!
//! 覆盖范围（O）：
//! - 窗族 ty 0x38（cottage）/0x39（gothic）的墙目的有限路径：
//!   corner → door → half/plain。
//! - `is_half_dormer` 全公式。
//!
//! 未覆盖（U）：`derive_decorator_subtype` 的其余跳表分支（Ghidra 打散，
//! 地址 0x142a8f990 跳表无法归一化）。这些 ty 不插入 `DerivedDecoratorSubtypes`，
//! 由调用方可见缺失，而不是伪造一个子类型。arrow-slit 0x3a 在 `is_half_dormer`
//! 内使用同一族掩码，但其 subtype/asset 生产者不在四族包内（findings 明确）。

use bevy_ecs::prelude::*;
use std::sync::Arc;

use country_core::resources::roofs::geometry::surface::Curve2 as WallCurve2;
use country_core::resources::roofs::roof_shape::Roof;
use country_core::resources::walls::decorator::DecoratorInfo;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::decorator_subtype::{
    DecoratorLibKey, DecoratorMeshLibrary, DecoratorSubtype, DerivedDecoratorInfo,
    DerivedDecoratorSubtypes,
};
use country_core::resources::walls::public_wall_state::PublicWallState;
use country_core::resources::walls::public_walls::PublicWalls;
use glam::Vec3;
use utils::wall_space::WallSpaceCoord;

/// 已验证的子类型常量（O：half-dormer-findings 的原始 producer accept 集）。
pub const SUBTYPE_COTTAGE_PLAIN: u8 = 0;
pub const SUBTYPE_COTTAGE_HALF: u8 = 1;
/// cottage corner；**不属于**四个 opening family（原值保留，消费者单列）。
pub const SUBTYPE_COTTAGE_CORNER: u8 = 3;
pub const SUBTYPE_COTTAGE_DOOR: u8 = 4;
/// gothic plain 与 corner 共用 6（原始指令，不是命名猜测）。
pub const SUBTYPE_GOTHIC_CORNER_OR_PLAIN: u8 = 6;
pub const SUBTYPE_GOTHIC_HALF: u8 = 7;
pub const SUBTYPE_GOTHIC_DOOR: u8 = 9;

/// 窗族 ty（O：findings “window cottage38/gothic39”）。
pub const TY_COTTAGE_WINDOW: u8 = 0x38;
pub const TY_GOTHIC_WINDOW: u8 = 0x39;
pub const TY_ARROW_SLIT: u8 = 0x3a;

/// `is_half_dormer` 的墙/屋顶输入（O：0x140814470 寄存器面）。
#[derive(Clone, Copy)]
pub struct WallDecoratorSubtypeContext<'a> {
    /// 原始 2D source curve（PublicWallState::simplified_curve_2d）。
    pub source_curve: &'a WallCurve2,
    /// 已发布的映射轮廓（PublicWallState::geometry.outline，Vec3）。
    pub mapped_outline: &'a [[f32; 3]],
    /// 映射段高度（PublicWallState::segment_heights）。
    pub arch_heights: &'a [f32],
    /// 墙形 tag（shape().tag()）：>1 → 非 premade，直接 false。
    pub shape_tag: u32,
    /// 实际 Roof 组件（可选；None = 无屋顶）。
    pub roof: Option<&'a Roof>,
}

/// 碰撞体的垂直界（I：reconstruction `CollisionBoundingBox::vertical_bounds` =
/// bbx 的 Y 轴 [min,max]；`decorator_placement.rs:137`）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CollisionVerticalBounds {
    pub min: f32,
    pub max: f32,
}

impl CollisionVerticalBounds {
    pub fn from_bbx(bbx_min: Vec3, bbx_max: Vec3) -> Self {
        Self {
            min: bbx_min.y,
            max: bbx_max.y,
        }
    }
}

/// 子类型派生失败（不允许静默默认值）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SubtypeError {
    /// 映射轮廓点数/有限性不满足（原 `get_approx_coord_from_pos` 断言面）。
    InvalidPublishedOutline,
    /// 高度数组与轮廓点数不一致或非有限。
    InvalidHeights,
    /// 屋顶形状不是有效矩形。
    InvalidRoofRectangle,
    /// gable direction > 1。
    InvalidRoofDirection,
    /// 碰撞垂直界非有限。
    InvalidCollision,
    /// 碰撞库缺少该 DecoratorLibKey（原 panic "The decorator mesh library
    /// doesn't contain {}"）。
    MissingCollision,
    /// 该 ty 的派生分支未恢复（U，禁止伪造）。
    UnsupportedDecoratorType(u8),
}

/// 窗族（O/I）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WallWindowKind {
    Cottage,
    Gothic,
}

impl WallWindowKind {
    fn plain_subtype(self) -> u8 {
        match self {
            Self::Cottage => SUBTYPE_COTTAGE_PLAIN,
            Self::Gothic => SUBTYPE_GOTHIC_CORNER_OR_PLAIN,
        }
    }
}

fn read_f32(bytes: &[u8; 88], offset: usize) -> f32 {
    f32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap())
}

/// 墙空间坐标 → 世界（O：0x140c92090；移植自 validated reconstruction
/// `wall_coord_to_world`，保持超出端点时的切线外推与 clamp 顺序）。
pub fn wall_coord_to_world(
    curve: &WallCurve2,
    coordinate: WallSpaceCoord,
) -> Result<Vec3, SubtypeError> {
    if curve.points.len() < 2 || !(curve.length > 0.0) {
        return Err(SubtypeError::InvalidPublishedOutline);
    }
    if !coordinate.u.is_finite() || !coordinate.y.is_finite() {
        return Err(SubtypeError::InvalidPublishedOutline);
    }
    let u = coordinate.u / curve.length;
    if !u.is_finite() {
        return Err(SubtypeError::InvalidPublishedOutline);
    }
    let (i, t) = curve.coord_at_u(u.clamp(0.0, 1.0));
    let [a, b] = [curve.points[i], curve.points[i + 1]];
    let mut output = Vec3::new(
        a[0] * (1.0 - t) + b[0] * t,
        coordinate.y,
        a[1] * (1.0 - t) + b[1] * t,
    );
    if coordinate.u < 0.0 || coordinate.u > curve.length {
        let dx = b[0] - a[0];
        let dz = b[1] - a[1];
        let reciprocal = 1.0 / (dx * dx + dz * dz).sqrt();
        let tangent = [reciprocal * dx, reciprocal * dz];
        let reciprocal2 = 1.0 / (tangent[0] * tangent[0] + tangent[1] * tangent[1]).sqrt();
        let tangent = [tangent[0] * reciprocal2, tangent[1] * reciprocal2];
        if coordinate.u > curve.length {
            let excess = coordinate.u - curve.length;
            output.x += excess * tangent[0];
            output.z += excess * tangent[1];
        } else {
            output.x -= coordinate.u * tangent[0];
            output.z -= coordinate.u * tangent[1];
        }
    }
    Ok(output)
}

/// 阈值化 XZ 最近段（O：0x14126a450；移植自 validated reconstruction
/// `nearest_segment`，分母下限 1e-8 = `f32::from_bits(0x322bcc77)`）。
pub fn nearest_segment(
    points: &[[f32; 3]],
    position: Vec3,
) -> Result<(usize, f32), SubtypeError> {
    if points.len() < 2 || points.iter().flatten().any(|v| !v.is_finite()) {
        return Err(SubtypeError::InvalidPublishedOutline);
    }
    let mut best_distance = f32::MAX;
    let mut best = (0, 0.0);
    for (i, pair) in points.windows(2).enumerate() {
        let a = pair[0];
        let b = pair[1];
        let dx = b[0] - a[0];
        let dz = b[2] - a[2];
        let numerator = (position.z - a[2]) * dz + (position.x - a[0]) * dx;
        let denominator = (dz * dz + dx * dx).max(f32::from_bits(0x322bcc77));
        let t = (numerator / denominator).clamp(0.0, 1.0);
        let x = t * b[0] + (1.0 - t) * a[0];
        let z = t * b[2] + (1.0 - t) * a[2];
        let ex = position.x - x;
        let ez = position.z - z;
        let distance = (ez * ez + ex * ex).sqrt();
        if distance < best_distance {
            best_distance = distance;
            best = (i, t);
        }
    }
    Ok(best)
}

/// 屋顶矩形最近边（O：0x140c9a9d0；移植自 validated reconstruction
/// `closest_roof_edge`，初值 1000、分母下限 `f32::from_bits(0x1e3ce508)`、
/// 严格 `<` 保留首条更近边）。
pub fn closest_roof_edge(roof: &Roof, position: Vec3) -> Result<usize, SubtypeError> {
    let bytes = roof.observed();
    if u32::from_le_bytes(bytes[8..12].try_into().unwrap()) != 1 {
        return Err(SubtypeError::InvalidRoofRectangle);
    }
    let [ux, uz, cx, cz, w, l] = [
        read_f32(bytes, 12),
        read_f32(bytes, 16),
        read_f32(bytes, 20),
        read_f32(bytes, 24),
        read_f32(bytes, 28),
        read_f32(bytes, 32),
    ];
    if [ux, uz, cx, cz, w, l]
        .iter()
        .any(|v| !v.is_finite())
        || !(w * l > 0.0)
    {
        return Err(SubtypeError::InvalidRoofRectangle);
    }
    // 矩形角（局部坐标 → 世界；axis=(ux,uz)，法向=(-uz,ux)）。
    let local = [
        [-w * 0.5, -l * 0.5],
        [-w * 0.5, l * 0.5],
        [w * 0.5, l * 0.5],
        [w * 0.5, -l * 0.5],
    ];
    let points = local.map(|[x, z]| [(-uz * z + ux * x) + cx, 0.0, (ux * z + uz * x) + cz]);
    let mut best_distance = 1000.0f32;
    let mut best = 0;
    for i in 0..4 {
        let a = points[i];
        let b = points[(i + 1) % 4];
        let dx = b[0] - a[0];
        let dz = b[2] - a[2];
        let numerator = (position.z - a[2]) * dz + (position.x - a[0]) * dx;
        let t = (numerator / (dz * dz + dx * dx).max(f32::from_bits(0x1e3ce508)))
            .clamp(0.0, 1.0);
        let x = t * b[0] + (1.0 - t) * a[0];
        let z = t * b[2] + (1.0 - t) * a[2];
        let ex = position.x - x;
        let ez = position.z - z;
        let distance = (ez * ez + ex * ex).sqrt();
        if distance < best_distance {
            best_distance = distance;
            best = i;
        }
    }
    Ok(best)
}

/// 族掩码（O：`0x160600 >> ((ty-0x38 & 3) << 3)`；越界 ty 原为 panic）。
pub fn family_mask(ty: u8) -> u8 {
    assert!(
        ty.wrapping_sub(TY_COTTAGE_WINDOW) <= 2,
        "is_half_dormer: ty {ty:#x} outside family range (original panic)"
    );
    let index = (ty.wrapping_sub(TY_COTTAGE_WINDOW) & 3) * 8;
    ((0x160600u32 >> index) & 0xff) as u8
}

/// `is_half_dormer` @0x140814470（O 全指令；verified reconstruction port）。
///
/// 步骤（与原始指令一一对应）：
/// 1. 墙形 tag > 1 → false；
/// 2. 有屋顶：flat → false；gable → 最近边命中 direction 或 direction|2 → false；
/// 3. 墙空间坐标 → 世界，阈值化最近段（映射轮廓）；
/// 4. 段高度线性插值 = 墙高；
/// 5. 碰撞垂直界：`((top-bottom) + (top+bottom)*0.5)*0.5 + world.y >
///    wall_height`（严格 `>`）。
pub fn is_half_dormer(
    coord: WallSpaceCoord,
    context: WallDecoratorSubtypeContext<'_>,
    collision: CollisionVerticalBounds,
) -> Result<bool, SubtypeError> {
    if context.shape_tag > 1 {
        return Ok(false);
    }
    if let Some(roof) = context.roof {
        if roof.is_flat() {
            return Ok(false);
        }
        if roof.is_gable() {
            let sides = roof.gable_sides().ok_or(SubtypeError::InvalidRoofDirection)?;
            let direction = usize::from(sides[0]);
            if direction > 1 {
                return Err(SubtypeError::InvalidRoofDirection);
            }
            let world = wall_coord_to_world(context.source_curve, coord)?;
            let edge = closest_roof_edge(roof, world)?;
            if edge == direction || edge == (direction | 2) {
                return Ok(false);
            }
        }
    }
    let world = wall_coord_to_world(context.source_curve, coord)?;
    let (segment, t) = nearest_segment(context.mapped_outline, world)?;
    if context.arch_heights.len() != context.mapped_outline.len()
        || context.arch_heights.iter().any(|v| !v.is_finite())
    {
        return Err(SubtypeError::InvalidHeights);
    }
    let wall_height = t * (context.arch_heights[segment + 1] - context.arch_heights[segment])
        + context.arch_heights[segment];
    if !collision.min.is_finite() || !collision.max.is_finite() {
        return Err(SubtypeError::InvalidCollision);
    }
    let middle = (collision.min + collision.max) * 0.5;
    let top = ((collision.max - collision.min) + middle) * 0.5 + world.y;
    Ok(top > wall_height)
}

/// 角/门优先级（O：reconstruction `wall_attachment_subtype`；corner → door →
/// half/plain）。cottage corner = 3、gothic corner/plain = 6、gothic door = 9。
pub fn wall_attachment_subtype(
    kind: WallWindowKind,
    door_attachment: bool,
    corner_attached: bool,
) -> Option<u8> {
    if corner_attached {
        return Some(match kind {
            WallWindowKind::Cottage => SUBTYPE_COTTAGE_CORNER,
            WallWindowKind::Gothic => SUBTYPE_GOTHIC_CORNER_OR_PLAIN,
        });
    }
    if door_attachment {
        return Some(match kind {
            WallWindowKind::Cottage => SUBTYPE_COTTAGE_DOOR,
            WallWindowKind::Gothic => SUBTYPE_GOTHIC_DOOR,
        });
    }
    None
}

/// `derive_decorator_subtype` 的墙目的有限路径（O：corner/door/half/plain）。
pub fn derive_wall_subtype(
    kind: WallWindowKind,
    coord: WallSpaceCoord,
    door_attachment: bool,
    corner_attached: bool,
    context: WallDecoratorSubtypeContext<'_>,
    plain_collision: CollisionVerticalBounds,
) -> Result<DecoratorSubtype, SubtypeError> {
    if let Some(subtype) = wall_attachment_subtype(kind, door_attachment, corner_attached) {
        return Ok(DecoratorSubtype(subtype));
    }
    let half = is_half_dormer(coord, context, plain_collision)?;
    Ok(DecoratorSubtype(match kind {
        WallWindowKind::Cottage => u8::from(half),
        WallWindowKind::Gothic => SUBTYPE_GOTHIC_CORNER_OR_PLAIN + u8::from(half),
    }))
}

/// DecoratorInfo 的墙空间坐标（O：Info+0x00/+0x04 两个 f32 = WallSpaceCoord）。
#[inline]
pub fn wall_space_coord_of(info: &DecoratorInfo) -> WallSpaceCoord {
    WallSpaceCoord::new_unchecked(
        f32::from_bits(info.dst_wall as u32),
        f32::from_bits((info.dst_wall >> 32) as u32),
    )
}

/// DecoratorInfo 的角/门附着字节（O：+0x20 corner、+0x21 is_bottom_door）。
#[inline]
pub fn attachment_flags_of(info: &DecoratorInfo) -> (bool, bool) {
    (
        (info.u20 & 0xff) != 0,
        ((info.u20 >> 8) & 1) != 0,
    )
}

/// 碰撞引用（I：本 crate 侧视图；`DecoratorMeshLibrary::collision` 的条目）。
#[derive(Clone, Copy, Debug)]
pub struct PlainCollisionEntry {
    pub vertical_bounds: CollisionVerticalBounds,
    pub bbx_min: Vec3,
    pub bbx_max: Vec3,
}

/// 按 plain 碰撞键查表（O：`DecoratorLibKey::new` + HashMap get）。
///
/// 键空间存在两处证据：reconstruction 的已验证资产键为 `{subtype, rank}`
/// （`wall_decorator_key`），而 `is_half_dormer` 的机器码把族掩码作为第二字节
/// 传入。按“先精确 rank、后族掩码”顺序解析；两者都缺失 → MissingCollision
/// （对应原 panic，不伪造）。
pub fn plain_collision_entry(
    mesh_lib: &DecoratorMeshLibrary,
    plain_subtype: u8,
    rank: u8,
    ty: u8,
) -> Result<PlainCollisionEntry, SubtypeError> {
    let by_rank = DecoratorLibKey::new(plain_subtype, rank);
    let by_mask = DecoratorLibKey::new(plain_subtype, family_mask(ty));
    let info = mesh_lib
        .collision
        .get(&by_rank)
        .or_else(|| mesh_lib.collision.get(&by_mask))
        .ok_or(SubtypeError::MissingCollision)?;
    let (bbx_min, bbx_max) = info.bbx_ls();
    Ok(PlainCollisionEntry {
        vertical_bounds: CollisionVerticalBounds::from_bbx(bbx_min, bbx_max),
        bbx_min,
        bbx_max,
    })
}

/// `derive_decorator_subtypes` @0x140812b20（O 结构；U 项显式跳过）。
///
/// 原版每帧全存储迭代并重建 `DerivedDecoratorSubtypes`。本实现只在有
/// mesh library（`Option<Res>`）时派生窗族两个 ty；其余 ty 的跳表分支仍 U，
/// 不插入（调用方可见缺失，非伪造子类型）。
pub fn derive_decorator_subtypes(
    storage: Res<DecoratorStorage>,
    walls: Res<PublicWalls>,
    roofs: Query<&Roof>,
    mesh_lib: Option<Res<DecoratorMeshLibrary>>,
    mut subtypes: ResMut<DerivedDecoratorSubtypes>,
) {
    // O：puffin 剖面；原版先清空再重建。
    puffin::profile_function!();
    *subtypes = DerivedDecoratorSubtypes::default();
    let Some(mesh_lib) = mesh_lib else {
        log::warn!(
            "derive_decorator_subtypes: DecoratorMeshLibrary resource missing; subtypes stay empty"
        );
        return;
    };
    for (wall, bucket) in storage.iter_walls() {
        let Some(wall_state) = walls.get(wall) else {
            continue;
        };
        let roof = walls.roof_component(wall, |entity| roofs.get(entity).ok());
        for (id, info) in bucket.iter_maybe_mut() {
            match derive_one(wall_state, roof, &mesh_lib, info) {
                Ok(derived) => {
                    subtypes.insert(id, Arc::new(derived));
                }
                Err(SubtypeError::UnsupportedDecoratorType(ty)) => {
                    // U：跳表未恢复；不伪造。
                    log::debug!(
                        "derive_decorator_subtypes: ty {ty:#x} branch not recovered (U); id {} skipped",
                        id
                    );
                }
                Err(error) => {
                    // O：原版对缺失/无效数据 panic/格式化；此处保留可见错误。
                    log::warn!(
                        "derive_decorator_subtypes: id {} derivation failed: {error:?}",
                        id
                    );
                }
            }
        }
    }
}

fn derive_one(
    wall_state: &PublicWallState,
    roof: Option<&Roof>,
    mesh_lib: &DecoratorMeshLibrary,
    info: &DecoratorInfo,
) -> Result<DerivedDecoratorInfo, SubtypeError> {
    let ty = info.ty;
    let kind = match ty {
        TY_COTTAGE_WINDOW => WallWindowKind::Cottage,
        TY_GOTHIC_WINDOW => WallWindowKind::Gothic,
        // 0x3a arrow-slit 使用同一 half 助手，但其 subtype/asset 生产者 U。
        _ => return Err(SubtypeError::UnsupportedDecoratorType(ty)),
    };
    let collision = plain_collision_entry(mesh_lib, kind.plain_subtype(), info.rank, ty)?;
    let context = WallDecoratorSubtypeContext {
        source_curve: wall_state.simplified_curve_2d(),
        mapped_outline: wall_state.geometry.outline.as_slice(),
        arch_heights: &wall_state.segment_heights,
        shape_tag: wall_state.shape().tag(),
        roof,
    };
    let (corner, door) = attachment_flags_of(info);
    let subtype = derive_wall_subtype(
        kind,
        wall_space_coord_of(info),
        door,
        corner,
        context,
        collision.vertical_bounds,
    )?;
    Ok(DerivedDecoratorInfo::new(
        subtype,
        collision.bbx_min,
        collision.bbx_max,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use country_core::resources::roofs::geometry::surface::Curve2;

    fn flat_curve() -> Curve2 {
        Curve2::try_new(vec![[0.0, 0.0], [4.0, 0.0]], false).unwrap()
    }

    fn line_outline() -> Vec<[f32; 3]> {
        vec![[0.0, 0.0, 0.0], [4.0, 0.0, 0.0]]
    }

    #[test]
    fn family_mask_matches_machine_bytes() {
        // O：0x160600 >> {0,8,16} → {0x00,0x06,0x16}
        assert_eq!(family_mask(0x38), 0x00);
        assert_eq!(family_mask(0x39), 0x06);
        assert_eq!(family_mask(0x3a), 0x16);
    }

    #[test]
    fn non_premade_shape_is_never_half() {
        let curve = flat_curve();
        let outline = line_outline();
        let context = WallDecoratorSubtypeContext {
            source_curve: &curve,
            mapped_outline: &outline,
            arch_heights: &[0.0, 0.0],
            shape_tag: 2,
            roof: None,
        };
        let coord = WallSpaceCoord::new_unchecked(2.0, 0.0);
        assert_eq!(
            is_half_dormer(
                coord,
                context,
                CollisionVerticalBounds {
                    min: 0.0,
                    max: 2.0
                }
            ),
            Ok(false)
        );
    }

    #[test]
    fn higher_collision_top_classifies_half() {
        let curve = flat_curve();
        let outline = line_outline();
        let context = WallDecoratorSubtypeContext {
            source_curve: &curve,
            mapped_outline: &outline,
            arch_heights: &[1.0, 1.0],
            shape_tag: 1,
            roof: None,
        };
        // world.y = 0 → wall_height = 1.
        // top = ((max-min) + (max+min)/2)/2 + 0 = ((1-0)+0.5)/2 = 0.75 → false.
        let coord = WallSpaceCoord::new_unchecked(2.0, 0.0);
        let bounds = CollisionVerticalBounds {
            min: 0.0,
            max: 1.0,
        };
        assert_eq!(is_half_dormer(coord, context, bounds), Ok(false));
        let bounds = CollisionVerticalBounds {
            min: 0.0,
            max: 3.0,
        };
        // ((3)+1.5)/2 = 2.25 > 1 → true.
        assert_eq!(is_half_dormer(coord, context, bounds), Ok(true));
    }

    #[test]
    fn corner_and_door_precedence() {
        assert_eq!(
            wall_attachment_subtype(WallWindowKind::Cottage, true, true),
            Some(SUBTYPE_COTTAGE_CORNER)
        );
        assert_eq!(
            wall_attachment_subtype(WallWindowKind::Cottage, true, false),
            Some(SUBTYPE_COTTAGE_DOOR)
        );
        assert_eq!(
            wall_attachment_subtype(WallWindowKind::Gothic, true, false),
            Some(SUBTYPE_GOTHIC_DOOR)
        );
        assert_eq!(wall_attachment_subtype(WallWindowKind::Gothic, false, false), None);
    }

    #[test]
    fn coord_and_flags_read_observed_offsets() {
        // Info+0x00/+0x04 = WallSpaceCoord，+0x20 corner，+0x21 is_bottom_door。
        let info = DecoratorInfo {
            dst_wall: u64::from(2.5f32.to_bits()) | (u64::from((-1.5f32).to_bits()) << 32),
            u20: 0x0000_0101,
            ..DecoratorInfo::default()
        };
        let coord = wall_space_coord_of(&info);
        assert_eq!(coord.u, 2.5);
        assert_eq!(coord.y, -1.5);
        assert_eq!(attachment_flags_of(&info), (true, true));
    }
}
