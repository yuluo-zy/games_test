//! B building history owner expansion over the already recovered transaction ledger.
//! Roof owners and proven new/deleted Wall concurrent groups are executable. The original
//! replay_edit expands forward, then reverses the *commands*, not arbitrary edits.
use crate::resources::history::transactions::HistoryCommit;
use crate::resources::history::transactions::HistoryCursorStep;
use crate::resources::history::transactions::HistoryEntry;
use crate::resources::history::transactions::HistoryMutationToken;
use crate::resources::history::transactions::HistoryRecordingValidation;
use crate::resources::history::transactions::HistoryTransactionError;
use crate::resources::history::transactions::HistoryTransactions;
use crate::resources::history::transactions::ObservedHistoryLedger;
use crate::resources::roofs::customization::RoofRimHistoryMutation;
use crate::resources::roofs::geometry::edit_rules::RoofEditEvent;
use crate::resources::roofs::geometry::replay::ObservedReplayEvent;
use crate::resources::roofs::geometry::replay::VisualReplayState;
use crate::resources::roofs::geometry::replay::visual_edit_from_observed;
use tg_history::RoofVisualEdit;

/// The B constructor wraps this one operation in a one-group ConcurrentEdits
/// owner (outer EditType tag7). It is kept separate until actual Wall authority
/// replay and all compound owners are integrated; it is never treated as Roof.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WallHeightHistoryEdit {
    pub wall_id: u64,
    pub from: f32,
    pub to: f32,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WallHeightReplayRequest {
    pub wall_id: u64,
    pub absolute_height: f32,
    /// Consumer parameter evidence identifies this as an audio policy flag,
    /// not History::add ownership or record-history enablement.
    pub observed_audio_flag: u8,
}
impl WallHeightHistoryEdit {
    /// Meaningful Arc allocation fields from new_height_changed; allocation
    /// size264 is enum capacity, not proof that all bytes are game fields.
    pub fn write_observed_arc_fields(&self, bytes: &mut [u8; 264]) {
        bytes[..8].copy_from_slice(&1u64.to_le_bytes());
        bytes[8..16].copy_from_slice(&1u64.to_le_bytes());
        bytes[16..24].copy_from_slice(&0x8000000000000002u64.to_le_bytes());
        bytes[24..32].copy_from_slice(&self.wall_id.to_le_bytes());
        bytes[32..36].copy_from_slice(&self.from.to_le_bytes());
        bytes[36..40].copy_from_slice(&self.to.to_le_bytes());
    }
    /// Original instant dispatch idx3, not an implementation of height clamp,
    /// authority lookup, timed cubic easing or source/derived wall mutation.
    pub fn instant_request(&self, undo: bool) -> WallHeightReplayRequest {
        WallHeightReplayRequest {
            wall_id: self.wall_id,
            absolute_height: if undo { self.from } else { self.to },
            observed_audio_flag: 1,
        }
    }
}
impl WallHeightReplayRequest {
    pub fn write_observed_fields(&self, bytes: &mut [u8; 40]) {
        bytes[..4].copy_from_slice(&0u32.to_le_bytes());
        bytes[16..20].copy_from_slice(&1u32.to_le_bytes());
        bytes[20..24].copy_from_slice(&self.absolute_height.to_le_bytes());
        bytes[24..32].copy_from_slice(&self.wall_id.to_le_bytes());
        bytes[32] = self.observed_audio_flag;
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RoofHistoryEdit([u8; 64]);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BuildingHistoryError {
    Transaction(HistoryTransactionError),
    InvalidRoofTag(u8),
    UnresolvedOwner(u64),
    AuxiliaryReplayOwnerRequired,
    UnrestoredWallExpansion,
}
impl From<HistoryTransactionError> for BuildingHistoryError {
    fn from(error: HistoryTransactionError) -> Self {
        Self::Transaction(error)
    }
}
impl RoofHistoryEdit {
    /// Observed B RoofEdit payload excluding the Arc header and outer edit tag.
    /// Padding and unknown presence fields remain bytes, not invented defaults.
    pub fn from_observed(payload: [u8; 64]) -> Result<Self, BuildingHistoryError> {
        if payload[0] > 4 {
            return Err(BuildingHistoryError::InvalidRoofTag(payload[0]));
        }
        Ok(Self(payload))
    }
    pub fn payload(&self) -> &[u8; 64] {
        &self.0
    }
    pub fn tag(&self) -> u8 {
        self.0[0]
    }
    pub fn wall_id(&self) -> u64 {
        let offset = match self.tag() {
            0 | 1 => 48,
            2 => 40,
            _ => 8,
        };
        u64::from_le_bytes(self.0[offset..offset + 8].try_into().unwrap())
    }
    /// Constructs the confirmed fields only; zero bytes are enum padding.
    pub fn create(wall_id: u64, presence: [u8; 44]) -> Self {
        Self::presence(0, wall_id, presence)
    }
    pub fn delete(wall_id: u64, presence: [u8; 44]) -> Self {
        Self::presence(1, wall_id, presence)
    }
    /// Original invalid-roof recording tail after the query row and matching
    /// invalid-wall notification have been resolved. It is not auto-recording
    /// every DeleteRoof consumer; caller must establish those source guards.
    pub fn delete_from_roof(roof: &[u8; 88]) -> Self {
        let wall = u64::from_le_bytes(roof[..8].try_into().unwrap());
        let mut presence = [0; 44];
        presence[..4].copy_from_slice(&roof[0x50..0x54]);
        presence[4..16].copy_from_slice(&roof[0x40..0x4c]);
        presence[16..44].copy_from_slice(&roof[0x24..0x40]);
        Self::delete(wall, presence)
    }
    fn presence(tag: u8, wall_id: u64, presence: [u8; 44]) -> Self {
        let mut payload = [0; 64];
        payload[0] = tag;
        payload[4..48].copy_from_slice(&presence);
        payload[48..56].copy_from_slice(&wall_id.to_le_bytes());
        Self(payload)
    }
    pub fn direction(wall_id: u64, from: u8, to: u8) -> Self {
        let mut payload = [0; 64];
        payload[..3].copy_from_slice(&[3, from, to]);
        payload[8..16].copy_from_slice(&wall_id.to_le_bytes());
        Self(payload)
    }
    pub fn rim(edit: RoofRimHistoryMutation) -> Self {
        let mut payload = [0; 64];
        payload[..3].copy_from_slice(&[4, edit.from, edit.to]);
        payload[8..16].copy_from_slice(&edit.wall_id.0.to_le_bytes());
        Self(payload)
    }
    pub fn visual(edit: &RoofVisualEdit) -> Self {
        let mut payload = [0; 64];
        payload[0] = 2;
        for (offset, value) in [
            (4, edit.prev_height),
            (8, edit.prev_profile),
            (12, edit.prev_ridge),
            (16, edit.prev_eave),
            (20, edit.new_height),
            (24, edit.new_profile),
            (28, edit.new_ridge),
            (32, edit.new_eave),
            (48, edit.prev_tip_offset[0]),
            (52, edit.prev_tip_offset[1]),
            (56, edit.new_tip_offset[0]),
            (60, edit.new_tip_offset[1]),
        ] {
            payload[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
        }
        payload[40..48].copy_from_slice(&edit.wall_index.to_le_bytes());
        Self(payload)
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct SavedRoofPresence {
    wall_index: u64,
    color_id: u32,
    params: crate::resources::roofs::roof_shape::RoofShapeParams,
    #[serde(default = "history_flat_style_default")]
    flat_roof_style: crate::resources::roofs::roof_shape::FlatRoofStyle,
}
fn history_flat_style_default() -> crate::resources::roofs::roof_shape::FlatRoofStyle {
    let mut bytes = [0; 12];
    bytes[0] = 1;
    crate::resources::roofs::roof_shape::FlatRoofStyle::from_observed(bytes)
}
#[derive(serde::Serialize, serde::Deserialize)]
enum SavedRoofRim {
    Rim,
    NoRim,
}
impl SavedRoofRim {
    fn native_tag(self) -> u8 {
        match self {
            Self::Rim => 0,
            Self::NoRim => 1,
        }
    }
    fn from_tag(tag: u8) -> Self {
        match tag {
            0 => Self::Rim,
            1 => Self::NoRim,
            _ => panic!("invalid FlatRoofRimStyle"),
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
enum SavedRoofEdit {
    CreateRoof(SavedRoofPresence),
    DeleteRoof(SavedRoofPresence),
    VisualEdit(tg_history::RoofVisualEdit),
    SwitchRoofRidgeDir(tg_history::RoofDirectionEdit),
    ChangeFlatRoofRimStyle {
        wall_index: u64,
        before: SavedRoofRim,
        after: SavedRoofRim,
    },
}
impl serde::Serialize for RoofHistoryEdit {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let b = &self.0;
        let f = |i| f32::from_le_bytes(b[i..i + 4].try_into().unwrap());
        let direction = |x| {
            if x == 0 {
                tg_history::RidgeDirection::Width
            } else {
                tg_history::RidgeDirection::Length
            }
        };
        let value = match self.tag() {
            0 | 1 => {
                let mut roof = [0; 88];
                roof[0x24..0x40].copy_from_slice(&b[20..48]);
                let presence = SavedRoofPresence {
                    wall_index: self.wall_id(),
                    color_id: u32::from_le_bytes(b[4..8].try_into().unwrap()),
                    params: crate::resources::roofs::roof_shape::RoofShapeParams::from_roof(&roof),
                    flat_roof_style:
                        crate::resources::roofs::roof_shape::FlatRoofStyle::from_observed(
                            b[8..20].try_into().unwrap(),
                        ),
                };
                if self.tag() == 0 {
                    SavedRoofEdit::CreateRoof(presence)
                } else {
                    SavedRoofEdit::DeleteRoof(presence)
                }
            }
            2 => SavedRoofEdit::VisualEdit(tg_history::RoofVisualEdit {
                wall_index: self.wall_id(),
                prev_height: f(4),
                prev_profile: f(8),
                prev_ridge: f(12),
                prev_eave: f(16),
                new_height: f(20),
                new_profile: f(24),
                new_ridge: f(28),
                new_eave: f(32),
                prev_tip_offset: [f(48), f(52)],
                new_tip_offset: [f(56), f(60)],
                extensions: Default::default(),
            }),
            3 => SavedRoofEdit::SwitchRoofRidgeDir(tg_history::RoofDirectionEdit {
                wall_index: self.wall_id(),
                from: direction(b[1]),
                to: direction(b[2]),
                extensions: Default::default(),
            }),
            4 => SavedRoofEdit::ChangeFlatRoofRimStyle {
                wall_index: self.wall_id(),
                before: SavedRoofRim::from_tag(b[1]),
                after: SavedRoofRim::from_tag(b[2]),
            },
            _ => return Err(serde::ser::Error::custom("invalid RoofEdit tag")),
        };
        serde::Serialize::serialize(&value, serializer)
    }
}
impl<'de> serde::Deserialize<'de> for RoofHistoryEdit {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <SavedRoofEdit as serde::Deserialize>::deserialize(deserializer)?;
        let direction = |x| match x {
            tg_history::RidgeDirection::Width => 0,
            tg_history::RidgeDirection::Length => 1,
        };
        let presence = |p: SavedRoofPresence, tag| {
            let mut bytes = [0; 44];
            bytes[..4].copy_from_slice(&p.color_id.to_le_bytes());
            bytes[4..16].copy_from_slice(p.flat_roof_style.observed());
            bytes[16..44].copy_from_slice(p.params.observed());
            Self::presence(tag, p.wall_index, bytes)
        };
        Ok(match value {
            SavedRoofEdit::ChangeFlatRoofRimStyle {
                wall_index,
                before,
                after,
            } => Self::rim(RoofRimHistoryMutation {
                observed_tag: 4,
                wall_id: crate::resources::walls::publication_fields::WallId(wall_index),
                from: before.native_tag(),
                to: after.native_tag(),
            }),
            SavedRoofEdit::CreateRoof(p) => presence(p, 0),
            SavedRoofEdit::DeleteRoof(p) => presence(p, 1),
            SavedRoofEdit::VisualEdit(v) => Self::visual(&v),
            SavedRoofEdit::SwitchRoofRidgeDir(v) => {
                Self::direction(v.wall_index, direction(v.from), direction(v.to))
            }
        })
    }
}

/// Native deferred visual recording consumes only the last unread change.
/// This is the actual producer payload mapping; UI selection/start ownership
/// remains with the caller. The record includes its 8-byte queue prefix.
pub fn collect_last_visual_history(records: &[[u8; 152]]) -> Option<BuildingEdit> {
    let record = records.last()?;
    let mut payload = [0; 64];
    payload[0] = 2;
    for (destination, source, length) in [
        (4, 0x24, 4),
        (8, 0x20, 4),
        (12, 0x28, 4),
        (16, 0x2c, 4),
        (20, 0x68, 4),
        (24, 0x64, 4),
        (28, 0x6c, 4),
        (32, 0x70, 4),
        (40, 0x10, 8),
        (48, 0x18, 8),
        (56, 0x5c, 8),
    ] {
        payload[destination..destination + length]
            .copy_from_slice(&record[source..source + length]);
    }
    Some(BuildingEdit::Roof(RoofHistoryEdit(payload)))
}
#[derive(Clone, Debug)]
pub struct RoofVisualHistoryRecord([u8; 152]);
impl RoofVisualHistoryRecord {
    pub fn from_observed(record: [u8; 152]) -> Self {
        Self(record)
    }
    /// CPU adapter writes only the fields the original recording producer reads.
    /// This is not a reconstruction of the complete 152-byte UI event schema.
    pub fn from_before_after(wall_id: u64, before: &[u8; 88], after: &[u8; 88]) -> Self {
        let mut record = [0; 152];
        record[0x10..0x18].copy_from_slice(&wall_id.to_le_bytes());
        record[0x18..0x20].copy_from_slice(&before[0x24..0x2c]);
        record[0x20..0x30].copy_from_slice(&before[0x2c..0x3c]);
        record[0x5c..0x64].copy_from_slice(&after[0x24..0x2c]);
        record[0x64..0x74].copy_from_slice(&after[0x2c..0x3c]);
        Self(record)
    }
    pub fn record(&self) -> &[u8; 152] {
        &self.0
    }
    pub fn edit(&self) -> BuildingEdit {
        collect_last_visual_history(std::slice::from_ref(&self.0)).unwrap()
    }
}
/// The separate ridge recording reader consumes every unread message, in order.
pub fn collect_direction_history(records: &[[u8; 24]]) -> Vec<BuildingEdit> {
    records
        .iter()
        .map(|record| {
            let wall = u64::from_le_bytes(record[8..16].try_into().unwrap());
            BuildingEdit::Roof(RoofHistoryEdit::direction(wall, record[16], record[17]))
        })
        .collect()
}

#[derive(Clone, Debug)]
pub enum BuildingEdit {
    Wall(std::sync::Arc<crate::resources::history::wall_edit::WallCompoundHistoryEdit>),
    Roof(RoofHistoryEdit),
    /// Preserved observed owner identity; never silently interpreted as Roof.
    UnresolvedOwner {
        observed_tag: u64,
    },
}
// Ledger identity comparisons never infer equality for original float snapshots.
impl PartialEq for BuildingEdit {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Roof(a), Self::Roof(b)) => a == b,
            (Self::Wall(a), Self::Wall(b)) => std::sync::Arc::ptr_eq(a, b),
            (
                Self::UnresolvedOwner { observed_tag: a },
                Self::UnresolvedOwner { observed_tag: b },
            ) => a == b,
            _ => false,
        }
    }
}
impl Eq for BuildingEdit {}
impl From<RoofRimHistoryMutation> for BuildingEdit {
    fn from(edit: RoofRimHistoryMutation) -> Self {
        Self::Roof(RoofHistoryEdit::rim(edit))
    }
}

/// Building 编辑的 Wall 复合校验由 `BuildingHistoryTransactions::add_with_nonce_provider`
/// 的 `UnresolvedOwner` 拒绝承接（本 wrapper 层）；内层 ledger 无需再校验。
impl HistoryRecordingValidation for BuildingEdit {}

#[derive(Clone, Debug)]
pub enum BuildingReplayOutput {
    Discrete(ObservedReplayEvent),
    Visual(RoofEditEvent),
    NoEvent,
}
#[derive(Clone, Debug)]
pub struct RoofReplayCommand {
    edit: std::sync::Arc<RoofHistoryEdit>,
    state: VisualReplayState,
}
impl RoofReplayCommand {
    pub(crate) fn expanded(edit: RoofHistoryEdit, undo: bool, instant: bool) -> Self {
        Self::expanded_shared(std::sync::Arc::new(edit), undo, instant)
    }
    pub(crate) fn expanded_shared(
        edit: std::sync::Arc<RoofHistoryEdit>,
        undo: bool,
        instant: bool,
    ) -> Self {
        // Original B expansion: mode = !instant, duration = 0.2f32, elapsed = 0,
        // undo flag selected by cursor, finished = false. Arc pointer is owned
        // through Rust instead of copied into state bytes 8..16.
        let mut state = [0; 24];
        state[..4].copy_from_slice(&u32::from(!instant).to_le_bytes());
        state[4..8].copy_from_slice(&0x3e4ccccdu32.to_le_bytes());
        state[20] = u8::from(undo);
        Self {
            edit,
            state: VisualReplayState::from_observed(state),
        }
    }
    pub fn payload(&self) -> &[u8; 64] {
        self.edit.payload()
    }
    pub fn state(&self) -> &VisualReplayState {
        &self.state
    }
    /// ReplayBuffer::make_animations_instant, original B jump-table tag 5:
    /// write mode=0 and preserve elapsed/duration/undo/completion.
    pub fn make_instant(&mut self) {
        let mut state = *self.state.bytes();
        state[..4].copy_from_slice(&0u32.to_le_bytes());
        self.state = VisualReplayState::from_observed(state);
    }
    /// Executes recovered process policy only; caller owns actual World lookup,
    /// queue/bucket retention and original scheduler completion rules.
    pub fn advance(
        &mut self,
        delta_time: f32,
    ) -> Result<BuildingReplayOutput, BuildingHistoryError> {
        if self.edit.tag() == 2 {
            let edit = visual_edit_from_observed(self.edit.payload())
                .map_err(|_| BuildingHistoryError::InvalidRoofTag(2))?;
            Ok(BuildingReplayOutput::Visual(
                self.state.advance(&edit, delta_time),
            ))
        } else {
            let event = self
                .state
                .advance_discrete_observed(self.edit.payload(), delta_time)
                .map_err(|_| BuildingHistoryError::InvalidRoofTag(self.edit.tag()))?;
            Ok(event
                .map(BuildingReplayOutput::Discrete)
                .unwrap_or(BuildingReplayOutput::NoEvent))
        }
    }
}
#[derive(Clone, Debug)]
pub struct BuildingReplaySelection {
    pub entry_index: usize,
    pub undo: bool,
    pub instant: bool,
    pub commands: Vec<RoofReplayCommand>,
    /// Legacy owner-partition observation used by CPU probes. These are not
    /// native outer ReplayBuckets: the live resource preserves mixed commands
    /// in resources::history::ReplayBucket. `commands` is the Roof projection.
    pub buckets: Vec<BuildingReplayBucket>,
}
#[derive(Clone, Debug)]
pub enum BuildingReplayBucket {
    Roof(Vec<RoofReplayCommand>),
    Wall(
        std::collections::VecDeque<
            Vec<crate::systems::history::wall_replay_interpreter::WallReplayCommand>,
        >,
    ),
}
#[derive(Clone, Debug)]
pub struct BuildingReplayBucketSelection {
    pub entry_index: usize,
    pub owner: BuildingReplayBucket,
}
impl BuildingReplaySelection {
    /// Legacy probe partition, not the native scheduling boundary. The live
    /// history owner must use resources::history::ReplayBucket::expand, whose
    /// mixed command list is visited by each domain consumer in the same frame.
    pub fn into_replay_buckets(self) -> Vec<BuildingReplayBucketSelection> {
        self.buckets
            .into_iter()
            .map(|owner| BuildingReplayBucketSelection {
                entry_index: self.entry_index,
                owner,
            })
            .collect()
    }
}

/// Original replay_roof's per-bucket policy: process all Roof commands in order,
/// then remove completed commands while preserving the survivors. The caller
/// supplies Time.delta * ReplayBuffer.speed; this is not the global scheduler,
/// auxiliary restoration or an implicit command-by-command World flush.
pub fn advance_roof_bucket(
    commands: &mut Vec<RoofReplayCommand>,
    scaled_delta_time: f32,
) -> Result<Vec<BuildingReplayOutput>, BuildingHistoryError> {
    let mut outputs = Vec::with_capacity(commands.len());
    for command in commands.iter_mut() {
        outputs.push(command.advance(scaled_delta_time)?);
    }
    commands.retain(|command| command.state.bytes()[21] != 1);
    Ok(outputs)
}

#[derive(Clone, Debug)]
pub struct BuildingHistoryTransactions<S = ()> {
    transactions: HistoryTransactions<BuildingEdit, S>,
}
impl<S: Clone> BuildingHistoryTransactions<S> {
    pub fn from_observed(ledger: ObservedHistoryLedger<BuildingEdit, S>) -> Self {
        Self {
            transactions: HistoryTransactions::from_observed(ledger),
        }
    }
    pub fn ledger(&self) -> &ObservedHistoryLedger<BuildingEdit, S> {
        self.transactions.ledger()
    }
    pub fn pending(&self) -> Option<&HistoryEntry<BuildingEdit>> {
        self.transactions.pending()
    }
    pub fn is_recording(&self) -> bool {
        self.transactions.is_recording()
    }
    pub fn start_input_recording(&mut self, mode: u32) -> Result<(), BuildingHistoryError> {
        self.transactions
            .start_input_recording(mode)
            .map_err(Into::into)
    }
    pub fn try_start_input_recording(&mut self, mode: u32) -> bool {
        self.transactions.try_start_input_recording(mode)
    }
    pub fn add(&mut self, edit: BuildingEdit) -> Result<(), BuildingHistoryError> {
        self.add_with_nonce_provider(edit, &mut || fastrand::u64(..))
    }
    pub fn add_with_nonce_provider(
        &mut self,
        edit: BuildingEdit,
        nonce: &mut impl FnMut() -> u64,
    ) -> Result<(), BuildingHistoryError> {
        if let BuildingEdit::UnresolvedOwner { observed_tag } = edit {
            return Err(BuildingHistoryError::UnresolvedOwner(observed_tag));
        }
        self.transactions
            .add_with_nonce_provider(edit, nonce)
            .map_err(Into::into)
    }
    pub fn finish_input_recording(
        &mut self,
    ) -> Result<Option<HistoryCommit>, BuildingHistoryError> {
        self.transactions
            .finish_input_recording()
            .map_err(Into::into)
    }
    pub fn finish_with_nonce_provider(
        &mut self,
        nonce: &mut impl FnMut() -> u64,
    ) -> Result<Option<HistoryCommit>, BuildingHistoryError> {
        self.transactions
            .finish_with_nonce_provider(nonce)
            .map_err(Into::into)
    }
    pub fn finish_potential_mutation(&mut self, token: HistoryMutationToken) {
        self.transactions.finish_potential_mutation(token)
    }
    pub fn cursor_step(
        &mut self,
        step: HistoryCursorStep,
        instant: bool,
    ) -> Result<Option<BuildingReplaySelection>, BuildingHistoryError> {
        if self.is_recording() {
            return Ok(None);
        }
        let ledger = self.ledger();
        let target = match step {
            HistoryCursorStep::Undo => {
                if ledger.cursor < 0 || ledger.cursor <= ledger.undo_anchor {
                    return Ok(None);
                }
                ledger.cursor as usize
            }
            HistoryCursorStep::Redo => {
                if ledger.cursor < -1 {
                    return Err(HistoryTransactionError::InvalidCursor.into());
                }
                let next = ledger.cursor.wrapping_add(1);
                if next < 0 {
                    return Ok(None);
                }
                next as usize
            }
        };
        let Some(entry) = ledger.entries.get(target) else {
            return Ok(None);
        };
        if entry.observed_auxiliary_counts != [0; 3] || !entry.auxiliary.is_empty() {
            return Err(BuildingHistoryError::AuxiliaryReplayOwnerRequired);
        }
        let undo = step == HistoryCursorStep::Undo;
        let mut commands = Vec::new();
        let mut buckets = Vec::new();
        for edit in &entry.edits {
            match edit {
                BuildingEdit::Roof(edit) => {
                    let command = RoofReplayCommand::expanded(edit.clone(), undo, instant);
                    commands.push(command.clone());
                    buckets.push(BuildingReplayBucket::Roof(vec![command]));
                }
                BuildingEdit::Wall(edit) => {
                    let groups =
                        crate::systems::history::wall_replay_interpreter::expand_wall_compound(
                            edit, undo, instant,
                        )
                        .map_err(|_| BuildingHistoryError::UnrestoredWallExpansion)?;
                    buckets.push(BuildingReplayBucket::Wall(groups));
                }
                BuildingEdit::UnresolvedOwner { observed_tag } => {
                    return Err(BuildingHistoryError::UnresolvedOwner(*observed_tag));
                }
            }
        }
        if undo {
            commands.reverse();
            buckets.reverse();
        }
        // There is no pending group on this path. Reuse the recovered ledger
        // API, preserve all owners/snapshots/nonce, change only original cursor.
        let mut next = ledger.clone();
        next.cursor = if undo {
            ledger.cursor - 1
        } else {
            target as i32
        };
        self.transactions = HistoryTransactions::from_observed(next);
        Ok(Some(BuildingReplaySelection {
            entry_index: target,
            undo,
            instant,
            commands,
            buckets,
        }))
    }
}
impl BuildingHistoryTransactions {
    /// Verified future-clear reset, not a guessed original startup constructor.
    pub fn after_observed_future_clear() -> Self {
        Self {
            transactions: HistoryTransactions::after_observed_future_clear(),
        }
    }
}
