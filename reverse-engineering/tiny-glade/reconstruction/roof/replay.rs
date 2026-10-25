//! B RoofReplayCmd::process VisualEdit branch, with event writers already resolved.
//! Does not implement history cursor selection, ECS lookup or event scheduling.
use crate::edit_rules::{RoofEditEvent, ScalarEdit, TipEdit};
use glam::Vec2;
use tg_history::{RidgeDirection, RoofDirectionEdit, RoofVisualEdit};

/// Game event fields from the other B replay branches. Queue metadata and ECS
/// consumers are external. Presence data stays opaque until its schema/defaults
/// are recovered; copying the original bytes does not invent missing values.
#[derive(Clone, Debug)]
pub enum ObservedReplayEvent {
    Create {
        wall_index: u64,
        presence: [u8; 44],
        restoring_deleted: bool,
    },
    Delete {
        wall_index: u64,
    },
    Direction {
        wall_index: u64,
        direction: u8,
    },
    FlatRimStyle {
        wall_index: u64,
        style: u8,
    },
}

impl ObservedReplayEvent {
    /// Updates confirmed fields only; keeps the queue's message ID and padding.
    /// Returns the exclusive end of the meaningful payload beginning at byte 8.
    pub fn write_observed_fields(&self, record: &mut [u8; 64]) -> usize {
        let wall_index = match self {
            Self::Create { wall_index, .. }
            | Self::Delete { wall_index }
            | Self::Direction { wall_index, .. }
            | Self::FlatRimStyle { wall_index, .. } => *wall_index,
        };
        record[8..16].copy_from_slice(&wall_index.to_le_bytes());
        match self {
            Self::Create {
                presence,
                restoring_deleted,
                ..
            } => {
                record[16..60].copy_from_slice(presence);
                record[60] = u8::from(*restoring_deleted);
                61
            }
            Self::Delete { .. } => {
                record[16] = 1;
                17
            }
            Self::Direction { direction, .. } => {
                record[16] = *direction;
                17
            }
            Self::FlatRimStyle { style, .. } => {
                record[16] = *style;
                record[17] = *style;
                record[18] = 0;
                19
            }
        }
    }

    pub fn observed_tag(&self) -> u8 {
        match self {
            Self::Create { .. } => 0,
            Self::Delete { .. } => 1,
            Self::Direction { .. } => 3,
            Self::FlatRimStyle { .. } => 4,
        }
    }
}

/// Connect the recovered replay producer to the recovered resolved edit core.
/// The caller supplies the Roof that ECS lookup has already resolved.
pub fn step_visual_replay_resolved(
    roof: &mut [u8; 88],
    state: &mut VisualReplayState,
    edit: &RoofVisualEdit,
    delta_time: f32,
    changed_tick: &mut u32,
    current_tick: u32,
) -> crate::edit_rules::EditOutcome {
    let event = state.advance(edit, delta_time);
    crate::edit_rules::apply_edit(roof, &event, changed_tick, current_tick)
}

#[derive(Clone, Debug)]
pub struct VisualReplayState([u8; 24]);
impl VisualReplayState {
    pub fn from_observed(bytes: [u8; 24]) -> Self {
        Self(bytes)
    }
    pub fn bytes(&self) -> &[u8; 24] {
        &self.0
    }
    pub fn elapsed(&self) -> f32 {
        get(&self.0, 16)
    }
    pub fn finished(&self) -> bool {
        self.0[21] != 0
    }

    /// Original tags 0/1/3/4, excluding the history entry's 16-byte prefix.
    /// Create/delete emit immediately and complete. Direction/style emit only
    /// when elapsed equals zero, then wait only if the complete mode equals 1.
    pub fn advance_discrete_observed(
        &mut self,
        payload: &[u8; 64],
        delta_time: f32,
    ) -> Result<Option<ObservedReplayEvent>, &'static str> {
        let tag = payload[0];
        if !matches!(tag, 0 | 1 | 3 | 4) {
            return Err("Expected B discrete roof replay tag 0/1/3/4");
        }
        let undo = self.0[20];
        if undo > 1 {
            return Err("Invalid original bool representation");
        }
        if tag <= 1 {
            let wall_index = u64::from_le_bytes(payload[48..56].try_into().unwrap());
            let create = (tag == 0) == (undo == 0);
            self.0[21] = 1;
            return Ok(Some(if create {
                ObservedReplayEvent::Create {
                    wall_index,
                    presence: payload[4..48].try_into().unwrap(),
                    restoring_deleted: tag == 1,
                }
            } else {
                ObservedReplayEvent::Delete { wall_index }
            }));
        }
        let elapsed = self.elapsed();
        let event = if elapsed == 0.0 {
            let wall_index = u64::from_le_bytes(payload[8..16].try_into().unwrap());
            let selected = payload[2 - undo as usize];
            Some(if tag == 3 {
                ObservedReplayEvent::Direction {
                    wall_index,
                    direction: selected,
                }
            } else {
                ObservedReplayEvent::FlatRimStyle {
                    wall_index,
                    style: selected,
                }
            })
        } else {
            None
        };
        if u32::from_le_bytes(self.0[..4].try_into().unwrap()) == 1 {
            let advanced = invalid_result(elapsed + delta_time, elapsed, delta_time);
            self.0[16..20].copy_from_slice(&advanced.to_le_bytes());
            let duration = get(&self.0, 4);
            let ratio = invalid_result(advanced / duration, advanced, duration);
            // MAXSS(0,ratio), then MINSS(1,nonnegative), then ordered equality.
            let nonnegative = if 0.0 > ratio { 0.0 } else { ratio };
            let bounded = if 1.0 < nonnegative { 1.0 } else { nonnegative };
            if bounded == 1.0 {
                self.0[21] = 1;
            }
        } else {
            self.0[21] = 1;
        }
        Ok(event)
    }

    pub fn advance_direction(
        &mut self,
        edit: &RoofDirectionEdit,
        delta_time: f32,
    ) -> Option<ObservedReplayEvent> {
        let bit = |direction: &RidgeDirection| match direction {
            RidgeDirection::Width => 0,
            RidgeDirection::Length => 1,
        };
        let mut payload = [0; 64];
        payload[0] = 3;
        payload[1] = bit(&edit.from);
        payload[2] = bit(&edit.to);
        payload[8..16].copy_from_slice(&edit.wall_index.to_le_bytes());
        self.advance_discrete_observed(&payload, delta_time)
            .expect("valid direction payload")
    }

    /// Delta time is the original function's XMM1 input. Completion is sticky.
    pub fn advance(&mut self, edit: &RoofVisualEdit, delta_time: f32) -> RoofEditEvent {
        let mode = u32::from_le_bytes(self.0[0..4].try_into().unwrap());
        let mut progress = 1.0;
        if mode & 1 != 0 {
            let elapsed = self.elapsed() + delta_time;
            self.0[16..20].copy_from_slice(&elapsed.to_le_bytes());
            let duration = get(&self.0, 4);
            // Original DIVSS produces the x86 indefinite NaN for 0/0. Keep
            // that bit pattern across hosts; MAXSS then selects its second
            // operand and UCOMISS/JB takes the unordered branch.
            let ratio = if elapsed == 0.0 && duration == 0.0 {
                f32::from_bits(0xffc00000)
            } else {
                elapsed / duration
            };
            let nonnegative = if 0.0 > ratio { 0.0 } else { ratio };
            if nonnegative < 1.0 || nonnegative.is_nan() {
                progress = nonnegative;
            } else {
                self.0[21] = 1;
            }
        } else {
            self.0[21] = 1;
        }
        if self.0[20] != 0 {
            progress = 1.0 - progress;
        }
        let complement = 1.0 - progress;
        let blend = |before: f32, after: f32| after * progress + before * complement;
        RoofEditEvent {
            profile: ScalarEdit {
                mode: 1,
                value: blend(edit.prev_profile, edit.new_profile),
            },
            height: ScalarEdit {
                mode: 1,
                value: blend(edit.prev_height, edit.new_height),
            },
            ridge: ScalarEdit {
                mode: 1,
                value: blend(edit.prev_ridge, edit.new_ridge),
            },
            eave: ScalarEdit {
                mode: 1,
                value: progress * edit.new_eave + complement * edit.prev_eave,
            },
            wall_id: edit.wall_index,
            tip: TipEdit {
                mode: 1,
                value: Vec2::new(
                    progress * edit.new_tip_offset[0] + complement * edit.prev_tip_offset[0],
                    progress * edit.new_tip_offset[1] + complement * edit.prev_tip_offset[1],
                ),
            },
            source_flag: 1,
        }
    }
}

fn get(bytes: &[u8], offset: usize) -> f32 {
    f32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap())
}
fn invalid_result(value: f32, a: f32, b: f32) -> f32 {
    if value.is_nan() && !a.is_nan() && !b.is_nan() {
        f32::from_bits(0xffc00000)
    } else {
        value
    }
}

/// Observed RoofEdit enum payload, excluding the outer history entry's 16 bytes.
pub fn visual_edit_from_observed(bytes: &[u8; 64]) -> Result<RoofVisualEdit, &'static str> {
    if bytes[0] != 2 {
        return Err("Expected original B RoofEdit::VisualEdit tag 2");
    }
    Ok(RoofVisualEdit {
        wall_index: u64::from_le_bytes(bytes[40..48].try_into().unwrap()),
        prev_height: get(bytes, 4),
        prev_profile: get(bytes, 8),
        prev_ridge: get(bytes, 12),
        prev_eave: get(bytes, 16),
        new_height: get(bytes, 20),
        new_profile: get(bytes, 24),
        new_ridge: get(bytes, 28),
        new_eave: get(bytes, 32),
        prev_tip_offset: [get(bytes, 48), get(bytes, 52)],
        new_tip_offset: [get(bytes, 56), get(bytes, 60)],
        extensions: Default::default(),
    })
}

/// Serialize only confirmed RoofEditEvent fields, retaining caller-owned padding.
pub fn write_observed_event(event: &RoofEditEvent, bytes: &mut [u8; 64]) {
    for (offset, field) in [
        (0, event.profile),
        (8, event.height),
        (16, event.ridge),
        (24, event.eave),
    ] {
        bytes[offset..offset + 4].copy_from_slice(&field.mode.to_le_bytes());
        bytes[offset + 4..offset + 8].copy_from_slice(&field.value.to_le_bytes());
    }
    bytes[32..40].copy_from_slice(&event.wall_id.to_le_bytes());
    bytes[40..44].copy_from_slice(&event.tip.mode.to_le_bytes());
    bytes[44..48].copy_from_slice(&event.tip.value.x.to_le_bytes());
    bytes[48..52].copy_from_slice(&event.tip.value.y.to_le_bytes());
    bytes[52] = event.source_flag;
}
