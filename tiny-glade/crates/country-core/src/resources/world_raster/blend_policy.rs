//! Resolved raster write dispatch, coverage and brush CPU policies.
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
