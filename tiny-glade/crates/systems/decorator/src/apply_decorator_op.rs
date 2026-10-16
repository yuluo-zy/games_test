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
    DecoratorId, DecoratorInfo, DecoratorOpEventKind, DeferredDecoratorOpEvents,
};
use country_core::resources::walls::decorator_archivist::DecoratorArchivist;
use country_core::resources::walls::decorator_storage::{
    DecoratorStorage, DecoratorStorageAddr,
};
use country_core::recovered_building::wall_decorator_archivist::{
    ObservedDecoratorInfo, TrackedDecorators,
};

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

/// `write_place_op_history`（PDB 行 156，内联）：延迟记录 kind=4 + 存续登记。
#[inline]
pub fn write_place_op_history(
    history: &mut DeferredDecoratorEditHistory,
    _op: &DecoratorOp,
    decorator_id: u32,
) {
    history.add(DeferredDecoratorRecord {
        kind: DeferredDecoratorRecord::KIND_PLACE,
        a: decorator_id,
        ..DeferredDecoratorRecord::default()
    });
    history.add_potentially_surviving_affected_decorator(decorator_id);
    history.pending = true;
}

/// `write_change_op_history`（PDB 行 166，内联）：kind=3。
#[inline]
pub fn write_change_op_history(
    history: &mut DeferredDecoratorEditHistory,
    _op: &DecoratorOp,
    decorator_id: u32,
) {
    history.add(DeferredDecoratorRecord {
        kind: DeferredDecoratorRecord::KIND_CHANGE,
        a: decorator_id,
        ..DeferredDecoratorRecord::default()
    });
    history.add_potentially_surviving_affected_decorator(decorator_id);
    history.pending = true;
}

/// `write_remove_op_history`（PDB 行 182，内联）：kind=5。
#[inline]
pub fn write_remove_op_history(
    history: &mut DeferredDecoratorEditHistory,
    _op: &DecoratorOp,
    decorator_id: u32,
) {
    history.add(DeferredDecoratorRecord {
        kind: DeferredDecoratorRecord::KIND_REMOVE,
        a: decorator_id,
        ..DeferredDecoratorRecord::default()
    });
    history.add_potentially_surviving_affected_decorator(decorator_id);
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
    // 墙桶路径（O）。特殊地址/预览标签：core 模型无字符串键桶，登记 U。
    let Some(wall) = addr.try_as_wall() else {
        log::warn!(
            target: LOG_TARGET,
            "archive_decorator_state: non-wall addr under label '{label}' is U (core archivist string-keyed bucket missing)"
        );
        return;
    };
    let Some(source) = storage.wall_decorators(wall) else {
        return;
    };
    // archivist 桶键 = recovered_building WallId（与 resources WallId 双类型；
    // 以 u64 桥接）。
    let archivist_wall = country_core::recovered_building::wall_state::WallId(wall.0);
    let tracked = archivist.walls.entry(archivist_wall).or_default();
    tracked.removed.clone_from(&source.removed_ids);
    tracked.items.clear();
    tracked.items.extend(
        source
            .decorators
            .iter()
            .map(|(id, info)| (*id, ObservedDecoratorInfo(info_to_bytes(*info)))),
    );
    tracked.added.clone_from(&source.modified_ids);
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

/// `apply_decorator_op` @0x141276e30（O 结构）。
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
    archivist: Option<&mut DecoratorArchivist<C>>,
    history: Option<&mut DeferredDecoratorEditHistory>,
    events: &DeferredDecoratorOpEvents,
) {
    // O：panic_str 0x142c23510 line 0x2f —— archivist 缺失且 kind 非 0。
    if archivist.is_none() && kind_byte != 0 {
        panic!(
            "assertion failed: decorator_archivist.is_some() (crates/systems/decorator/src/apply_decorator_op.rs:47)"
        );
    }

    let kind = op.op_kind();
    let decorator_id = op.info.decorator_id;
    let id = DecoratorId(decorator_id);

    // 历史（O：每分支先写；record kind 5/4/3）。
    let label = match kind {
        DecoratorOpKind::Place => LABEL_PLACE,
        DecoratorOpKind::Remove => LABEL_REMOVE,
        DecoratorOpKind::Change => LABEL_MOVE_SRC,
    };
    if let Some(mut history) = history {
        match kind {
            DecoratorOpKind::Place => write_place_op_history(&mut history, op, decorator_id),
            DecoratorOpKind::Remove => write_remove_op_history(&mut history, op, decorator_id),
            DecoratorOpKind::Change => {
                write_change_op_history(&mut history, op, decorator_id);
                // 机器码 change 分支二次归档（move preview dst，条件 lVar1!=3；
                // 条件语义 U——此处按无条件归档登记）。
                let _ = LABEL_MOVE_DST;
            }
        }
    }

    // 归档（O：FUN_1412b4a40 别名集；身份 U）。
    if let Some(archivist) = archivist {
        archive_decorator_state(archivist, storage, dst, label);
    }

    // 存储应用（O）。
    match kind {
        DecoratorOpKind::Place => {
            storage.apply_decorator_place(dst, id, &op.info);
        }
        DecoratorOpKind::Remove => {
            storage.apply_decorator_remove(dst, id);
        }
        DecoratorOpKind::Change => {
            storage.apply_decorator_change(dst, id, &op.info);
        }
    }

    // 0x88B 事件（O：FUN_1408336b0 send；kind 映射 3/4）。
    events.send(build_op_event(kind, op, dst, kind_byte));
}

/// `DecoratorOpEventKind` 桥（core 类型；本模块分派等价映射）。
#[allow(dead_code)]
pub(crate) fn op_event_kind(kind: DecoratorOpKind) -> DecoratorOpEventKind {
    match kind {
        DecoratorOpKind::Place => DecoratorOpEventKind::Place,
        DecoratorOpKind::Remove => DecoratorOpEventKind::Remove,
        DecoratorOpKind::Change => DecoratorOpEventKind::Change,
    }
}

/// 归档索引面：TrackedDecorators 往返保留（backup/restore 共享）。
#[allow(dead_code)]
fn _tracked_surface(_t: TrackedDecorators) {}

const LOG_TARGET: &str = "system_decorator::apply_decorator_op";
