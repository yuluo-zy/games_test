//! 装饰操作应用。原B141276e30/1467B按实际操作分派，归档先于历史、存储和事件。
//!
//! 原八参第三参是独立的16B位置/状态载荷，源地址在操作union内；第七参是
//! DecoratorArchivalResources，不能把二者误认为目标地址与空sink。
//! 现代七参保留显式源地址；生产cull先以真实资源归档，本层数据兼容入口只保留首快照。
//!
//! 本轮按原完整remove分支恢复：真正移出的Info72B写事件+18，独立map ID写+60，
//! 位置16B写+70（+7c为!recording），kind_flag独立写+80，实际类型写+81。
//! remove历史只激活判别5，不把Info.word2c冒充附加载荷，也不登记存续定位器。
//! 证据保存在reports/module-recovery-2026-10-23/m01-cull-{apply.asm,apply-fresh.c}。
//! place和旧兼容change的事件写面不纳入本轮完成声明；typed change由原owner维护。

use country_core::resources::walls::decorator::{
    DecoratorDst, DecoratorExtraInfo, DecoratorId, DecoratorInfo, DecoratorLocator, DecoratorType, DeferredDecoratorOpEvents,
    StairExtra,
};
use country_core::resources::walls::decorator_archivist::DecoratorArchivist;
use country_core::resources::walls::decorator_storage::{
    DecoratorStorage, DecoratorStorageAddr,
};
use country_core::resources::walls::decorator_archivist::changes::TrackedDecorators;

use crate::deferred_decorator_history::{
    DeferredDecoratorEditHistory, DeferredDecoratorRecord,
};

/// 原 112B 操作 union 的现代语义模型，不复刻原传输布局。
/// 独立 map key 与 Info.word2c 分离；typed change 由 change 载荷表达。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DecoratorOp {
    /// +0x00：操作 kind（3=place、4=remove、其余=change/move）。
    pub kind: u64,
    /// 原 place +0x54 / change +0x68 的独立 map key，不是 Info.word2c。
    pub decorator_id: DecoratorId,
    /// 完整记录或旧兼容载荷；不能从该记录反推操作的 map key。
    pub info: DecoratorInfo,
    /// remove前16B为位置xyz和状态字节；其余保留兼容载荷，不声明Rust ABI偏移。
    pub extra: [u8; 0x20],
    /// 原生 `+0x10` 第二定位器；`None` = 哨兵 3（原地更新，不迁移存储）。
    pub dst: Option<DecoratorStorageAddr>,
    /// Typed change request. Place/remove still carry a complete Info, whereas
    /// change is a patch over an existing Info. These have different native
    /// unions and must never share the old guessed `Info @+8` interpretation.
    pub change: Option<DecoratorChangePatch>,
}

/// Semantic change fields from the original 112-byte union. `id` is the map
/// key at native +68, separate from the mutable Info word at +2c.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DecoratorChangePatch {
    pub id: DecoratorId,
    pub destination: Option<(DecoratorStorageAddr, DecoratorDst)>,
    pub rank: Option<u8>,
    pub ty: Option<DecoratorType>,
    pub word_2c_delta: Option<i8>,
    /// Rust执行上下文中的唯一demo门；不是原112B操作/136B事件union字段。
    /// 仅word_2c_delta为Some时消费，seed生产者必须显式填写真实DemoModeEnabled。
    pub seed_demo_mode: bool,
    pub extra_low: Option<u32>,
    pub extra_high: Option<u32>,
    pub extra_byte: Option<u8>,
    pub event_position: glam::Vec3,
    /// 原独立16B位置union的bool niche：0/1有活动XYZ，2没有位置。
    pub event_position_flag: u8,
}

impl DecoratorChangePatch {
    pub fn rotation(id: DecoratorId, addr: DecoratorStorageAddr, destination: DecoratorDst, position: glam::Vec3) -> Self {
        Self { id, destination: Some((addr, destination)), rank: None, ty: None,
            word_2c_delta: None, seed_demo_mode: false, extra_low: None, extra_high: None, extra_byte: None,
            event_position: position, event_position_flag: 0 }
    }

    fn apply_fields(self, entry: &mut DecoratorInfo) {
        if let Some((_, destination)) = self.destination { entry.set_destination(destination); }
        if let Some(rank) = self.rank { entry.rank = rank; }
        if let Some(ty) = self.ty { entry.ty = ty.0; }
        if let Some(delta) = self.word_2c_delta {
            entry.decorator_id = entry.decorator_id.wrapping_add(delta as i32 as u32);
            entry.u45 = if self.seed_demo_mode { 0 } else { 5 };
        }
        if let Some(value) = self.extra_low {
            let disc = (((entry.u40 >> 8) & 0xff) as u8).wrapping_sub(2).min(6) | 4;
            assert_eq!(disc, 6, "invalid DecoratorExtraInfo rank");
            entry.u30 = (entry.u30 & 0xffff_ffff_0000_0000) | u64::from(value);
        }
        if let Some(value) = self.extra_high {
            assert!((((entry.u40 >> 8) & 0xff) as u8 | 0xf8) < 0xfa, "invalid DecoratorExtraInfo type");
            entry.u30 = (entry.u30 & 0xffff_ffff) | (u64::from(value) << 32);
        }
        if let Some(value) = self.extra_byte {
            assert_eq!((entry.u40 >> 8) & 0xff, 5, "invalid DecoratorExtraInfo rank");
            entry.u30 = (entry.u30 & !0xff) | u64::from(value);
        }
    }
}

impl DecoratorOp {
    /// 原place位置载荷与+5c标志独立于Info；不能再把Info当作112B操作union。
    /// `position_flag`是原独立16B位置记录末字节，`placement_flag`流向事件+44。
    pub fn place_at(
        id: DecoratorId,
        info: DecoratorInfo,
        position: glam::Vec3,
        position_flag: u8,
        placement_flag: bool,
        dev_override_is_demo_mode: bool,
    ) -> Self {
        // 原140a20c94/ce7与ca5/ceb：新ID独立于旧Info，进程demo覆盖门写0或5。
        let info = DecoratorInfo { decorator_id: id.0,
            u45: if dev_override_is_demo_mode { 0 } else { 5 }, ..info };
        let mut extra = [0; 32];
        for (index, value) in position.to_array().into_iter().enumerate() {
            extra[index * 4..index * 4 + 4].copy_from_slice(&value.to_le_bytes());
        }
        extra[12] = position_flag;
        extra[16] = u8::from(placement_flag);
        Self { kind: Self::KIND_PLACE, decorator_id: id, info, extra,
            dst: None, change: None }
    }

    /// 原 remove 的位置载荷独立于72B Info；末字节保留 !History::is_recording。
    pub fn remove_at(
        id: DecoratorId,
        info: DecoratorInfo,
        position: glam::Vec3,
        non_recording: bool,
    ) -> Self {
        let mut extra = [0; 32];
        for (index, value) in position.to_array().into_iter().enumerate() {
            extra[index * 4..index * 4 + 4].copy_from_slice(&value.to_le_bytes());
        }
        extra[12] = u8::from(non_recording);
        Self {
            kind: Self::KIND_REMOVE,
            decorator_id: id,
            info,
            extra,
            dst: None,
            change: None,
        }
    }
    pub fn from_change(addr: DecoratorStorageAddr, patch: DecoratorChangePatch) -> Self {
        let mut extra = [0; 32];
        for (index, value) in patch.event_position.to_array().into_iter().enumerate() {
            extra[index * 4..index * 4 + 4].copy_from_slice(&value.to_le_bytes());
        }
        extra[12] = patch.event_position_flag;
        Self { kind: address_words(addr).0, decorator_id: patch.id,
            info: DecoratorInfo { decorator_id: patch.id.0, ..Default::default() },
            extra, dst: patch.destination.map(|(addr, _)| addr), change: Some(patch) }
    }
    /// 原壁面NoMove等已定义载荷仍由同一op.extra持有，不另造旁路队列或哨兵。
    pub fn from_change_with_event_data(addr: DecoratorStorageAddr, patch: DecoratorChangePatch, data: [u8; 16]) -> Self {
        let mut op = Self::from_change(addr, patch);
        op.extra[..16].copy_from_slice(&data);
        op
    }
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
/// `HashMap<DecoratorLocator,()>::insert`；原地址和独立 key 同时入集合）。
#[inline]
pub fn write_place_op_history(
    history: &mut DeferredDecoratorEditHistory,
    op: &DecoratorOp,
    dst: DecoratorStorageAddr,
) {
    history.add(DeferredDecoratorRecord {
        kind: DeferredDecoratorRecord::KIND_PLACE,
        ..DeferredDecoratorRecord::default()
    });
    history.add_potentially_surviving_affected_decorator(DecoratorLocator::new(dst, op.decorator_id.0));
    // 原只追加记录与locator，不请求提交；preview不能因apply而提前提交历史。
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
    // 原 destination 判别3沿源，否则使用明确目标地址；记录仍先于集合写入。
    history.add_potentially_surviving_affected_decorator(DecoratorLocator::new(
        op.dst.unwrap_or(dst), op.decorator_id.0,
    ));
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
    _op: &DecoratorOp,
) {
    // 原remove仅写判别5，其余union字节不活跃；不能冒充Info.word2c业务。
    history.add(DeferredDecoratorRecord {
        kind: DeferredDecoratorRecord::KIND_REMOVE,
        ..DeferredDecoratorRecord::default()
    });
    history.pending = true;
}

/// 数据桶兼容归档：保留同地址首次状态，禁止逐次操作覆盖原快照。
/// 生产 cull 在进入本函数前通过真实归档资源同时保存两类碰撞载荷。
pub fn archive_decorator_state<C: Send + Sync + 'static>(
    archivist: &mut DecoratorArchivist<C>,
    storage: &DecoratorStorage,
    addr: DecoratorStorageAddr,
    label: &str,
) {
    // 原同一地址仅保存首次快照，不能让后续 remove 覆盖 oob backup。
    // 生产调用在此前用带物理资源的入口归档；这里保留数据桶兼容面。
    let empty = country_core::resources::walls::decorator_storage::Decorators::default();
    let source = storage.get(addr).unwrap_or(&empty);
    let (selector, wall) = match addr {
        DecoratorStorageAddr::Wall(wall) => (0, Some(wall)),
        DecoratorStorageAddr::Terrain => (1, None),
        DecoratorStorageAddr::Stair => (2, None),
    };
    archivist.archive_storage_decorators(selector, source, wall);
    let _ = label;
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

/// 构建0x88B延迟事件：remove按原活跃字段完整编码，其它兼容分支另行核验。
#[allow(clippy::too_many_arguments)]
pub fn build_op_event(
    op_kind: DecoratorOpKind,
    op: &DecoratorOp,
    dst: DecoratorStorageAddr,
    kind_byte: u8,
) -> [u8; 0x88] {
    if op_kind == DecoratorOpKind::Place {
        // 141277260..2e9：只有这些字段在place变体中活跃；额外信息和颜色只写权威Info。
        let mut event = [0; 0x88];
        let (tag, payload) = address_words(dst);
        event[..8].copy_from_slice(&3_u64.to_le_bytes());
        event[8..16].copy_from_slice(&tag.to_le_bytes());
        event[16..24].copy_from_slice(&payload.to_le_bytes());
        event[0x18..0x40].copy_from_slice(&info_to_bytes(op.info)[..40]);
        event[0x40..0x44].copy_from_slice(&op.decorator_id.0.to_le_bytes());
        event[0x44] = op.extra[16];
        event[0x45] = op.info.rank;
        event[0x70..0x80].copy_from_slice(&op.extra[..16]);
        event[0x80] = kind_byte;
        event[0x81] = op.info.ty;
        return event;
    }
    if op_kind == DecoratorOpKind::Remove {
        let mut event = [0; 0x88];
        let (tag, payload) = address_words(dst);
        event[..8].copy_from_slice(&4_u64.to_le_bytes());
        event[8..16].copy_from_slice(&tag.to_le_bytes());
        event[16..24].copy_from_slice(&payload.to_le_bytes());
        event[0x18..0x60].copy_from_slice(&info_to_bytes(op.info));
        event[0x60..0x64].copy_from_slice(&op.decorator_id.0.to_le_bytes());
        event[0x70..0x80].copy_from_slice(&op.extra[..16]);
        event[0x80] = kind_byte;
        event[0x81] = op.info.ty;
        return event;
    }
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
    write_u64(0x30, u64::from(op.decorator_id.0));
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
    if let Some(patch) = op.change {
        apply_typed_change(patch, op.extra[..16].try_into().unwrap(), kind_byte, dst, storage, decorator_archivist, history, events);
        return;
    }
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
    let id = op.decorator_id;
    match kind {
        DecoratorOpKind::Place => {
            // 原140a20cd8从独立apply参数写Info.state，不能沿用调用者旧state。
            let info = DecoratorInfo { state: kind_byte, ..op.info };
            storage.apply_decorator_place(dst, id, &info);
        }
        DecoratorOpKind::Remove => {
            // 原事件携带真正移出的记录，而非调用者可能陈旧的 Info。
            let removed = storage.apply_decorator_remove(dst, id);
            let actual = DecoratorOp { info: removed, ..*op };
            events.send(build_op_event(kind, &actual, dst, kind_byte));
            return;
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

/// 原Place八参中的真实归档资源面。必须在数据桶变化前保存两张碰撞表，
/// 之后同一archivist的首次快照规则使兼容数据入口不重复覆盖快照。
#[allow(clippy::too_many_arguments)]
pub fn apply_place_with_resources(
    op: &DecoratorOp,
    state: u8,
    address: DecoratorStorageAddr,
    storage: &mut DecoratorStorage,
    archivist: &mut country_core::resources::walls::decorator_archivist::LiveDecoratorArchivist,
    history: Option<&mut DeferredDecoratorEditHistory>,
    resources: &country_core::resources::walls::decorator_archivist::DecoratorArchivalResources<'_>,
    events: &DeferredDecoratorOpEvents,
) {
    assert_eq!(op.op_kind(), DecoratorOpKind::Place);
    archivist.archive_current_decorators_at_with_resources(address, storage, resources, LABEL_PLACE);
    apply_decorator_op(op, state, address, storage, Some(archivist), history, events);
}

fn address_words(addr: DecoratorStorageAddr) -> (u64, u64) {
    match addr { DecoratorStorageAddr::Wall(wall) => (0, wall.0), DecoratorStorageAddr::Terrain => (1, 0), DecoratorStorageAddr::Stair => (2, 0) }
}

/// Modern owned patch path. Original remove/reinsert retains the removed Info
/// and applies the new destination/rank; patch bytes are never inserted as Info.
fn apply_typed_change<C: Send + Sync + 'static>(
    patch: DecoratorChangePatch, event_data: [u8; 16], kind_byte: u8, source: DecoratorStorageAddr,
    storage: &mut DecoratorStorage, archivist: Option<&mut DecoratorArchivist<C>>,
    history: Option<&mut DeferredDecoratorEditHistory>, events: &DeferredDecoratorOpEvents,
) {
    assert!(!storage.immutable, "assertion failed: !self.immutable");
    if kind_byte != 0 { assert!(archivist.is_some()); }
    let archivist = archivist.unwrap();
    let before = *storage.get(source).and_then(|bucket| bucket.get(patch.id)).expect("apply_decorator_change: missing decorator entry");
    archive_decorator_state(archivist, storage, source, LABEL_MOVE_SRC);
    let target = patch.destination.map_or(source, |(target, _)| target);
    if patch.destination.is_some() { archive_decorator_state(archivist, storage, target, LABEL_MOVE_DST); }
    if let Some(history) = history {
        // Native add writes only the record discriminant; the other three
        // qwords are inactive padding. Commit is armed by the UI release path.
        history.add(DeferredDecoratorRecord { kind: DeferredDecoratorRecord::KIND_CHANGE, f1: 0, f2: 0, f3: 0 });
        history.add_potentially_surviving_affected_decorator(DecoratorLocator::new(target, patch.id.0));
    }
    let mut after = before;
    patch.apply_fields(&mut after);
    if patch.destination.is_some() {
        storage.apply_decorator_remove(source, patch.id);
        storage.get_or_insert_default(target).insert(patch.id, after);
        storage.decorator_to_storage.update(patch.id, target);
    } else {
        let bucket = storage.get_mut(source).expect("apply_decorator_change: target bucket missing");
        *bucket.decorators.get_mut(&patch.id.0).expect("apply_decorator_change: missing decorator entry") = after;
        bucket.modified_ids.insert(patch.id.0);
    }
    events.send(typed_change_event(patch, before, source, kind_byte, event_data));
}

/// Canonical raw DeferredDecoratorOpEvents uses native +0..87. Only active
/// union fields are serialized; padding is normalized to zero.
pub(crate) fn typed_change_event(patch: DecoratorChangePatch, before: DecoratorInfo, source: DecoratorStorageAddr, kind_byte: u8, event_data: [u8; 16]) -> [u8; 0x88] {
    let mut event = [0u8; 0x88];
    let mut destination = before;
    let target = if let Some((target, value)) = patch.destination { destination.set_destination(value); target } else { source };
    let (source_tag, source_payload) = address_words(source);
    event[0..8].copy_from_slice(&source_tag.to_le_bytes());
    event[8..16].copy_from_slice(&source_payload.to_le_bytes());
    let (target_tag, target_payload) = if patch.destination.is_some() { address_words(target) } else { (3, 0) };
    event[0x10..0x18].copy_from_slice(&target_tag.to_le_bytes());
    event[0x18..0x20].copy_from_slice(&target_payload.to_le_bytes());
    if patch.destination.is_some() {
        let bytes = info_to_bytes(destination);
        event[0x20..0x44].copy_from_slice(&bytes[0..0x24]);
        event[0x48] = before.rank;
    }
    if let Some(value) = patch.extra_low { event[0x50..0x54].copy_from_slice(&1u32.to_le_bytes()); event[0x54..0x58].copy_from_slice(&value.to_le_bytes()); }
    if let Some(value) = patch.extra_high { event[0x58] = 1; event[0x5c..0x60].copy_from_slice(&value.to_le_bytes()); }
    if let Some(value) = patch.rank { event[0x60] = 1; event[0x61] = value; }
    if let Some(value) = patch.word_2c_delta { event[0x62] = 1; event[0x63] = value as u8; }
    event[0x64] = patch.extra_byte.unwrap_or(2);
    event[0x65] = patch.ty.map_or(0x40, |ty| ty.0);
    event[0x68..0x6c].copy_from_slice(&patch.id.0.to_le_bytes());
    event[0x70..0x80].copy_from_slice(&event_data);
    event[0x80] = kind_byte;
    event[0x81] = before.ty;
    event
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
            decorator_id: DecoratorId(id),
            info: DecoratorInfo {
                decorator_id: id,
                dst_secondary: 3,
                u30: 1 | (u64::from(height.to_bits()) << 32),
                rank: 2,
                u45: 0x40,
                ..DecoratorInfo::default()
            },
            extra: [0; 0x20],
                change: None,
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
                // The real Stair extra niche at Info+41 is exactly0/1.
                // Tag0 also passes native wrapping `(b-2).min(6)|4 ==6`.
                u40: 0,
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
    fn identity_storage(source: DecoratorStorageAddr) -> DecoratorStorage {
        let mut storage = DecoratorStorage::default();
        for (id, word) in [(11, 900), (29, 1900)] {
            storage.apply_decorator_place(source, DecoratorId(id), &DecoratorInfo {
                decorator_id: word, state: 1, ..DecoratorInfo::default()
            });
        }
        storage
    }

    fn identity_patch(id: u32) -> DecoratorChangePatch {
        DecoratorChangePatch {
            id: DecoratorId(id), destination: None, rank: None, ty: None,
            word_2c_delta: Some(1), seed_demo_mode: false, extra_low: None, extra_high: None,
            extra_byte: None, event_position: glam::Vec3::new(1.0, 2.0, 3.0), event_position_flag: 0,
        }
    }

    #[test]
    fn same_wall_place_ids_survive_and_repeated_locator_only_merges_the_set() {
        let addr = DecoratorStorageAddr::Wall(WallId(7));
        let mut storage = DecoratorStorage::default();
        storage.get_or_insert_default(addr);
        let mut archivist = DecoratorArchivist::<()>::default();
        let mut history = DeferredDecoratorEditHistory::default();
        let events = DeferredDecoratorOpEvents::default();
        for (id, word) in [(11, 900), (29, 1900)] {
            let op = DecoratorOp {
                kind: DecoratorOp::KIND_PLACE, decorator_id: DecoratorId(id),
                info: DecoratorInfo { decorator_id: word, state: 1, ..Default::default() },
                ..DecoratorOp::default()
            };
            apply_decorator_op(&op, 0, addr, &mut storage, Some(&mut archivist),
                Some(&mut history), &events);
            assert_eq!(storage.get(addr).unwrap().get(DecoratorId(id)).unwrap().decorator_id, word);
            assert!(storage.get(addr).unwrap().get(DecoratorId(word)).is_none());
        }
        // 原集合按完整 locator 去重，逐次历史记录不因同键去重。
        let repeat = DecoratorOp { kind: DecoratorOp::KIND_PLACE,
            decorator_id: DecoratorId(11), ..Default::default() };
        write_place_op_history(&mut history, &repeat, addr);
        assert_eq!(history.records.len(), 3);
        assert!(history.records.iter().all(|record| record.kind == DeferredDecoratorRecord::KIND_PLACE));
        assert_eq!(history.potentially_surviving_affected_decorators.len(), 2);
        for id in [11, 29] {
            assert!(history.potentially_surviving_affected_decorators.contains_key(&DecoratorLocator::new(addr, id)));
        }
        let remove = DecoratorOp { kind: DecoratorOp::KIND_REMOVE,
            decorator_id: DecoratorId(29), info: *storage.get(addr).unwrap().get(DecoratorId(29)).unwrap(),
            ..Default::default() };
        let mut remove_history = DeferredDecoratorEditHistory::default();
        apply_decorator_op(&remove, 0, addr, &mut storage, Some(&mut archivist),
            Some(&mut remove_history), &events);
        assert!(storage.get(addr).unwrap().get(DecoratorId(29)).is_none());
        assert!(storage.get(addr).unwrap().get(DecoratorId(11)).is_some());
        assert_eq!(remove_history.records[0].kind, DeferredDecoratorRecord::KIND_REMOVE);
        assert!(remove_history.potentially_surviving_affected_decorators.is_empty());
    }

    #[test]
    fn change_history_keeps_same_id_at_distinct_addresses_and_none_uses_source() {
        let source = DecoratorStorageAddr::Wall(WallId(7));
        let target = DecoratorStorageAddr::Terrain;
        let mut history = DeferredDecoratorEditHistory::default();
        let mut op = DecoratorOp { decorator_id: DecoratorId(29),
            info: DecoratorInfo { decorator_id: 1900, ..Default::default() },
            ..Default::default() };
        write_change_op_history(&mut history, &op, source);
        op.dst = Some(target);
        write_change_op_history(&mut history, &op, source);
        write_change_op_history(&mut history, &op, source);
        assert_eq!(history.records.len(), 3);
        assert!(history.records.iter().all(|record| record.kind == DeferredDecoratorRecord::KIND_CHANGE));
        assert_eq!(history.potentially_surviving_affected_decorators.len(), 2);
        for addr in [source, target] {
            assert!(history.potentially_surviving_affected_decorators.contains_key(&DecoratorLocator::new(addr, 29)));
            assert!(!history.potentially_surviving_affected_decorators.contains_key(&DecoratorLocator::new(addr, 1900)));
        }
    }

    #[test]
    fn typed_changes_preserve_map_ids_while_mutable_info_word_changes() {
        let source = DecoratorStorageAddr::Wall(WallId(7));
        let mut storage = identity_storage(source);
        let mut archivist = DecoratorArchivist::<()>::default();
        let mut history = DeferredDecoratorEditHistory::default();
        let events = DeferredDecoratorOpEvents::default();
        for id in [11, 29, 11] {
            let mut op = DecoratorOp::from_change(source, identity_patch(id));
            assert_eq!(op.decorator_id, DecoratorId(id));
            // typed 分支不读取旧兼容 Info carrier，更不能以它重定位 map key。
            op.info.decorator_id = 4000;
            apply_decorator_op(&op, 0, source, &mut storage, Some(&mut archivist),
                Some(&mut history), &events);
        }
        let bucket = storage.get(source).unwrap();
        assert_eq!(bucket.get(DecoratorId(11)).unwrap().decorator_id, 902);
        assert_eq!(bucket.get(DecoratorId(29)).unwrap().decorator_id, 1901);
        assert!(bucket.get(DecoratorId(4000)).is_none());
        assert_eq!(history.records.len(), 3);
        assert_eq!(history.potentially_surviving_affected_decorators.len(), 2);
        for id in [11, 29] {
            assert!(history.potentially_surviving_affected_decorators.contains_key(&DecoratorLocator::new(source, id)));
            assert!(bucket.modified_ids.contains(&id));
            assert!(!bucket.removed_ids.contains(&id));
        }
        let mut emitted = Vec::new();
        events.unleash(&mut emitted);
        let ids: Vec<_> = emitted.iter().map(|event| u32::from_le_bytes(event[0x68..0x6c].try_into().unwrap())).collect();
        assert_eq!(ids, vec![11, 29, 11]);
    }

    #[test]
    fn typed_move_registers_actual_target_with_the_independent_patch_id() {
        use country_core::resources::walls::decorator::{DecoratorRotationXZ, TerrainDecoratorDst};
        let source = DecoratorStorageAddr::Wall(WallId(7));
        let target = DecoratorStorageAddr::Terrain;
        let mut storage = identity_storage(source);
        let mut archivist = DecoratorArchivist::<()>::default();
        let mut history = DeferredDecoratorEditHistory::default();
        let events = DeferredDecoratorOpEvents::default();
        let mut patch = identity_patch(29);
        patch.destination = Some((target, DecoratorDst::Terrain(TerrainDecoratorDst {
            position_xz: glam::Vec2::new(5.0, 7.0), yaw: DecoratorRotationXZ(0.0),
        })));
        let op = DecoratorOp::from_change(source, patch);
        apply_decorator_op(&op, 0, source, &mut storage, Some(&mut archivist),
            Some(&mut history), &events);
        assert!(storage.get(source).unwrap().get(DecoratorId(29)).is_none());
        assert_eq!(storage.get(target).unwrap().get(DecoratorId(29)).unwrap().decorator_id, 1901);
        assert_eq!(storage.lookup_storage(DecoratorId(29)), Some(target));
        assert_eq!(history.potentially_surviving_affected_decorators.len(), 1);
        assert!(history.potentially_surviving_affected_decorators.contains_key(&DecoratorLocator::new(target, 29)));
        assert!(!history.potentially_surviving_affected_decorators.contains_key(&DecoratorLocator::new(source, 29)));
        let mut emitted = Vec::new();
        events.unleash(&mut emitted);
        assert_eq!(u32::from_le_bytes(emitted[0][0x68..0x6c].try_into().unwrap()), 29);
    }

    #[test]
    fn invalid_typed_field_keeps_registered_locator_before_storage_and_event_commit() {
        let source = DecoratorStorageAddr::Wall(WallId(7));
        let mut storage = identity_storage(source);
        let before = *storage.get(source).unwrap().get(DecoratorId(11)).unwrap();
        let mut archivist = DecoratorArchivist::<()>::default();
        let mut history = DeferredDecoratorEditHistory::default();
        let events = DeferredDecoratorOpEvents::default();
        let mut patch = identity_patch(11);
        patch.word_2c_delta = None;
        patch.extra_byte = Some(9); // 真实分支要求 extra tag5，当前记录tag0会在赋值前失败。
        let op = DecoratorOp::from_change(source, patch);
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            apply_decorator_op(&op, 0, source, &mut storage, Some(&mut archivist),
                Some(&mut history), &events);
        }));
        assert!(result.is_err());
        assert_eq!(history.records.len(), 1);
        assert_eq!(history.potentially_surviving_affected_decorators.len(), 1);
        assert!(history.potentially_surviving_affected_decorators.contains_key(&DecoratorLocator::new(source, 11)));
        assert_eq!(*storage.get(source).unwrap().get(DecoratorId(11)).unwrap(), before);
        let mut emitted = Vec::new();
        events.unleash(&mut emitted);
        assert!(emitted.is_empty());
    }

    #[test]
    fn remove_native_event_uses_authoritative_info_map_id_and_independent_status_and_kind_flag() {
        let address = DecoratorStorageAddr::Wall(WallId(53));
        for non_recording in [false, true] {
            for kind_flag in [0, 1] {
                let actual = DecoratorInfo {
                    decorator_id: 0xfeed,
                    ty: 57,
                    rank: 2,
                    u40: 6 << 8,
                    dst_secondary: 17,
                    ..Default::default()
                };
                let mut storage = DecoratorStorage::default();
                storage.apply_decorator_place(address, DecoratorId(11), &actual);
                let mut archive = DecoratorArchivist::<()>::default();
                let mut history = DeferredDecoratorEditHistory::default();
                let events = DeferredDecoratorOpEvents::default();
                let position = glam::Vec3::new(-1.5, 23.0, 4.25);
                let stale = DecoratorInfo {
                    decorator_id: 999,
                    ty: 3,
                    ..Default::default()
                };
                let op = DecoratorOp::remove_at(DecoratorId(11), stale, position, non_recording);
                apply_decorator_op(
                    &op,
                    kind_flag,
                    address,
                    &mut storage,
                    Some(&mut archive),
                    Some(&mut history),
                    &events,
                );
                assert!(storage.lookup_storage(DecoratorId(11)).is_none());
                assert!(storage.get(address).unwrap().removed_ids.contains(&11));
                assert_eq!(
                    archive.walls.values().next().unwrap().items[&11].0,
                    info_to_bytes(actual)
                );
                assert_eq!(history.records.len(), 1);
                assert_eq!(
                    history.records[0],
                    DeferredDecoratorRecord {
                        kind: DeferredDecoratorRecord::KIND_REMOVE,
                        ..Default::default()
                    }
                );
                assert!(history.potentially_surviving_affected_decorators.is_empty());
                events.with_events(|events| {
                    assert_eq!(events.len(), 1);
                    let event = &events[0];
                    assert_eq!(&event[0x18..0x60], &info_to_bytes(actual));
                    assert_eq!(&event[0x60..0x64], &11_u32.to_le_bytes());
                    assert_eq!(&event[8..16], &[0; 8]);
                    assert_eq!(&event[16..24], &53_u64.to_le_bytes());
                    for (i, value) in position.to_array().into_iter().enumerate() {
                        assert_eq!(&event[0x70 + i * 4..0x74 + i * 4], &value.to_le_bytes());
                    }
                    assert_eq!(event[0x7c], u8::from(non_recording));
                    assert_eq!(event[0x80], kind_flag);
                    assert_eq!(event[0x81], actual.ty);
                });
            }
        }
    }

    #[test]
    fn remove_native_preserves_first_oob_snapshot_across_multiple_removes_and_empty_bucket_archive()
    {
        let address = DecoratorStorageAddr::Wall(WallId(53));
        let mut storage = DecoratorStorage::default();
        let mut archive = DecoratorArchivist::<()>::default();
        let events = DeferredDecoratorOpEvents::default();
        for id in [11, 12] {
            storage.apply_decorator_place(
                address,
                DecoratorId(id),
                &DecoratorInfo {
                    decorator_id: id + 100,
                    ty: 56,
                    u40: 6 << 8,
                    ..Default::default()
                },
            );
        }
        archive_decorator_state(&mut archive, &storage, address, "oob backup");
        for id in [11, 12] {
            let op = DecoratorOp::remove_at(
                DecoratorId(id),
                DecoratorInfo::default(),
                glam::Vec3::ZERO,
                false,
            );
            apply_decorator_op(
                &op,
                0,
                address,
                &mut storage,
                Some(&mut archive),
                None,
                &events,
            );
        }
        let snapshot = &archive.walls.values().next().unwrap().items;
        assert_eq!(snapshot.len(), 2);
        assert_eq!(bytes_to_info(&snapshot[&11].0).decorator_id, 111);
        assert_eq!(bytes_to_info(&snapshot[&12].0).decorator_id, 112);
        assert!(storage.get(address).unwrap().decorators.is_empty());
        archive_decorator_state(
            &mut archive,
            &storage,
            DecoratorStorageAddr::Terrain,
            LABEL_REMOVE,
        );
        assert!(archive.special1.as_ref().unwrap().items.is_empty());
    }

    #[test]
    fn remove_native_missing_record_panics_after_archive_and_history_before_emitting_any_event() {
        let address = DecoratorStorageAddr::Wall(WallId(53));
        let mut storage = DecoratorStorage::default();
        storage.get_or_insert_default(address);
        let mut archive = DecoratorArchivist::<()>::default();
        let mut history = DeferredDecoratorEditHistory::default();
        let events = DeferredDecoratorOpEvents::default();
        let op = DecoratorOp::remove_at(
            DecoratorId(11),
            DecoratorInfo::default(),
            glam::Vec3::ONE,
            false,
        );
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            apply_decorator_op(
                &op,
                0,
                address,
                &mut storage,
                Some(&mut archive),
                Some(&mut history),
                &events,
            );
        }));
        assert!(result.is_err());
        assert_eq!(archive.walls.len(), 1);
        assert_eq!(history.records.len(), 1);
        assert_eq!(
            history.records[0].kind,
            DeferredDecoratorRecord::KIND_REMOVE
        );
        assert_eq!(events.with_events(|events| events.len()), 0);
    }
}
#[cfg(test)]
mod place_native_tests {
    use super::*;

    #[test]
    fn place_complete_original_converter_and_event_fields_match_native() {
        use country_core::resources::walls::WallId;
        let fixture = include_str!(concat!(env!("CARGO_MANIFEST_DIR"),
            "/../../../reports/module-recovery-2026-10-23/141276e30-place-native.txt"));
        let decode = |text: &str| -> Vec<u8> {
            (0..text.len()).step_by(2)
                .map(|offset| u8::from_str_radix(&text[offset..offset + 2], 16).unwrap()).collect()
        };
        for (index, row) in fixture.lines().enumerate() {
            let words: Vec<_> = row.split_whitespace().collect();
            let demo = words[0] == "1";
            let state = words[1].parse::<u8>().unwrap();
            let data = decode(words[2]);
            let position = decode(words[3]);
            let native_info = decode(words[4]);
            let native_event = decode(words[5]);
            let read64 = |offset| u64::from_le_bytes(data[offset..offset + 8].try_into().unwrap());
            let read32 = |offset| u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap());
            let address = match read64(0) {
                0 => DecoratorStorageAddr::Wall(WallId(read64(8))),
                1 => DecoratorStorageAddr::Terrain,
                2 => DecoratorStorageAddr::Stair,
                _ => unreachable!(),
            };
            let mut input = [0; 72];
            input[..40].copy_from_slice(&data[16..56]);
            input[40..44].copy_from_slice(&data[80..84]);
            input[48..68].copy_from_slice(&data[56..76]);
            input[68] = data[86];
            input[70] = data[85];
            // 故意给旧记录不同ID、demo字节和state，必须由真实place写面纠正。
            input[44..48].copy_from_slice(&0xdead_beef_u32.to_le_bytes());
            input[69] = 0x7b;
            input[71] = 0xe7;
            let xyz = glam::Vec3::from_array(std::array::from_fn(|lane| {
                f32::from_le_bytes(position[lane * 4..lane * 4 + 4].try_into().unwrap())
            }));
            let id = DecoratorId(read32(76));
            let op = DecoratorOp::place_at(id, bytes_to_info(&input), xyz, position[12], data[84] != 0, demo);
            let mut storage = DecoratorStorage::default();
            let mut archive = DecoratorArchivist::<()>::default();
            let mut history = DeferredDecoratorEditHistory::default();
            let events = DeferredDecoratorOpEvents::default();
            apply_decorator_op(&op, state, address, &mut storage, Some(&mut archive), Some(&mut history), &events);
            let actual = storage.get(address).unwrap().decorators[&id.0];
            assert_eq!(info_to_bytes(actual).as_slice(), native_info, "原Info case {index}");
            events.with_events(|events| {
                let event = events.last().unwrap();
                // Terrain/Stair地址没有活跃wall载荷；原生拷贝的union残留不构成业务字段。
                if let DecoratorStorageAddr::Wall(_) = address {
                    assert_eq!(&event[0x10..0x18], &native_event[0x10..0x18]);
                }
                for range in [0..0x10, 0x18..0x46, 0x70..0x7d, 0x80..0x82] {
                    assert_eq!(&event[range.clone()], &native_event[range], "原Place事件 case {index}");
                }
            });
            assert_eq!(history.records.len(), 1);
            assert_eq!(history.records[0].kind, DeferredDecoratorRecord::KIND_PLACE);
            assert!(!history.pending, "预览apply本身不得要求提交历史");
            assert!(history.potentially_surviving_affected_decorators.contains_key(&DecoratorLocator::new(address, id.0)));
        }
    }
}
