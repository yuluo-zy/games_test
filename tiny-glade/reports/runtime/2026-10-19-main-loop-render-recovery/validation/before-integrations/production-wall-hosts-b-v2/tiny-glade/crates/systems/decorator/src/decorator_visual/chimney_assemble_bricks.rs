//! B canonical chimney payload and explicit wall-diff consumer protocol.
//!
//! The original 1407f8db0/117B entry marks the real ResMut changed and passes a
//! resource closure to with_diffs 1412ae660. This adapter preserves that real
//! resource borrow and requires a complete, explicitly supplied wall consumer.
//! It is not the original seven-SystemParam geometry/audio system. Its real
//! PublicWalls/SSBO/terrain/derived-info/audio host remains a separate gate.

use bevy_ecs::change_detection::ResMut;
use country_core::resources::walls::WallId;

use super::deco_differential_state::{ConsumedDiffs, DecoDiffLog, WallAttachedDecoTracker};

/// Exact 40-byte carrier. Field meanings beyond the native offset contract are
/// not invented. 140824fe0/172B proves the comparison used by fullvisual.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ChimneyAssemblyParams {
    pub words: [u32; 10],
}

impl PartialEq for ChimneyAssemblyParams {
    fn eq(&self, other: &Self) -> bool {
        (2..8).all(|i| f32::from_bits(self.words[i]) == f32::from_bits(other.words[i]))
            && self.words[0] == other.words[0]
            && (self.words[0] == 1
                || f32::from_bits(self.words[1]) == f32::from_bits(other.words[1]))
            && self.words[8] == other.words[8]
            && f32::from_bits(self.words[9]) == f32::from_bits(other.words[9])
    }
}

const _: () = assert!(std::mem::size_of::<ChimneyAssemblyParams>() == 40);
const _: () = assert!(std::mem::align_of::<ChimneyAssemblyParams>() == 4);

/// Explicit protocol seam. The callback must perform the real current-value
/// work and explicitly consume removals; no default or empty host is supplied.
/// Register the actual seven-parameter system only when that host is restored.
pub fn chimney_assemble_bricks<F>(
    mut tracker: ResMut<'_, WallAttachedDecoTracker<ChimneyAssemblyParams>>,
    consume_wall: F,
) where
    F: for<'a> FnMut(
        WallId,
        DecoDiffLog<'a, ChimneyAssemblyParams>,
    ) -> ConsumedDiffs<'a, ChimneyAssemblyParams>,
{
    tracker.with_diffs(consume_wall);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_float_equality_and_tag_one_inactive_word() {
        let a = ChimneyAssemblyParams { words: [0; 10] };
        let mut b = a;
        b.words[2] = (-0.0f32).to_bits();
        assert_eq!(a, b);
        b.words[2] = f32::NAN.to_bits();
        assert_ne!(b, b);
        let mut c = a;
        c.words[0] = 1;
        let mut d = c;
        d.words[1] = f32::NAN.to_bits();
        assert_eq!(c, d);
        d.words[8] = 1;
        assert_ne!(c, d);
    }
}
