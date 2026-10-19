//! B canonical door-stair payload and explicit wall-diff consumer protocol.
//!
//! Original entry 1407ff130/52B marks the real ResMut changed and forwards its
//! resource closure to with_diffs 1412af3a0. This is a protocol adapter, not the
//! original four-SystemParam wall-geometry/signifier host. No empty host is
//! supplied or registered here.

use bevy_ecs::change_detection::ResMut;
use country_core::resources::walls::WallId;

use super::deco_differential_state::{ConsumedDiffs, DecoDiffLog, WallAttachedDecoTracker};

/// Original add clones 32 bytes, and 140825150/152B compares eight ordered f32s.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct DoorStairsAssemblyParams {
    pub words: [u32; 8],
}

impl PartialEq for DoorStairsAssemblyParams {
    fn eq(&self, other: &Self) -> bool {
        self.words
            .iter()
            .zip(other.words.iter())
            .all(|(a, b)| f32::from_bits(*a) == f32::from_bits(*b))
    }
}

const _: () = assert!(std::mem::size_of::<DoorStairsAssemblyParams>() == 32);
const _: () = assert!(std::mem::align_of::<DoorStairsAssemblyParams>() == 4);

/// The real wall consumer must build all current values, then explicitly choose
/// the original Door no-op removal callback. A missing host is not success.
pub fn door_stairs_assemble_bricks<F>(
    mut tracker: ResMut<'_, WallAttachedDecoTracker<DoorStairsAssemblyParams>>,
    consume_wall: F,
) where
    F: for<'a> FnMut(
        WallId,
        DecoDiffLog<'a, DoorStairsAssemblyParams>,
    ) -> ConsumedDiffs<'a, DoorStairsAssemblyParams>,
{
    tracker.with_diffs(consume_wall);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_ordered_float_equality_is_not_bit_equality() {
        let a = DoorStairsAssemblyParams { words: [0; 8] };
        let mut b = a;
        b.words[7] = (-0.0f32).to_bits();
        assert_eq!(a, b);
        b.words[7] = f32::NAN.to_bits();
        assert_ne!(b, b);
    }
}
