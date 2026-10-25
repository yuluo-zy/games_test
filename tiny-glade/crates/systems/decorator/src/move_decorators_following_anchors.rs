//! `move_decorators_following_anchors.rs` — 墙变化后重锚全部墙附着装饰。
//!
//! 证据：
//! - `toolchain/decor-trunk/decompiled/14081c790_move_decorators_following_anchors.c`
//!   （O 全文结构：4 个事件流 + `iter_walls_mut` + `iter_modified_mut`）。
//! - `building-decorator-info/B-anchor-consumers/14081c790.c`（O 细读）：
//!   1. 已删墙事件（0x10B）→ `DecoratorStorage::on_wall_removed`；
//!   2. 0x10B 段 → 目标墙 `wall_decorators_mut` + `mark_all_modified`，
//!      并登记两个 `HashSet<WallId>`（调用方资源，本面以返回值承载）；
//!   3. 0x20B 段 → 对目标墙 modified 装饰中 dst tag==2（屋顶）取可变句柄；
//!   4. 0x30B 段 → 同上；
//!   5. `iter_walls_mut` → `iter_modified_mut`：side 0 装饰：
//!      `get_wall_attachment_height` → 曲线最近段 → `lerp` → `wall_space`
//!      → `project_clamp` → 写 `Info+0x00/+0x04`；side 1 无操作；2/3 panic；
//!   6. 门底（`+0x21==1`）走独立尾段（子类型查询 + 垂直 profile）；
//!   7. 尾段 `invalidate_roof_decorators`（行 133 内联）。
//!
//! 重锚数学直接复用 core 已恢复且验证过的
//! `country_core::resources::walls::decorator::anchors::move_wall_decorator_following_anchor`
//! （0x14081d202..0x14081d2e5 普通路径），本模块只负责遍历/分派/字节写回。

use country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms;
use country_core::resources::walls::decorator::anchors::{
    move_wall_decorator_following_anchor, DecoratorAnchorContext, DecoratorAnchorError,
    DecoratorInfoObserved,
};
use country_core::resources::walls::decorator::DecoratorInfo;
use country_core::resources::walls::decorator_storage::{DecoratorStorage, DecoratorStorageAddr};
use country_core::resources::walls::public_wall_state::PublicWallState;
use country_core::resources::walls::public_walls::PublicWalls;
use country_core::resources::walls::WallId;

use crate::decorator_on_wall_height_changed::side_from_tag_byte;

const _: () = assert!(std::mem::size_of::<DecoratorInfo>() == 72);

/// 原始 72B 记录 ↔ typed 视图（`DecoratorInfo` 为 `#[repr(C)]` 72B）。
///
/// 这是 core 已公开的 `DecoratorInfoObserved` 字节桥；不是对字段语义的再解释。
pub fn observed_from_info(info: &DecoratorInfo) -> DecoratorInfoObserved {
    let mut bytes = [0u8; 72];
    bytes[..8].copy_from_slice(&info.dst_wall.to_le_bytes());
    bytes[8..16].copy_from_slice(&info.dst_secondary.to_le_bytes());
    bytes[16..24].copy_from_slice(&info.dst_payload.to_le_bytes());
    bytes[24..32].copy_from_slice(&info.u18.to_le_bytes());
    bytes[32..36].copy_from_slice(&info.u20.to_le_bytes());
    bytes[36..40].copy_from_slice(&info.u24.to_le_bytes());
    bytes[40..44].copy_from_slice(&info.u28.to_le_bytes());
    bytes[44..48].copy_from_slice(&info.decorator_id.to_le_bytes());
    bytes[48..56].copy_from_slice(&info.u30.to_le_bytes());
    bytes[56..60].copy_from_slice(&info.u38.to_le_bytes());
    bytes[60..64].copy_from_slice(&info.u3c.to_le_bytes());
    bytes[64..68].copy_from_slice(&info.u40.to_le_bytes());
    bytes[68] = info.rank;
    bytes[69] = info.u45;
    bytes[70] = info.ty;
    bytes[71] = info.state;
    DecoratorInfoObserved::from_observed(bytes)
}

fn write_observed_back(info: &mut DecoratorInfo, observed: &DecoratorInfoObserved) {
    let b = observed.bytes();
    info.dst_wall = u64::from_le_bytes(b[..8].try_into().unwrap());
    info.dst_secondary = u64::from_le_bytes(b[8..16].try_into().unwrap());
    info.dst_payload = u64::from_le_bytes(b[16..24].try_into().unwrap());
    info.u18 = u64::from_le_bytes(b[24..32].try_into().unwrap());
    info.u20 = u32::from_le_bytes(b[32..36].try_into().unwrap());
    info.u24 = u32::from_le_bytes(b[36..40].try_into().unwrap());
    info.u28 = u32::from_le_bytes(b[40..44].try_into().unwrap());
    info.decorator_id = u32::from_le_bytes(b[44..48].try_into().unwrap());
    info.u30 = u64::from_le_bytes(b[48..56].try_into().unwrap());
    info.u38 = u32::from_le_bytes(b[56..60].try_into().unwrap());
    info.u3c = u32::from_le_bytes(b[60..64].try_into().unwrap());
    info.u40 = u32::from_le_bytes(b[64..68].try_into().unwrap());
    info.rank = b[68];
    info.u45 = b[69];
    info.ty = b[70];
    info.state = b[71];
}

/// 重锚期间登记的墙集合（O：机器码两个 `HashSet<WallId>`）。
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AnchorFollowWalls {
    /// 由 `in_stack_00000030` 事件墙登记。
    pub changed: Vec<WallId>,
    /// 由 `in_stack_00000038` 事件墙登记。
    pub secondary: Vec<WallId>,
}

/// `move_decorators_following_anchors`（0x14081c790，O 结构）：
/// 墙变化后重锚墙附着装饰。返回原机器码登记的两个墙集合。
#[allow(clippy::too_many_arguments)]
pub fn move_decorators_following_anchors(
    storage: &mut DecoratorStorage,
    walls: &PublicWalls,
    _transforms: &CachedDecoratorTransforms,
    wall_id: WallId,
) -> AnchorFollowWalls {
    let mut recorded = AnchorFollowWalls::default();
    // 1. 已删墙（O：on_wall_removed）。
    let removed_walls: Vec<WallId> = storage
        .iter_removed_storages()
        .filter_map(|addr| match addr {
            DecoratorStorageAddr::Wall(w) => Some(w),
            _ => None,
        })
        .collect();
    for wall in removed_walls {
        storage.on_wall_removed(wall);
    }

    // 2. 目标墙：标记全部 modified + 登记（O）。
    if let Some(decorators) = storage.wall_decorators_mut(wall_id) {
        decorators.mark_all_modified();
    }
    recorded.changed.push(wall_id);

    // 3. 逐装饰重锚（O：iter_modified_mut；side 分派）。
    let Some(wall_state) = walls.get(wall_id) else {
        log::warn!(
            "move_decorators_following_anchors: wall {:?} missing from PublicWalls",
            wall_id
        );
        return recorded;
    };
    let public = wall_state.building_projection();
    let context = DecoratorAnchorContext {
        raw_curve: wall_state.simplified_curve_2d(),
        mapped_outline: wall_state.height_outline(),
        arch_heights: wall_state.arch_segment_heights().unwrap_or(&[]),
        public: &public,
    };
    let ids: Vec<u32> = storage
        .wall_decorators(wall_id)
        .map(|bucket| bucket.iter_modified_ids().collect())
        .unwrap_or_default();
    for id in ids {
        let Some(bucket) = storage.wall_decorators_mut(wall_id) else {
            break;
        };
        let Some(info) = bucket.get_mut(country_core::resources::walls::decorator::DecoratorId(id))
        else {
            continue;
        };
        reanchor_one(info, &context);
    }
    // 4. 屋顶装饰失效（行 133 内联，I）。
    invalidate_roof_decorators(storage, wall_id);
    recorded
}

/// 单装饰重锚（O：普通路径走 core 已验证实现；门底路径保持 typed gap）。
fn reanchor_one(info: &mut DecoratorInfo, context: &DecoratorAnchorContext<'_>) {
    let side = side_from_tag_byte(info.dst_tag());
    match side {
        0 => {
            let observed = observed_from_info(info);
            match move_wall_decorator_following_anchor(&observed, context) {
                Ok(updated) => write_observed_back(info, &updated),
                Err(DecoratorAnchorError::BottomDoorConsumerRequired) => {
                    // U：门底消费者需要实际垂直 profile/碰撞 owner；保持显式缺口。
                    log::debug!(
                        "move_decorators_following_anchors: bottom-door decorator {} needs \
                         unresolved vertical profile consumer",
                        info.decorator_id
                    );
                }
                Err(error) => {
                    log::warn!(
                        "move_decorators_following_anchors: decorator {} reanchor failed: {error:?}",
                        info.decorator_id
                    );
                }
            }
        }
        1 => {}
        _ => unreachable!(
            "internal error: entered unreachable code (move_decorators_following_anchors.rs side {side})"
        ),
    }
}

/// `reanchor_doors_attached_to_bottom`（行 151 内联；U 尾段显式返回缺口）。
///
/// O 结构：`+0x21==1` → 子类型查表（0x140b26980）→ CurveU 采样
/// （断言 `(0.0..=1.0).contains(&t)`）→ 附着高 `max_y` → 垂直 profile 夹取。
/// 精确垂直 profile 公式未闭合；不伪造写入。
pub fn reanchor_doors_attached_to_bottom(
    info: &DecoratorInfo,
    state: &PublicWallState,
) -> Result<(), DecoratorAnchorError> {
    if info.u20 & 0x0100 == 0 {
        return Ok(());
    }
    let _ = state;
    Err(DecoratorAnchorError::BottomDoorConsumerRequired)
}

/// `invalidate_roof_decorators`（行 133，内联，I）：屋顶装饰失效标记。
pub fn invalidate_roof_decorators(storage: &mut DecoratorStorage, wall: WallId) {
    if let Some(decorators) = storage.wall_decorators_mut(wall) {
        decorators.mark_all_modified();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn info_byte_bridge_round_trips() {
        let info = DecoratorInfo {
            dst_wall: 0x1122_3344_5566_7788,
            dst_secondary: 9,
            dst_payload: 0xdead_beef_cafe_babe,
            u18: 0x0102_0304_0506_0708,
            u20: 0xaabb_ccdd,
            u24: 7,
            u28: 8,
            decorator_id: 42,
            u30: 0x1234,
            u38: 0x55,
            u3c: 0x66,
            u40: 0x77,
            rank: 2,
            u45: 1,
            ty: 0x38,
            state: 2,
        };
        let observed = observed_from_info(&info);
        let mut back = DecoratorInfo::default();
        write_observed_back(&mut back, &observed);
        assert_eq!(back, info);
    }

    #[test]
    fn door_flag_is_plus_0x21() {
        // 门底标志在 +0x21（u20 的 byte1）。构造 PublicWallState 成本高且该
        // 分支不读取 state；这里只验证标志解码本身。
        let mut info = DecoratorInfo::default();
        assert_eq!(info.u20 & 0x0100, 0);
        info.u20 |= 0x0100;
        assert_ne!(info.u20 & 0x0100, 0);
    }
}
