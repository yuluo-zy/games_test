//! system-water CPU leaves: rolling audio bits and resolved raster-write callbacks.
//! No world-raster constructor, guessed sampling grid, raycast or zeno replacement.

/// B 0x141baefe0 uses the pre-increment local frame count and masks to 64 bits.
pub fn frame_bit(counter_before: u64) -> u32 {
    (counter_before
        .wrapping_add(counter_before.wrapping_add(1).wrapping_mul(10))
        .wrapping_add(1)
        & 63) as u32
}

/// `bit` must be in 0..64, as produced by frame_bit. `present` is the resolved
/// result of the original distance/water/occlusion queries, supplied by caller.
/// The two counters retain their observed PADDW/PSUBW semantics, not guessed names.
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WriteMode {
    EraseMin,
    Max,
    AddWeighted,
}

/// B 0x142262221..0x1422622c8. None represents the original panic combination;
/// the pure projection reports selection and leaves error handling to its caller.
pub fn select_write_mode(erase_flag_zero: bool, bit0: bool) -> Option<WriteMode> {
    match (erase_flag_zero, bit0) {
        (true, false) => Some(WriteMode::EraseMin),
        (false, false) => Some(WriteMode::Max),
        (false, true) => Some(WriteMode::AddWeighted),
        (true, true) => None,
    }
}

/// Only legal nonnegative zeno coverage is accepted by this unsigned interface.
/// Its absolute fixed-point scale and negative input behavior are not recovered.
pub fn coverage_to_alpha(coverage: u32) -> u8 {
    (coverage >> 9).min(0xff) as u8
}

/// Recovered for finite brush values: clamp to [0,1], then multiply by 65536.
/// Nonfinite brush behavior is outside the accepted input contract.
pub fn brush_fixed(brush: f32) -> u32 {
    (brush.clamp(0.0, 1.0) * 65536.0) as u32
}

/// B callback bodies at 0x142828b30 / 0x142821d90 / 0x142823e50.
/// `weight` must be in 0..=65536, produced by brush_fixed for a finite input.
pub fn apply_write(mode: WriteMode, destination: u8, alpha: u8, weight: u32) -> u8 {
    match mode {
        WriteMode::Max => destination.max(alpha),
        WriteMode::EraseMin => destination.min(255 - alpha),
        WriteMode::AddWeighted => {
            let add = ((u32::from(alpha) * weight) >> 16).min(255) as u8;
            destination.saturating_add(add)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // Cases retained from the original audit source tests; no raster substitute.
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
    #[test]
    fn original_raster_dispatch_and_fixed_point_contract() {
        assert_eq!(select_write_mode(true, false), Some(WriteMode::EraseMin));
        assert_eq!(select_write_mode(false, false), Some(WriteMode::Max));
        assert_eq!(select_write_mode(false, true), Some(WriteMode::AddWeighted));
        assert_eq!(select_write_mode(true, true), None);
        assert_eq!(coverage_to_alpha(0x7f << 9), 0x7f);
        assert_eq!(coverage_to_alpha(0x100 << 9), 0xff);
        assert_eq!(brush_fixed(2.0), 65536);
        assert_eq!(brush_fixed(-1.0), 0);
        assert_eq!(brush_fixed(0.5), 32768);
    }
    #[test]
    fn original_raster_write_callback_results() {
        assert_eq!(apply_write(WriteMode::Max, 10, 200, 0), 200);
        assert_eq!(apply_write(WriteMode::Max, 250, 200, 0), 250);
        assert_eq!(apply_write(WriteMode::EraseMin, 255, 200, 0), 55);
        assert_eq!(apply_write(WriteMode::EraseMin, 20, 200, 0), 20);
        assert_eq!(apply_write(WriteMode::AddWeighted, 100, 100, 65536), 200);
        assert_eq!(apply_write(WriteMode::AddWeighted, 100, 100, 32768), 150);
        assert_eq!(apply_write(WriteMode::AddWeighted, 200, 200, 65536), 255);
    }
}
