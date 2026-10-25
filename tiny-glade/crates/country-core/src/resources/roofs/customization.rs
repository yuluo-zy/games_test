//! B system_color::set_roof_customization's resolved game consumers.
//! PublicWalls/ECS resolution and event/Arc/History container implementations
//! remain dependency/owner boundaries. Missing associations/query rows skip.
use crate::resources::walls::publication_fields::WallId;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlatRoofStyleObserved([u8; 12]);
impl FlatRoofStyleObserved {
    pub fn from_observed(bytes: [u8; 12]) -> Self {
        Self(bytes)
    }
    pub fn bytes(&self) -> &[u8; 12] {
        &self.0
    }
    pub fn tag(&self) -> u8 {
        self.0[0]
    }
    /// Only tag2 carries compared fields. Padding and inactive union payloads
    /// are ignored for notification equality but copied by the assignment.
    pub fn notification_eq(&self, other: &Self) -> bool {
        self.tag() == other.tag()
            && (self.tag() != 2
                || (self.0[1] == other.0[1]
                    && self.0[4..8] == other.0[4..8]
                    && self.0[8..12] == other.0[8..12]))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlatRoofStyleRequest {
    pub wall_id: WallId,
    pub style: FlatRoofStyleObserved,
    /// The original tests exactly byte == 1, rather than != 0.
    pub observed_play_sound: u8,
}
impl FlatRoofStyleRequest {
    /// EventInstance32: message ID0..8, WallId8..16, style16..28,
    /// sound flag28, remaining padding unchanged/uninterpreted.
    pub fn from_event_record(bytes: &[u8; 32]) -> Self {
        Self {
            wall_id: WallId(u64::from_le_bytes(bytes[8..16].try_into().unwrap())),
            style: FlatRoofStyleObserved::from_observed(bytes[16..28].try_into().unwrap()),
            observed_play_sound: bytes[28],
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlatRoofRimRequest {
    pub wall_id: WallId,
    pub from: u8,
    pub to: u8,
    /// Independent from play_sound. Replay writes zero here.
    pub observed_record_history: u8,
}
impl FlatRoofRimRequest {
    /// EventInstance24: WallId8..16, from16, to17, record-history18.
    pub fn from_event_record(bytes: &[u8; 24]) -> Self {
        Self {
            wall_id: WallId(u64::from_le_bytes(bytes[8..16].try_into().unwrap())),
            from: bytes[16],
            to: bytes[17],
            observed_record_history: bytes[18],
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RoofCustomizationSound {
    pub observed_kind: u8,
    pub pivot: [f32; 3],
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RoofRimHistoryMutation {
    pub observed_tag: u8,
    pub wall_id: WallId,
    pub from: u8,
    pub to: u8,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RoofCustomizationOutcome {
    pub state_changed: bool,
    pub recalculate_wall_holes: Option<WallId>,
    pub sound: Option<RoofCustomizationSound>,
    /// Original Arc<Mutation> payload supplied to History::add. History
    /// recording/commit/cursor and Arc headers are not reconstructed here.
    pub history_mutation: Option<RoofRimHistoryMutation>,
}

fn pivot(roof: &[u8; 88]) -> [f32; 3] {
    let shape = u32::from_le_bytes(roof[8..12].try_into().unwrap()) as usize;
    assert!(shape <= 1, "unrecovered Roof shape for pivot");
    let f = |offset| f32::from_le_bytes(roof[offset..offset + 4].try_into().unwrap());
    [f(12 + shape * 8), f(0x4c), f(16 + shape * 8)]
}

/// Executes for any successfully resolved Roof, including marked/animated or
/// pitched roofs. The original Query<&mut Roof> has no deletion filter and the
/// consumer has no roof-height/type guard. Equal styles still assign and mark.
pub fn apply_flat_style_resolved(
    roof: &mut [u8; 88],
    request: FlatRoofStyleRequest,
    roof_tick: &mut u32,
    current_tick: u32,
) -> RoofCustomizationOutcome {
    let old = FlatRoofStyleObserved::from_observed(roof[0x40..0x4c].try_into().unwrap());
    let changed = !old.notification_eq(&request.style);
    *roof_tick = current_tick;
    roof[0x40..0x4c].copy_from_slice(request.style.bytes());
    RoofCustomizationOutcome {
        state_changed: changed,
        recalculate_wall_holes: changed.then_some(request.wall_id),
        sound: (request.observed_play_sound == 1).then(|| RoofCustomizationSound {
            observed_kind: 11,
            pivot: pivot(roof),
        }),
        history_mutation: None,
    }
}

/// Rim changes the parameter byte3d, never the 12-byte FlatRoofStyle block.
/// History's from value comes from the command even when it differs from the
/// current Roof byte; record-history ==1 records even a semantically equal edit.
pub fn apply_flat_rim_resolved(
    roof: &mut [u8; 88],
    request: FlatRoofRimRequest,
    roof_tick: &mut u32,
    current_tick: u32,
) -> RoofCustomizationOutcome {
    let changed = roof[0x3d] != request.to;
    *roof_tick = current_tick;
    roof[0x3d] = request.to;
    RoofCustomizationOutcome {
        state_changed: changed,
        recalculate_wall_holes: changed.then_some(request.wall_id),
        sound: None,
        history_mutation: (request.observed_record_history == 1).then_some(
            RoofRimHistoryMutation {
                observed_tag: 4,
                wall_id: request.wall_id,
                from: request.from,
                to: request.to,
            },
        ),
    }
}
