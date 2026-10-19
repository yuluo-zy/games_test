//! B `crates/systems/decorator/src/cull_oob_decorators.rs` — 越界装饰剔除族
//! （D4 restorer tg_d4）。
//!
//! 证据（fresh RE-MCP，groupD/misc/decomp/，O 结构）：
//! - `is_decorator_location_valid` 0x14080f470：dst tag 判别（+0x22；
//!   tag∈{2,3,4} → 槽 1..3，其余 0）经 0x142a8eab0 跳表分派墙/楼梯/屋顶/地形
//!   有效性检查（墙 → `is_wall_decorator_location_valid` 行 707 内联）。
//! - `auto_allow_decorator_culling` 0x14080d3d0（行 48）：`History::is_recording`
//!   （0x140a86970）时把归档表全部装饰加入 cull 放行（`decorator_cull_allow`
//!   行 302 内联 → core `DecoratorCullPolicy.allowed_ids`）。
//! - `cull_oob_decorators` 0x14080d540（行 71）：逐归档装饰 → cull 放行检查
//!   （`is_decorator_cull_allowed` 0x140b2bd90）→ 位置（`CachedDecoratorTransforms::
//!   position` 0x140a1a9f0，未命中 panic）→ `is_decorator_location_valid` →
//!   无效 → 归档标签 "oob backup"（0x142a8e950，`archive_storage_decorators`
//!   0x1412b2a70）+ `apply_decorator_op`（0x141276e30，KIND_REMOVE）+
//!   `DecoratorBlueprints::remove`。
//! - `cull_stair_attached_decorators_impl` 0x1412b6c30/0x1412b6420（行 354，
//!   双实例）：楼梯装饰记录（0x28B）逐条：`assert!(is_decorator_cull_allowed(id))`
//!   （"assertion failed: decorator_archivist.is_decorator_cull_allowed(
//!   locator.decorator)" 0x142c2be40/0x142a8e8e3）+ 边存在性
//!   （`StairsState::contains_edge_or_stub` 0x140900e80；invalid_stairs 实例缺该
//!   检查）→ 移除（storage map 删除 0x141264820 + `apply_decorator_op` +
//!   蓝图删除）。
//! - `should_snap_to_corner` 0x140811900：`DecoratorType::should_offer_snap_to_corner`
//!   （t!=0x3f）∧ 距离阈值（`WallSpaceCoord::subtract`；`dist < threshold*0.5 +
//!   (halftimbered ? 3.0 : 0.46)`；0x1429cd068=3.0 / 0x1429a0764=0.46）。
//! - `check_if_dormer_ridge_is_within_roof_bounds` 0x140811440：dormer 变体
//!   ∈{7,8}（gothic 半 dormer / dormer 轮廓曲线 0x1409a4cc0 / 0x1407c7500）+
//!   屋顶查表 + 圆/矩形最近点判据（屋顶几何面 I，登记 RESTORE.md）。
//!
//! `DecoratorLocationCheckResult`（行 493 is_valid / 497 get_invalid_reason /
//! impl$1::clone 行 506 含 Rectangle2d）：{tag 1=Valid / 9 / 0xb=Invalid}。

use bevy_ecs::prelude::*;
use country_core::resources::history::History;
use country_core::resources::stairs::stairs_state::StairsState;
use country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms;
use country_core::resources::walls::decorator::{DecoratorId, DecoratorInfo, DecoratorType};
use country_core::resources::walls::decorator_archivist::DecoratorArchivist;
use country_core::resources::walls::decorator_storage::{DecoratorStorage, DecoratorStorageAddr};
use country_core::resources::walls::public_wall_state::PublicWallState;
use country_core::resources::walls::WallId;

use crate::apply_decorator_op::{apply_decorator_op, DecoratorOp};
use crate::blueprint::DecoratorBlueprints;

/// 越界检查结果（B：tag 1=Valid、9=无效（记录缺失/不适用）、0xb=Invalid；
/// `get_invalid_reason` 返回 Rectangle2d 载荷，行 497）。
#[derive(Clone, Debug, PartialEq)]
pub struct DecoratorLocationCheckResult {
    /// 有效（tag==1，O）。
    pub valid: bool,
    /// 位置（O：{tag@0, x@4(u64), z@0xc(f32), …@0x10, id@0x20}）。
    pub position: glam::Vec3,
    /// 无效载荷（O：Rectangle2d 克隆行 506）。
    pub reason_rect: Option<utils::geometry::rectangle::Rectangle2d>,
}

impl DecoratorLocationCheckResult {
    /// `is_valid`（行 493，内联，O）：tag==1。
    pub fn is_valid(&self) -> bool {
        self.valid
    }

    /// `get_invalid_reason`（行 497，内联，O）：无效矩形载荷。
    pub fn get_invalid_reason(&self) -> Option<utils::geometry::rectangle::Rectangle2d> {
        self.reason_rect
    }
}

/// `is_decorator_location_valid`（0x14080f470，O 结构）：dst tag 分派
/// （tag∈{2,3,4} → 槽 1..3）到各目的有效性检查。
pub fn is_decorator_location_valid(
    info: &DecoratorInfo,
    transforms: &CachedDecoratorTransforms,
) -> DecoratorLocationCheckResult {
    let id = info.decorator_id();
    // 位置（O：position 0x140a1a9f0 未命中 panic —— Rust 侧 Option 承载）。
    let Some(position) = transforms.try_position(id.0) else {
        return DecoratorLocationCheckResult {
            valid: false,
            position: glam::Vec3::ZERO,
            reason_rect: None,
        };
    };
    match info.dst_tag() {
        // 墙附着（tag 0/1；跳表 idx0 亦为 tag 5+ 的默认槽）：墙空间有效性
        // （行 707 内联；曲线采样面 I）。
        0..=1 => DecoratorLocationCheckResult {
            valid: is_wall_decorator_location_valid(info),
            position,
            reason_rect: None,
        },
        // 屋顶附着（tag 2；跳表 idx1 → 0x14080f66d）：屋顶查表 + 界判据。
        // 几何主体 U（登记 RESTORE.md）。
        2 => DecoratorLocationCheckResult {
            valid: true,
            position,
            reason_rect: None,
        },
        // 地形（tag 3，跳表 idx2）与楼梯（tag 4，跳表 idx3）**恒无效**
        // （O：fresh capstone 0x14080f542 = `*out = 0xb; return`）。
        3 | 4 => DecoratorLocationCheckResult {
            valid: false,
            position,
            reason_rect: None,
        },
        // tag 5+ 走跳表默认槽 idx0（墙处理器）。
        _ => DecoratorLocationCheckResult {
            valid: is_wall_decorator_location_valid(info),
            position,
            reason_rect: None,
        },
    }
}

/// `is_wall_decorator_location_valid`（行 707，内联，O 结构）：墙附着装饰
/// 有效性 = 墙存在 + 可放置位置（I：曲线采样/u/y 界检查 —— 本面以
/// dst/type 承载，登记 RESTORE.md）。
fn is_wall_decorator_location_valid(_info: &DecoratorInfo) -> bool {
    true
}

/// `auto_allow_decorator_culling`（0x14080d3d0，O 结构）：录制中自动放行
/// 全部归档装饰的剔除（`decorator_cull_allow` 行 302 内联 → allowed_ids）。
pub fn auto_allow_decorator_culling<C: Send + Sync + 'static>(
    history: Res<History>,
    mut archivist: ResMut<DecoratorArchivist<C>>,
) {
    if !history.is_recording() {
        return;
    }
    // 先收集再写（O：allowed_ids 集扩展；避免借用冲突）。
    let ids: Vec<u32> = archivist
        .walls
        .values()
        .flat_map(|d| d.items.keys().copied())
        .chain(archivist.special1.iter().flat_map(|d| d.items.keys().copied()))
        .chain(archivist.special2.iter().flat_map(|d| d.items.keys().copied()))
        .collect();
    archivist.culling.allowed_ids.extend(ids);
}

/// `cull_oob_decorators`（0x14080d540，O 结构）：越界装饰剔除主系统。
/// 逐归档装饰：cull 放行 ∧ 位置无效 → "oob backup" 归档 + remove 操作 +
/// 蓝图删除。
pub fn cull_oob_decorators<C: Send + Sync + 'static>(
    mut storage: ResMut<DecoratorStorage>,
    mut archivist: ResMut<DecoratorArchivist<C>>,
    transforms: Res<CachedDecoratorTransforms>,
    mut blueprints: ResMut<DecoratorBlueprints>,
    history: Res<History>,
) {
    let recording = history.is_recording();
    // 事件队列（I：原 DeferredDecoratorOpEvents 共享面由 D3 接线；本面以
    // 默认空队列承载，RESTORE.md 登记）。
    let events = country_core::resources::walls::decorator::DeferredDecoratorOpEvents::default();
    // 墙桶（O：归档墙表；clone 避免借用冲突 —— 原实现 0x30B 双表遍历）。
    let walls: Vec<(u64, Vec<u32>)> = archivist
        .walls
        .iter()
        .map(|(wall, d)| (wall.0, d.items.keys().copied().collect()))
        .collect();
    for (wall, ids) in walls {
        for id in ids {
            cull_one(
                id,
                &mut storage,
                &mut archivist,
                &transforms,
                &mut blueprints,
                recording,
                &events,
                WallId(wall),
            );
        }
    }
    // special 桶（I：O 双表 = 墙 + 楼梯；special 语义登记 RESTORE.md）。
    let specials: Vec<Vec<u32>> = [&archivist.special1, &archivist.special2]
        .into_iter()
        .flatten()
        .map(|d| d.items.keys().copied().collect())
        .collect();
    for ids in specials {
        for id in ids {
            cull_one(
                id,
                &mut storage,
                &mut archivist,
                &transforms,
                &mut blueprints,
                recording,
                &events,
                WallId(u64::MAX),
            );
        }
    }
}

/// 单装饰剔除（O 结构）。
fn cull_one<C: Send + Sync + 'static>(
    id: u32,
    storage: &mut DecoratorStorage,
    archivist: &mut DecoratorArchivist<C>,
    transforms: &CachedDecoratorTransforms,
    blueprints: &mut DecoratorBlueprints,
    _recording: bool,
    events: &country_core::resources::walls::decorator::DeferredDecoratorOpEvents,
    wall: WallId,
) {
    // cull 放行（O：is_decorator_cull_allowed 0x140b2bd90）。
    if !archivist.is_decorator_cull_allowed(id) {
        return;
    }
    // 存储记录（O：lookup_decorator_info 0x140a1c930）。
    let Some(info) = storage.lookup_decorator_info(DecoratorId::new(id)) else {
        return;
    };
    let result = is_decorator_location_valid(info, transforms);
    if result.is_valid() {
        return;
    }
    // 移除（O：KIND_REMOVE=4；"oob backup" 归档标签 0x142a8e950 ——
    // 归档面由 D3 archive_decorator_state 承载，本面经 apply 链登记）。
    blueprints.remove(id);
    let op = DecoratorOp {
        kind: DecoratorOp::KIND_REMOVE,
        info: *info,
        extra: [0; 0x20],
        dst: None,
    };
    let dst = storage
        .lookup_storage(DecoratorId::new(id))
        .unwrap_or(DecoratorStorageAddr::Wall(wall));
    apply_decorator_op(&op, 0, dst, storage, Some(archivist), None, events);
}

/// `cull_stair_attached_decorators_impl`（0x1412b6c30 / 0x1412b6420，O 结构）：
/// 楼梯附着装饰剔除实现（双实例 = 边失效 / 楼梯失效）。
pub fn cull_stair_attached_decorators_impl<C: Send + Sync + 'static>(
    storage: &mut DecoratorStorage,
    mut archivist: Option<&mut DecoratorArchivist<C>>,
    transforms: &CachedDecoratorTransforms,
    blueprints: &mut DecoratorBlueprints,
    stairs: &StairsState,
    events: &country_core::resources::walls::decorator::DeferredDecoratorOpEvents,
    check_edge: bool,
) {
    let mut remove_ids = Vec::new();
    for decorators in storage.iter_stair_decorators() {
        for (id, info) in decorators.decorators.iter() {
            // 断言（O：assertion failed: is_decorator_cull_allowed
            // (locator.decorator)，0x142c2be40）。
            assert!(
                archivist.as_ref().is_none_or(|a| a.is_decorator_cull_allowed(*id)),
                "assertion failed: decorator_archivist.is_decorator_cull_allowed(locator.decorator)"
            );
            let _ = transforms.try_position(*id);
            // 边检查（O：contains_edge_or_stub；invalid_stairs 实例缺该检查）。
            if check_edge {
                let Some(extra) = info.extra().try_as_stair() else {
                    continue;
                };
                let a = u32::from_le_bytes(extra.0[0..4].try_into().unwrap());
                let b = u32::from_le_bytes(extra.0[4..8].try_into().unwrap());
                if stairs.contains_edge_or_stub(a, b) {
                    continue;
                }
            }
            remove_ids.push(*id);
        }
    }
    for id in remove_ids {
        let Some(info) = storage.lookup_decorator_info(DecoratorId::new(id)).copied() else {
            continue;
        };
        blueprints.remove(id);
        let op = DecoratorOp {
            kind: DecoratorOp::KIND_REMOVE,
            info,
            extra: [0; 0x20],
            dst: None,
        };
        let dst = storage
            .lookup_storage(DecoratorId::new(id))
            .unwrap_or(DecoratorStorageAddr::Stair);
        // 逐迭代重借（Option<&mut> 非 Copy）。
        apply_decorator_op(
            &op,
            0,
            dst,
            storage,
            archivist.as_mut().map(|r| &mut **r),
            None,
            events,
        );
    }
}

/// `cull_stair_attached_decorators`（0x14080ec10，O）：楼梯边失效剔除。
pub fn cull_stair_attached_decorators<C: Send + Sync + 'static>(
    mut storage: ResMut<DecoratorStorage>,
    mut archivist: ResMut<DecoratorArchivist<C>>,
    transforms: Res<CachedDecoratorTransforms>,
    mut blueprints: ResMut<DecoratorBlueprints>,
    stairs: Res<StairsState>,
) {
    // 事件队列（I：D3 共享面；默认空队列承载，登记 RESTORE.md）。
    let events = country_core::resources::walls::decorator::DeferredDecoratorOpEvents::default();
    cull_stair_attached_decorators_impl(
        &mut storage,
        Some(&mut archivist),
        &transforms,
        &mut blueprints,
        &stairs,
        &events,
        true,
    );
}

/// `cull_stair_attached_decorators_on_invalid_stairs`（0x14080f040，O）：
/// 楼梯整体失效剔除（边检查关闭）。
pub fn cull_stair_attached_decorators_on_invalid_stairs<C: Send + Sync + 'static>(
    mut storage: ResMut<DecoratorStorage>,
    mut archivist: ResMut<DecoratorArchivist<C>>,
    transforms: Res<CachedDecoratorTransforms>,
    mut blueprints: ResMut<DecoratorBlueprints>,
    stairs: Res<StairsState>,
) {
    // 事件队列（I：D3 共享面；默认空队列承载，登记 RESTORE.md）。
    let events = country_core::resources::walls::decorator::DeferredDecoratorOpEvents::default();
    cull_stair_attached_decorators_impl(
        &mut storage,
        Some(&mut archivist),
        &transforms,
        &mut blueprints,
        &stairs,
        &events,
        false,
    );
}

/// `should_snap_to_corner`（0x140811900，O）：是否吸附到墙角。
/// `should_offer_snap_to_corner(type)`（t!=0x3f）∧ 距角距离阈值：
/// `dist < threshold * 0.5 + (halftimbered ? 3.0 : 0.46)`。
pub fn should_snap_to_corner(
    decorator_type: DecoratorType,
    halftimbered: bool,
    wall: &PublicWallState,
    a: utils::wall_space::WallSpaceCoord,
    b: utils::wall_space::WallSpaceCoord,
    threshold: f32,
) -> bool {
    if !decorator_type.should_offer_snap_to_corner() {
        return false;
    }
    let space = utils::wall_space::WallSpace {
        is_loop: false,
        length: wall.wall_space_length(),
    };
    let offset = if halftimbered { 3.0 } else { 0.46 };
    let dist = a.subtract(&space, b).length();
    dist < threshold * 0.5 + offset
}

/// `check_if_dormer_ridge_is_within_roof_bounds`（0x140811440，O 结构）：
/// dormer 脊线是否在屋顶界内（变体 ∈{7,8}；轮廓曲线 + 屋顶查表 +
/// 圆/矩形最近点距离判据 0x142a8e504=0.709）。
/// 屋顶几何面 I（本面以变体门控承载，登记 RESTORE.md）。
pub fn check_if_dormer_ridge_is_within_roof_bounds(
    variant: u8,
    _info: &DecoratorInfo,
) -> bool {
    // O：变体门控（7=gothic 半 dormer 轮廓 0x1409a4cc0，8=dormer 轮廓
    // 0x1407c7500；其他 → tag 0xb 无效）。
    if !(variant == 7 || variant == 8) {
        return false;
    }
    // 屋顶界判据（I：圆/矩形最近点 + 距离阈值；本面以屋顶存在性承载）。
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransform;
    use glam::{Quat, Vec3};

    fn transforms_with(id: u32) -> CachedDecoratorTransforms {
        let mut cache = CachedDecoratorTransforms::default();
        cache.insert(
            id,
            CachedDecoratorTransform {
                rotation: Quat::IDENTITY,
                translation: Vec3::new(1.0, 2.0, 3.0),
                scale: Vec3::ONE,
            },
        );
        // insert 写当前册；读取册是另一册 → 交换后才可查询。
        cache.swap_buffers_and_clear();
        cache
    }

    fn info_with_tag(id: u32, tag: u8) -> DecoratorInfo {
        DecoratorInfo {
            decorator_id: id,
            u20: u32::from(tag) << 16,
            ..DecoratorInfo::default()
        }
    }

    #[test]
    fn roof_tag_two_is_dispatched_to_roof_slot() {
        // O 跳表：idx1（tag 2）→ 0x14080f66d（屋顶处理器）。
        let cache = transforms_with(1);
        let info = info_with_tag(1, 2);
        let out = is_decorator_location_valid(&info, &cache);
        assert!(out.is_valid());
        assert_eq!(out.position, Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn terrain_and_stair_tags_are_invalid() {
        // O：idx2（tag 3）与 idx3（tag 4）同址 0x14080f542 → tag 0xb。
        let cache = transforms_with(2);
        let terrain = info_with_tag(2, 3);
        let stair = info_with_tag(2, 4);
        assert!(!is_decorator_location_valid(&terrain, &cache).is_valid());
        assert!(!is_decorator_location_valid(&stair, &cache).is_valid());
    }

    #[test]
    fn missing_transform_is_invalid() {
        let cache = CachedDecoratorTransforms::default();
        let info = info_with_tag(9, 0);
        assert!(!is_decorator_location_valid(&info, &cache).is_valid());
    }
}
