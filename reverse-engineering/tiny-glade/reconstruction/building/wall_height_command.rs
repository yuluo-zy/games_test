//! Original B AdjustWallHeightCmd consumer after authority/public/query owners
//! are resolved. The command's observed flag controls audio policy; this system
//! has no History/AppMode parameter and does not record a history edit itself.
use crate::{
    building_wall_collection::TerrainHeightGrid512,
    building_wall_height_authority::{WallHeightAuthorityOutcome, assign_resolved_height},
    building_wall_state::WallId,
    surface::Curve2,
    wall_height_rules::clamp_resolved_wall_height,
};

#[derive(Clone, Copy, Debug)]
pub struct AdjustWallHeightRequest {
    pub anchor: Option<[f32; 3]>,
    pub mode: u32,
    pub value: f32,
    pub wall: WallId,
    pub observed_audio_flag: u8,
}
impl AdjustWallHeightRequest {
    /// Actual game payload40, preceding the EventId in the native48-byte row.
    pub fn from_observed(payload: &[u8; 40]) -> Self {
        let u = |o| u32::from_le_bytes(payload[o..o + 4].try_into().unwrap());
        Self {
            anchor: if u(0) & 1 != 0 {
                Some([
                    f32::from_bits(u(4)),
                    f32::from_bits(u(8)),
                    f32::from_bits(u(12)),
                ])
            } else {
                None
            },
            mode: u(16),
            value: f32::from_bits(u(20)),
            wall: WallId(u64::from_le_bytes(payload[24..32].try_into().unwrap())),
            observed_audio_flag: payload[32],
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub struct ResolvedWallHeightClamp {
    pub roof_height_01: Option<f32>,
    pub special_flag: u8,
}
#[derive(Clone, Copy, Debug)]
pub enum WallHeightFeedback {
    WallMaterial {
        wall: WallId,
        anchor_world: [f32; 3],
        observed_kind: u8,
        observed_flag: u8,
    },
    Building {
        wall: WallId,
        anchor_world: [f32; 3],
        observed_kind: u8,
        intensity: f32,
    },
}
#[derive(Clone, Debug)]
pub struct WallHeightCommandOutcome {
    pub previous_height: f32,
    pub new_height: f32,
    pub anchor_world: [f32; 3],
    pub authority_updates: Vec<WallHeightAuthorityOutcome>,
    pub feedback: WallHeightFeedback,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WallHeightCommandError {
    UnsupportedScalarMode(u32),
    MissingAuthority,
    InvalidCurve,
}

fn native_divide(a: f32, b: f32) -> f32 {
    let value = a / b;
    if value.is_nan() && !a.is_nan() && !b.is_nan() {
        f32::from_bits(0xffc00000)
    } else {
        value
    }
}

pub fn apply_resolved_height_command(
    request: &AdjustWallHeightRequest,
    authority_height: &mut f32,
    source_curve_kind: u32,
    source_curve: &Curve2,
    terrain: &TerrainHeightGrid512,
    public_clamp: Option<ResolvedWallHeightClamp>,
    delta_time: f32,
) -> Result<Option<WallHeightCommandOutcome>, WallHeightCommandError> {
    if request.mode == 0 {
        return Ok(None);
    }
    if request.mode != 1 {
        return Err(WallHeightCommandError::UnsupportedScalarMode(request.mode));
    }
    let previous = *authority_height;
    let desired = public_clamp
        .map(|clamp| {
            clamp_resolved_wall_height(request.value, clamp.roof_height_01, clamp.special_flag)
        })
        .unwrap_or(request.value);
    // Original UCOMISS/JZ also skips unordered operands, unlike Rust !=.
    if desired == previous || desired.is_nan() || previous.is_nan() {
        return Ok(None);
    }
    let mut anchor = if let Some(anchor) = request.anchor {
        anchor
    } else {
        if source_curve.points.len() < 2 {
            return Err(WallHeightCommandError::InvalidCurve);
        }
        let point = source_curve.pos_at_u(0.5);
        [point[0], 0., point[1]]
    };
    anchor[1] += terrain.sample_at_world_xz(anchor[0], anchor[2]);
    let mut updates = vec![assign_resolved_height(
        authority_height,
        request.wall,
        desired,
    )];
    let feedback = if source_curve_kind < 2 && request.observed_audio_flag == 0 {
        // Original regular-wall/audio branch reassigns and dirties a second
        // time. The set deduplicates domain IDs; call count remains business.
        updates.push(assign_resolved_height(
            authority_height,
            request.wall,
            desired,
        ));
        let rate = native_divide((previous - desired).abs(), delta_time);
        let amount = native_divide(rate - 0., 100. - 0.);
        let nonnegative = if 0. > amount { 0. } else { amount };
        let intensity = if 1. < nonnegative { 1. } else { nonnegative };
        WallHeightFeedback::Building {
            wall: request.wall,
            anchor_world: anchor,
            observed_kind: 3,
            intensity,
        }
    } else {
        WallHeightFeedback::WallMaterial {
            wall: request.wall,
            anchor_world: anchor,
            observed_kind: 0,
            observed_flag: if source_curve_kind < 2 {
                1
            } else {
                request.observed_audio_flag
            },
        }
    };
    Ok(Some(WallHeightCommandOutcome {
        previous_height: previous,
        new_height: desired,
        anchor_world: anchor,
        authority_updates: updates,
        feedback,
    }))
}
