//! `restore_archived_decorators.rs` — 归档装饰恢复（0x140817a40，O 结构）。
//!
//! 机器码证据（fresh RE-MCP `groupD/decomp/decomp-0x140817a40-*.txt`，
//! 47KB）：
//! 1. `mark_mutable(storage)`（0x140a1c2b0）；
//! 2. 历史游标守卫（EventReader/ReplayBuffer 双游标 + 状态字节 `(9,1)`
//!    快速路径——U：状态语义）；
//! 3. `DecoratorArchivist::all_storages`（0x140b2bb80）→ 逐地址 `get_mut` +
//!    `clear_modification_tracking`（0x141257900）；
//! 4. `all_decorators`（0x140b2bb00）→ 逐装饰：地址桶缺失 → archivist 侧
//!    map 查（0x14085a560，U 身份）；`get_or_insert_default` +
//!    `DecoMoveToStorage::new`（0x140a1b430）+ 内嵌 map 逐条 insert（哈希
//!    XOR 掩码 _UNK_1429cd080/_UNK_1429cd090 = core SNAPSHOT_ID_MASK）+ 状态
//!    字节==2 → `d2s insert`（0x140a1b4e0）否则 `d2s update`（0x140a1b5a0）；
//! 5. `decorator_colliders`（0x140005c60）/`decorator_interactions`
//!    （0x1403c03c0）→ Arc 克隆 + 落位（FUN_141298110，U 身份）；
//! 6. 游标条件 → `extract_diff`（0x140b2cb30）+ 0x70B 记录冲刷
//!    （FUN_140b743e0）。
//!
//! core 对接：`restore_archived_decorators`（shared building modules，等价
//! extract_and_restore_to_storage 0x140b2c4b0）+ `DecoratorStorage` 桶 API +
//! `StairsState::restore_backup` + `InnerWalls::get`（机器码调用面）。
//! core 缺口登记：archivist `all_storages/all_decorators/extract_diff` 未在
//! core 实现（章节 O）——本模块以核心模型字段提供等价迭代
//! （[`archivist_all_storages`]/[`archivist_all_decorators`]），由集成 owner
//! 裁决是否上收 core。

use bevy_ecs::prelude::*;

use country_core::resources::stairs::stairs_state::StairsState;
use country_core::resources::walls::decorator_archivist::DecoratorArchivist;
use country_core::resources::walls::decorator_storage::{
    DecoratorStorage, DecoratorStorageAddr,
};
use country_core::resources::walls::decorator::DecoratorId;
use country_core::resources::walls::inner_walls::InnerWalls;
use country_core::resources::walls::WallId;
use country_core::resources::walls::decorator_archivist::changes::DecoratorStorageAddress;

/// archivist 全存储地址迭代（等价 `all_storages` 0x140b2bb80；core 模型面）。
pub fn archivist_all_storages<C: Send + Sync + 'static>(
    archivist: &DecoratorArchivist<C>,
) -> Vec<DecoratorStorageAddress> {
    let mut out: Vec<DecoratorStorageAddress> = archivist
        .walls
        .keys()
        .map(|w| DecoratorStorageAddress::Wall(*w))
        .collect();
    if archivist.special1.is_some() {
        out.push(DecoratorStorageAddress::ObservedSpecial1);
    }
    if archivist.special2.is_some() {
        out.push(DecoratorStorageAddress::ObservedSpecial2);
    }
    out
}

/// archivist 全装饰迭代（等价 `all_decorators` 0x140b2bb00；core 模型面）。
pub fn archivist_all_decorators<C: Send + Sync + 'static>(
    archivist: &DecoratorArchivist<C>,
) -> Vec<(DecoratorStorageAddress, u32)> {
    let mut out = Vec::new();
    for (wall, tracked) in &archivist.walls {
        for id in tracked.items.keys() {
            out.push((DecoratorStorageAddress::Wall(*wall), *id));
        }
    }
    if let Some(t) = &archivist.special1 {
        for id in t.items.keys() {
            out.push((DecoratorStorageAddress::ObservedSpecial1, *id));
        }
    }
    if let Some(t) = &archivist.special2 {
        for id in t.items.keys() {
            out.push((DecoratorStorageAddress::ObservedSpecial2, *id));
        }
    }
    out
}

/// 地址 → 存储桶（core 模型面）。
pub(crate) fn tracked_at<C: Send + Sync + 'static>(
    archivist: &DecoratorArchivist<C>,
    address: DecoratorStorageAddress,
) -> Option<&country_core::resources::walls::decorator_archivist::changes::TrackedDecorators> {
    match address {
        DecoratorStorageAddress::Wall(w) => archivist.walls.get(&w),
        DecoratorStorageAddress::ObservedSpecial1 => archivist.special1.as_ref(),
        DecoratorStorageAddress::ObservedSpecial2 => archivist.special2.as_ref(),
    }
}

/// `restore_archived_decorators` @0x140817a40（O 结构；core 模型面）。
///
/// 机器码 ECS 面（候选 I）：`(History 状态, EventReader×2, ResMut<Decorator
/// Archivist>, ResMut<DecoratorStorage>, ResMut<StairsState>, ResMut<辅助>,
/// ResMut<回放/游标资源>)`——本实现按核心模型落地可接线部分：
/// 1. `mark_mutable`；2. 逐存储清修改追踪；3. 逐装饰落位（get_or_insert
/// default + d2s 索引 + 状态字节 2 → insert）；4. 楼梯 restore_backup；
/// 5. 尾部 core `restore_archived_decorators` 原子落位 + archivist 清空。
pub fn restore_archived_decorators<C: Send + Sync + 'static>(
    mut storage: ResMut<DecoratorStorage>,
    archivist: ResMut<DecoratorArchivist<C>>,
    mut stairs: ResMut<StairsState>,
    inner_walls: Res<InnerWalls>,
) {
    let _ = inner_walls;
    storage.mark_mutable();
    // 机器码步骤 3：逐 archivist 地址 get_mut + clear_modification_tracking。
    for address in archivist_all_storages(&archivist) {
        if let Some(bucket) = storage.get_mut(addr_from_address(address)) {
            bucket.clear_modification_tracking();
        }
    }
    // 机器码步骤 4：逐装饰落位（含 d2s 状态字节 2 → insert）。
    for (address, id) in archivist_all_decorators(&archivist) {
        overwrite_decorator_state_from_archivist(&mut storage, &archivist, address, id);
    }
    // 机器码步骤：楼梯备份恢复。
    stairs.restore_backup();
    // 机器码步骤 5/6（U：colliders/interactions 落位 FUN_141298110 身份 +
    // extract_diff 冲刷）——colliders/interactions 落位在 core 归档类型
    // （DecoratorArchivist collision_a/b）接线后补齐；extract_diff 冲刷属
    // 历史差异链（core add_decorator_diff 缺口，见 RESTORE.md）。
}

/// 单装饰归档状态覆盖（0x140817a40 步骤 4 内联）：archivist 条目的
/// `ObservedDecoratorInfo` 解码为 `DecoratorInfo` → 目标桶 insert → 按
/// 插入返回的旧Option选择 d2s insert/update，不改输入flags。
///
/// 返回是否写入（归档条目缺失 → false，不伪造默认状态）。
pub fn overwrite_decorator_state_from_archivist<C: Send + Sync + 'static>(
    storage: &mut DecoratorStorage,
    archivist: &DecoratorArchivist<C>,
    address: DecoratorStorageAddress,
    id: u32,
) -> bool {
    let Some(tracked) = tracked_at(archivist, address) else {
        return false;
    };
    let Some(info) = tracked.items.get(&id) else {
        return false;
    };
    let decorator_info = crate::apply_decorator_op::bytes_to_info(&info.0);
    let addr = addr_from_address(address);
    let bucket = storage.get_or_insert_default(addr);
    let old = bucket.insert(DecoratorId(id), decorator_info);
    if old.is_none() {
        storage.decorator_to_storage.insert(DecoratorId(id), addr);
    } else {
        storage.decorator_to_storage.update(DecoratorId(id), addr);
    }
    true
}

/// DecoratorStorageAddress → DecoratorStorageAddr（core 模型桥；
/// recovered WallId → resources WallId 双类型转换）。
pub(crate) fn addr_from_address(address: DecoratorStorageAddress) -> DecoratorStorageAddr {
    match address {
        DecoratorStorageAddress::Wall(w) => DecoratorStorageAddr::Wall(WallId(w.0)),
        DecoratorStorageAddress::ObservedSpecial1 => DecoratorStorageAddr::Terrain,
        DecoratorStorageAddress::ObservedSpecial2 => DecoratorStorageAddr::Stair,
    }
}

/// 模型桥：WallId 保留（all_storages 墙键面）。
#[allow(dead_code)]
fn _wall_id_surface(_w: WallId) {}

const LOG_TARGET: &str = "system_decorator::restore_archived_decorators";

#[cfg(test)]
mod tests {
    use super::*;
    use country_core::resources::walls::decorator_archivist::changes::{
        ObservedDecoratorInfo, TrackedDecorators,
    };
    use country_core::resources::walls::publication_fields::WallId as ArchivedWallId;

    #[test]
    fn overwrite_restores_entry_and_d2s() {
        let mut archivist = DecoratorArchivist::<()>::default();
        let info = country_core::resources::walls::decorator::DecoratorInfo {
            decorator_id: 5,
            state: 0,
            ..Default::default()
        };
        let mut tracked = TrackedDecorators::default();
        tracked.items.insert(
            5,
            ObservedDecoratorInfo(crate::apply_decorator_op::info_to_bytes(info)),
        );
        archivist.walls.insert(ArchivedWallId(9), tracked);

        let mut storage = DecoratorStorage::default();
        assert!(overwrite_decorator_state_from_archivist(
            &mut storage,
            &archivist,
            DecoratorStorageAddress::Wall(ArchivedWallId(9)),
            5
        ));
        let entry = storage
            .lookup_decorator_info(DecoratorId(5))
            .expect("restored entry");
        assert_eq!(entry.state, 0);
        assert_eq!(
            storage.lookup_storage(DecoratorId(5)),
            Some(DecoratorStorageAddr::Wall(WallId(9)))
        );
    }

    #[test]
    fn overwrite_returns_false_for_missing_archive_entry() {
        let archivist = DecoratorArchivist::<()>::default();
        let mut storage = DecoratorStorage::default();
        assert!(!overwrite_decorator_state_from_archivist(
            &mut storage,
            &archivist,
            DecoratorStorageAddress::Wall(ArchivedWallId(9)),
            5
        ));
        assert!(storage.lookup_decorator_info(DecoratorId(5)).is_none());
    }

    #[test]
    fn overwrite_uses_old_option_for_both_legal_flags_and_preserves_partial_failure() {
        use std::panic::{AssertUnwindSafe, catch_unwind};
        use country_core::resources::walls::decorator::DecoratorInfo;
        for flag in [0, 1] {
            let address = DecoratorStorageAddress::Wall(ArchivedWallId(9));
            let addr = DecoratorStorageAddr::Wall(WallId(9));
            let id = DecoratorId(5);
            let info = DecoratorInfo { decorator_id: id.0, state: flag, u45: 1 - flag, u28: 0x7fc1_2345, ..Default::default() };
            let mut archive = DecoratorArchivist::<()>::default();
            let mut tracked = TrackedDecorators::default();
            tracked.items.insert(id.0, ObservedDecoratorInfo(crate::apply_decorator_op::info_to_bytes(info)));
            archive.walls.insert(ArchivedWallId(9), tracked);
            let mut storage = DecoratorStorage::default();
            assert!(overwrite_decorator_state_from_archivist(&mut storage, &archive, address, id.0));
            assert_eq!(storage.lookup_decorator_info(id), Some(&info));
            assert_eq!(storage.lookup_storage(id), Some(addr));
            // 已有旧值必须走update，重复恢复不触发insert冲突。
            assert!(overwrite_decorator_state_from_archivist(&mut storage, &archive, address, id.0));
            assert_eq!(storage.lookup_decorator_info(id), Some(&info));
            let mut conflict = DecoratorStorage::default();
            conflict.decorator_to_storage.insert(id, DecoratorStorageAddr::Terrain);
            assert!(catch_unwind(AssertUnwindSafe(|| overwrite_decorator_state_from_archivist(&mut conflict, &archive, address, id.0))).is_err());
            assert_eq!(conflict.get(addr).unwrap().get(id), Some(&info));
            assert!(conflict.get(addr).unwrap().modified_ids.contains(&id.0));
        }
    }

}
