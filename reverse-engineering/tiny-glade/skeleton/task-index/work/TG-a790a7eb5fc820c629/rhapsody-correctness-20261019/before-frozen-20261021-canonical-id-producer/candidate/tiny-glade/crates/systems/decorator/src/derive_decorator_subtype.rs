//! 原装饰子类型完整派生链：8类×目的分支、懒读取half-dormer，
//! 全存储唯一ID表与烟囱19/20邻近第二遍。原B140813d00/1893B、
//! 140814470/998B和140812b20/4576B均有完整匹配原字节证据。
//! 旧公开有限输入helper保留现Result方言；新生产路径复用canonical几何内核。

use bevy_ecs::prelude::*;

use country_core::resources::roofs::geometry::surface::Curve2 as WallCurve2;
use country_core::resources::roofs::roof_shape::Roof;
use country_core::resources::walls::decorator::DecoratorInfo;
use country_core::resources::walls::decorator_storage::{DecoratorStorage, DecoratorStorageAddr, Decorators};
use country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms;
use country_core::resources::walls::WallId;
use country_core::systems::wall::valid_enclosure::ShapeParameters;
use country_core::resources::walls::decorator_subtype::{
    DecoratorLibKey, DecoratorMeshLibrary, DecoratorSubtype, DerivedDecoratorInfoView,
    DerivedDecoratorSubtypes,
};
use country_core::resources::walls::public_wall_state::PublicWallState;
use country_core::resources::walls::public_walls::PublicWalls;
use glam::{Vec2, Vec3};
use utils::comparative_space::ComparativeSpace;
use utils::wall_space::{WallSpace, WallSpaceCoord};

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
    let nearest = utils::curve::approx_curve_coord(
        points, Vec2::new(position.x, position.z), |p| Vec2::new(p[0], p[2]),
    );
    Ok((nearest.segment, nearest.t))
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

/// 原控制内核在读取点交给supplier的真实选段结果。
#[derive(Clone, Copy)]
struct HalfDormerSample {
    world_y: f32,
    height0: f32,
    height1: f32,
    fraction: f32,
}

/// 只实现一份原控制/数值序列；supplier在真正读取点才获取墙缓存和碰撞。
fn half_dormer_kernel(
    shape_tag: u32,
    roof: Option<&Roof>,
    gable_excludes: impl FnOnce() -> Result<bool, SubtypeError>,
    sample: impl FnOnce() -> Result<HalfDormerSample, SubtypeError>,
    collision: impl FnOnce() -> Result<CollisionVerticalBounds, SubtypeError>,
) -> Result<bool, SubtypeError> {
    if shape_tag > 1 {
        return Ok(false);
    }
    if let Some(roof) = roof {
        if roof.is_flat() {
            return Ok(false);
        }
        if roof.is_gable() && gable_excludes()? {
            return Ok(false);
        }
    }
    let sample = sample()?;
    let collision = collision()?;
    let wall_height = sample.fraction * (sample.height1 - sample.height0) + sample.height0;
    let middle = (collision.min + collision.max) * 0.5;
    let top = ((collision.max - collision.min) + middle) * 0.5 + sample.world_y;
    Ok(top > wall_height)
}

/// 旧公开Result接口保留既有强校验；原生产路径以同一内核懒读实际owner。
pub fn is_half_dormer(
    coord: WallSpaceCoord,
    context: WallDecoratorSubtypeContext<'_>,
    collision: CollisionVerticalBounds,
) -> Result<bool, SubtypeError> {
    half_dormer_kernel(
        context.shape_tag,
        context.roof,
        || {
            let roof = context.roof.expect("gable branch requires actual roof");
            let sides = roof.gable_sides().ok_or(SubtypeError::InvalidRoofDirection)?;
            let direction = usize::from(sides[0]);
            if direction > 1 {
                return Err(SubtypeError::InvalidRoofDirection);
            }
            let world = wall_coord_to_world(context.source_curve, coord)?;
            let edge = closest_roof_edge(roof, world)?;
            Ok(edge == direction || edge == (direction | 2))
        },
        || {
            let world = wall_coord_to_world(context.source_curve, coord)?;
            let (segment, fraction) = nearest_segment(context.mapped_outline, world)?;
            // 旧公开helper的强前提仅保在该方言，不污染原生产路径。
            if context.arch_heights.len() != context.mapped_outline.len()
                || context.arch_heights.iter().any(|height| !height.is_finite())
            {
                return Err(SubtypeError::InvalidHeights);
            }
            Ok(HalfDormerSample {
                world_y: world.y,
                height0: context.arch_heights[segment],
                height1: context.arch_heights[segment + 1],
                fraction,
            })
        },
        || {
            if !collision.min.is_finite() || !collision.max.is_finite() {
                return Err(SubtypeError::InvalidCollision);
            }
            Ok(collision)
        },
    )
}

/// 原998B路径：只读真实prepared positions与选中arch高度，不全数组扫描。
fn native_half_dormer(
    info: &DecoratorInfo,
    wall: &PublicWallState,
    roof: Option<&Roof>,
    mesh_lib: &DecoratorMeshLibrary,
) -> bool {
    let coord = info.wall_space_coord();
    let source_curve = wall.simplified_curve_2d();
    half_dormer_kernel(
        wall.shape().tag(),
        roof,
        || {
            let roof = roof.expect("gable branch requires actual roof");
            let shape = ShapeParameters::from_observed(roof.observed()[8..0x24].try_into().unwrap());
            let rectangle = shape.as_rectangle();
            let sides = roof.gable_sides().expect("actual gable sides");
            let world = coord.to_world_space(source_curve);
            let edge = rectangle.get_closest_edge_id(Vec2::new(world.x, world.z));
            Ok(edge == usize::from(sides[0]) || edge == usize::from(sides[1]))
        },
        || {
            let world = coord.to_world_space(source_curve);
            let positions = &wall.prepared_path.as_ref()
                .expect("wall path has not been prepared").positions;
            let nearest = utils::curve::approx_curve_coord(
                positions, Vec2::new(world.x, world.z), |p| Vec2::new(p[0], p[2]),
            );
            let heights = wall.arch_segment_heights().expect("wall arch heights have not been calculated");
            Ok(HalfDormerSample {
                world_y: world.y,
                height0: heights[nearest.segment],
                height1: heights[nearest.segment + 1],
                fraction: nearest.t,
            })
        },
        || {
            let collision = mesh_lib.collision_record(DecoratorLibKey::new(family_mask(info.ty), info.rank));
            Ok(CollisionVerticalBounds { min: collision.bounds.bottom, max: collision.bounds.top })
        },
    ).expect("native half-dormer dependencies must be present")
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

/// 原1893B完整派生；拟放置位置不能从旧ID缓存反查子类型。
pub fn derive_decorator_subtype(
    info: &DecoratorInfo,
    storage: DecoratorStorageAddr,
    derived: DerivedDecoratorInfoView<'_>,
    walls: &PublicWalls,
    roofs: &Query<&Roof>,
) -> DecoratorSubtype {
    let destination = match info.dst_tag() { 2 => 1, 3 => 2, 4 => 3, _ => 0 };
    let subtype = match info.ty {
        TY_COTTAGE_WINDOW | TY_GOTHIC_WINDOW => {
            let gothic = info.ty == TY_GOTHIC_WINDOW;
            match destination {
                0 => {
                    let wall_id = storage.as_wall();
                    let wall = walls.get(wall_id).expect("window storage wall must exist");
                    let roof = walls.roof_component(wall_id, |entity| roofs.get(entity).ok());
                    if info.u20 as u8 != 0 {
                        if gothic { 6 } else { 3 }
                    } else if WallId(info.dst_secondary).is_valid() {
                        if gothic { 9 } else { 4 }
                    } else {
                        let half = u8::from(native_half_dormer(info, wall, roof, derived.mesh_library()));
                        if gothic { 6 + half } else { half }
                    }
                }
                1 => {
                    let roof = walls.roof_component(storage.as_wall(), |entity| roofs.get(entity).ok());
                    if roof.is_some_and(Roof::is_flat) {
                        if gothic { 10 } else { 5 }
                    } else if gothic { 8 } else { 2 }
                }
                _ => panic!("window subtype does not support this destination"),
            }
        }
        TY_ARROW_SLIT => {
            // 这一分支原直接检查tag<=1，不接受其它类型跳表的raw-tag回退。
            assert!(info.dst_tag() < 2, "arrow slit requires a wall destination");
            let wall_id = storage.as_wall();
            let wall = walls.get(wall_id).expect("arrow-slit storage wall must exist");
            let roof = walls.roof_component(wall_id, |entity| roofs.get(entity).ok());
            if info.u20 as u8 != 0 { 22 }
            else { 22 + u8::from(native_half_dormer(info, wall, roof, derived.mesh_library())) }
        }
        0x3b => match destination {
            0 => 19,
            1 => 21,
            _ => panic!("chimney subtype does not support this destination"),
        },
        0x3c => match destination { 0 => 16, 2 => 17, _ => 18 },
        0x3d => match destination {
            0 => if info.u20 as u8 != 0 { 12 } else { 11 },
            1 => {
                let roof = walls.roof_component(storage.as_wall(), |entity| roofs.get(entity).ok());
                if roof.is_some_and(Roof::is_flat) { 14 } else { 13 }
            }
            2 => 15,
            _ => 14,
        },
        0x3f => 25,
        _ => 24,
    };
    DecoratorSubtype(subtype)
}

/// 按原Chain顺序借用全存储；现core iter()仍只遍历wall，不能直接照搬其简化。
fn all_storages(storage: &DecoratorStorage) -> impl Iterator<Item = (DecoratorStorageAddr, &Decorators)> {
    storage.iter_walls().map(|(wall, decorators)| (DecoratorStorageAddr::Wall(wall), decorators))
        .chain(storage.iter_terrain_decorators().map(|decorators| (DecoratorStorageAddr::Terrain, decorators)))
        .chain(storage.iter_stair_decorators().map(|decorators| (DecoratorStorageAddr::Stair, decorators)))
}

/// 原4576B：保同一ID表allocation，完整派生后执行烟囱邻近第二遍。
pub fn derive_decorator_subtypes(
    mut subtypes: ResMut<DerivedDecoratorSubtypes>,
    storage: Res<DecoratorStorage>,
    mesh_lib: Res<DecoratorMeshLibrary>,
    walls: Res<PublicWalls>,
    roofs: Query<&Roof>,
    transforms: Res<CachedDecoratorTransforms>,
) {
    puffin::profile_function!();
    subtypes.clear();
    for (address, decorators) in all_storages(&storage) {
        for (id, info) in decorators.iter_maybe_mut() {
            let subtype = derive_decorator_subtype(
                info, address, DerivedDecoratorInfoView::new(&subtypes, &mesh_lib), &walls, &roofs,
            );
            if let Some(previous) = subtypes.insert(id, subtype) {
                let (previous_address, previous_info) = all_storages(&storage)
                    .find_map(|(address, decorators)| decorators.decorators.get(&id).map(|info| (address, info)))
                    .expect("original duplicate ID must still exist in storage");
                // 现架构借Debug呈现真实Info/Addr；保原error门、全部诊断值及最终panic。
                log::error!("Duplicate decorator ID: {id}\nInfo: {info:?}\nStorage addr: {address:?}\nPrev Info: {previous_info:?}\nPrev storage addr: {previous_address:?}\nSubtype: {subtype:?}, Prev subtype: {previous:?}");
                panic!("Duplicate decorator ID: {id}");
            }
        }
    }
    for (wall_id, decorators) in storage.iter_walls() {
        for (id, info) in decorators.iter_maybe_mut() {
            if info.ty != 0x3b || info.dst_tag() > 1 {
                continue;
            }
            let subtype = if chimney_has_neighbor(id, info, wall_id, decorators, &subtypes, &mesh_lib, &walls, &transforms) {
                20
            } else { 19 };
            let _ = subtypes.insert(id, DecoratorSubtype(subtype));
        }
    }
}

fn chimney_has_neighbor(
    id: u32,
    info: &DecoratorInfo,
    wall_id: WallId,
    decorators: &Decorators,
    subtypes: &DerivedDecoratorSubtypes,
    meshes: &DecoratorMeshLibrary,
    walls: &PublicWalls,
    transforms: &CachedDecoratorTransforms,
) -> bool {
    let wall = walls.get(wall_id).expect("chimney storage wall must exist");
    let coord = info.wall_space_coord();
    let maximum_y = wall.max_y();
    let extension = info.extra().as_chimney().f;
    let remaining = maximum_y - coord.y;
    let remaining = if remaining > 0.0 { remaining } else { 0.0 };
    let span = (remaining + 3.0) + extension;
    let space = WallSpace { is_loop: wall.wall_space().is_some(), length: wall.wall_space_length() };
    // 原乘零也保留：span为Inf时的NaN不能用常量offset=0遮掉。
    let u = space.project_unbounded(coord.u + (span * 0.0) * 0.5);
    let point = WallSpaceCoord::new_unchecked(u, maximum_y).to_world_space(wall.simplified_curve_2d());
    let half_span = span * 0.5;
    let center_y = coord.y + half_span;
    for (other_id, other) in decorators.iter_maybe_mut() {
        let close = match other.dst_tag() {
            2 => {
                let other_position = transforms.position(other_id);
                let delta = other_position - point;
                (delta.z * delta.z + (delta.y * delta.y + delta.x * delta.x)).sqrt() < 1.0
            }
            3 | 4 => unreachable!("non-wall/roof decorator in chimney wall storage"),
            _ => {
                let collision = meshes.collision_record(DecoratorLibKey::new(subtypes.get(other_id).0, other.rank));
                let other_coord = other.wall_space_coord();
                let half_height = (collision.bounds.top - collision.bounds.bottom) * 0.5;
                if half_height + half_span > (other_coord.y - center_y).abs() {
                    space.distance(other_coord.u, u) < collision.bounds.footprint.size.x * 0.5 + f32::from_bits(0x3ed9999a)
                } else { false }
            }
        };
        // 原先读取/比较，再排除自己；否则缺本体mesh/cache会被静默吞掉。
        if other_id != id && close {
            return true;
        }
    }
    false
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


#[cfg(test)]
mod canonical_subtype_producer_tests {
    use super::*;
    use bevy_ecs::{schedule::Schedule, system::SystemState};
    use country_core::resources::walls::{InnerWalls, WallState};
    use country_core::resources::walls::decorator::{DecoratorExtraInfo, DecoratorId, ChimneyExtra};
    use country_core::resources::walls::decorator_subtype::{DecoratorCollision, DecoratorOutline, DecoratorCollisionOutline};
    use country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransform;
    use country_core::resources::walls::publication_source::TerrainHeightGrid512;
    use country_core::resources::walls::publication_fields::WallShapeSnapshot;
    use country_core::systems::collision::world::RaycastMeshData;
    use country_core::systems::wall::valid_enclosure::Shape;
    use country_core::systems::wall::wall_state::path_preparation::PathRasters512;
    use utils::geometry::{Rectangle2d, Rectangle3d};
    use std::{panic::{catch_unwind, AssertUnwindSafe}, sync::Arc};

    fn info(ty: u8, destination: u8, u: f32, y: f32) -> DecoratorInfo {
        DecoratorInfo {
            ty, rank: 1, u20: u32::from(destination) << 16,
            dst_wall: u64::from(u.to_bits()) | (u64::from(y.to_bits()) << 32),
            ..DecoratorInfo::default()
        }
    }

    fn public_wall(premade: bool) -> (PublicWalls, WallId) {
        let curve = WallCurve2::try_new(vec![[0.0, 0.0], [8.0, 0.0]], false).unwrap();
        let mut state = WallState::new(curve, 4.0, 0, 0, false);
        if premade {
            let (mut snapshot, freehand) = state.into_snapshot();
            snapshot.source.shape = WallShapeSnapshot::from_observed(ShapeParameters::from_shape(
                Shape::Rectangle(Rectangle2d::new(Vec2::new(4.0, 0.0), Vec2::new(8.0, 2.0), Vec2::X)),
            ).to_record());
            state = WallState::from_snapshot(snapshot, freehand);
        }
        let mut inner = InnerWalls::new();
        let id = inner.new_wall(state);
        let terrain = TerrainHeightGrid512 {
            encoded: vec![0; 512 * 512].into(), minimum_y: 0.0, maximum_y: 0.0, sampling_fix: false,
        };
        let mut walls = PublicWalls::default();
        walls.consume(&mut inner, &terrain);
        assert!(walls.get(id).is_some());
        (walls, id)
    }

    fn derive_in_world(
        world: &mut World, info: &DecoratorInfo, address: DecoratorStorageAddr,
        subtypes: &DerivedDecoratorSubtypes, meshes: &DecoratorMeshLibrary, walls: &PublicWalls,
    ) -> DecoratorSubtype {
        let mut state = SystemState::<Query<&Roof>>::new(world);
        let roofs = state.get(world);
        derive_decorator_subtype(info, address, DerivedDecoratorInfoView::new(subtypes, meshes), walls, &roofs)
    }

    fn collision(width: f32, height: f32) -> DecoratorCollision {
        let size = Vec3::new(width, height, 1.0);
        DecoratorCollision {
            triangles: Vec::new(),
            mesh: Arc::new(RaycastMeshData::new_cuboid_at(size, Vec3::ZERO)),
            bounds: Rectangle3d::new(Vec3::ZERO, size, Vec2::X),
        }
    }

    fn insert_mesh(meshes: &mut DecoratorMeshLibrary, subtype: u8, rank: u8, width: f32, height: f32) {
        meshes.insert_collision_and_interaction_as_same(
            DecoratorLibKey::new(subtype, rank), collision(width, height),
            DecoratorOutline::None, DecoratorCollisionOutline::None,
        );
    }

    #[test]
    fn all_eight_primary_and_destination_branches_use_the_real_owner_dependencies() {
        let (walls, wall_id) = public_wall(false);
        let mut world = World::new();
        let subtypes = DerivedDecoratorSubtypes::default();
        let meshes = DecoratorMeshLibrary::default();
        let matrix = [
            [Some(0), Some(2), None, None], [Some(6), Some(8), None, None],
            [Some(22), None, None, None], [Some(19), Some(21), None, None],
            [Some(16), Some(18), Some(17), Some(18)], [Some(11), Some(13), Some(15), Some(14)],
            [Some(24); 4], [Some(25); 4],
        ];
        for (index, expected) in matrix.into_iter().enumerate() {
            for (destination, expected) in [0, 2, 3, 4].into_iter().zip(expected) {
                let input = info(0x38 + index as u8, destination, 2.0, 0.0);
                let result = catch_unwind(AssertUnwindSafe(|| derive_in_world(
                    &mut world, &input, DecoratorStorageAddr::Wall(wall_id), &subtypes, &meshes, &walls,
                )));
                match expected {
                    Some(subtype) => assert_eq!(result.unwrap(), DecoratorSubtype(subtype)),
                    None => assert!(result.is_err()),
                }
            }
        }
        assert_eq!(derive_in_world(&mut world, &info(0x38, 255, 2.0, 0.0),
            DecoratorStorageAddr::Wall(wall_id), &subtypes, &meshes, &walls), DecoratorSubtype(0));
        assert!(catch_unwind(AssertUnwindSafe(|| derive_in_world(&mut world, &info(0x3a, 255, 2.0, 0.0),
            DecoratorStorageAddr::Wall(wall_id), &subtypes, &meshes, &walls))).is_err());
        let empty_walls = PublicWalls::default();
        for ty in [0, 0x37, 0x3e, 0x40, 0xff] {
            assert_eq!(derive_in_world(&mut world, &info(ty, 4, 0.0, 0.0), DecoratorStorageAddr::Stair,
                &subtypes, &meshes, &empty_walls), DecoratorSubtype(24));
        }
    }

    #[test]
    fn corner_then_actual_platform_precedes_half_without_unused_geometry_reads() {
        let (walls, wall_id) = public_wall(false);
        let mut world = World::new();
        let subtypes = DerivedDecoratorSubtypes::default();
        let meshes = DecoratorMeshLibrary::default();
        for (ty, corner, door, plain) in [(0x38, 3, 4, 0), (0x39, 6, 9, 6)] {
            let mut input = info(ty, 0, 2.0, 0.0);
            input.dst_secondary = 11;
            assert_eq!(derive_in_world(&mut world, &input, DecoratorStorageAddr::Wall(wall_id),
                &subtypes, &meshes, &walls), DecoratorSubtype(door));
            input.u20 |= 1;
            assert_eq!(derive_in_world(&mut world, &input, DecoratorStorageAddr::Wall(wall_id),
                &subtypes, &meshes, &walls), DecoratorSubtype(corner));
            input.u20 = 1 << 8;
            input.dst_secondary = 0;
            assert_eq!(derive_in_world(&mut world, &input, DecoratorStorageAddr::Wall(wall_id),
                &subtypes, &meshes, &walls), DecoratorSubtype(plain));
        }
    }

    #[test]
    fn optional_roof_uses_query_presence_and_does_not_require_an_unused_wall_record() {
        let mut world = World::new();
        let roof = world.spawn(Roof::from_observed([0; 88])).id();
        let wall = WallId(99);
        let mut walls = PublicWalls::default();
        walls.associate_roof(wall, roof);
        let subtypes = DerivedDecoratorSubtypes::default();
        let meshes = DecoratorMeshLibrary::default();
        let input = info(0x38, 2, 0.0, 0.0);
        assert_eq!(derive_in_world(&mut world, &input, DecoratorStorageAddr::Wall(wall),
            &subtypes, &meshes, &walls), DecoratorSubtype(5));
        assert!(world.despawn(roof));
        assert_eq!(derive_in_world(&mut world, &input, DecoratorStorageAddr::Wall(wall),
            &subtypes, &meshes, &walls), DecoratorSubtype(2));
        assert!(catch_unwind(AssertUnwindSafe(|| derive_in_world(&mut world, &input,
            DecoratorStorageAddr::Terrain, &subtypes, &meshes, &walls))).is_err());
    }

    #[test]
    fn native_half_reads_selected_arch_heights_and_exact_rank_without_a_global_scan() {
        let (mut walls, wall_id) = public_wall(true);
        let paths = PathRasters512::from_encoded(std::array::from_fn(|_| Arc::<[u8]>::from(vec![0; 512 * 512])), false).unwrap();
        let count = {
            let wall = walls.get_mut(wall_id).unwrap();
            wall.prepare_paths(&paths).unwrap();
            let count = wall.prepared_path.as_ref().unwrap().positions.len();
            wall.regular_heights.as_mut().unwrap().arch_segment_heights = vec![1.0; count];
            count
        };
        let mut meshes = DecoratorMeshLibrary::default();
        for subtype in [0, 6, 22] { insert_mesh(&mut meshes, subtype, 2, 2.0, 3.0); }
        let mut subtypes = DerivedDecoratorSubtypes::default();
        assert_eq!(subtypes.insert(0, DecoratorSubtype(25)), None);
        let mut world = World::new();
        let mut input = info(0x38, 0, 2.0, 0.0); input.rank = 2;
        assert_eq!(derive_in_world(&mut world, &input, DecoratorStorageAddr::Wall(wall_id),
            &subtypes, &meshes, &walls), DecoratorSubtype(1));
        {
            let wall = walls.get(wall_id).unwrap();
            let collision = meshes.collision_record(DecoratorLibKey::new(0, 2));
            let context = WallDecoratorSubtypeContext {
                source_curve: wall.simplified_curve_2d(),
                mapped_outline: &wall.prepared_path.as_ref().unwrap().positions,
                arch_heights: wall.arch_segment_heights().unwrap(),
                shape_tag: wall.shape().tag(), roof: None,
            };
            assert_eq!(is_half_dormer(input.wall_space_coord(), context,
                CollisionVerticalBounds { min: collision.bounds.bottom, max: collision.bounds.top }), Ok(true));
        }
        {
            let wall = walls.get_mut(wall_id).unwrap();
            let heights = &mut wall.regular_heights.as_mut().unwrap().arch_segment_heights;
            heights.extend([1.0, f32::NAN]);
            assert_eq!(heights.len(), count + 2);
            wall.segment_heights.fill(10000.0);
            // 未选中高度和未参与XZ投影的Y，不是原路径的whole-array前提。
            for position in &mut wall.prepared_path.as_mut().unwrap().positions { position[1] = f32::NAN; }
        }
        for (ty, subtype) in [(0x38, 1), (0x39, 7), (0x3a, 23)] {
            input.ty = ty;
            assert_eq!(derive_in_world(&mut world, &input, DecoratorStorageAddr::Wall(wall_id),
                &subtypes, &meshes, &walls), DecoratorSubtype(subtype));
        }
        input = info(0x38, 0, 2.0, -10.0); input.rank = 2;
        // 相同旧ID缓存25，拟位置变化仍必须重新算成plain0。
        assert_eq!(derive_in_world(&mut world, &input, DecoratorStorageAddr::Wall(wall_id),
            &subtypes, &meshes, &walls), DecoratorSubtype(0));
        input = info(0x38, 0, 2.0, 0.0); input.rank = 1;
        assert!(catch_unwind(AssertUnwindSafe(|| derive_in_world(&mut world, &input,
            DecoratorStorageAddr::Wall(wall_id), &subtypes, &meshes, &walls))).is_err());
    }

    fn producer_world(storage: DecoratorStorage, walls: PublicWalls, meshes: DecoratorMeshLibrary) -> World {
        let mut world = World::new();
        world.insert_resource(storage); world.insert_resource(walls); world.insert_resource(meshes);
        world.insert_resource(DerivedDecoratorSubtypes::default());
        world.insert_resource(CachedDecoratorTransforms::default());
        world
    }

    fn run_producer(world: &mut World) {
        let mut schedule = Schedule::default();
        schedule.add_systems(derive_decorator_subtypes);
        schedule.run(world);
    }

    #[test]
    fn producer_publishes_wall_terrain_stair_ids_and_clears_removed_ids() {
        let mut storage = DecoratorStorage::default();
        storage.get_or_insert_default(DecoratorStorageAddr::Wall(WallId(1))).insert(DecoratorId(11), info(0x3e, 0, 0.0, 0.0));
        storage.get_or_insert_default(DecoratorStorageAddr::Terrain).insert(DecoratorId(22), info(0x3c, 3, 0.0, 0.0));
        storage.get_or_insert_default(DecoratorStorageAddr::Stair).insert(DecoratorId(33), info(0x3d, 4, 0.0, 0.0));
        let mut world = producer_world(storage, PublicWalls::default(), DecoratorMeshLibrary::default());
        run_producer(&mut world);
        assert_eq!(world.resource::<DerivedDecoratorSubtypes>().get(11), DecoratorSubtype(24));
        assert_eq!(world.resource::<DerivedDecoratorSubtypes>().get(22), DecoratorSubtype(17));
        assert_eq!(world.resource::<DerivedDecoratorSubtypes>().get(33), DecoratorSubtype(14));
        world.resource_mut::<DecoratorStorage>().terrain.as_mut().unwrap().decorators.remove(&22);
        run_producer(&mut world);
        assert!(catch_unwind(AssertUnwindSafe(|| world.resource::<DerivedDecoratorSubtypes>().get(22))).is_err());
        assert_eq!(world.resource::<DerivedDecoratorSubtypes>().get(33), DecoratorSubtype(14));
    }

    #[test]
    fn duplicate_id_panics_after_preserving_the_replacement_in_the_same_map() {
        let mut storage = DecoratorStorage::default();
        storage.get_or_insert_default(DecoratorStorageAddr::Wall(WallId(1))).insert(DecoratorId(7), info(0x3e, 0, 0.0, 0.0));
        storage.get_or_insert_default(DecoratorStorageAddr::Terrain).insert(DecoratorId(7), info(0x3c, 3, 0.0, 0.0));
        let mut world = producer_world(storage, PublicWalls::default(), DecoratorMeshLibrary::default());
        assert!(catch_unwind(AssertUnwindSafe(|| run_producer(&mut world))).is_err());
        assert_eq!(world.resource::<DerivedDecoratorSubtypes>().get(7), DecoratorSubtype(17));
    }

    fn chimney_world(neighbor: Option<DecoratorInfo>) -> (World, WallId, Vec3) {
        let (walls, wall_id) = public_wall(false);
        let maximum = walls.get(wall_id).unwrap().max_y();
        let point = WallSpaceCoord::new_unchecked(0.0, maximum).to_world_space(walls.get(wall_id).unwrap().simplified_curve_2d());
        let mut storage = DecoratorStorage::default();
        let bucket = storage.get_or_insert_default(DecoratorStorageAddr::Wall(wall_id));
        let mut chimney = info(0x3b, 0, 0.0, 0.0);
        chimney.set_extra(DecoratorExtraInfo::Chimney(ChimneyExtra { f: 0.0, smoke: 0 }));
        bucket.insert(DecoratorId(1), chimney);
        if let Some(neighbor) = neighbor { bucket.insert(DecoratorId(2), neighbor); }
        let mut meshes = DecoratorMeshLibrary::default();
        insert_mesh(&mut meshes, 19, 1, 1.0, 1.0);
        insert_mesh(&mut meshes, 20, 1, 1.0, 1.0);
        insert_mesh(&mut meshes, 24, 1, 2.0, 2.0);
        (producer_world(storage, walls, meshes), wall_id, point)
    }

    #[test]
    fn chimney_second_pass_uses_strict_wall_span_and_reverts_after_neighbor_removal() {
        let (mut world, wall_id, _) = chimney_world(Some(info(0x3e, 0, 0.0, 3.5)));
        run_producer(&mut world);
        assert_eq!(world.resource::<DerivedDecoratorSubtypes>().get(1), DecoratorSubtype(20));
        world.resource_mut::<DecoratorStorage>().per_wall.get_mut(&wall_id).unwrap().decorators.remove(&2);
        run_producer(&mut world);
        assert_eq!(world.resource::<DerivedDecoratorSubtypes>().get(1), DecoratorSubtype(19));
        let threshold = 1.0 + f32::from_bits(0x3ed9999a);
        world.resource_mut::<DecoratorStorage>().per_wall.get_mut(&wall_id).unwrap()
            .insert(DecoratorId(2), info(0x3e, 0, threshold, 3.5));
        run_producer(&mut world);
        assert_eq!(world.resource::<DerivedDecoratorSubtypes>().get(1), DecoratorSubtype(19));
    }

    #[test]
    fn chimney_roof_neighbor_reads_the_real_cache_and_uses_strict_world_distance() {
        let (mut world, _, point) = chimney_world(Some(info(0x3e, 2, 0.0, 0.0)));
        world.resource_mut::<CachedDecoratorTransforms>().insert(2, CachedDecoratorTransform {
            rotation: glam::Quat::IDENTITY, translation: point + Vec3::Z * 0.5, scale: Vec3::ONE,
        });
        run_producer(&mut world);
        assert_eq!(world.resource::<DerivedDecoratorSubtypes>().get(1), DecoratorSubtype(20));
        world.resource_mut::<CachedDecoratorTransforms>().insert(2, CachedDecoratorTransform {
            rotation: glam::Quat::IDENTITY, translation: point + Vec3::Z, scale: Vec3::ONE,
        });
        run_producer(&mut world);
        assert_eq!(world.resource::<DerivedDecoratorSubtypes>().get(1), DecoratorSubtype(19));
    }

    #[test]
    fn chimney_self_reads_required_mesh_before_self_exclusion_and_missing_cache_fails() {
        let (mut world, _, _) = chimney_world(None);
        world.resource_mut::<DecoratorMeshLibrary>().collision.clear();
        assert!(catch_unwind(AssertUnwindSafe(|| run_producer(&mut world))).is_err());
        assert_eq!(world.resource::<DerivedDecoratorSubtypes>().get(1), DecoratorSubtype(19));
        let (mut missing_cache, _, _) = chimney_world(Some(info(0x3e, 2, 0.0, 0.0)));
        assert!(catch_unwind(AssertUnwindSafe(|| run_producer(&mut missing_cache))).is_err());
    }
}
