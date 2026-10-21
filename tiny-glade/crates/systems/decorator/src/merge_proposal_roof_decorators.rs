//! `merge_proposal_roof_decorators.rs` — 屋顶合并装饰位置提议
//! （find_best_roof_decorator_merge_position 0x142000e60 +
//! intersect_roof_decorators 泛型实例 0x141fe7280）。
//!
//! 证据：
//! - `toolchain/decor-trunk/decompiled/142000e60_find_best_roof_decorator_merge_position.c`
//!   （O 主体：候选逐 id 查屋顶装饰桶 → 同族/rank → 碰撞宽度差 →
//!   角/周长归一 → `base ± delta` → 距离 SQRT → min_by）。
//! - `groupD/decomp/decomp-0x141fe7280-intersect_roof_decorators.txt`
//!   （O 结构：四元数旋转 → `Rectangle3d` 构造 → 逐屋顶装饰谓词；
//!   谓词体 `FUN_141fe6f20` U）。
//!
//! 已落地（O）：
//! - 同族检查（`min(ty-0x38,6)`；族 6 要求 ty 全等）；
//! - rank 和 <= `DecoratorType::max_rank()`；
//! - `delta = (new_along_width - existing_along_width) * 0.5`；
//! - 非角：`delta /= shape.perimeter()`；角：`delta = delta /
//!   rectangle.edge_length(edge_id) * 0.25`；
//! - `proposed = base ∓ delta`（base 在 target 左侧时减，右侧时加）；
//! - 距离 `sqrt((proposed.u - target.u)^2 + (base.y - target.y)^2)`，取最近；
//! - 无有效候选 → None（原 `+0x5f = 2`）。
//!
//! U（登记，不伪造）：
//! - 反编译中 `local_2b8 + 8` 的角标志与 `local_278` 的 Rectangle2d 来源字段
//!   语义按 contract §2.4 #48 解释；
//! - 重叠检查在 0x142000e60 内不可见（谓词闭包 U）；本实现不做重叠过滤，
//!   由调用方的 `intersect_roof_decorators` 承担（见下）；
//! - `intersect_roof_decorators` 的谓词体 U：本实现以 Rectangle3d 相交表达
//!   可复现结构，并以 `RoofDecoratorShape` 明确输入。
//!
//! “合并执行”统一走 `crate::try_merge_decorators`（lib.rs 行 77 原符号）；
//! 旧本地重复函数已删除（CONTRACT §5.1 #5）。

use country_core::resources::walls::decorator::{DecoratorId, DecoratorInfo};
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::decorator_subtype::{
    DecoratorMeshLibrary, DerivedDecoratorSubtypes,
};
use country_core::resources::walls::WallId;
use country_core::systems::wall::valid_enclosure::ShapeParameters;
use utils::geometry::rectangle::{Rectangle2d, Rectangle3d};
use utils::wall_space::WallSpaceCoord;

/// 屋顶合并位置提议（I：原 0x34B 提案的消费子集）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RoofMergePosition {
    /// 提议装饰 id（原候选 id）。
    pub decorator_id: u32,
    /// 被合并目标 id。
    pub target_id: u32,
    /// 提议屋顶墙空间坐标（原 `+0xd..+0x1a` 拷贝的目标记录坐标）。
    pub coord: WallSpaceCoord,
    /// 合并后 rank（原 +? rank 和）。
    pub rank_sum: u8,
    /// 族（原 `+0x5d` 低字节）。
    pub family: u8,
    /// 类型（原 `+0x5d` 高字节）。
    pub ty: u8,
    /// 原 `+0x5f`：2 = blocked；其它 = 新装饰 state。
    pub status: u8,
    /// 最近距离（SQRT）。
    pub distance: f32,
    /// 角分支使用的边 id（非角 = None）。
    pub edge_id: Option<usize>,
}

impl RoofMergePosition {
    #[inline]
    pub fn is_blocked(&self) -> bool {
        self.status == 2
    }
}

/// 族（O：`(ty-0x38)` 夹取 6）。
#[inline]
pub fn decorator_family(ty: u8) -> u8 {
    ty.wrapping_sub(0x38).min(6)
}

#[inline]
fn families_compatible(new_ty: u8, existing_ty: u8) -> bool {
    let new_family = decorator_family(new_ty);
    let existing_family = decorator_family(existing_ty);
    (new_family != 6 || existing_ty == new_ty) && existing_family == new_family
}

/// 屋顶形状输入（O：`get_roof_shape_from_pos_ws_clamped` + ShapeParameters/
/// Rectangle2d）。调用方从实际屋顶形状构造。
pub struct RoofMergeShape<'a> {
    pub shape: &'a ShapeParameters,
    pub rectangle: Rectangle2d,
    /// 原 `local_2b8 + 8 & 1`：角分支。
    pub corner_attached: bool,
    /// `get_roof_shape_from_pos_ws_clamped` 输出的屋顶局部位置（角分支
    /// `Rectangle2d::get_closest_edge_id` 的实参）。
    pub closest_edge_point: glam::Vec2,
}

/// `find_best_roof_decorator_merge_position` @0x142000e60（O 主体）。
///
/// `candidates` = 同一屋顶桶上的既有装饰 id（原 `+0x38` u32 数组）；
/// `new_along_width` = 新装饰碰撞记录的沿墙宽度（原 mesh 查表 +0x30）；
/// `new_coord` = 新装饰目标坐标（原 `dst_as_roof` 读面）。
#[allow(clippy::too_many_arguments)]
pub fn find_best_roof_decorator_merge_position(
    info: &DecoratorInfo,
    storage: &DecoratorStorage,
    wall: WallId,
    candidates: &[u32],
    new_along_width: f32,
    new_coord: WallSpaceCoord,
    mesh_lib: &DecoratorMeshLibrary,
    derived_subtypes: &DerivedDecoratorSubtypes,
    shape: &RoofMergeShape<'_>,
) -> Option<RoofMergePosition> {
    let bucket = storage.wall_decorators(wall)?;
    let mut best: Option<RoofMergePosition> = None;
    for &target_id in candidates {
        let Some(existing) = bucket.get(DecoratorId(target_id)) else {
            // O：原 map 查找；缺失候选跳过（原闭包返回 None）。
            continue;
        };
        if !families_compatible(info.ty, existing.ty) {
            continue;
        }
        let rank_sum = existing.rank.wrapping_add(info.rank);
        if rank_sum > info.decorator_type().max_rank() {
            continue;
        }
        // O：既有记录碰撞宽度（+0x30）。subtype 来自 DerivedDecoratorSubtypes。
        let derived = derived_subtypes.get(target_id);
        let key = country_core::resources::walls::decorator_subtype::DecoratorLibKey::new(
            derived.0,
            existing.rank,
        );
        let existing_collision = mesh_lib.collision_record(key);
        // 原记录+0x30是逻辑宽度，不是旋转后AABB的X投影。
        let existing_along = existing_collision.bounds.footprint.size.x;
        let mut delta = (new_along_width - existing_along) * 0.5;
        // O：非角 / 角归一。
        let edge_id = if shape.corner_attached {
            let edge = shape.rectangle.get_closest_edge_id(shape.closest_edge_point);
            delta = (delta / shape.rectangle.edge_length(edge)) * 0.25;
            Some(edge)
        } else {
            delta /= shape.shape.perimeter();
            None
        };
        // O：`dst_as_roof` 坐标读面（info+0x00/+0x04）。
        let base = WallSpaceCoord::new_unchecked(
            f32::from_bits(existing.dst_wall as u32),
            f32::from_bits((existing.dst_wall >> 32) as u32),
        );
        // O：`base ± delta`：base 在 target 左侧 → 减，右侧 → 加。
        let proposed_u = if base.u < new_coord.u {
            base.u - delta
        } else {
            base.u + delta
        };
        let proposed = WallSpaceCoord::new_unchecked(proposed_u, base.y);
        let distance = ((proposed.u - new_coord.u).powi(2)
            + (base.y - new_coord.y).powi(2))
        .sqrt();
        let proposal = RoofMergePosition {
            decorator_id: info.decorator_id,
            target_id,
            coord: proposed,
            rank_sum,
            family: decorator_family(info.ty),
            ty: info.ty,
            status: info.state,
            distance,
            edge_id,
        };
        // O：NAN unwrap + 严格更小者胜。
        best = match best {
            None => Some(proposal),
            Some(current) => {
                if proposal.distance.is_nan() || current.distance.is_nan() {
                    panic!("attempted to compare NaN roof merge distance");
                }
                if proposal.distance < current.distance {
                    Some(proposal)
                } else {
                    Some(current)
                }
            }
        };
    }
    best
}

/// 屋顶装饰相交（O 结构：Quat 旋转 + Rectangle3d；谓词体 U）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RoofDecoratorShape {
    pub id: u32,
    pub rect: Rectangle3d,
}

/// 泛型 `intersect_roof_decorators` 的语义等价面（O：两 Rectangle3d 相交）。
/// U：原谓词 `FUN_141fe6f20` 未反编译；本实现用 Rectangle3d 重叠表达可复现
/// 结构，不声称逐字节等价。
pub fn intersect_roof_decorators(
    a: RoofDecoratorShape,
    b: RoofDecoratorShape,
) -> Option<(u32, u32)> {
    if a.rect.overlaps_rectangle3d(b.rect) {
        Some((a.id, b.id))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::Vec2;

    #[test]
    fn family_compat_rules() {
        assert_eq!(decorator_family(0x38), 0);
        assert_eq!(decorator_family(0x3e), 6);
        assert!(families_compatible(0x38, 0x38));
        assert!(!families_compatible(0x38, 0x39));
        assert!(families_compatible(0x3e, 0x3e));
        assert!(!families_compatible(0x3e, 0x3f));
    }

    #[test]
    fn rectangle_overlap_intersects() {
        let a = RoofDecoratorShape {
            id: 1,
            rect: Rectangle3d::new(glam::Vec3::ZERO, glam::Vec3::new(2.0, 1.0, 2.0), Vec2::X),
        };
        let b = RoofDecoratorShape {
            id: 2,
            rect: Rectangle3d::new(
                glam::Vec3::new(1.0, 0.0, 0.0),
                glam::Vec3::new(2.0, 1.0, 2.0),
                Vec2::X,
            ),
        };
        assert_eq!(intersect_roof_decorators(a, b), Some((1, 2)));
        let far = RoofDecoratorShape {
            id: 3,
            rect: Rectangle3d::new(
                glam::Vec3::new(10.0, 0.0, 0.0),
                glam::Vec3::new(1.0, 1.0, 1.0),
                Vec2::X,
            ),
        };
        assert_eq!(intersect_roof_decorators(a, far), None);
    }

    #[test]
    fn delta_direction_matches_edge_contact() {
        // base 在 target 左侧、delta=0.5 → proposed = base - 0.5（右边缘保持）。
        let base = WallSpaceCoord::new_unchecked(1.0, 2.0);
        let target = WallSpaceCoord::new_unchecked(3.0, 2.0);
        let delta = 0.5;
        let proposed = if base.u < target.u {
            base.u - delta
        } else {
            base.u + delta
        };
        assert_eq!(proposed, 0.5);
    }
}
