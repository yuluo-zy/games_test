//! B resolved wall-bulldoze business: actual snapshot groups and animation.
//! UI hit-testing/selection supplies the observed wall, slice range and anchor.

// Preserve the original instruction-level expression order/NaN policy, API call shapes, and evidence/test transcription. See README.md.
#![allow(clippy::manual_clamp)]
use crate::resources::history::building_transactions::RoofHistoryEdit;
use crate::resources::history::wall_edit::WallAuthoritySnapshot;
use crate::resources::history::wall_edit::WallCompoundHistoryEdit;
use crate::resources::history::wall_edit::WallHistoryEdit;
use crate::resources::history::wall_edit::WallReplayAudio;
use crate::resources::history::wall_edit::WallSnapshotError;
use crate::resources::history::wall_edit::slice_source_curve;
use crate::resources::walls::publication_fields::WallId;
use crate::resources::walls::publication_fields::WallShapeSnapshot;

#[derive(Clone, Debug)]
pub struct WallBulldozeHistory {
    /// The original caller records RoofDelete FIRST, then this Wall compound.
    pub roof: Option<RoofHistoryEdit>,
    pub wall: WallCompoundHistoryEdit,
}
pub fn record_wall_bulldoze(
    wall: WallId,
    previous: &WallAuthoritySnapshot,
    low: f32,
    high: f32,
    color_id: u32,
    roof: Option<&[u8; 88]>,
) -> Result<WallBulldozeHistory, WallSnapshotError> {
    let mut next = previous.clone();
    next.source.curve = slice_source_curve(&previous.source.curve, low, high)?;
    next.source.shape = WallShapeSnapshot::from_observed({
        let mut b = [0; 28];
        b[..4].copy_from_slice(&2u32.to_le_bytes());
        b
    });
    next.source.curve_kind_tag = 2;
    let mut op = [0; 12];
    op[..4].copy_from_slice(&1u32.to_le_bytes());
    op[4..8].copy_from_slice(&low.to_le_bytes());
    op[8..12].copy_from_slice(&high.to_le_bytes());
    // The original uses the selected WallId, not the queried Roof row's ID.
    let roof = roof.map(|row| {
        let observed = RoofHistoryEdit::delete_from_roof(row);
        RoofHistoryEdit::delete(wall.0, observed.payload()[4..48].try_into().unwrap())
    });
    Ok(WallBulldozeHistory {
        roof,
        wall: WallCompoundHistoryEdit {
            groups: vec![
                vec![WallHistoryEdit::StateChange {
                    wall,
                    previous: previous.clone(),
                    next: next.clone(),
                    observed_operation: op,
                }],
                vec![WallHistoryEdit::Delete {
                    wall,
                    restored_height: next.source.extra,
                    source: next,
                    color_id,
                }],
            ],
        },
    })
}
#[derive(Clone, Debug)]
pub struct WallBulldozeCommand {
    pub original: WallAuthoritySnapshot,
    pub wall: WallId,
    pub low: f32,
    pub high: f32,
    pub anchor: [f32; 3],
    /// Original phase is a fractional progress, not elapsed seconds.
    pub phase: f32,
    pub finished: bool,
}
#[derive(Clone, Debug, Default)]
pub struct WallBulldozeOutcome {
    pub updated: Option<WallId>,
    pub removed: Option<WallId>,
    pub audio: Option<WallReplayAudio>,
}
fn add(a: f32, b: f32) -> f32 {
    let v = a + b;
    if v.is_nan() && !a.is_nan() && !b.is_nan() {
        f32::from_bits(0xffc00000)
    } else {
        v
    }
}
fn ease(t: f32) -> f32 {
    if t < 0.5 {
        ((4. * t) * t) * t
    } else {
        let x = (t + t) - 2.;
        ((0.5 * x) * x) * x + 1.
    }
}
impl WallBulldozeCommand {
    /// Actual GameTime+58 step, with .2 duration. Progress advances BEFORE
    /// applying this animation; missing authority marks the buffer row finished.
    pub fn advance(
        &mut self,
        sources: &mut std::collections::HashMap<WallId, WallAuthoritySnapshot>,
        dt: f32,
    ) -> Result<WallBulldozeOutcome, WallSnapshotError> {
        let mut out = WallBulldozeOutcome::default();
        if self.finished {
            return Ok(out);
        }
        if !sources.contains_key(&self.wall) {
            self.finished = true;
            return Ok(out);
        }
        let p = add(dt / 0.2, self.phase);
        self.phase = if p < 0. {
            0.
        } else if p > 1. {
            1.
        } else {
            p
        };
        if self.phase == 1. {
            sources.remove(&self.wall);
            out.removed = Some(self.wall);
            self.finished = true;
            return Ok(out);
        }
        let e = ease(self.phase);
        let low = add(self.low * e, 0. * (1. - e));
        let high = add(1. - e, self.high * e);
        let mut current = self.original.clone();
        current.source.curve = slice_source_curve(&current.source.curve, low, high)?;
        current.source.shape = WallShapeSnapshot::from_observed({
            let mut b = [0; 28];
            b[..4].copy_from_slice(&2u32.to_le_bytes());
            b
        });
        current.source.curve_kind_tag = 2;
        current.source.style = u8::from(self.original.source.style == 1);
        sources.insert(self.wall, current);
        out.updated = Some(self.wall);
        out.audio = Some(WallReplayAudio {
            wall: self.wall,
            anchor: self.anchor,
            kind: 0,
            flag: 0,
        });
        Ok(out)
    }
}
