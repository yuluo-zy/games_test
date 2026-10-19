//! `apply_decorator_op.rs` — 装饰操作应用（生命周期核心，0x141276e30，O）。
//!
//! 机器码证据（fresh RE-MCP `groupD/decomp/decomp-0x141276e30-*.txt`）：
//! 参数面（8 参，handle_external_decorator_cmds 0x14081a380 调用点 O）：
//! `(op: &DecoratorOp, kind_flag: u8, dst: &DecoratorStorageAddr,
//! storage: &mut DecoratorStorage, archivist: Option<&DecoratorArchivist>,
//! history: Option<&mut DeferredDecoratorEditHistory>, sink, events:
//! &DeferredDecoratorOpEvents)`。
//!
//! 行为（O 结构，逐分支）：
//! 1. `assert!(archivist.is_some() || kind_flag == 0,
//!    "assertion failed: decorator_archivist.is_some()")`（panic_str 0x142c23510
//!    line 0x2f，file apply_decorator_op.rs）；
//! 2. 按 `op.kind` 分派（O：`(kind-3)<2` → 3=place、4=remove；其余 change）：
//!    place → `apply_decorator_place` 0x140a20c70、remove →
//!    `apply_decorator_remove` 0x140a20670、change → `apply_decorator_change`
//!    0x140a20890；
//! 3. 每分支先写历史（write_place/change/remove_op_history，PDB 内联行
//!    156/166/182 → `DeferredDecoratorEditHistory::add` 行 34），record kind =
//!    5/4/3（remove/place/change）；
//! 4. 归档预览（别名集 FUN_1412b4a40 count=4，U：身份未解析；归档标签串
//!    "place preview"/"remove preview"/"move preview src"/"move preview dst"）；
//! 5. 应用存储变更；
//! 6. `DeferredDecoratorOpEvents::send` 0x1408336b0 推 0x88B 事件。
//!
//! 0x88B 事件字段放置（O 读面）：`{kind u64 @0, wall u64, secondary u64,
//! payload, u18, u20.., id.., extra 状态字节簇, dst 16B, kind 字节}`——
//! 精确解码以消费端（visual/ui 批次）复核为准（尾部 U 登记）。
//!
//! **overlay 冲突登记（D2）**：D2 曾以 `(storage, id, request, history)` 4 参
//! 签名 + `EditType::Decorator(Default)` 空载荷覆盖本文件；其 "History tag
//! 0xd/0xe/0x10" 为把 FUN_1412b4a40 的归档标签串长（"place preview"=13=0xd、
//! "remove preview"=14=0xe、"move preview src"=16=0x10，rne-read 实证）误读为
//! tag。本文件按机器码恢复（8 参面 + tag 10 历史链 + 5/4/3 kind 记录），
//! 由集成 owner 裁决。
//!
//! core 缺口登记：`DecoratorArchivist::archive_storage_decorators`
//! （0x140b2be70/0x1412b2a70<&str>/0x1412b3910<String>，章节 O）未在 core 实现；
//! 本模块提供等价归档（墙桶 → archivist.walls TrackedDecorators，
//! [`archive_decorator_state`]）；预览标签桶（special）登记 U。

use country_core::resources::walls::decorator::{
    DecoratorExtraInfo, DecoratorId, DecoratorInfo, DecoratorLocator, DeferredDecoratorOpEvents,
    StairExtra,
};
use country_core::resources::walls::decorator_archivist::DecoratorArchivist;
use country_core::resources::walls::decorator_storage::{
    DecoratorStorage, DecoratorStorageAddr,
};
use country_core::resources::walls::decorator_archivist::changes::ObservedDecoratorInfo;
use country_core::resources::walls::decorator_archivist::changes::TrackedDecorators;

use crate::deferred_decorator_history::{
    DeferredDecoratorEditHistory, DeferredDecoratorRecord,
};

/// 装饰操作记录（112B = {kind u64 @0, DecoratorInfo @8, 32B pad}）。
/// 机器码读面（O）：`param_1[0..0xd]`（0x70B）；`[1]=wall`（DecoratorLocator
/// 参数 1）、`[2]=secondary`、`[0xb]@0x5c..0x5f` 状态字节簇、`[0xc]/[0xd]`
/// 载荷（语义 U）。pad 区语义 U。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DecoratorOp {
    /// +0x00：操作 kind（3=place、4=remove、其余=change/move）。
    pub kind: u64,
    /// +0x08：DecoratorInfo 主体。
    pub info: DecoratorInfo,
    /// +0x50：未解析载荷（含 +0x5c..0x5f 状态字节簇，U）。
    pub extra: [u8; 0x20],
    /// 原生 `+0x10` 第二定位器；`None` = 哨兵 3（原地更新，不迁移存储）。
    pub dst: Option<DecoratorStorageAddr>,
}

impl DecoratorOp {
    /// kind 常量（O：dispatch `(kind-3)<2` → place/remove）。
    pub const KIND_PLACE: u64 = 3;
    pub const KIND_REMOVE: u64 = 4;

    /// 0x141276e30 分派（O）：3=place、4=remove；其余 change。
    pub fn op_kind(&self) -> DecoratorOpKind {
        if self.kind == Self::KIND_PLACE {
            DecoratorOpKind::Place
        } else if self.kind == Self::KIND_REMOVE {
            DecoratorOpKind::Remove
        } else {
            DecoratorOpKind::Change
        }
    }

    /// +0x08 墙 id（机器码 `[1]`，O 读面；语义 I：wall 目的时 = WallId）。
    pub fn wall(&self) -> u64 {
        self.info.dst_wall
    }
}

/// 装饰操作种类（分派结果，O）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DecoratorOpKind {
    Place,
    Remove,
    Change,
}

/// 归档标签（O 字符串，apply_decorator_op 各分支 archive 键）。
pub const LABEL_PLACE: &str = "place preview";
pub const LABEL_REMOVE: &str = "remove preview";
pub const LABEL_MOVE_SRC: &str = "move preview src";
pub const LABEL_MOVE_DST: &str = "move preview dst";

/// `write_place_op_history`（PDB 行 156，内联）：延迟记录 kind=4 + 存续登记
/// （O：records.push(kind) + `DecoratorLocator::new` +
/// `HashMap<DecoratorLocator,()>::insert`；locator 构造参 U，以 dst 承载）。
#[inline]
pub fn write_place_op_history(
    history: &mut DeferredDecoratorEditHistory,
    op: &DecoratorOp,
    dst: DecoratorStorageAddr,
) {
    history.add(DeferredDecoratorRecord {
        kind: DeferredDecoratorRecord::KIND_PLACE,
        f1: u64::from(op.info.decorator_id),
        ..DeferredDecoratorRecord::default()
    });
    history.add_potentially_surviving_affected_decorator(DecoratorLocator::new(dst));
    history.pending = true;
}

/// `write_change_op_history`（PDB 行 166，内联）：kind=3。
#[inline]
pub fn write_change_op_history(
    history: &mut DeferredDecoratorEditHistory,
    op: &DecoratorOp,
    dst: DecoratorStorageAddr,
) {
    history.add(DeferredDecoratorRecord {
        kind: DeferredDecoratorRecord::KIND_CHANGE,
        f1: u64::from(op.info.decorator_id),
        ..DeferredDecoratorRecord::default()
    });
    history.add_potentially_surviving_affected_decorator(DecoratorLocator::new(dst));
    history.pending = true;
}

/// `write_remove_op_history`（PDB 行 182，内联）：kind=5。
///
/// fresh 0x141276e30（2026-10-17）：remove 分支**只有**记录 push（kind=5），
/// **没有** `DecoratorLocator::new` + `HashMap::insert`（仅 place/change 分支有）。
/// 早期版本对 remove 也登记 locator，与机器码不符，已删。
#[inline]
pub fn write_remove_op_history(
    history: &mut DeferredDecoratorEditHistory,
    op: &DecoratorOp,
) {
    history.add(DeferredDecoratorRecord {
        kind: DeferredDecoratorRecord::KIND_REMOVE,
        f1: u64::from(op.info.decorator_id),
        ..DeferredDecoratorRecord::default()
    });
    history.pending = true;
}

/// 归档装饰状态（等价 `archive_storage_decorators` 墙路径，机器码 0x1412b4a40
/// 行为；U：别名集身份 + 预览标签桶）。
///
/// 机器码：archivist 侧 `get_or_insert_default(addr)` + 存储侧 `get(addr)`
/// 断言存在 + `Decorators::tracked_clone_from`（0x1412573b0）——把存储桶拷入
/// archivist 的 per-wall 桶。core 模型 `TrackedDecorators{removed, items,
/// added}` = 存储桶 `{removed_ids, decorators, modified_ids}` 同构映射。
pub fn archive_decorator_state<C: Send + Sync + 'static>(
    archivist: &mut DecoratorArchivist<C>,
    storage: &DecoratorStorage,
    addr: DecoratorStorageAddr,
    label: &str,
) {
    // O：三路归档（0x1412b2a70 墙 / 0x1412b3910<String> 特殊桶）：
    // 墙 → archivist.walls；地形 → special1（原生 +0xc0）；楼梯 → special2
    // （原生 +0x150）。旧实现把非墙地址 log 后跳过（W16），导致恢复不完整。
    let (slot, source) = match addr {
        DecoratorStorageAddr::Wall(wall) => (
            ArchiveSlot::Wall(
                country_core::resources::walls::publication_fields::WallId(wall.0),
            ),
            storage.wall_decorators(wall),
        ),
        DecoratorStorageAddr::Terrain => (ArchiveSlot::Special1, storage.terrain.as_ref()),
        DecoratorStorageAddr::Stair => (ArchiveSlot::Special2, storage.stairs.as_ref()),
    };
    let Some(source) = source else {
        return;
    };
    let tracked = match slot {
        ArchiveSlot::Wall(wall) => archivist.walls.entry(wall).or_default(),
        ArchiveSlot::Special1 => archivist.special1.get_or_insert_with(Default::default),
        ArchiveSlot::Special2 => archivist.special2.get_or_insert_with(Default::default),
    };
    tracked.removed.clone_from(&source.removed_ids);
    tracked.items.clear();
    tracked.items.extend(
        source
            .decorators
            .iter()
            .map(|(id, info)| (*id, ObservedDecoratorInfo(info_to_bytes(*info)))),
    );
    tracked.added.clone_from(&source.modified_ids);
    let _ = label;
}

enum ArchiveSlot {
    Wall(country_core::resources::walls::publication_fields::WallId),
    Special1,
    Special2,
}

/// DecoratorInfo(72B, repr(C)) → 原始字节（归档桶载荷；字段序 = 机器码
/// +0x00..+0x47 布局，O）。供 backup.rs 复用。
pub(crate) fn info_to_bytes(info: DecoratorInfo) -> [u8; 72] {
    let mut out = [0u8; 72];
    out[0x00..0x08].copy_from_slice(&info.dst_wall.to_le_bytes());
    out[0x08..0x10].copy_from_slice(&info.dst_secondary.to_le_bytes());
    out[0x10..0x18].copy_from_slice(&info.dst_payload.to_le_bytes());
    out[0x18..0x20].copy_from_slice(&info.u18.to_le_bytes());
    out[0x20..0x24].copy_from_slice(&info.u20.to_le_bytes());
    out[0x24..0x28].copy_from_slice(&info.u24.to_le_bytes());
    out[0x28..0x2c].copy_from_slice(&info.u28.to_le_bytes());
    out[0x2c..0x30].copy_from_slice(&info.decorator_id.to_le_bytes());
    out[0x30..0x38].copy_from_slice(&info.u30.to_le_bytes());
    out[0x38..0x3c].copy_from_slice(&info.u38.to_le_bytes());
    out[0x3c..0x40].copy_from_slice(&info.u3c.to_le_bytes());
    out[0x40..0x44].copy_from_slice(&info.u40.to_le_bytes());
    out[0x44] = info.rank;
    out[0x45] = info.u45;
    out[0x46] = info.ty;
    out[0x47] = info.state;
    out
}

/// 原始字节 → DecoratorInfo（归档桶载荷还原；与 [`info_to_bytes`] 互逆，O）。
pub(crate) fn bytes_to_info(bytes: &[u8; 72]) -> DecoratorInfo {
    let r = |off: usize| u64::from_le_bytes(bytes[off..off + 8].try_into().unwrap());
    let r4 = |off: usize| u32::from_le_bytes(bytes[off..off + 4].try_into().unwrap());
    DecoratorInfo {
        dst_wall: r(0x00),
        dst_secondary: r(0x08),
        dst_payload: r(0x10),
        u18: r(0x18),
        u20: r4(0x20),
        u24: r4(0x24),
        u28: r4(0x28),
        decorator_id: r4(0x2c),
        u30: r(0x30),
        u38: r4(0x38),
        u3c: r4(0x3c),
        u40: r4(0x40),
        rank: bytes[0x44],
        u45: bytes[0x45],
        ty: bytes[0x46],
        state: bytes[0x47],
    }
}

/// 构建 0x88B 延迟操作事件（机器码字段放置 O；精确解码以消费端复核为准）。
#[allow(clippy::too_many_arguments)]
pub fn build_op_event(
    op_kind: DecoratorOpKind,
    op: &DecoratorOp,
    dst: DecoratorStorageAddr,
    kind_byte: u8,
) -> [u8; 0x88] {
    let mut event = [0u8; 0x88];
    let mut write_u64 = |off: usize, v: u64| event[off..off + 8].copy_from_slice(&v.to_le_bytes());
    // 机器码事件首字段（O）：remove→4、place→3；change 分支 = op 首 qword（U）。
    let first = match op_kind {
        DecoratorOpKind::Place => 3u64,
        DecoratorOpKind::Remove => 4,
        DecoratorOpKind::Change => op.kind,
    };
    write_u64(0x00, first);
    write_u64(0x08, op.info.dst_wall);
    write_u64(0x10, op.info.dst_secondary);
    write_u64(0x18, op.info.dst_payload);
    write_u64(0x20, op.info.u18);
    write_u64(0x28, u64::from(op.info.u20));
    write_u64(0x30, u64::from(op.info.decorator_id));
    drop(write_u64);
    // +0x38..0x40：op.extra 状态字节簇（机器码 op[0xb]@0x5c..0x5f 放置，U）。
    event[0x38..0x3c].copy_from_slice(&op.extra[0x0c..0x10]);
    // +0x40..0x50：dst 16B（{判别 u64, payload u64}，O 放置）。
    match dst {
        DecoratorStorageAddr::Wall(w) => {
            event[0x40..0x48].copy_from_slice(&0u64.to_le_bytes());
            event[0x48..0x50].copy_from_slice(&w.0.to_le_bytes());
        }
        DecoratorStorageAddr::Terrain => {
            event[0x40..0x48].copy_from_slice(&1u64.to_le_bytes());
        }
        DecoratorStorageAddr::Stair => {
            event[0x40..0x48].copy_from_slice(&2u64.to_le_bytes());
        }
    }
    // +0x50..0x88：未解析区（机器码 uStack_167/bStack_168 等，U）。
    event[0x50] = kind_byte;
    event
}

/// `apply_decorator_op` @0x141276e30（O 结构，2026-10-17 fresh 反编译逐分支核）。
///
/// fresh 控制流（0x141276e30）：
/// ```
/// if (archivist.is_none() && kind_byte != 0) → panic   // assert 串
///   "assertion failed: decorator_archivist.is_some()"（0x142c23510，行 50）
/// kind 顶层判别：`(kind-3)<2` → 3=place(lVar1=1)/4=remove(lVar1=2)；
///   其余(0/1/2/5..)=change(lVar1=0)
/// change: 子判别 op[+0x10]（U：子字段语义）→ archive 无条件一次，
///   子判别 !=3 时二次 archive → history(有则) push kind=3 +
///   DecoratorLocator::new + HashMap<DecoratorLocator,()>::insert →
///   apply_decorator_change
/// place:  archive 一次 → history push kind=4 + locator+map insert →
///   apply_decorator_place
/// remove: archive 一次 → history push kind=5 → apply_decorator_remove
/// 末尾 DeferredDecoratorOpEvents::send
/// ```
///
/// core 对接：`DecoratorStorage::apply_decorator_place/remove/change` +
/// `DeferredDecoratorOpEvents::send`（0x1408336b0）。历史经
/// [`DeferredDecoratorEditHistory`] 延迟（decorator_record_history 排空）。
#[allow(clippy::too_many_arguments)]
pub fn apply_decorator_op<C: Send + Sync + 'static>(
    op: &DecoratorOp,
    kind_byte: u8,
    dst: DecoratorStorageAddr,
    storage: &mut DecoratorStorage,
    decorator_archivist: Option<&mut DecoratorArchivist<C>>,
    history: Option<&mut DeferredDecoratorEditHistory>,
    events: &DeferredDecoratorOpEvents,
) {
    // O：panic_str 0x142c23510 行 50 —— `assert!(decorator_archivist.is_some())`
    // 在 kind_byte != 0 门控下（非 change 类必须带归档）。消息串与文件/行已
    // read_memory 核验；本仓源码路径/行不同，消息文本保持一致。
    if kind_byte != 0 {
        assert!(decorator_archivist.is_some());
    }
    // O：三个分派分支各自对 archivist None 执行 `core::option::unwrap_failed`
    // （0x141276e30 change/place/remove 内 `if (stack28==0) unwrap_failed`）——
    // 镜像 `Option::unwrap`，不静默跳过（reviewer-state FIX-7）。
    let decorator_archivist = decorator_archivist.unwrap();

    let kind = op.op_kind();

    // 归档 + 历史（O 顺序：**归档在前，历史在后**；历史经 write_*_op_history）。
    let label = match kind {
        DecoratorOpKind::Place => LABEL_PLACE,
        DecoratorOpKind::Remove => LABEL_REMOVE,
        DecoratorOpKind::Change => LABEL_MOVE_SRC,
    };
    match kind {
        DecoratorOpKind::Place | DecoratorOpKind::Remove => {
            archive_decorator_state(&mut *decorator_archivist, storage, dst, label);
            if let Some(mut history) = history {
                match kind {
                    DecoratorOpKind::Place => write_place_op_history(&mut history, op, dst),
                    _ => write_remove_op_history(&mut history, op),
                }
            }
        }
        DecoratorOpKind::Change => {
            archive_decorator_state(&mut *decorator_archivist, storage, dst, label);
            // O：change 分支第二定位器 != 哨兵 3 时按目标地址二次归档
            // （fresh 0x141276e30：`lVar1 = op[+0x10..0x18]`；`!= 3` →
            // `FUN_1412b4a40(..., "move preview dst", 0x10)`）。
            if let Some(target) = op.dst {
                archive_decorator_state(
                    &mut *decorator_archivist,
                    storage,
                    target,
                    LABEL_MOVE_DST,
                );
            }
            if let Some(mut history) = history {
                write_change_op_history(&mut history, op, dst);
            }
        }
    }

    // 存储应用（O，在历史之后）。
    let id = DecoratorId(op.info.decorator_id);
    match kind {
        DecoratorOpKind::Place => {
            storage.apply_decorator_place(dst, id, &op.info);
        }
        DecoratorOpKind::Remove => {
            storage.apply_decorator_remove(dst, id);
        }
        DecoratorOpKind::Change => {
            match op.dst {
                // 正常迁移（O：0x140a20890 非哨兵路径）：先 remove(源)，
                // 再 get_or_insert_default(目标) + insert + d2s update +
                // modified_ids 标记 + 逐字段更新。core
                // `apply_decorator_change` 只能同址 remove/insert（无法表达
                // 源≠目标），故此处按机器码恢复迁移路径。
                Some(target) => {
                    apply_decorator_change_move(storage, dst, target, id, &op.info);
                }
                // O 哨兵 3（fresh 0x140a20890）：**不 remove/reinsert**，仅
                // `get_mut` + `modified_ids.insert` + 内嵌条目逐字段更新。
                // core `apply_decorator_change` 仅实现正常路径（已登记缺口），
                // 此处按机器码恢复哨兵路径。
                None => apply_decorator_change_in_place(storage, dst, id, &op.info),
            }
        }
    }

    // 0x88B 事件（O：FUN_1408336b0 send；kind 映射 3/4）。
    events.send(build_op_event(kind, op, dst, kind_byte));
}

/// `apply_decorator_change` 哨兵路径（fresh 0x140a20890，O）：目标 == 源时
/// **不迁移存储**，仅标记 modified + 逐字段更新内嵌条目。
///
/// 机器码（哨兵 `param_2[2] == 3` 分支）：
/// 1. `DecoratorStorage::get_mut(addr)`（0x140a1cde0；未命中 panic）；
/// 2. `modified_ids.insert(id)`（`FUN_1402c9200(bucket+0x60, id)`）；
/// 3. 内嵌 map 探针（`FUN_1405b0ed0(bucket+0x50, &id)`）→ 逐判别写字段。
///
/// core 的 `DecoratorStorage::apply_decorator_change` 只实现正常
/// remove/reinsert 路径（章节 §942 已登记），故哨兵路径在此按机器码恢复。
pub(crate) fn apply_decorator_change_in_place(
    storage: &mut DecoratorStorage,
    addr: DecoratorStorageAddr,
    id: DecoratorId,
    info: &DecoratorInfo,
) {
    let Some(decorators) = storage.get_mut(addr) else {
        panic!("apply_decorator_change: target bucket missing");
    };
    decorators.modified_ids.insert(id.0);
    let Some(entry) = decorators.decorators.get_mut(&id.0) else {
        panic!("apply_decorator_change: missing decorator entry");
    };
    apply_decorator_change_field_update(entry, info);
}

/// `apply_decorator_change` 正常迁移路径（fresh 0x140a20890，O）：
/// `remove(src)` → `get_or_insert_default(dst)` + `insert` → d2s update →
/// `modified_ids.insert` → 内嵌条目逐字段更新。
pub(crate) fn apply_decorator_change_move(
    storage: &mut DecoratorStorage,
    src: DecoratorStorageAddr,
    target: DecoratorStorageAddr,
    id: DecoratorId,
    info: &DecoratorInfo,
) {
    storage.apply_decorator_remove(src, id);
    {
        let bucket = storage.get_or_insert_default(target);
        bucket.insert(id, *info);
    }
    storage.decorator_to_storage.update(id, target);
    let bucket = storage
        .get_mut(target)
        .unwrap_or_else(|| panic!("apply_decorator_change: target bucket missing"));
    bucket.modified_ids.insert(id.0);
    let entry = bucket
        .decorators
        .get_mut(&id.0)
        .unwrap_or_else(|| panic!("apply_decorator_change: missing decorator entry"));
    apply_decorator_change_field_update(entry, info);
}

/// 内嵌条目逐字段更新（fresh 0x140a20890 判别跳表，O）。
///
/// 判别源 = op.info 的 `u30/u38/u40/u45/rank`；写面与 core
/// `apply_decorator_change_field_update`（0x140a20890）逐条一致：
/// - `u40 低字节 == 1` → `rank = u40 字节1`；
/// - `u45 != 0x40` → `ty = u45`（`0x40` 为“跳过”哨兵）；
/// - `u40 字节2 == 1` → `decorator_id += (i8)u40 字节3`（demo 恒 false →
///   `u45 = 5`）；
/// - `u30 低 32 位 == 1` → rank 判别后 `entry.u30 低 32 = info.u30 高 32`
///   （楼梯高度载荷）；
/// - `u38 & 1` → type 判别后 `entry.u30 高 32 = info.u3c`；
/// - `rank != 2` → `entry.u40 字节1 == 5` 断言后 `entry.u30 最低字节 = rank`。
fn apply_decorator_change_field_update(entry: &mut DecoratorInfo, info: &DecoratorInfo) {
    if info.u40 & 0xff == 1 {
        entry.rank = ((info.u40 >> 8) & 0xff) as u8;
    }
    if info.u45 != 0x40 {
        entry.ty = info.u45;
    }
    if (info.u40 >> 16) & 0xff == 1 {
        entry.decorator_id = entry
            .decorator_id
            .wrapping_add(((info.u40 >> 24) & 0xff) as i8 as u32);
        const DEV_OVERRIDE_IS_DEMO_MODE: bool = false;
        entry.u45 = if DEV_OVERRIDE_IS_DEMO_MODE { 0 } else { 5 };
    }
    if info.u30 as u32 == 1 {
        let disc = (((entry.u40 >> 8) & 0xff) as u8)
            .wrapping_sub(2)
            .min(6)
            | 4;
        if disc != 6 {
            panic!("apply_decorator_change: invalid DecoratorExtraInfo rank");
        }
        let height_bits = (info.u30 >> 32) as u32;
        entry.u30 = (entry.u30 & 0xffff_ffff_0000_0000) | u64::from(height_bits);
        // core 模型适配：`DecoratorInfo::extra()` 把楼梯载荷编码在
        // dst_payload/u18/u20（章节登记偏移偏离）；同步首 f32，保证
        // `extra().as_stair()` 消费者可见新高度。
        let mut raw: [u8; 20] = entry.extra().as_stair().0;
        raw[0..4].copy_from_slice(&height_bits.to_le_bytes());
        entry.set_extra(DecoratorExtraInfo::Stair(StairExtra(raw)));
    }
    if info.u38 & 1 != 0 {
        let disc = (((entry.u40 >> 8) & 0xff) as u8) | 0xf8;
        if disc >= 0xfa {
            panic!("apply_decorator_change: invalid DecoratorExtraInfo type");
        }
        entry.u30 = (entry.u30 & 0xffff_ffff) | (u64::from(info.u3c) << 32);
    }
    if info.rank != 2 {
        if (entry.u40 >> 8) & 0xff != 5 {
            panic!("apply_decorator_change: invalid DecoratorExtraInfo rank");
        }
        entry.u30 = (entry.u30 & 0xffff_ffff_ff00_0000) | u64::from(info.rank);
    }
}

/// 归档索引面：TrackedDecorators 往返保留（backup/restore 共享）。
#[allow(dead_code)]
fn _tracked_surface(_t: TrackedDecorators) {}

#[cfg(test)]
mod tests {
    use super::*;
    use country_core::resources::walls::decorator_storage::Decorators;
    use country_core::resources::walls::WallId;

    fn height_op(id: u32, height: f32, dst: Option<DecoratorStorageAddr>) -> DecoratorOp {
        DecoratorOp {
            kind: 0,
            info: DecoratorInfo {
                decorator_id: id,
                dst_secondary: 3,
                u30: 1 | (u64::from(height.to_bits()) << 32),
                rank: 2,
                u45: 0x40,
                ..DecoratorInfo::default()
            },
            extra: [0; 0x20],
            dst,
        }
    }

    fn storage_with(wall: WallId, id: u32) -> DecoratorStorage {
        let mut storage = DecoratorStorage::default();
        let mut bucket = Decorators::default();
        bucket.decorators.insert(
            id,
            DecoratorInfo {
                decorator_id: id,
                // 楼梯 extra_style 判别：低 3 位 0/1（8/9）才通过 as_stair 与
                // u30 rank 判别（`(b-2).min(6)|4 == 6`）。
                u40: 0x0000_0800,
                state: 2,
                ..DecoratorInfo::default()
            },
        );
        storage.per_wall.insert(wall, bucket);
        storage
            .decorator_to_storage
            .insert(DecoratorId(id), DecoratorStorageAddr::Wall(wall));
        storage
    }

    #[test]
    fn sentinel_change_updates_in_place_without_removing() {
        let wall = WallId(7);
        let mut storage = storage_with(wall, 9);
        let mut archivist = DecoratorArchivist::<()>::default();
        let events = DeferredDecoratorOpEvents::default();
        let addr = DecoratorStorageAddr::Wall(wall);
        let op = height_op(9, 3.5, None);
        apply_decorator_op(
            &op,
            0,
            addr,
            &mut storage,
            Some(&mut archivist),
            None,
            &events,
        );
        let bucket = storage.wall_decorators(wall).unwrap();
        let entry = bucket.decorators.get(&9).expect("entry retained in place");
        assert_eq!(entry.u30 as u32, 3.5f32.to_bits());
        // core 载荷编码同步。
        assert_eq!(
            f32::from_le_bytes(entry.extra().as_stair().0[0..4].try_into().unwrap()),
            3.5
        );
        assert!(bucket.modified_ids.contains(&9));
        assert!(!bucket.removed_ids.contains(&9), "in-place path must not remove");
        assert_eq!(
            storage.lookup_storage(DecoratorId(9)),
            Some(DecoratorStorageAddr::Wall(wall))
        );
    }

    #[test]
    fn change_move_removes_source_and_inserts_target() {
        let src = WallId(7);
        let target = WallId(8);
        let mut storage = storage_with(src, 9);
        // 目标墙桶存在（native get_or_insert_default 亦可新建；这里验证迁移）。
        storage.per_wall.insert(target, Decorators::default());
        let mut archivist = DecoratorArchivist::<()>::default();
        let events = DeferredDecoratorOpEvents::default();
        let op = height_op(9, 6.25, Some(DecoratorStorageAddr::Wall(target)));
        apply_decorator_op(
            &op,
            0,
            DecoratorStorageAddr::Wall(src),
            &mut storage,
            Some(&mut archivist),
            None,
            &events,
        );
        assert!(storage.wall_decorators(src).unwrap().decorators.get(&9).is_none());
        assert!(storage.wall_decorators(src).unwrap().removed_ids.contains(&9));
        let target_bucket = storage.wall_decorators(target).unwrap();
        let entry = target_bucket.decorators.get(&9).expect("moved entry");
        assert_eq!(entry.u30 as u32, 6.25f32.to_bits());
        assert!(target_bucket.modified_ids.contains(&9));
        assert_eq!(
            storage.lookup_storage(DecoratorId(9)),
            Some(DecoratorStorageAddr::Wall(target))
        );
    }

    #[test]
    fn field_update_rejects_invalid_rank_discriminant() {
        // entry.u40 字节1=5 → `(5-2).min(6)|4 = 7 != 6` → panic。
        let mut entry = DecoratorInfo {
            u40: 0x0000_0500,
            ..DecoratorInfo::default()
        };
        let info = height_op(1, 1.0, None).info;
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            apply_decorator_change_field_update(&mut entry, &info);
        }));
        assert!(result.is_err());
    }
}
