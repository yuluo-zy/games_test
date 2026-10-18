//! Rolling audio-bit projection retained from the earlier CPU recovery.
//! The counter helper uses unit increments, unlike the native jitter-weighted
//! sums. It has no live producer; native sum semantics remain a separate gap.

/// B 0x141baefe0 uses the pre-increment local frame count and masks to 64 bits.
pub fn frame_bit(counter_before: u64) -> u32 {
    (counter_before
        .wrapping_add(counter_before.wrapping_add(1).wrapping_mul(10))
        .wrapping_add(1)
        & 63) as u32
}

/// `bit` must be in 0..64, as produced by frame_bit. `present` is the resolved
/// result of the original distance/water/occlusion queries, supplied by caller.
/// The two projected counters wrap at u16 boundaries. This unit-increment
/// helper does not implement the original jitter-weighted PADDW/PSUBW operands.
pub fn apply_audio_bit(mask: &mut u64, counters: &mut [u16; 2], bit: u32, present: bool) {
    let selected = 1u64 << bit;
    let was_present = *mask & selected != 0;
    if present {
        *mask |= selected;
        if !was_present {
            *counters = counters.map(|value| value.wrapping_add(1));
        }
    } else {
        *mask &= !selected;
        if was_present {
            *counters = counters.map(|value| value.wrapping_sub(1));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn original_frame_bit_and_wrapping_counter_contract() {
        assert_eq!([frame_bit(0), frame_bit(1), frame_bit(2)], [11, 22, 33]);
        let mut mask = 1;
        let mut counters = [0, u16::MAX];
        apply_audio_bit(&mut mask, &mut counters, 0, false);
        assert_eq!((mask, counters), (0, [u16::MAX, u16::MAX - 1]));
    }
    #[test]
    fn original_counters_change_only_on_bit_transition() {
        let mut mask = 0;
        let mut counters = [0, 0];
        apply_audio_bit(&mut mask, &mut counters, 3, true);
        apply_audio_bit(&mut mask, &mut counters, 3, true);
        assert_eq!((mask, counters), (8, [1, 1]));
        apply_audio_bit(&mut mask, &mut counters, 3, false);
        apply_audio_bit(&mut mask, &mut counters, 3, false);
        assert_eq!((mask, counters), (0, [0, 0]));
    }
}
