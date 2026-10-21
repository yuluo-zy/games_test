//! `collision_detection.rs` — 装饰碰撞检测族（墙空间相交谓词 + 投影）。
//!
//! 证据：
//! - `intersect_wall_decorators<F>` 0x141fe5900（merge proposal 实例）/
//!   0x141fe5c20（interaction 实例）：`groupD/misc/decomp/decomp-141fe5900.txt`
//!   与 `toolchain/decor-trunk/decompiled/141fe5900_*.c`。两实例仅在
//!   `ComparativeSpace`（开墙 `ZeroBoundedComparativeSpace` / 闭环
//!   `CircularComparativeSpace`）上不同；遍历 `Decorators` 内嵌 map，逐条
//!   (DecoratorId, DecoratorInfo) 调闭包，命中把 id push 到输出 Vec。
//! - 谓词闭包 `propose_wall_decorator_merge_position_inner::closure_env$1`
//!   0x141fe5310：`groupD/misc/decomp/decomp-141fe5310.txt`。公式（O）：
//!   * 排除集（两个 id）：命中 → false；
//!   * 既有装饰 dst side：1（roof）→ false，2/3 → panic（原不可达）；
//!   * `dy = |candidate.y - existing.y|`；
//!   * ty != 0x3a：`0.3 * existing_vertical_extent < dy` → false；
//!   * `margin = 0.6`（ty != 0x3a）/ `0.2`（ty == 0x3a），
//!     `vertical_extra = 0.0`（!=0x3a）/ `1.0`（==0x3a）；
//!   * `dy < (existing_vertical_extent + vertical_extra) * 0.5 +
//!     candidate_vertical_extent * 0.5` 且
//!     `dx < candidate_along_width * 0.5 +
//!     (existing_along_width + margin) * 0.5` → true。
//!   常量经 `rne-read.py` 复核：0.3=0x3e99999a、0.6=0x3f19999a、
//!   0.2=0x3e4ccccd、0.5=0x3f000000、abs mask=0x7fffffff。
//! - 既有装饰的碰撞查找（O）：`DerivedDecoratorSubtypes::get(id)` → subtype，
//!   `DecoratorLibKey::new(subtype, info.rank)` → mesh collision entry。
//!
//! 记号：O = 机器码直接观察；I = 由调用面/reconstruction 模型映射；U = 未闭合。
//!
//! 真实碰撞度量借用唯一subtype表与四表owner；旧region辅助接口仍显式接收
//! legacy元数据，不用于伪造canonical碰撞网格。

use country_core::resources::walls::decorator::DecoratorInfo;
use country_core::resources::walls::decorator_storage::Decorators;
use country_core::resources::walls::decorator_subtype::{
    DecoratorLibKey, DecoratorMeshLibrary, DerivedDecoratorInfo, DerivedDecoratorSubtypes,
};
use glam::{Vec2, Vec3};
use utils::curve::Curve2 as SamplingCurve2;
use utils::wall_space::{WallSpaceCoord, WallSpaceRegion};

/// 原碰撞记录的沿墙逻辑宽度与垂直跨度。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DecoratorCollisionMetrics {
    /// 沿墙宽度（原记录+0x30，即bounds.footprint.size.x）。
    pub along_width: f32,
    /// 垂直跨度（原记录+0x3c-+0x38，即bounds.top-bottom）。
    pub vertical_extent: f32,
}

impl DecoratorCollisionMetrics {
    /// I：DerivedDecoratorInfo 的 bbx 映射。core 模型没有原 0x48B 布局的
    /// +0x30 槽，故沿墙宽度取 X 跨度、垂直跨度取 Y 跨度；登记为跨 crate 缺口。
    pub fn from_bbx(bbx_min: Vec3, bbx_max: Vec3) -> Self {
        Self {
            along_width: bbx_max.x - bbx_min.x,
            vertical_extent: bbx_max.y - bbx_min.y,
        }
    }
}

/// 候选合并位置（O：0x141fe5310 的 `param_1[2]` 四浮点
/// `{x, y, along_width, vertical_extent}`）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WallIntersectCandidate {
    pub coord: WallSpaceCoord,
    pub along_width: f32,
    pub vertical_extent: f32,
}

/// `intersect_wall_decorators`（0x141fe5900 / 0x141fe5c20，O 结构）：
/// 对 `Decorators` 内嵌 map 逐装饰调 `predicate(id, info)`；命中 id 收集返回。
/// （原闭包实参 = (DecoratorId, DecoratorInfo)；两 ComparativeSpace 变体只影响
/// 宿主外层，谓词本体相同。）
pub fn intersect_wall_decorators<F>(decorators: &Decorators, mut predicate: F) -> Vec<u32>
where
    F: FnMut(u32, &DecoratorInfo) -> bool,
{
    let mut hits = Vec::new();
    for (id, info) in decorators.decorators.iter() {
        if predicate(*id, info) {
            hits.push(*id);
        }
    }
    hits
}

/// dst side 公式（O：`(tag-2)<=2 ? tag-1 : 0`，即 tag∈{2,3,4} → 1/2/3）。
#[inline]
pub fn wall_attachment_side(tag: u8) -> u8 {
    if 2 < tag.wrapping_sub(2) {
        0
    } else {
        tag.wrapping_sub(1)
    }
}

/// 0x141fe5310 谓词本体（O 全文；排除集与既有装饰信息由调用方提供）。
pub fn wall_decorator_intersects(
    candidate: WallIntersectCandidate,
    existing_id: u32,
    existing_coord: WallSpaceCoord,
    existing_metrics: DecoratorCollisionMetrics,
    existing_ty: u8,
    excluded: &[u32],
) -> bool {
    if excluded.contains(&existing_id) {
        return false;
    }
    let dy = (candidate.coord.y - existing_coord.y).abs();
    let dx = (candidate.coord.u - existing_coord.u).abs();
    let (margin, vertical_extra) = if existing_ty == 0x3a {
        // O：0x3a arrow slit 分支：margin 0.2、额外 1.0，且无 0.3 早退。
        (0.2f32, 1.0f32)
    } else {
        if 0.3 * existing_metrics.vertical_extent < dy {
            return false;
        }
        (0.6f32, 0.0f32)
    };
    let vertical_overlap = dy
        < (existing_metrics.vertical_extent + vertical_extra) * 0.5
            + candidate.vertical_extent * 0.5;
    if !vertical_overlap {
        return false;
    }
    dx < candidate.along_width * 0.5 + (existing_metrics.along_width + margin) * 0.5
}

/// 既有装饰的碰撞度量查找（O：`DerivedDecoratorSubtypes::get(id)` → subtype，
/// key = {subtype, rank}，再查 mesh collision 表）。两条缺失路径在原版均为
/// panic（`DerivedDecoratorSubtypes::get` / mesh `expect`），本实现保留。
pub fn existing_collision_metrics(
    id: u32,
    info: &DecoratorInfo,
    subtypes: &DerivedDecoratorSubtypes,
    mesh_lib: &DecoratorMeshLibrary,
) -> DecoratorCollisionMetrics {
    let derived = subtypes.get(id);
    let key = DecoratorLibKey::new(derived.0, info.rank);
    let collision = mesh_lib.collision_record(key);
    // 原+0x30与+0x3c/+0x38：读取同一真实记录的逻辑宽度和垂直界。
    DecoratorCollisionMetrics {
        along_width: collision.bounds.footprint.size.x,
        vertical_extent: collision.bounds.top - collision.bounds.bottom,
    }
}

/// 组合谓词：候选 vs 一个既有装饰（供 merge proposal / interaction 使用）。
#[allow(clippy::too_many_arguments)]
pub fn wall_decorator_intersects_existing(
    candidate: WallIntersectCandidate,
    existing_id: u32,
    info: &DecoratorInfo,
    subtypes: &DerivedDecoratorSubtypes,
    mesh_lib: &DecoratorMeshLibrary,
    excluded: &[u32],
) -> bool {
    let side = wall_attachment_side(info.dst_tag());
    if side == 1 {
        // O：roof side 不参与墙相交。
        return false;
    }
    assert!(
        side == 0,
        "internal error: entered unreachable code (wall decorator side {side})"
    );
    let metrics = existing_collision_metrics(existing_id, info, subtypes, mesh_lib);
    wall_decorator_intersects(
        candidate,
        existing_id,
        WallSpaceCoord::new_unchecked(
            f32::from_bits(info.dst_wall as u32),
            f32::from_bits((info.dst_wall >> 32) as u32),
        ),
        metrics,
        info.ty,
        excluded,
    )
}

/// 旧名保留（lib.rs 文档提到）；语义 = region 相交 + 吸附方向一致性。
/// U：旧签名丢失了原谓词的长度字段（O 0x141fe5310 用 along/vertical 半宽而
/// 非 region）；精确合并谓词走 [`wall_decorator_intersects`]。
#[allow(clippy::too_many_arguments)]
pub fn intersect_wall_decorators_predicate(
    a: WallSpaceRegion,
    b: WallSpaceRegion,
    derived_b: DerivedDecoratorInfo,
    snap_dir_a: i8,
    snap_dir_b: i8,
    space: &utils::wall_space::WallSpace,
) -> bool {
    let dims = derived_b.dims_wall_space();
    let expanded = WallSpaceRegion {
        center: b.center,
        half_extent: b.half_extent + Vec2::new(dims.x, dims.y) * 0.5,
    };
    if !a.intersects(expanded, space) {
        return false;
    }
    snap_dir_a == snap_dir_b || snap_dir_a == 0 || snap_dir_b == 0
}

/// `DecoratorWorldSpace::project_to_wall_space`（O：原 `CurveUBoundless::clamp`
/// 0x140c904e0；断言 t∈[0,1]）。输入为已构建的 utils Curve2（原始 source curve）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DecoratorWorldSpace {
    /// 墙空间 u 坐标（弧长，0..length）。
    pub u: f32,
    /// 墙空间 y（附着高）。
    pub y: f32,
}

impl DecoratorWorldSpace {
    pub fn project_to_wall_space(world: Vec3, curve: &SamplingCurve2) -> Self {
        let normalized = curve.get_approx_u_from_pos(Vec2::new(world.x, world.z));
        debug_assert!(
            (0.0..=1.0).contains(&normalized),
            "assertion failed: (0.0..=1.0).contains(&t)"
        );
        Self {
            u: normalized * curve.length,
            y: world.y,
        }
    }

    pub fn as_coord(self) -> WallSpaceCoord {
        WallSpaceCoord::new_unchecked(self.u, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn metrics(along: f32, vertical: f32) -> DecoratorCollisionMetrics {
        DecoratorCollisionMetrics {
            along_width: along,
            vertical_extent: vertical,
        }
    }

    #[test]
    fn side_formula_matches_machine_code() {
        // b=0/1 → 0；2→1；3→2；4→3；5→0。
        assert_eq!(wall_attachment_side(0), 0);
        assert_eq!(wall_attachment_side(1), 0);
        assert_eq!(wall_attachment_side(2), 1);
        assert_eq!(wall_attachment_side(3), 2);
        assert_eq!(wall_attachment_side(4), 3);
        assert_eq!(wall_attachment_side(5), 0);
    }

    #[test]
    fn normal_type_overlap_uses_point_six_margin() {
        let candidate = WallIntersectCandidate {
            coord: WallSpaceCoord::new_unchecked(1.0, 0.0),
            along_width: 1.0,
            vertical_extent: 1.0,
        };
        let existing = WallSpaceCoord::new_unchecked(1.0, 0.0);
        // dx=0, dy=0 → overlap；margin 0.6。
        assert!(wall_decorator_intersects(
            candidate,
            7,
            existing,
            metrics(1.0, 1.0),
            0x38,
            &[]
        ));
        // excluded id → false。
        assert!(!wall_decorator_intersects(
            candidate,
            7,
            existing,
            metrics(1.0, 1.0),
            0x38,
            &[7]
        ));
    }

    #[test]
    fn normal_type_rejects_before_point_three_vertical_check() {
        let candidate = WallIntersectCandidate {
            coord: WallSpaceCoord::new_unchecked(1.0, 0.0),
            along_width: 0.1,
            vertical_extent: 0.1,
        };
        // existing vertical extent 1.0；dy = 0.5 > 0.3 → early false。
        let existing = WallSpaceCoord::new_unchecked(1.0, 0.5);
        assert!(!wall_decorator_intersects(
            candidate,
            1,
            existing,
            metrics(0.1, 1.0),
            0x38,
            &[]
        ));
    }

    #[test]
    fn arrow_slit_uses_point_two_and_one_extra() {
        let candidate = WallIntersectCandidate {
            coord: WallSpaceCoord::new_unchecked(0.0, 0.0),
            along_width: 0.2,
            vertical_extent: 0.2,
        };
        let existing = WallSpaceCoord::new_unchecked(0.0, 0.9);
        // ty=0x3a: dy=0.9 < (1.0+1.0)*0.5 + 0.1 = 1.1 → vertical overlap；
        // dx=0 < 0.1 + (1.0+0.2)*0.5 = 0.7 → true。
        assert!(wall_decorator_intersects(
            candidate,
            1,
            existing,
            metrics(1.0, 1.0),
            0x3a,
            &[]
        ));
        // 非 0x3a：0.3*1.0 < 0.9 → early false。
        assert!(!wall_decorator_intersects(
            candidate,
            1,
            existing,
            metrics(1.0, 1.0),
            0x38,
            &[]
        ));
    }

    #[test]
    fn project_to_wall_space_uses_nearest_curve_u() {
        let curve =
            SamplingCurve2::try_new_from_points(vec![Vec2::new(0.0, 0.0), Vec2::new(4.0, 0.0)], false)
                .unwrap();
        let projected =
            DecoratorWorldSpace::project_to_wall_space(Vec3::new(1.0, 2.0, 0.0), &curve);
        assert!((projected.u - 1.0).abs() < 1e-5);
        assert_eq!(projected.y, 2.0);
        assert_eq!(
            projected.as_coord(),
            WallSpaceCoord::new_unchecked(1.0, 2.0)
        );
    }
}

#[cfg(test)]
mod canonical_metrics_regressions {
    use super::*;
    use std::sync::Arc;
    use country_core::resources::walls::decorator_subtype::{
        DecoratorCollision, DecoratorCollisionOutline, DecoratorOutline, DecoratorSubtype,
    };
    use country_core::systems::collision::world::RaycastMeshData;
    use country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::box_trigger_volume;
    use country_core::utils::components::transform::Transform;
    use utils::geometry::Rectangle3d;

    #[test]
    fn actual_metrics_borrow_exact_rank_logical_width_and_vertical_bounds() {
        let mut subtypes = DerivedDecoratorSubtypes::default();
        subtypes.insert(41, DecoratorSubtype(7));
        let mut library = DecoratorMeshLibrary::default();
        let key = DecoratorLibKey::new(7, 2);
        let size = Vec3::new(2.0, 4.0, 7.0);
        let center = Vec3::new(5.0, 13.0, -2.0);
        let transform = Transform { translation: center, ..Transform::identity() };
        let mesh = Arc::new(RaycastMeshData::new_cuboid_at(size, center));
        let probe = Arc::clone(&mesh);
        library.insert_collision_and_interaction_as_same(key, DecoratorCollision {
            triangles: box_trigger_volume(size.x, size.y, size.z, &transform),
            mesh,
            // 旋转footprint使AABB的X跨度与原逻辑width不同，底高也不为零。
            bounds: Rectangle3d::new(center, size, Vec2::Y),
        }, DecoratorOutline::None, DecoratorCollisionOutline::None);
        let mut info = DecoratorInfo::default(); info.rank = 2;
        let count = Arc::strong_count(&probe);
        assert_eq!(existing_collision_metrics(41, &info, &subtypes, &library),
            DecoratorCollisionMetrics { along_width: 2.0, vertical_extent: 4.0 });
        assert_eq!(Arc::strong_count(&probe), count);
        info.rank = 1;
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            existing_collision_metrics(41, &info, &subtypes, &library)
        })).is_err());
        info.rank = 2;
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            existing_collision_metrics(42, &info, &subtypes, &library)
        })).is_err());
    }
}
