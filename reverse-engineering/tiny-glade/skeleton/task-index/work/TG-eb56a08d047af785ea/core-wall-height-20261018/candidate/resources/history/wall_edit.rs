//! B Wall compound snapshot business. Vec/Arc/ledger ownership remains std.

// Preserve the original instruction-level expression order/NaN policy, API call shapes, and evidence/test transcription. See README.md.
#![allow(clippy::neg_cmp_op_on_partial_ord)]
use crate::resources::roofs::geometry::surface::Curve2;
use crate::resources::roofs::geometry::surface::CurveError;
use crate::resources::walls::publication_fields::WallId;
use crate::resources::walls::publication_fields::WallShapeSnapshot;
use crate::resources::walls::publication_source::RawWallPublicationSource;
use crate::systems::wall::valid_enclosure::ShapeParameters;

/// Native variant 5 `MorphPremadeShape` payload (fresh 0x141464a60 /
/// `WallHistoryEdit` niche record).
///
/// Layout proven from the finish-edit producer: `{wall u64 @+0x08,
/// previous=ShapeParameters @+0x10, next=ShapeParameters @+0x2c}`. Both shapes
/// are premade records (`tag` bit 0: 0 = Circle, 1 = Rectangle); the previous
/// shape comes from the finish-event record and the next shape from the
/// `InnerWalls` source. The serde field names are `wall_index` / `before` /
/// `after` as required by the downstream schema.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct MorphPremadeShapeEdit {
    /// Native `+0x08` wall id.
    pub wall_index: u64,
    /// Native `previous` ShapeParameters (`+0x10`, 28 B).
    pub before: ShapeParameters,
    /// Native `next` ShapeParameters (`+0x2c`, 28 B).
    pub after: ShapeParameters,
}

impl MorphPremadeShapeEdit {
    pub fn new(wall: WallId, before: ShapeParameters, after: ShapeParameters) -> Self {
        Self {
            wall_index: wall.0,
            before,
            after,
        }
    }

    /// Native `+0x08` wall id.
    pub fn wall(&self) -> WallId {
        WallId(self.wall_index)
    }
}

/// Additional original source fields copied into Wall history. Their meanings
/// are not inferred; caller supplies actual observed bits, including global-
/// dependent source+69 rather than manufacturing a universal creation default.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WallHistoryMetadata {
    pub source_60_bits: u32,
    /// Original source +66 (FreehandHeightType). None is reserved for older
    /// portable inputs that did not capture it; native/core snapshots keep it.
    pub source_66: Option<u8>,
    pub source_67: u8,
    pub source_69: u8,
}
impl WallHistoryMetadata {
    /// Actual new_wall_w_params stores. The named game global is supplied by
    /// its startup owner; this function does not guess that owner's value.
    pub fn for_new_wall(dev_override_is_demo_mode: bool) -> Self {
        Self {
            source_60_bits: 0,
            source_66: Some(0),
            source_67: 0,
            source_69: if dev_override_is_demo_mode { 0 } else { 5 },
        }
    }
}
#[derive(Clone, Debug)]
pub struct WallAuthoritySnapshot {
    pub source: RawWallPublicationSource,
    pub metadata: WallHistoryMetadata,
}
/// 原生 `WallHistoryEdit`（`enum2`，0xf8B niche 编码枚举）。
///
/// fresh 反编译定案（0x14092f990 eraser finish / 0x140a84ec0 History::add /
/// 0x140a82b40 next / 0x140a83fb0 new_height_changed / 0x140a83300 ev_new_wall /
/// 0x140abfb30 WallReplayEditCmd::process）：
/// - 记录 0xf8B；判别字段在 T+0x00（u64）。`Arc<WallHistoryEdit>` 分配 0x108 =
///   0x10 Arc 头 + 0xf8 载荷。
/// - 变体 0（WallChanged）用**非 niche 值**作判别（eraser finish 的 bucket0
///   直接把 wall id 写入判别槽）；变体 1..16 用 niche 值
///   `0x8000000000000000..0x800000000000000f`。
/// - `History::add` 判别解码：`variant = disc - 0x7fffffffffffffff`（disc 在
///   niche 区 [0x8000000000000000, 0x800000000000000f] 时），否则 variant = 0。
///   `process` 同式解码得操作号后按 17 操作跳表分派。
///
/// 已确认载荷（fresh）：
/// - 变体 0 `WallChanged`：disc=wall id；`u32 @T+0xe8`（op 号）、
///   `f32 @T+0xec`（start）、`f32 @T+0xf0`（end）。本型 `StateChange` 的
///   `observed_operation: {u32, f32, f32}` 即该三字段（add 的
///   `Slice start_u must be smaller than end_u` 校验面）。
/// - 变体 1 `CreateWall` / 变体 2 `DeleteWall`：disc=`0x8000000000000000/1`；
///   `Option<source> @T+0x08`（None → add panic "None is not allowed in
///   CreateWall/DeleteWall..."）。
/// - 变体 3 `AdjustHeight`：disc=`0x8000000000000002`；`wall_id u64 @T+0x08`、
///   `from f32 @T+0x10`、`to f32 @T+0x14`（`new_height_changed` 0x140a83fb0 +
///   `write_observed_arc_fields`）。
/// - 变体 4 `ChangeIsEnclosedState`：disc=`0x8000000000000003`（**旧注释
///   0x8000000000000004 差一错，fresh 修正**）。
/// - 变体 5..16：disc=`0x8000000000000004..0x800000000000000f`；载荷未恢复
///   （**登记缺类型**：见 `WallHistoryOperationTag` + 布局报告）。不编造字段，
///   以 `UnresolvedWallEditPayload` 显式标记。
#[derive(Clone, Debug)]
pub enum WallHistoryEdit {
    /// 原生变体 0 `WallChanged`（disc = wall id，非 niche）。
    StateChange {
        wall: WallId,
        previous: WallAuthoritySnapshot,
        next: WallAuthoritySnapshot,
        /// 原生 `{u32 op @T+0xe8, f32 start @T+0xec, f32 end @T+0xf0}`；
        /// 由实际 replay 消费方解释，不按枚举名猜测。
        observed_operation: [u8; 12],
    },
    /// 原生变体 1 `CreateWall`，disc = `0x8000000000000000`。
    Create {
        wall: WallId,
        source: WallAuthoritySnapshot,
        color_id: u32,
        restored_height: f32,
        restored_elevation: f32,
    },
    /// 原生变体 2 `DeleteWall`，disc = `0x8000000000000001`。
    Delete {
        wall: WallId,
        source: WallAuthoritySnapshot,
        color_id: u32,
        /// Native T+0x80, distinct from the captured source height. Delete's
        /// Some-source undo preserves that source; only native None fallback
        /// uses this height to construct an empty source (140ac1322).
        restored_height: f32,
    },
    /// 原生变体 3 `AdjustHeight`，disc = `0x8000000000000002`。
    AdjustHeight { wall: WallId, from: f32, to: f32 },
    /// 原生变体 4 `ChangeIsEnclosedState`，disc = `0x8000000000000003`。
    EnclosureChange {
        wall: WallId,
        previous: WallShapeSnapshot,
        next: WallShapeSnapshot,
    },
    /// 原生变体 5 `MorphPremadeShape`，disc = `0x8000000000000004`；
    /// 载荷 = [`MorphPremadeShapeEdit`]（fresh 2026-10-17）。
    MorphPremadeShape(MorphPremadeShapeEdit),
    /// 原生变体 6 `ChangeElevation`，disc = `0x8000000000000005`；载荷未恢复。
    ChangeElevation(UnresolvedWallEditPayload),
    /// 原生变体 7 `ChangeArchHeight`，disc = `0x8000000000000006`；载荷未恢复。
    ChangeArchHeight(UnresolvedWallEditPayload),
    /// 原生变体 8 `ChangeSupports`，disc = `0x8000000000000007`；载荷未恢复。
    ChangeSupports(UnresolvedWallEditPayload),
    /// 原生变体 9 `ChangeStairsType`，disc = `0x8000000000000008`；载荷未恢复。
    ChangeStairsType(UnresolvedWallEditPayload),
    /// 原生变体 10 `ChangePlasterPeelAmount`，disc = `0x8000000000000009`；载荷未恢复。
    ChangePlasterPeelAmount(UnresolvedWallEditPayload),
    /// 原生变体 11 `ChangeFreehandHeightType`，disc = `0x800000000000000a`；载荷未恢复。
    ChangeFreehandHeightType(UnresolvedWallEditPayload),
    /// 原生变体 12 `ChangeArchType`，disc = `0x800000000000000b`；载荷未恢复。
    ChangeArchType(UnresolvedWallEditPayload),
    /// 原生变体 13 `MorphFreehandWall`，disc = `0x800000000000000c`；载荷未恢复。
    MorphFreehandWall(UnresolvedWallEditPayload),
    /// 原生变体 14 `DeprecatedMorphOriginal`，disc = `0x800000000000000d`；载荷未恢复。
    DeprecatedMorphOriginal(UnresolvedWallEditPayload),
    /// 原生变体 15（名未解），disc = `0x800000000000000e`；载荷未恢复。
    Unresolved15(UnresolvedWallEditPayload),
    /// 原生变体 16 `ChangeSkirt`，disc = `0x800000000000000f`；载荷未恢复。
    ChangeSkirt(UnresolvedWallEditPayload),
}

impl WallHistoryEdit {
    /// 原生判别槽（T+0x00）值。变体 0 为非 niche（本型 `StateChange` 的 wall id，
    /// 即 eraser bucket0 直接写入的 OriginalWallId）；变体 1..16 为 niche 值。
    pub fn native_discriminant(&self) -> u64 {
        match self {
            WallHistoryEdit::StateChange { wall, .. } => wall.0,
            WallHistoryEdit::Create { .. } => 0x8000_0000_0000_0000,
            WallHistoryEdit::Delete { .. } => 0x8000_0000_0000_0001,
            WallHistoryEdit::AdjustHeight { .. } => 0x8000_0000_0000_0002,
            WallHistoryEdit::EnclosureChange { .. } => 0x8000_0000_0000_0003,
            WallHistoryEdit::MorphPremadeShape(..) => 0x8000_0000_0000_0004,
            WallHistoryEdit::ChangeElevation(..) => 0x8000_0000_0000_0005,
            WallHistoryEdit::ChangeArchHeight(..) => 0x8000_0000_0000_0006,
            WallHistoryEdit::ChangeSupports(..) => 0x8000_0000_0000_0007,
            WallHistoryEdit::ChangeStairsType(..) => 0x8000_0000_0000_0008,
            WallHistoryEdit::ChangePlasterPeelAmount(..) => 0x8000_0000_0000_0009,
            WallHistoryEdit::ChangeFreehandHeightType(..) => 0x8000_0000_0000_000a,
            WallHistoryEdit::ChangeArchType(..) => 0x8000_0000_0000_000b,
            WallHistoryEdit::MorphFreehandWall(..) => 0x8000_0000_0000_000c,
            WallHistoryEdit::DeprecatedMorphOriginal(..) => 0x8000_0000_0000_000d,
            WallHistoryEdit::Unresolved15(..) => 0x8000_0000_0000_000e,
            WallHistoryEdit::ChangeSkirt(..) => 0x8000_0000_0000_000f,
        }
    }

    /// 原生 `History::add` 判别解码：`disc - 0x7fffffffffffffff`（niche 区）否则 0。
    /// 结果即 `WallHistoryOperationTag` 的操作号。守卫为有符号比较
    /// `disc > -0x7ffffffffffffff1`（= `0x800000000000000f`）→ 变体 0。
    pub fn operation_tag(&self) -> WallHistoryOperationTag {
        let disc = self.native_discriminant();
        let variant = disc.wrapping_sub(0x7fff_ffff_ffff_ffff);
        if (disc as i64) > -0x7fff_ffff_ffff_fff1i64 {
            WallHistoryOperationTag::WallChanged
        } else {
            WallHistoryOperationTag::from_variant_number(variant)
        }
    }

    /// 已恢复载荷变体的 wall id；未恢复变体（5..16）返回 None（登记缺类型）。
    pub fn wall_id(&self) -> Option<WallId> {
        match self {
            WallHistoryEdit::StateChange { wall, .. }
            | WallHistoryEdit::Create { wall, .. }
            | WallHistoryEdit::Delete { wall, .. }
            | WallHistoryEdit::AdjustHeight { wall, .. }
            | WallHistoryEdit::EnclosureChange { wall, .. } => Some(*wall),
            WallHistoryEdit::MorphPremadeShape(payload) => Some(payload.wall()),
            WallHistoryEdit::ChangeElevation(..)
            | WallHistoryEdit::ChangeArchHeight(..)
            | WallHistoryEdit::ChangeSupports(..)
            | WallHistoryEdit::ChangeStairsType(..)
            | WallHistoryEdit::ChangePlasterPeelAmount(..)
            | WallHistoryEdit::ChangeFreehandHeightType(..)
            | WallHistoryEdit::ChangeArchType(..)
            | WallHistoryEdit::MorphFreehandWall(..)
            | WallHistoryEdit::DeprecatedMorphOriginal(..)
            | WallHistoryEdit::Unresolved15(..)
            | WallHistoryEdit::ChangeSkirt(..) => None,
        }
    }
}

/// 原生变体 5..16 的载荷未恢复（fresh 仅确认判别值 + 操作号，字段语义待布局
/// 取证）。以单元标记显式登记缺类型，不编造字段。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnresolvedWallEditPayload;
/// 原生 `ConcurrentEdits`：**单个并发编辑组**（同索引合并为一组、插入序保留）。
/// `MultiEdit.records = Vec<ConcurrentEdits>`（0x10B 元素 = Box<[Arc<WallHistoryEdit>]>，
/// 见 `History::add` 0x140a84ec0 逐组迭代 + `build`/eraser finish 的
/// `from_iter<ConcurrentEdits, Map<IntoIter<Vec<Arc<WallHistoryEdit>>>, new::closure_env_0>>`）。
/// 组内编辑共享 Arc（同一编辑可出现在多个并发组）。Rust 侧以 `Vec<WallHistoryEdit>`
/// 为等价建模（省去组内 Arc 共享，消费方逐组迭代语义不变）。
pub type ConcurrentEdits = Vec<WallHistoryEdit>;

#[derive(Clone, Debug)]
pub struct WallCompoundHistoryEdit {
    /// Original ordered ConcurrentEdits groups; do not flatten the groups.
    pub groups: Vec<ConcurrentEdits>,
}

/// `CompoundEditBuilder`（0x140a82b20 new_edit_sequence / 0x140a82b40 next /
/// 0x140a82d50 build）：增量构建 `WallCompoundHistoryEdit` 的组序列
/// （card R3；章节 wall_compound_edit.rs.md O）。
///
/// fresh 反编译（2026-10-17）：`new_edit_sequence` 只写 Vec {ptr=0, cap=8, len=0}
/// **（无 started 标志位）**；`next` alloc 0xf8 + memcpy **单条 edit** → grow_one →
/// push 新组（**无 is_empty 门**）；`build` 直接 `from_iter` 收集（**无 None/空判据**）。
/// Rust 此前引入的 `started` 标志、`next` 空门与 `build` 的 None 分支在机器码中无对应。
#[derive(Clone, Debug, Default)]
pub struct CompoundEditBuilder {
    /// 组序列 Vec（builder 本体；原生 `new_edit_sequence` 预容量 8）。
    groups: Vec<Vec<WallHistoryEdit>>,
}
impl CompoundEditBuilder {
    /// `new_edit_sequence` 0x140a82b20：重置为空组序列（预容量 8）。
    pub fn new_edit_sequence(&mut self) {
        self.groups = Vec::with_capacity(8);
    }
    /// `next` 0x140a82b40：把单条 edit 作为**新组** push 进组序列（无 is_empty 门）。
    pub fn next(&mut self, edit: WallHistoryEdit) {
        self.groups.push(vec![edit]);
    }
    /// `build` 0x140a82d50：直接收集为 `WallCompoundHistoryEdit`（无 None/空判据）。
    pub fn build(self) -> WallCompoundHistoryEdit {
        WallCompoundHistoryEdit {
            groups: self.groups,
        }
    }
}

/// `panic_deprecated_operation` 0x140a82e20：已弃用操作 panic
/// （章节 wall_compound_edit.rs.md；原文案待取证）。
pub fn panic_deprecated_operation() -> ! {
    panic!("attempted to use a deprecated wall history operation")
}

/// WallHistoryEdit 操作变体 tag→名（E1 RE-EDITTYPE.md §2，visit_str 0x1414022c0 O）。
/// 判别 = 首字段 niche，与顶层 EditType 判别不共用。此 17 项即原生
/// `WallHistoryEdit` 的 17 变体（`History::add` 0x140a84ec0 与
/// `WallReplayEditCmd::process` 0x140abfb30 同式 niche 解码）。变体号 = 操作号。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u64)]
pub enum WallHistoryOperationTag {
    WallChanged = 0,
    CreateWall = 1,
    DeleteWall = 2,
    AdjustHeight = 3,
    ChangeIsEnclosedState = 4,
    MorphPremadeShape = 5,
    ChangeElevation = 6,
    ChangeArchHeight = 7,
    ChangeSupports = 8,
    ChangeStairsType = 9,
    ChangePlasterPeelAmount = 10,
    ChangeFreehandHeightType = 11,
    ChangeArchType = 12,
    MorphFreehandWall = 13,
    DeprecatedMorphOriginal = 14,
    /// tag 15 名未解（anon_95，13 字符）。
    Unresolved15 = 15,
    ChangeSkirt = 16,
}
impl WallHistoryOperationTag {
    /// 原生 niche 判别值：变体 N（N≥1）= `0x8000000000000000 + (N-1)`；
    /// 变体 0 无 niche（判别槽为数据值，如 wall id）。
    pub fn native_discriminant(self) -> u64 {
        match self {
            WallHistoryOperationTag::WallChanged => 0,
            WallHistoryOperationTag::CreateWall => 0x8000_0000_0000_0000,
            WallHistoryOperationTag::DeleteWall => 0x8000_0000_0000_0001,
            WallHistoryOperationTag::AdjustHeight => 0x8000_0000_0000_0002,
            WallHistoryOperationTag::ChangeIsEnclosedState => 0x8000_0000_0000_0003,
            WallHistoryOperationTag::MorphPremadeShape => 0x8000_0000_0000_0004,
            WallHistoryOperationTag::ChangeElevation => 0x8000_0000_0000_0005,
            WallHistoryOperationTag::ChangeArchHeight => 0x8000_0000_0000_0006,
            WallHistoryOperationTag::ChangeSupports => 0x8000_0000_0000_0007,
            WallHistoryOperationTag::ChangeStairsType => 0x8000_0000_0000_0008,
            WallHistoryOperationTag::ChangePlasterPeelAmount => 0x8000_0000_0000_0009,
            WallHistoryOperationTag::ChangeFreehandHeightType => 0x8000_0000_0000_000a,
            WallHistoryOperationTag::ChangeArchType => 0x8000_0000_0000_000b,
            WallHistoryOperationTag::MorphFreehandWall => 0x8000_0000_0000_000c,
            WallHistoryOperationTag::DeprecatedMorphOriginal => 0x8000_0000_0000_000d,
            WallHistoryOperationTag::Unresolved15 => 0x8000_0000_0000_000e,
            WallHistoryOperationTag::ChangeSkirt => 0x8000_0000_0000_000f,
        }
    }
    /// 原生 `History::add` 解码得到的变体号（1..=16 niche，0 = WallChanged）。
    pub fn from_variant_number(variant: u64) -> WallHistoryOperationTag {
        match variant {
            1 => WallHistoryOperationTag::CreateWall,
            2 => WallHistoryOperationTag::DeleteWall,
            3 => WallHistoryOperationTag::AdjustHeight,
            4 => WallHistoryOperationTag::ChangeIsEnclosedState,
            5 => WallHistoryOperationTag::MorphPremadeShape,
            6 => WallHistoryOperationTag::ChangeElevation,
            7 => WallHistoryOperationTag::ChangeArchHeight,
            8 => WallHistoryOperationTag::ChangeSupports,
            9 => WallHistoryOperationTag::ChangeStairsType,
            10 => WallHistoryOperationTag::ChangePlasterPeelAmount,
            11 => WallHistoryOperationTag::ChangeFreehandHeightType,
            12 => WallHistoryOperationTag::ChangeArchType,
            13 => WallHistoryOperationTag::MorphFreehandWall,
            14 => WallHistoryOperationTag::DeprecatedMorphOriginal,
            15 => WallHistoryOperationTag::Unresolved15,
            16 => WallHistoryOperationTag::ChangeSkirt,
            _ => WallHistoryOperationTag::WallChanged,
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub enum WallSnapshotError {
    InvalidSlice,
    Curve(CurveError),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WallReplaySourceError {
    MissingSource,
    DuplicateSource,
    EnclosureCommandRequired,
    /// 原生变体载荷未恢复，即时 authority 写入面不承接（操作号 = 判别变体号）。
    UnrestoredOperation(u64),
}
#[derive(Clone, Debug)]
pub struct WallRestoreColorRequest {
    pub wall: WallId,
    pub color_id: u32,
    pub observed_special_owner: bool,
}
#[derive(Clone, Debug)]
pub struct WallReplayAudio {
    pub wall: WallId,
    pub anchor: [f32; 3],
    pub kind: u8,
    pub flag: u8,
}
#[derive(Clone, Debug)]
pub struct InstantWallReplayOutcome {
    pub new_source_notification: Option<WallId>,
    pub updated_source_notification: Option<WallId>,
    pub removed_source_notification: Option<WallId>,
    pub color_request: Option<WallRestoreColorRequest>,
    pub audio: Option<WallReplayAudio>,
}

pub fn record_deleted_wall(
    wall: WallId,
    source: &WallAuthoritySnapshot,
    color_id: u32,
) -> WallCompoundHistoryEdit {
    record_deleted_wall_with_height(wall, source, color_id, source.source.extra)
}

/// B finish_wall(false) retains both the previous snapshot and the independent
/// target height (T+0x80). Do not overwrite the captured snapshot to store it.
pub fn record_deleted_wall_with_height(
    wall: WallId,
    source: &WallAuthoritySnapshot,
    color_id: u32,
    restored_height: f32,
) -> WallCompoundHistoryEdit {
    WallCompoundHistoryEdit {
        groups: vec![vec![WallHistoryEdit::Delete {
            wall,
            source: source.clone(),
            color_id,
            restored_height,
        }]],
    }
}

/// Actual instant Create/Delete/StateChange authority writes. The caller keeps
/// its ID counter; original new_wall_with_id never increments that counter.
/// Missing remove/duplicate restore are original panic domains represented as
/// typed errors, without claiming their partially mutated std-map semantics.
pub fn apply_instant_wall_edit(
    sources: &mut std::collections::HashMap<WallId, WallAuthoritySnapshot>,
    edit: &WallHistoryEdit,
    undo: bool,
    terrain: &crate::resources::walls::publication_source::TerrainHeightGrid512,
) -> Result<InstantWallReplayOutcome, WallReplaySourceError> {
    let mut out = InstantWallReplayOutcome {
        new_source_notification: None,
        updated_source_notification: None,
        removed_source_notification: None,
        color_request: None,
        audio: None,
    };
    let (wall, restore, source, color) = match edit {
        WallHistoryEdit::Create {
            wall,
            source,
            color_id,
            ..
        } => (*wall, !undo, source, *color_id),
        WallHistoryEdit::Delete {
            wall,
            source,
            color_id,
            ..
        } => (*wall, undo, source, *color_id),
        // Enclosure writes emit two updated notifications. Use the command
        // consumer, whose Vec preserves that original multiplicity.
        WallHistoryEdit::EnclosureChange { .. } => {
            return Err(WallReplaySourceError::EnclosureCommandRequired);
        }
        WallHistoryEdit::StateChange {
            wall,
            previous,
            next,
            ..
        } => {
            let selected = if undo { previous } else { next };
            if let Some(target) = sources.get_mut(wall) {
                *target = selected.clone();
                out.updated_source_notification = Some(*wall);
            }
            let [x, z] = selected.source.curve.pos_at_u(0.5);
            out.audio = Some(WallReplayAudio {
                wall: *wall,
                anchor: [x, terrain.sample_at_world_xz(x, z), z],
                kind: 2,
                flag: 1,
            });
            return Ok(out);
        }
        // AdjustHeight 的即时面由 height 权威（WallHeightReplayRequest）消费，
        // 不经 authority 快照写入。其余未恢复载荷变体不在此即时写入面。
        WallHistoryEdit::AdjustHeight { .. }
        | WallHistoryEdit::MorphPremadeShape(..)
        | WallHistoryEdit::ChangeElevation(..)
        | WallHistoryEdit::ChangeArchHeight(..)
        | WallHistoryEdit::ChangeSupports(..)
        | WallHistoryEdit::ChangeStairsType(..)
        | WallHistoryEdit::ChangePlasterPeelAmount(..)
        | WallHistoryEdit::ChangeFreehandHeightType(..)
        | WallHistoryEdit::ChangeArchType(..)
        | WallHistoryEdit::MorphFreehandWall(..)
        | WallHistoryEdit::DeprecatedMorphOriginal(..)
        | WallHistoryEdit::Unresolved15(..)
        | WallHistoryEdit::ChangeSkirt(..) => {
            return Err(WallReplaySourceError::UnrestoredOperation(
                edit.operation_tag() as u64,
            ));
        }
    };
    if restore {
        if sources.contains_key(&wall) {
            return Err(WallReplaySourceError::DuplicateSource);
        }
        let mut restored = source.clone();
        // Delete Some-source restores the original height (140ac12be/14c0).
        // Its distinct restored_height is retained for native None fallback,
        // which the current mandatory-source model does not represent.
        if let WallHistoryEdit::Create {
            restored_height,
            restored_elevation,
            ..
        } = edit
        {
            restored.source.extra = *restored_height;
            restored.source.terrain_floor = *restored_elevation;
            out.updated_source_notification = Some(wall);
        }
        out.color_request = Some(WallRestoreColorRequest {
            wall,
            color_id: color,
            observed_special_owner: source.source.special,
        });
        sources.insert(wall, restored);
        out.new_source_notification = Some(wall);
    } else {
        sources
            .remove(&wall)
            .ok_or(WallReplaySourceError::MissingSource)?;
        out.removed_source_notification = Some(wall);
    }
    Ok(out)
}

/// Actual private Curve::crashy_slice rule; coordinate search/interpolation and
/// Curve recomputation reuse the existing recovered Curve2 APIs.
pub fn slice_source_curve(
    curve: &Curve2,
    start: f32,
    end: f32,
) -> Result<Curve2, WallSnapshotError> {
    if !start.is_finite() || !end.is_finite() || end <= start {
        return Err(WallSnapshotError::InvalidSlice);
    }
    let (first, _) = curve.coord_at_u(start);
    let (last, _) = curve.coord_at_u(end);
    let a = curve.pos_at_u(start);
    let b = curve.pos_at_u(end);
    let first_inside = first + if a == curve.points[first + 1] { 2 } else { 1 };
    let last_inside = if b == curve.points[last] {
        last.saturating_sub(1)
    } else {
        last
    };
    let mut points = vec![a];
    if first_inside <= last_inside {
        points.extend_from_slice(&curve.points[first_inside..=last_inside]);
    }
    points.push(b);
    Curve2::try_new(points, false).map_err(WallSnapshotError::Curve)
}

/// Actual ev_new_wall: seed-create group, then seed->complete-state group.
/// Geometry slice occurs before all cloning/recording, as in the original.
pub fn record_new_wall(
    wall: WallId,
    source: &WallAuthoritySnapshot,
    color_id: u32,
) -> Result<WallCompoundHistoryEdit, WallSnapshotError> {
    let ratio = 0.2 / source.source.curve.length;
    let end = if ratio < 0.0 { 0.0 } else { ratio };
    let end = if end > 1.0 { 1.0 } else { end };
    let curve = slice_source_curve(&source.source.curve, 0.0, end)?;
    let mut seed = source.clone();
    seed.source.curve = curve;
    seed.source.shape = WallShapeSnapshot::from_observed({
        let mut bytes = [0; 28];
        bytes[..4].copy_from_slice(&2u32.to_le_bytes());
        bytes
    });
    seed.source.curve_kind_tag = 2;
    let mut observed_operation = [0; 12];
    observed_operation[8..12].copy_from_slice(&end.to_le_bytes());
    Ok(WallCompoundHistoryEdit {
        groups: vec![
            vec![WallHistoryEdit::Create {
                wall,
                source: seed.clone(),
                color_id,
                restored_height: source.source.extra,
                restored_elevation: source.source.terrain_floor,
            }],
            vec![WallHistoryEdit::StateChange {
                wall,
                previous: seed,
                next: source.clone(),
                observed_operation,
            }],
        ],
    })
}

/// Rectangle2d::lerp business composition. The original angle approximation,
/// normalized dot and signed cross are glam Vec2::angle_to, reused as a dependency.
/// Other enclosure variants have not been recovered by this producer yet.
pub fn interpolate_rectangle_enclosure(
    previous: &WallShapeSnapshot,
    next: &WallShapeSnapshot,
    progress: f32,
) -> Result<WallShapeSnapshot, crate::resources::walls::freeform_height_cache::ShapeHeightError> {
    use crate::resources::walls::freeform_height_cache::ShapeHeightError;
    use glam::Vec2;
    if previous.tag() != 1 || next.tag() != 1 {
        return Err(ShapeHeightError::InvalidRectangle);
    }
    let read = |s: &WallShapeSnapshot, at: usize| {
        Vec2::new(
            f32::from_le_bytes(s.bytes()[at..at + 4].try_into().unwrap()),
            f32::from_le_bytes(s.bytes()[at + 4..at + 8].try_into().unwrap()),
        )
    };
    let old_axes = read(previous, 4);
    let next_axes = read(next, 4);
    let angle = old_axes.angle_to(next_axes) * progress;
    let (sin, cos) = angle.sin_cos();
    // Preserve the original SSE operand order rather than Vec2::rotate's
    // equivalent subtraction expression (signed-zero bits can differ).
    let axes = Vec2::new(
        (-sin) * old_axes.y + cos * old_axes.x,
        cos * old_axes.y + sin * old_axes.x,
    );
    let center = read(previous, 12) * (1.0 - progress) + read(next, 12) * progress;
    let dimensions = read(previous, 20) * (1.0 - progress) + read(next, 20) * progress;
    if !dimensions.is_finite()
        || !((axes.length_squared() - 1.0).abs() <= f32::from_bits(0x3951b717))
    {
        return Err(ShapeHeightError::InvalidRectangle);
    }
    let mut bytes = [0; 28];
    bytes[..4].copy_from_slice(&1u32.to_le_bytes());
    for (index, value) in [
        axes.x,
        axes.y,
        center.x,
        center.y,
        dimensions.x,
        dimensions.y,
    ]
    .into_iter()
    .enumerate()
    {
        bytes[4 + index * 4..8 + index * 4].copy_from_slice(&value.to_le_bytes());
    }
    Ok(WallShapeSnapshot::from_observed(bytes))
}

pub fn rectangle_from_enclosure(
    shape: &WallShapeSnapshot,
) -> Result<
    crate::resources::walls::freeform_height_cache::PreviewRectangleObserved,
    crate::resources::walls::freeform_height_cache::ShapeHeightError,
> {
    if shape.tag() != 1 {
        return Err(
            crate::resources::walls::freeform_height_cache::ShapeHeightError::InvalidRectangle,
        );
    }
    let value = |at| f32::from_le_bytes(shape.bytes()[at..at + 4].try_into().unwrap());
    Ok(
        crate::resources::walls::freeform_height_cache::PreviewRectangleObserved {
            axes: [value(4), value(8)],
            center: [value(12), value(16)],
            dimensions: [value(20), value(24)],
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 原生 0xf8B 记录 + 0x10 Arc 头 = 0x108 分配（`next` 0x140a82b40 /
    /// ev_new_wall 0x140a83300 的 `memcpy(dst, &arc_header, 0x108)`）。
    /// 断言上限（Arc 头 + 载荷 ≤ 0x108），不伪造字节相等。
    #[test]
    fn arc_allocation_holds_header_plus_f8_payload() {
        assert!(std::mem::size_of::<WallHistoryEdit>() + 0x10 <= 0x108);
    }

    /// `native_discriminant`（T+0x00）逐变体：变体 0 非 niche（wall id），
    /// 变体 1..16 = `0x8000000000000000..0x800000000000000f`。
    #[test]
    fn niche_discriminants_match_fresh() {
        // 变体 1 CreateWall = 0x8000000000000000。
        assert_eq!(
            WallHistoryOperationTag::CreateWall.native_discriminant(),
            0x8000_0000_0000_0000,
        );
        assert_eq!(
            WallHistoryOperationTag::DeleteWall.native_discriminant(),
            0x8000_0000_0000_0001,
        );
        assert_eq!(
            WallHistoryOperationTag::AdjustHeight.native_discriminant(),
            0x8000_0000_0000_0002,
        );
        // 变体 4 ChangeIsEnclosedState = 0x8000000000000003（旧注释 4 差一错，fresh 修正）。
        assert_eq!(
            WallHistoryOperationTag::ChangeIsEnclosedState.native_discriminant(),
            0x8000_0000_0000_0003,
        );
        assert_eq!(
            WallHistoryOperationTag::MorphPremadeShape.native_discriminant(),
            0x8000_0000_0000_0004,
        );
        assert_eq!(
            WallHistoryOperationTag::ChangeSkirt.native_discriminant(),
            0x8000_0000_0000_000f,
        );
    }

    /// `History::add`（0x140a84ec0）判别解码：`disc - 0x7fffffffffffffff`
    /// （niche 区）否则 0。逐值核对该判等。
    #[test]
    fn history_add_discriminant_decode_matches_fresh() {
        let decode = |disc: u64| {
            let lvar7 = disc.wrapping_sub(0x7fff_ffff_ffff_ffff);
            if (disc as i64) > -0x7fff_ffff_ffff_fff1i64 {
                0
            } else {
                lvar7
            }
        };
        // 非 niche（wall id）→ 变体 0。
        assert_eq!(decode(9), 0);
        assert_eq!(decode(0x8000_0000_0000_0010), 0);
        // niche 区 → 变体 1..16。
        assert_eq!(decode(0x8000_0000_0000_0000), 1);
        assert_eq!(decode(0x8000_0000_0000_0001), 2);
        assert_eq!(decode(0x8000_0000_0000_0002), 3);
        assert_eq!(decode(0x8000_0000_0000_000f), 16);
    }

    /// ConcurrentEdits = 单个并发编辑组；`WallCompoundHistoryEdit.groups`
    /// 逐组迭代、插入序保留（原生 5 桶→from_iter 收集语义）。
    #[test]
    fn concurrent_edits_groups_preserve_insertion_order() {
        let compound = WallCompoundHistoryEdit {
            groups: vec![
                vec![WallHistoryEdit::ChangeSkirt(UnresolvedWallEditPayload)],
                vec![WallHistoryEdit::ChangeSkirt(UnresolvedWallEditPayload)],
            ],
        };
        let ops: Vec<_> = compound
            .groups
            .iter()
            .map(|g| {
                g.iter()
                    .map(WallHistoryEdit::operation_tag)
                    .collect::<Vec<_>>()
            })
            .collect();
        assert_eq!(
            ops,
            vec![
                vec![WallHistoryOperationTag::ChangeSkirt],
                vec![WallHistoryOperationTag::ChangeSkirt],
            ]
        );
    }

    /// fresh 0x141464a60 payload `{wall @+8, previous @+0x10, next @+0x2c}`
    /// exposed under the required serde schema `wall_index` / `before` /
    /// `after`. The wire uses the named Circle/Rectangle variants, as in the
    /// shipped gate fixture; native tag bytes are not JSON fields.
    #[test]
    fn morph_premade_shape_payload_uses_wall_index_before_after_schema() {
        let mut before = [0u8; 28];
        before[..4].copy_from_slice(&1u32.to_le_bytes());
        before[4..8].copy_from_slice(&1.0_f32.to_bits().to_le_bytes());
        before[20..24].copy_from_slice(&2.0_f32.to_bits().to_le_bytes());
        before[24..28].copy_from_slice(&3.0_f32.to_bits().to_le_bytes());
        let mut after = [0u8; 28];
        after[..4].copy_from_slice(&0u32.to_le_bytes());
        after[12..16].copy_from_slice(&1.0_f32.to_bits().to_le_bytes());
        let payload = MorphPremadeShapeEdit::new(
            WallId(9),
            ShapeParameters::from_observed(before),
            ShapeParameters::from_observed(after),
        );
        assert_eq!(payload.wall(), WallId(9));
        let json = serde_json::to_value(payload).unwrap();
        assert_eq!(json["wall_index"], 9);
        assert!(json["before"]["Rectangle"].is_object());
        assert!(json["after"]["Circle"].is_object());
        let back: MorphPremadeShapeEdit = serde_json::from_value(json).unwrap();
        assert_eq!(back, payload);
        assert_eq!(
            WallHistoryEdit::MorphPremadeShape(payload).wall_id(),
            Some(WallId(9))
        );
    }
    #[test]
    fn delete_retains_independent_height_but_some_source_undo_restores_snapshot() {
        let curve = Curve2::try_new(vec![[0.0, 0.0], [4.0, 0.0]], false).unwrap();
        let state = crate::resources::walls::WallState::new(curve, 3.9, 3, 0, false);
        let source = state.snapshot();
        let terrain = crate::resources::walls::publication_source::TerrainHeightGrid512 {
            encoded: vec![0; 512 * 512].into(),
            minimum_y: 0.0,
            maximum_y: 4.0,
            sampling_fix: true,
        };
        for height in [7.0, -0.0, f32::from_bits(0x7fc12345)] {
            let compound = record_deleted_wall_with_height(WallId(9), source, 17, height);
            let edit = &compound.groups[0][0];
            let WallHistoryEdit::Delete {
                source: captured,
                restored_height,
                color_id,
                ..
            } = edit
            else {
                panic!("delete record");
            };
            assert_eq!(restored_height.to_bits(), height.to_bits());
            assert_eq!(captured.source.extra.to_bits(), 3.9_f32.to_bits());
            assert_eq!(*color_id, 17);
            let mut sources = std::collections::HashMap::new();
            let out = apply_instant_wall_edit(&mut sources, edit, true, &terrain).unwrap();
            assert_eq!(out.new_source_notification, Some(WallId(9)));
            assert_eq!(
                sources[&WallId(9)].source.extra.to_bits(),
                3.9_f32.to_bits()
            );
            apply_instant_wall_edit(&mut sources, edit, false, &terrain).unwrap();
            assert!(sources.is_empty());
        }
        let compound = record_deleted_wall(WallId(9), source, 17);
        let WallHistoryEdit::Delete {
            restored_height, ..
        } = &compound.groups[0][0]
        else {
            panic!("delete record");
        };
        assert_eq!(restored_height.to_bits(), source.source.extra.to_bits());
    }
}
