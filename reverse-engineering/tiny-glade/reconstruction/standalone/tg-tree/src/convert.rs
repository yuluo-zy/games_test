//! `convert_preplaced_trees` (VA 0x14226be70, convert_preplaced_trees.rs:32-137).
//!
//! Evidence level is LOW (tree-tool-extra-analysis.md §1: heavily inlined, only
//! the delay state machine and the water/rock gate shape are confirmed). Only
//! those two pieces are implemented; the spatial-hash query, History edit
//! collection and the arg2 event type are UNKNOWN and deliberately not built.

use bevy_ecs::prelude::*;

/// ResMut<{u64 count @+0, u8 flag @+8}> (arg3 R8). Owner resource UNKNOWN.
#[derive(Resource, Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ConvertState {
    pub count: u64,
    pub flag: bool,
}

pub const CONVERT_LOG: &str = "Adding converted pre_placed tree"; // .data 0x143035608, line 74

impl ConvertState {
    /// One frame of the delay pattern: a SpawnPreplacedTrees frame sets
    /// {count=1, flag=1}; afterwards count increments each quiet frame and when
    /// `count > 2` the flag clears and the conversion block runs.
    /// Returns (new state, run_conversion). The "only while flag set" condition
    /// is an inference from "清标志并执行" (UNKNOWN whether flag guards the count).
    pub fn tick(self, saw_spawn_event: bool) -> (Self, bool) {
        if saw_spawn_event {
            return (Self { count: 1, flag: true }, false);
        }
        if !self.flag {
            return (self, false);
        }
        let count = self.count + 1;
        if count > 2 {
            (Self { count, flag: false }, true)
        } else {
            (Self { count, flag: self.flag }, false)
        }
    }
}

/// Per-tree gate: skipped when in water (sample > 0) or on rock; otherwise the
/// clamped [0,1] sum of 4 samples must be <= 0 for the circle query to run.
/// WHICH of the 6 bilinear samples form the "in water" test vs the 4-sum is
/// UNKNOWN(evidence: tree-tool-extra-analysis.md §1); callers pass them in.
pub fn passes_conversion_gate(water_sample: f32, on_rock: bool, four_sample_sum: f32) -> bool {
    if water_sample > 0.0 || on_rock {
        return false;
    }
    four_sample_sum.clamp(0.0, 1.0) <= 0.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delay_pattern() {
        let s = ConvertState::default();
        let (s, run) = s.tick(true);
        assert_eq!((s, run), (ConvertState { count: 1, flag: true }, false));
        let (s, run) = s.tick(false);
        assert!(!run && s.count == 2 && s.flag);
        let (s, run) = s.tick(false);
        assert!(run && !s.flag);
        let (s2, run) = s.tick(false);
        assert!(!run && s2 == s);
        // event frame restarts
        let (s3, _) = s.tick(true);
        assert_eq!(s3, ConvertState { count: 1, flag: true });
    }

    #[test]
    fn gate() {
        assert!(passes_conversion_gate(0.0, false, 0.0));
        assert!(passes_conversion_gate(0.0, false, -0.5));
        assert!(!passes_conversion_gate(0.1, false, 0.0));
        assert!(!passes_conversion_gate(0.0, true, 0.0));
        assert!(!passes_conversion_gate(0.0, false, 0.2));
    }
}
