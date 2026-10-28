//! Live history uses the recovered transaction owner directly. Edit tags are
//! verified against original B Debug dispatch 0x140a842c0 / 0x142ad88d0.
//! Variants whose payloads remain unresolved are not represented by empty data.
pub mod auto_clutter_edit;
pub mod color_edit;
pub mod decorator_edit;
pub mod path_edit;
pub mod replay_cmd_buffer;
pub mod serialized;
pub mod stairs_history_edit;
pub mod tree_edit;
pub mod terrain_edit;

pub use crate::resources::history::building_transactions::RoofHistoryEdit;
use crate::resources::history::transactions::HistoryRecordingValidation;
use crate::resources::history::transactions::HistoryTransactions;
use crate::resources::history::transactions::ObservedHistoryLedger;
use crate::resources::history::wall_edit::WallCompoundHistoryEdit;
use crate::resources::raster_stroke::RasterStroke;
use crate::systems::terrain::deferred_terrain_history::TerrainEdit;
use std::sync::Arc;

pub use replay_cmd_buffer::{HistoryReplayError, ReplayBucket, ReplayBuffer, ReplayCommand};
pub type History<S = ()> = HistoryTransactions<EditType, S>;

impl<S> HistoryTransactions<EditType, S> {
    /// 新鲜空地（无编辑/无快照/无锚点/无 nonce）的 History 账本，供
    /// `register_core_startup` 注册 `History<S>` 资源（宿主 load 到观测账本后
    /// 以 `from_observed` 覆盖重装）。`cursor = -1` 是原生空账本状态：
    /// undo 守卫 `cursor < 0` 直接 None，redo 目标 = 0 = 首条编辑。不做猜测的
    /// 非零启动 cursor/anchor/nonce——全部空值显式写出。
    pub fn empty_glade() -> Self {
        Self::from_observed(ObservedHistoryLedger {
            entries: Vec::new(),
            cursor: -1,
            undo_anchor: -1,
            resolved_snapshots: Vec::new(),
            observed_count_58: 0,
            mutation_nonce: 0,
        })
    }

    /// `History::debug_display`（O：0x140a84ba0，154 B）——调试工具，非业务主干。
    ///
    /// fresh：三个格式片段（read_memory 0x142ad80d0）为 `"History: "` / `"/"` /
    /// `". Start at "`，三个参数依次是 `&History+0x68`（`history_chain::impl$2::fmt`，
    /// 读数 `chain+0x30`）、`*(i32*)(History+0x10) - 1`（数字 Display）、
    /// `&History+0x6c`（同一 chain formatter）。本 crate 以 `debug_chain` /
    /// `debug_history_start` 表出 +0x68/+0x6c 的显示值，cursor 取 ledger。
    pub fn debug_display(&self) -> String {
        format!(
            "History: {}/{}. Start at {}",
            self.debug_chain,
            self.ledger().cursor - 1,
            self.debug_history_start
        )
    }

    /// `History::debug_set_history_start`（O：0x140a84c40，4 B）：`*(u32*)(self+0x6c) = arg`。
    pub fn debug_set_history_start(&mut self, start: u32) {
        self.debug_history_start = start;
    }
}

/// B `HistoryIdentity`（u64 新类型；Identity 存档节载荷 = 裸 u64 十进制数字，
/// `to_json_string<HistoryIdentity>` 0x140b385e0）。`generate` 0x140a84220 = fastrand u64。
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
#[serde(transparent)]
pub struct HistoryIdentity(pub u64);
impl HistoryIdentity {
    /// `generate` 0x140a84220：fastrand u64 全范围。
    pub fn generate() -> Self {
        Self(fastrand::u64(..))
    }
    /// `mutate`（行 102 内联）：重随机。
    pub fn mutate(&mut self) {
        self.0 = fastrand::u64(..);
    }
}

#[derive(Clone, Debug)]
pub enum EditType {
    /// tag 0（① T3 权威 visit_str 0x140b3b630 case7："Terrain" 7 字符 → tag 0）。
    /// B deferred recorder 0x140ac9770 transfers the typed New/Edit/Move
    /// payload into this owner. Original serializer 0x140316660 and visitor
    /// 0x140b3b630 confirm the external `Terrain` variant; six copied words
    /// are not six independent business fields.
    Terrain(Arc<TerrainEdit>),
    Path(Arc<RasterStroke>),
    Cobblestone1(Arc<RasterStroke>),
    Cobblestone2(Arc<RasterStroke>),
    Cobblestone3(Arc<RasterStroke>),
    Water(Arc<RasterStroke>),
    Garden(Arc<RasterStroke>),
    Wall(Arc<WallCompoundHistoryEdit>),
    Roof(Arc<RoofHistoryEdit>),
    Color(Arc<color_edit::ColorEdit>),
    DecoratorV5(Arc<decorator_edit::DecoratorHistoryEdit>),
    Decorator(Arc<decorator_edit::DecoratorHistoryEdit>),
    /// tag 12（① T3：visit_str case4 else `0x142aee14b="Tree"` → tag 0xc；旧 tag 0 错）。
    Tree(Arc<tree_edit::TreeEdit>),
    AutoClutter(Arc<auto_clutter_edit::AutoClutterEdit>),
    Stairs(Arc<stairs_history_edit::StairsHistoryEdit>),
}
impl From<RoofHistoryEdit> for EditType {
    fn from(edit: RoofHistoryEdit) -> Self {
        Self::Roof(Arc::new(edit))
    }
}
impl EditType {
    pub const fn native_tag(&self) -> u64 {
        match self {
            // ① T3 t3-edittype-terrain-tag 契约 §2：权威 15 变体 tag 表
            // （visit_str 0x140b3b630）：Terrain=0、Path=1、Cobblestone1-3=2/3/4、
            // Water=5、Garden=6、Wall=7、Roof=8、Color=9、DecoratorV5=10、
            // Decorator=11、Tree=12、AutoClutter=13、Stairs=14。
            Self::Terrain(_) => 0,
            Self::Path(_) => 1,
            Self::Cobblestone1(_) => 2,
            Self::Cobblestone2(_) => 3,
            Self::Cobblestone3(_) => 4,
            Self::Water(_) => 5,
            Self::Garden(_) => 6,
            Self::Wall(_) => 7,
            Self::Roof(_) => 8,
            Self::Color(_) => 9,
            Self::DecoratorV5(_) => 10,
            Self::Decorator(_) => 11,
            Self::Tree(_) => 12,
            Self::AutoClutter(_) => 13,
            Self::Stairs(_) => 14,
        }
    }
}

/// `History::add`（0x140a84ec0）的 tag-7 Wall 复合编辑录制校验。
///
/// 原版在 pending 空 + cursor-len<−2 的 nuke 之后、push 之前，对 `*in_RDX==7` 的
/// Wall 复合编辑逐 group、逐操作 niche 解码（`+0x10` 判别）：
/// - 变体 0（slice 操作）：`u32[+0xf8]<2` 门内，`f32[+0x100] < f32[+0xfc] ||
///   ==` 即 slice 起点≥终点 → panic `"Slice start_u must be smaller than end_u.
///   Otherwise an invalid curve will be created!"`（0x142ad8118 字符串）。
/// - 变体 1（CreateWall）/变体 2（DeleteWall）：`+0x18` 的 Option source 为 None
///   （`SBORROW8(0,v)` 即 v>0 才 panic 的逆映射：None==0 视为违规）→ panic
///   `"None is not allowed in \`CreateWall\` history command recording"` /
///   `"None is not allowed in \`DeleteWall\` history command recording"`。
///
/// src 的 `WallHistoryEdit` 为原生 17 变体 niche 枚举的 Rust 建模（判别表见
/// `wall_history_edit.rs`）：变体 0 `StateChange`（`u32[+0xe8]<2` 门）、变体 1
/// `Create`/变体 2 `Delete`（`+0x18` Option source 面）。Create/Delete 的"source
/// 存在"以 `WallHistoryMetadata::source_66` Option 为结构最近拟（与 replay_wall 的
/// `MissingSource66` 同一缺口面，原生 `Option<source> @T+0x08` 属 walls authority
/// 快照，见布局报告登记）；StateChange 的 slice 以 `observed_operation`
/// 载荷 `{op_tag u32, start f32, end f32}` 判 `start < end`（即原生 @T+0xe8/@T+0xec/@T+0xf0）。
impl HistoryRecordingValidation for EditType {
    fn validate_for_recording(&self) {
        if let Self::Wall(compound) = self {
            for group in &compound.groups {
                for edit in group {
                    use crate::resources::history::wall_edit::WallHistoryEdit;
                    match edit {
                        WallHistoryEdit::Create { source, .. } => {
                            if source.metadata.source_66.is_none() {
                                panic!(
                                    "None is not allowed in `CreateWall` history command recording"
                                );
                            }
                        }
                        WallHistoryEdit::Delete { source, .. } => {
                            if source.metadata.source_66.is_none() {
                                panic!(
                                    "None is not allowed in `DeleteWall` history command recording"
                                );
                            }
                        }
                        WallHistoryEdit::StateChange {
                            observed_operation, ..
                        } => {
                            let op_tag =
                                u32::from_le_bytes(observed_operation[..4].try_into().unwrap());
                            if op_tag < 2 {
                                let start = f32::from_le_bytes(
                                    observed_operation[4..8].try_into().unwrap(),
                                );
                                let end = f32::from_le_bytes(
                                    observed_operation[8..12].try_into().unwrap(),
                                );
                                if start >= end {
                                    panic!(
                                        "Slice start_u must be smaller than end_u. Otherwise an invalid curve will be created!"
                                    );
                                }
                            }
                        }
                        WallHistoryEdit::EnclosureChange { .. } => {}
                        // 原生 add 只校验变体 0/1/2（slice/Create/Delete）；
                        // 变体 3（AdjustHeight）与 5..16（未恢复载荷）无入史断言。
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
                        | WallHistoryEdit::ChangeSkirt(..) => {}
                    }
                }
            }
        }
    }
}

/// 光栅编辑无 add 期校验（原版 tag 1..6 无入史断言）。
impl HistoryRecordingValidation for RasterStroke {}

/// `Arc<RasterStroke>` 由 world_raster 录制链直接作为 M 使用（无 add 期校验）。
impl HistoryRecordingValidation for std::sync::Arc<RasterStroke> {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::history::transactions::HistoryTransactions;
    use crate::resources::history::wall_edit::WallHistoryEdit;
    use crate::resources::history::wall_edit::record_new_wall;
    use crate::resources::roofs::geometry::surface::Curve2;
    use crate::resources::walls::inner_wall_state::WallState;
    use crate::resources::walls::publication_fields::WallId;

    fn wall_state() -> WallState {
        let curve = Curve2::try_new(vec![[0.0, 0.0], [4.0, 0.0]], false).unwrap();
        WallState::new(curve, 2.0, 3, 0, false)
    }

    fn recording_history() -> HistoryTransactions<EditType> {
        let mut history = HistoryTransactions::after_observed_future_clear();
        history.start_input_recording(0).unwrap();
        history
    }

    #[test]
    fn wall_compound_with_present_source_passes_add() {
        let state = wall_state();
        let edit = record_new_wall(WallId(9), state.snapshot(), 7).unwrap();
        let mut history = recording_history();
        assert!(
            history
                .add(EditType::Wall(std::sync::Arc::new(edit)))
                .is_ok()
        );
    }

    #[test]
    #[should_panic(expected = "None is not allowed in `CreateWall`")]
    fn create_wall_without_source_66_panics() {
        let mut state = wall_state();
        state.snapshot.metadata.source_66 = None;
        let mut edit = record_new_wall(WallId(9), state.snapshot(), 7).unwrap();
        // 模拟旧便携快照缺 +66：seed Create 直接以缺失字节进入 add。
        if let WallHistoryEdit::Create { source, .. } = &mut edit.groups[0][0] {
            source.metadata.source_66 = None;
        }
        let mut history = recording_history();
        let _ = history.add(EditType::Wall(std::sync::Arc::new(edit)));
    }

    #[test]
    #[should_panic(expected = "Slice start_u must be smaller than end_u")]
    fn state_change_with_degenerate_slice_panics() {
        let state = wall_state();
        let snapshot = state.snapshot().clone();
        let mut observed_operation = [0; 12];
        observed_operation[..4].copy_from_slice(&0u32.to_le_bytes()); // op_tag 0（slice 面）
        observed_operation[4..8].copy_from_slice(&0.0f32.to_le_bytes()); // start
        observed_operation[8..12].copy_from_slice(&0.0f32.to_le_bytes()); // end（start==end → 违规）
        let edit = WallCompoundHistoryEdit {
            groups: vec![vec![WallHistoryEdit::StateChange {
                wall: WallId(9),
                previous: snapshot.clone(),
                next: snapshot.clone(),
                observed_operation,
            }]],
        };
        let mut history = recording_history();
        let _ = history.add(EditType::Wall(std::sync::Arc::new(edit)));
    }

    #[test]
    fn non_wall_edits_pass_without_validation() {
        let mut history = recording_history();
        let stroke = std::sync::Arc::new(crate::resources::raster_stroke::RasterStroke {
            flags: 0,
            strength: 0.5,
            points: vec![glam::Vec3::ZERO].into(),
            radius: 1.0,
            draw: true,
        });
        assert!(history.add(EditType::Path(stroke)).is_ok());
    }
}

#[cfg(test)]
mod debug_display_tests {
    use super::*;

    #[test]
    fn debug_display_matches_fresh_format_pieces() {
        // O：0x140a84ba0 的三片格式 "History: " / "/" / ". Start at " +
        // cursor-1（History+0x10）与 +0x68/+0x6c 的显示值。
        let mut history = History::<()>::empty_glade();
        history.debug_chain = 5;
        history.debug_set_history_start(7);
        // empty_glade cursor = -1 → cursor-1 = -2。
        assert_eq!(history.debug_display(), "History: 5/-2. Start at 7");
        history.debug_set_history_start(9);
        assert_eq!(history.debug_history_start, 9);
    }
}

pub mod building_transactions;

pub mod snapshots;

pub mod transactions;

pub mod wall_edit;
