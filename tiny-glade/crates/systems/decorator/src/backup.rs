//! `backup.rs` — 装饰备份/恢复（DecoratorBackup 生命周期）。
//!
//! 机器码证据（fresh RE-MCP `groupD/decomp/decomp-0x140823660/0x1408238a0`）：
//!
//! `DecoratorBackup::clear` 0x140823660（O）：把备份重置为默认
//! （`FUN_140824680` = impl$3::default 0x140824680 → 逐字段 clone_from）。
//!
//! `create_or_restore_decorator_backup` 0x1408238a0（O 状态机，9 参）：备份状态
//! 字节 `+0x282`：0=无操作、1=CREATE、2=RESTORE（子标志 `+0x281`）；结束后
//! 若 `+0x280` 非零 → 重置备份为默认。
//! - CREATE：逐 archivist 存储地址 → 备份侧 `get_or_insert_default` +
//!   存储侧 `get`（未中 panic "assertion failed: ..." 0x142a91228，backup.rs）
//!   + `tracked_clone_from` 0x1412573b0（存储桶 → 备份 archivist 桶）；拷贝
//!   三个 0x30B 辅助状态（+0x1d0/+0x200/+0x230，对应 stairs/缓存类资源，
//!   U：具体类型待取证）、0x88B 事件 Vec（+0x260）、延迟记录 Vec（+0x00）；
//!   置 `+0x281 = 1`。
//! - RESTORE：反向拷贝；逐桶 `get_mut` + `clear_modification_tracking`
//!   0x141257900；modified 装饰 state@+0x47==1 → 0。
//!
//! 布局（O）：`{records Vec<0x20B> @+0x00, DecoratorStorage clone @+0x18,
//! A @+0x1d0, B @+0x200, C @+0x230 (各 0x30B), events Vec<[0x88B]> @+0x260,
//! flags @+0x280..0x282}`。
//!
//! core 对接：`StairsState::restore_backup`（create/restore 链）、
//! `DecoratorStorage` 桶 API。core 缺口登记：archivist
//! all_storages/all_decorators/extract_diff 未实现（本模块用
//! [`crate::restore_archived_decorators`] 的 archivist 迭代等价）。

use bevy_ecs::prelude::*;

use country_core::resources::stairs::stairs_state::StairsState;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::decorator_archivist::changes::DecoratorArchivist;
use country_core::resources::walls::decorator_archivist::changes::ObservedDecoratorInfo;
use country_core::resources::walls::decorator_archivist::changes::TrackedDecorators;

use crate::deferred_decorator_history::{DeferredDecoratorEditHistory, DeferredDecoratorRecord};

/// 装饰备份资源（布局 O；字段类型语义见文件头）。
#[derive(Resource, Clone, Debug, Default)]
pub struct DecoratorBackup {
    /// +0x00：延迟装饰历史记录（0x20B 元素）。
    pub records: Vec<DeferredDecoratorRecord>,
    /// +0x18：DecoratorStorage 备份（逐墙 TrackedDecorators 桶）。
    pub storage_backup: BackupStorage,
    /// +0x1d0/+0x200/+0x230：三个 0x30B 辅助状态（U：类型待取证；
    /// 与 StairsState 快照族对应）。
    pub auxiliary: [BackupAuxiliary; 3],
    /// +0x260：待发 0x88B 事件备份。
    pub events: Vec<[u8; 0x88]>,
    /// +0x280：重置备份标志（处理后清空备份）。
    pub reset_after: bool,
    /// +0x281：CREATE 完成标志（RESTORE 前置）。
    pub created: bool,
    /// +0x282：状态机字节（0/1=CREATE/2=RESTORE）。
    pub state: u8,
}

/// DecoratorStorage 备份（墙桶 + 地形/楼梯特殊桶；模型对应
/// `resources::walls::decorator_archivist::changes::DecoratorStorage`）。
#[derive(Clone, Debug, Default)]
pub struct BackupStorage {
    /// 分墙桶（TrackedDecorators = {removed, items, added}）。
    pub walls: hashbrown::HashMap<u64, TrackedDecorators>,
    /// 地形内联桶（原生 +0xc0 special1；None = 存储侧不存在）。
    pub terrain: Option<TrackedDecorators>,
    /// 楼梯内联桶（原生 +0x150 special2；None = 存储侧不存在）。
    pub stairs: Option<TrackedDecorators>,
}

/// 0x30B 辅助状态备份（U：类型待取证；机器码经 0x141263210/0x141262b00/
/// 0x141262fb0 三个 clone_from 实例拷贝，含 +0x20/+0x28 字段）。
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BackupAuxiliary {
    /// +0x00..+0x20：主体（U）。
    pub data: [u8; 0x20],
    /// +0x20/+0x28：尾部字段（U）。
    pub tail: [u64; 2],
}

impl DecoratorBackup {
    /// `clear` @0x140823660（O）：重置为默认。
    pub fn clear(&mut self) {
        *self = DecoratorBackup::default();
    }
}

/// `create_or_restore_decorator_backup` @0x1408238a0（O 状态机）。
///
/// ECS 参数面（候选 I：机器码 9 参，tick 回写面 8 个）：
/// `(ResMut<DecoratorBackup>, ResMut<DecoratorStorage>, ResMut<StairsState>
/// ×2, ResMut<辅助 A/B/C>, ResMut<events Vec>, ResMut<DeferredDecoratorEditHistory>)`。
/// 本实现按机器码状态机落地：CREATE 归档当前存储 + 辅助状态；RESTORE 反向
/// 恢复 + stairs restore_backup + clear_modification_tracking；reset 标志清备份。
pub fn create_or_restore_decorator_backup<C: Send + Sync + 'static>(
    mut backup: ResMut<DecoratorBackup>,
    mut storage: ResMut<DecoratorStorage>,
    mut stairs: ResMut<StairsState>,
    archivist: Res<DecoratorArchivist<C>>,
    mut deferred: ResMut<DeferredDecoratorEditHistory>,
) {
    let state = backup.state;
    backup.state = 0;
    match state {
        0 => {}
        1 => create_backup(&mut backup, &storage, &mut deferred),
        2 => {
            if backup.created {
                restore_backup(&mut backup, &mut storage, &mut stairs, &mut deferred, &archivist);
            }
        }
        _ => {}
    }
    if backup.reset_after {
        backup.clear();
    }
}

/// CREATE 分支（O 结构；archivist 逐地址归档 + 辅助状态拷贝）。
fn create_backup(
    backup: &mut DecoratorBackup,
    storage: &DecoratorStorage,
    deferred: &mut DeferredDecoratorEditHistory,
) {
    backup.storage_backup = BackupStorage {
        walls: storage
            .iter_walls()
            .map(|(wall, bucket)| (wall.0, tracked_from(bucket)))
            .collect(),
        terrain: storage.terrain.as_ref().map(tracked_from),
        stairs: storage.stairs.as_ref().map(tracked_from),
    };
    // 延迟记录/事件备份（O：0x20B 记录 3/4/5 开关拷贝 + 0x88B 事件 Vec）。
    backup.records.clone_from(&deferred.records);
    backup.created = true;
}

/// `Decorators` → `TrackedDecorators`（0x1412573b0 tracked_clone_from 的
/// 字段映射：removed_ids→removed、内嵌 map→items、modified_ids→added）。
fn tracked_from(bucket: &country_core::resources::walls::decorator_storage::Decorators) -> TrackedDecorators {
    TrackedDecorators {
        removed: bucket.removed_ids.clone(),
        items: bucket
            .decorators
            .iter()
            .map(|(id, info)| (*id, ObservedDecoratorInfo(crate::apply_decorator_op::info_to_bytes(*info))))
            .collect(),
        added: bucket.modified_ids.clone(),
    }
}

/// RESTORE 反向拷贝单个桶（O：clear_modification_tracking 后按备份三集合
/// 重置；modified 装饰 state@+0x47==1 且 history.pending 时 → 0）。
fn restore_bucket(
    bucket: &mut country_core::resources::walls::decorator_storage::Decorators,
    tracked: &TrackedDecorators,
    clear_modified: bool,
) {
    bucket.clear_modification_tracking();
    bucket.removed_ids.clone_from(&tracked.removed);
    bucket.modified_ids.clone_from(&tracked.added);
    for (id, info) in &tracked.items {
        bucket
            .decorators
            .insert(*id, crate::apply_decorator_op::bytes_to_info(&info.0));
    }
    if clear_modified {
        for id in &bucket.modified_ids {
            if let Some(info) = bucket.decorators.get_mut(id) {
                if info.state == 1 {
                    info.state = 0;
                }
            }
        }
    }
}

/// RESTORE 分支（O 结构；反向拷贝 + 状态清理）。
///
/// 2026-10-17 reviewer-state FIX-6：
/// - modified state@+0x47 清除**由 history+0x48（pending）门控**（二进制
///   `cVar2 = *[res+0x48]` 读于 RESTORE 入口，非无条件）；
/// - **辅助 0x30B 三 map（+0x1d0/+0x200/+0x230）RESTORE 反向拷贝缺失**——live
///   侧资源身份 U（core 未接线），标未实现登记。
fn restore_backup<C: Send + Sync + 'static>(
    backup: &mut DecoratorBackup,
    storage: &mut DecoratorStorage,
    stairs: &mut StairsState,
    deferred: &mut DeferredDecoratorEditHistory,
    archivist: &DecoratorArchivist<C>,
) {
    let _ = archivist;
    // O：二进制 RESTORE 入口读 history+0x48 pending 作门控（cVar2），用于
    // 下方 modified state 清除。
    let clear_modified = deferred.pending;
    // 机器码：逐 archivist 存储地址 get_mut + clear_modification_tracking，
    // 随后存储桶 tracked_clone_from ← 备份桶；modified state==1 且门控 → 0。
    for (wall, tracked) in &backup.storage_backup.walls {
        let wall_id = country_core::resources::walls::WallId(*wall);
        if let Some(bucket) = storage.wall_decorators_mut(wall_id) {
            restore_bucket(bucket, tracked, clear_modified);
        }
    }
    if let Some(tracked) = &backup.storage_backup.terrain {
        if let Some(bucket) = storage.terrain.as_mut() {
            restore_bucket(bucket, tracked, clear_modified);
        }
    }
    if let Some(tracked) = &backup.storage_backup.stairs {
        if let Some(bucket) = storage.stairs.as_mut() {
            restore_bucket(bucket, tracked, clear_modified);
        }
    }
    stairs.restore_backup();
    deferred.records.clone_from(&backup.records);
}

/// 备份族 ECS 辅助（机器码 create/restore 链中 StairsState 快照族；登记用）。
#[allow(dead_code)]
fn _stairs_backup_surface(_stairs: &StairsState, _wall: country_core::resources::walls::WallId) {}

const LOG_TARGET: &str = "system_decorator::backup";

#[cfg(test)]
mod tests {
    use super::*;
    use country_core::resources::walls::decorator::DecoratorInfo;
    use country_core::resources::walls::decorator_storage::Decorators;

    fn bucket_with(id: u32, state: u8) -> Decorators {
        let mut bucket = Decorators::default();
        bucket.decorators.insert(
            id,
            DecoratorInfo {
                decorator_id: id,
                state,
                ..DecoratorInfo::default()
            },
        );
        bucket.removed_ids.insert(id + 100);
        bucket.modified_ids.insert(id);
        bucket
    }

    #[test]
    fn tracked_roundtrip_restores_decorators_and_tracking_sets() {
        let source = bucket_with(5, 2);
        let tracked = tracked_from(&source);
        assert_eq!(tracked.removed, source.removed_ids);
        assert_eq!(tracked.added, source.modified_ids);
        assert!(tracked.items.contains_key(&5));

        let mut target = Decorators::default();
        restore_bucket(&mut target, &tracked, false);
        assert!(target.decorators.contains_key(&5));
        assert_eq!(target.decorators[&5].state, 2);
        assert!(target.removed_ids.contains(&105));
        assert!(target.modified_ids.contains(&5));
    }

    #[test]
    fn restore_clears_live_state_only_when_pending() {
        let source = bucket_with(5, 1);
        let tracked = tracked_from(&source);

        let mut gated = Decorators::default();
        restore_bucket(&mut gated, &tracked, true);
        assert_eq!(gated.decorators[&5].state, 0, "state 1 cleared when pending");

        let mut ungated = Decorators::default();
        restore_bucket(&mut ungated, &tracked, false);
        assert_eq!(ungated.decorators[&5].state, 1, "state kept without pending");
    }
}
