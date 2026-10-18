//! Observed B History recording/branch/cursor policy, using normal Rust owners.
//! This is not an ABI replica or a decoder of the unresolved auxiliary owners.
use crate::resources::roofs::customization::RoofRimHistoryMutation;
use crate::resources::roofs::geometry::replay::ObservedReplayEvent;

/// Native `History::add`（0x140a84ec0）在 push 前对载荷做录制校验
/// （tag-7 Wall 复合编辑的逐操作 niche 断言）。其他 edit 种类无 add 期校验。
/// 由 `add_with_nonce_provider` 在 push 前调用。
pub trait HistoryRecordingValidation {
    fn validate_for_recording(&self) {}
}

#[derive(Clone, Debug, PartialEq)]
pub struct HistoryEntry<M> {
    pub edits: Vec<M>,
    /// Three nonempty checks observed in MultiEdit's auxiliary owners.
    /// Their contents remain with those owners; counts are not fabricated deltas.
    pub observed_auxiliary_counts: [usize; 3],
    /// Native per-entry stairs graph checksum read by
    /// `History::add_decorator_diff` (0x140a854a0: the checksum lives at
    /// record `+0x28` behind the entry's record pointer). Reconstructed
    /// producers that do not own a checksum leave this `None`; the native
    /// slot itself is a plain `u64`.
    pub observed_stairs_checksum: Option<u64>,
}
impl<M> HistoryEntry<M> {
    /// The original recording initializer creates empty auxiliary containers.
    pub fn from_edits(edits: Vec<M>) -> Self {
        Self {
            edits,
            observed_auxiliary_counts: [0; 3],
            observed_stairs_checksum: None,
        }
    }
    fn is_empty(&self) -> bool {
        self.edits.is_empty() && self.observed_auxiliary_counts == [0; 3]
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct ResolvedHistorySnapshot<S> {
    pub chain_index: i32,
    pub value: S,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ObservedHistoryLedger<M, S> {
    pub entries: Vec<HistoryEntry<M>>,
    pub cursor: i32,
    pub undo_anchor: i32,
    pub resolved_snapshots: Vec<ResolvedHistorySnapshot<S>>,
    /// B +58 is clamped by nuke_future_states; its original name is unresolved.
    pub observed_count_58: usize,
    pub mutation_nonce: u64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HistoryTransactionError {
    ReplayRecordingForbidden,
    RecordingNotStarted,
    InvalidCursor,
    CursorNotAtCommittedTail,
    AuxiliaryReplayOwnerRequired,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HistoryCommit {
    pub entry_index: usize,
    /// Notification is backend InputReplayCommand, not HistoryCmd.
    pub observed_input_replay_command_tag: u8,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HistoryCursorStep {
    Undo,
    Redo,
}
#[derive(Clone, Debug, PartialEq)]
pub struct HistoryReplaySelection<M> {
    pub entry_index: usize,
    pub undo: bool,
    pub instant: bool,
    /// Original replay_edit expands forward, then reverses the command list on
    /// undo. For Rim each edit expands to exactly one command.
    pub ordered_edits: Vec<M>,
}
impl HistoryReplaySelection<RoofRimHistoryMutation> {
    pub fn rim_events(&self) -> Vec<ObservedReplayEvent> {
        self.ordered_edits
            .iter()
            .map(|edit| ObservedReplayEvent::FlatRimStyle {
                wall_index: edit.wall_id.0,
                style: if self.undo { edit.from } else { edit.to },
            })
            .collect()
    }
}

#[derive(Clone, Debug)]
pub struct HistoryTransactions<M, S = ()> {
    ledger: ObservedHistoryLedger<M, S>,
    pending: Option<HistoryEntry<M>>,
    /// Native `History +0x48`: `Arc<DecoratorHistoryDiff>` replaced by
    /// `History::add_decorator_diff` (0x140a854a0). Native
    /// `DecoratorHistoryDiff` is a 0x30-byte map value (see
    /// `decorator_edit::DecoratorHistoryDiffMap`).
    pub decorator_diff:
        Option<std::sync::Arc<crate::resources::history::decorator_edit::DecoratorHistoryDiffMap>>,
    /// Native `History +0x50`: `Arc<StairsHistoryDiff>` replaced by the same
    /// function (0x60-byte payload in the original; see stairs domain).
    pub stairs_diff:
        Option<std::sync::Arc<crate::resources::stairs::stairs_state::StairsHistoryDiff>>,
    /// Native `History +0x68`（`HistoryEditChain` 的调试显示值）：`history_chain::impl$2::fmt`
    /// (0x140b31c90) 读取 `chain+0x30` 的数字。语义未完整锚定，仅供 `debug_display`。
    pub debug_chain: u64,
    /// Native `History +0x6c`（`debug_set_history_start` 0x140a84c40 写 u32；
    /// `debug_display` 0x140a84ba0 读）。调试工具字段，不参与业务历史。
    pub debug_history_start: u32,
}
pub type RimHistoryTransactions = HistoryTransactions<RoofRimHistoryMutation>;
impl HistoryRecordingValidation for RoofRimHistoryMutation {} // rim 编辑无 add 期校验
impl<M, S> HistoryTransactions<M, S> {
    /// Caller-supplied observed ledger; no guessed startup cursor/anchor/nonce.
    pub fn from_observed(ledger: ObservedHistoryLedger<M, S>) -> Self {
        Self {
            ledger,
            pending: None,
            decorator_diff: None,
            stairs_diff: None,
            debug_chain: 0,
            debug_history_start: 0,
        }
    }
    pub fn ledger(&self) -> &ObservedHistoryLedger<M, S> {
        &self.ledger
    }
    /// 可写账本访问：`convert_preplaced_trees` 等需 `&mut ObservedHistoryLedger`
    /// 调用 `insert_starting_edit`（0x140a844f0，见 resources/history/snapshots.rs）。
    /// `HistoryTransactions::ledger()` 此前仅只读（分歧 15 证实，2026-10-17）。
    pub fn ledger_mut(&mut self) -> &mut ObservedHistoryLedger<M, S> {
        &mut self.ledger
    }
    pub fn is_recording(&self) -> bool {
        self.pending.is_some()
    }
    pub fn pending(&self) -> Option<&HistoryEntry<M>> {
        self.pending.as_ref()
    }
    /// Mutable pending access used by auxiliary-owner writers
    /// (`History::add_decorator_diff` 0x140a854a0 pushes tag-0xE records into
    /// the open MultiEdit). Returns `None` when recording is not started.
    pub fn pending_mut(&mut self) -> Option<&mut HistoryEntry<M>> {
        self.pending.as_mut()
    }
    /// Replaces the open pending group (used after the decorator-diff prologue
    /// nukes future states and reinitializes the original empty MultiEdit).
    pub fn set_pending(&mut self, pending: Option<HistoryEntry<M>>) {
        self.pending = pending;
    }
    /// `History::nuke_future_states` (0x140a84c50) as invoked by the
    /// decorator-diff prologue when `cursor - committed < -2`.
    pub fn nuke_future_states_with(
        &mut self,
        nonce: &mut impl FnMut() -> u64,
    ) -> Result<(), HistoryTransactionError> {
        self.nuke_future_states(nonce)
    }
    /// Select the original cursor entry, expand its commands, then commit the
    /// cursor. Expansion owns command reversal: reversing edits is incorrect
    /// for edits which expand into several commands. A failed expansion keeps
    /// the ledger unchanged so unresolved owners cannot silently lose history.
    /// The recording check belongs to each newly read HistoryCmd, not cursor
    /// selection: B @140af5f81..6134 has no recording gate on an existing queue.
    pub fn cursor_step_expanding<T, E: From<HistoryTransactionError>>(
        &mut self,
        step: HistoryCursorStep,
        instant: bool,
        expand: impl FnOnce(usize, &HistoryEntry<M>, bool, bool) -> Result<T, E>,
    ) -> Result<Option<T>, E> {
        let target = match step {
            HistoryCursorStep::Undo => {
                if self.ledger.cursor < 0 || self.ledger.cursor <= self.ledger.undo_anchor {
                    return Ok(None);
                }
                self.ledger.cursor as usize
            }
            HistoryCursorStep::Redo => {
                if self.ledger.cursor < -1 {
                    return Err(HistoryTransactionError::InvalidCursor.into());
                }
                let next = self.ledger.cursor.wrapping_add(1);
                if next < 0 {
                    return Ok(None);
                }
                next as usize
            }
        };
        let Some(entry) = self.ledger.entries.get(target) else {
            return Ok(None);
        };
        // Auxiliary owner diffs travel with the ReplayBucket (native History
        // +0x18/+0x20 Arcs); they are no longer rejected here.
        let undo = step == HistoryCursorStep::Undo;
        let result = expand(target, entry, undo, instant)?;
        self.ledger.cursor = if undo {
            self.ledger.cursor - 1
        } else {
            target as i32
        };
        Ok(Some(result))
    }
    pub fn start_input_recording(
        &mut self,
        app_mode_tag: u32,
    ) -> Result<(), HistoryTransactionError> {
        if app_mode_tag == 13 {
            return Err(HistoryTransactionError::ReplayRecordingForbidden);
        }
        // Explicit start replaces an old uncommitted group; try_start preserves it.
        self.pending = Some(HistoryEntry::from_edits(Vec::new()));
        Ok(())
    }
    pub fn try_start_input_recording(&mut self, app_mode_tag: u32) -> bool {
        if app_mode_tag == 13 || self.is_recording() {
            return false;
        }
        self.pending = Some(HistoryEntry::from_edits(Vec::new()));
        true
    }
    fn nuke_future_states(
        &mut self,
        nonce: &mut impl FnMut() -> u64,
    ) -> Result<(), HistoryTransactionError> {
        if self.ledger.cursor != self.ledger.entries.len() as i32 - 1 {
            if self.ledger.cursor < 0 {
                self.ledger.entries.clear();
                self.ledger.resolved_snapshots.clear();
                self.ledger.cursor = -1;
                self.ledger.undo_anchor = -1;
                self.ledger.observed_count_58 = 0;
                self.pending = None;
            } else {
                let retained = self.ledger.cursor as usize + 1;
                if retained > self.ledger.entries.len() {
                    return Err(HistoryTransactionError::InvalidCursor);
                }
                self.ledger.entries.truncate(retained);
                self.ledger.observed_count_58 = self.ledger.observed_count_58.min(retained);
                self.ledger
                    .resolved_snapshots
                    .retain(|snapshot| snapshot.chain_index <= self.ledger.cursor);
            }
        }
        self.ledger.mutation_nonce = nonce();
        Ok(())
    }
    pub fn add(&mut self, edit: M) -> Result<(), HistoryTransactionError>
    where
        M: HistoryRecordingValidation,
    {
        self.add_with_nonce_provider(edit, &mut || fastrand::u64(..))
    }
    /// The nonce provider is the original RNG dependency boundary, not a clock.
    /// Native `History::add`（0x140a84ec0）在 push 前执行
    /// [`HistoryRecordingValidation::validate_for_recording`]（tag-7 Wall
    /// 复合编辑校验），校验失败 panic（原版 panic_fmt）。
    pub fn add_with_nonce_provider(
        &mut self,
        edit: M,
        nonce: &mut impl FnMut() -> u64,
    ) -> Result<(), HistoryTransactionError>
    where
        M: HistoryRecordingValidation,
    {
        let current = self
            .pending
            .as_ref()
            .ok_or(HistoryTransactionError::RecordingNotStarted)?;
        if current.edits.is_empty()
            && self
                .ledger
                .cursor
                .wrapping_sub(self.ledger.entries.len() as i32)
                < -1
        {
            self.nuke_future_states(nonce)?;
            self.pending = Some(HistoryEntry::from_edits(Vec::new()));
        }
        edit.validate_for_recording();
        self.pending.as_mut().unwrap().edits.push(edit);
        Ok(())
    }
    pub fn finish_input_recording(
        &mut self,
    ) -> Result<Option<HistoryCommit>, HistoryTransactionError> {
        self.finish_with_nonce_provider(&mut || fastrand::u64(..))
    }
    pub fn finish_with_nonce_provider(
        &mut self,
        nonce: &mut impl FnMut() -> u64,
    ) -> Result<Option<HistoryCommit>, HistoryTransactionError> {
        let group = self
            .pending
            .take()
            .ok_or(HistoryTransactionError::RecordingNotStarted)?;
        if group.is_empty() {
            return Ok(None);
        }
        self.ledger.entries.push(group);
        self.ledger.mutation_nonce = nonce();
        if self.ledger.cursor < -1 {
            return Err(HistoryTransactionError::InvalidCursor);
        }
        self.ledger.cursor = self.ledger.cursor.wrapping_add(1);
        if self.ledger.cursor != self.ledger.entries.len() as i32 - 1 {
            return Err(HistoryTransactionError::CursorNotAtCommittedTail);
        }
        Ok(Some(HistoryCommit {
            entry_index: self.ledger.entries.len() - 1,
            observed_input_replay_command_tag: 1,
        }))
    }
    pub fn finish_potential_mutation(&mut self, mut token: HistoryMutationToken) {
        self.finish_potential_with_nonce_provider(&mut token, &mut || fastrand::u64(..));
    }
    pub fn finish_potential_with_nonce_provider(
        &mut self,
        token: &mut HistoryMutationToken,
        nonce: &mut impl FnMut() -> u64,
    ) {
        if token.active {
            self.ledger.resolved_snapshots.clear();
            self.ledger.mutation_nonce = nonce();
        }
        token.active = false;
    }
}
// Only Rim's one-edit/one-command expansion has been recovered. Other edit
// variants can expand to multiple commands, so raw edit reversal is not a
// supported generic substitute for the original command-list reversal.
impl<S> HistoryTransactions<RoofRimHistoryMutation, S> {
    pub fn cursor_step(
        &mut self,
        step: HistoryCursorStep,
        instant: bool,
    ) -> Result<Option<HistoryReplaySelection<RoofRimHistoryMutation>>, HistoryTransactionError>
    {
        // history_process consumes but ignores commands during input recording.
        if self.is_recording() {
            return Ok(None);
        }
        let target = match step {
            HistoryCursorStep::Undo => {
                if self.ledger.cursor < 0 || self.ledger.cursor <= self.ledger.undo_anchor {
                    return Ok(None);
                }
                self.ledger.cursor as usize
            }
            HistoryCursorStep::Redo => {
                if self.ledger.cursor < -1 {
                    return Err(HistoryTransactionError::InvalidCursor);
                }
                let target = self.ledger.cursor.wrapping_add(1);
                if target < 0 {
                    return Ok(None);
                }
                target as usize
            }
        };
        let Some(entry) = self.ledger.entries.get(target) else {
            return Ok(None);
        };
        // Auxiliary owner diffs travel with the ReplayBucket (native History
        // +0x18/+0x20 Arcs); they are no longer rejected here.
        let undo = step == HistoryCursorStep::Undo;
        self.ledger.cursor = if undo {
            self.ledger.cursor - 1
        } else {
            target as i32
        };
        let mut edits = entry.edits.clone();
        if undo {
            edits.reverse();
        }
        Ok(Some(HistoryReplaySelection {
            entry_index: target,
            undo,
            instant,
            ordered_edits: edits,
        }))
    }
}
impl<M> HistoryTransactions<M> {
    /// Proven nuke_future_states negative-cursor reset, not a claimed startup
    /// Default<History>. The original reset generates a new fastrand nonce.
    pub fn after_observed_future_clear() -> Self {
        Self::from_observed(ObservedHistoryLedger {
            entries: Vec::new(),
            cursor: -1,
            undo_anchor: -1,
            resolved_snapshots: Vec::new(),
            observed_count_58: 0,
            mutation_nonce: fastrand::u64(..),
        })
    }
}
/// This is a forgotten-mutation guard. Drop never commits a recording.
#[must_use = "finish_potential_mutation must consume an active mutation token"]
pub struct HistoryMutationToken {
    active: bool,
}
impl HistoryMutationToken {
    /// Only the observed active flag is reconstructed; token production by
    /// other history mutation owners remains their responsibility.
    pub fn from_observed_active(active: bool) -> Self {
        Self { active }
    }
}
impl Drop for HistoryMutationToken {
    fn drop(&mut self) {
        if self.active && !std::thread::panicking() {
            panic!("History was mutated, but wasn't told about it");
        }
    }
}
