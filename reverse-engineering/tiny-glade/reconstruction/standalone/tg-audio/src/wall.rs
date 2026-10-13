//! wall_audio_fmod tables (B §1; init A §3a). VA = jump-table target.

/// wall_audio_init_system 0x14094fb80: 10 `instantiate_loop` calls in this order (A §3a).
/// (name, doc string length).
pub const WALL_LOOP_NAMES: [(&str, usize); 10] = [
    ("event:/SFX/Wall Draw", 0x14),
    ("event:/SFX/Wall Change Height", 0x1d),
    ("event:/SFX/FenceExtend", 0x16),
    ("event:/SFX/LintelExtend", 0x17),
    ("event:/SFX/StoneArchExtend", 0x1a),
    ("event:/SFX/WideningGate", 0x17),
    ("event:/SFX/PlatformStairsExtend", 0x1f),
    ("event:/SFX/SewerGateWiden", 0x19),
    ("event:/SFX/FenceHeight", 0x16),
    ("event:/SFX/FenceMove", 0x14),
];
/// `AudioLoopDesc.count` / `.max` (A §3a; semantics UNKNOWN(evidence: A §9)).
pub const LOOP_DESC_COUNT: u64 = 3;
pub const LOOP_DESC_MAX: u32 = 0x32;

/// Parameter set on the loop element after `request_play`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WallParam {
    /// `inverse_lerp(lo, hi, event.field_c)` clamped.
    Range { name: &'static str, lo: f32, hi: f32 },
    /// kind 10: `"HasBricks"` = event byte @+9 != 0 ? 1.0 : 0.0.
    HasBricks,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WallAction {
    /// `AudioLoopPlayer::request_play` on loop `loop_idx` (index into `WALL_LOOP_NAMES`).
    Loop { loop_idx: usize, param: Option<WallParam> },
    /// `instantiate_audio_w_position_and_volume` (0x140aa3590) with this name.
    OneShot { name: &'static str, len: usize },
}

#[derive(Clone, Copy, Debug)]
pub struct WallKindRow {
    pub kind: u8,
    pub va: u64,
    pub action: WallAction,
}

const fn lp(loop_idx: usize, param: Option<WallParam>) -> WallAction {
    WallAction::Loop { loop_idx, param }
}
const fn os(name: &'static str, len: usize) -> WallAction {
    WallAction::OneShot { name, len }
}
const fn row(kind: u8, va: u64, action: WallAction) -> WallKindRow {
    WallKindRow { kind, va, action }
}

/// PlayWallAudio kind 0..=23 (B §1 table).
pub const WALL_KINDS: [WallKindRow; 24] = [
    row(0, 0x140950dce, lp(0, None)),
    row(1, 0x1409510d8, lp(1, Some(WallParam::Range { name: "WallHeight", lo: 0.5, hi: 14.0 }))),
    row(2, 0x140950f1f, lp(2, Some(WallParam::Range { name: "FenceLength", lo: 8.0, hi: 50.0 }))),
    row(3, 0x140950ffc, os("event:/SFX/FenceAppear", 0x16)),
    row(4, 0x140950e77, os("event:/SFX/FenceMerge", 0x15)),
    row(5, 0x1409511d8, os("event:/SFX/FenceRearrange", 0x19)),
    row(6, 0x140951282, lp(4, None)),
    row(7, 0x140951034, lp(3, Some(WallParam::Range { name: "FenceLength", lo: 3.0, hi: 15.0 }))),
    row(8, 0x14095137c, os("event:/SFX/LintelAppear", 0x17)),
    row(9, 0x140950ee7, os("event:/SFX/LintelRearrange", 0x1a)),
    row(10, 0x1409512f6, lp(5, Some(WallParam::HasBricks))),
    row(11, 0x140950e3f, os("event:/SFX/LintelMerge", 0x16)),
    row(12, 0x140950eaf, os("event:/SFX/StoneArchAppear", 0x1a)),
    row(13, 0x140951248, lp(6, None)),
    row(14, 0x140950e07, os("event:/SFX/PillarStoneAppear", 0x1c)),
    row(15, 0x140950fc4, os("event:/SFX/PillarWoodAppear", 0x1b)),
    row(16, 0x140950d45, os("event:/SFX/LintelRearrange", 0x1a)),
    row(17, 0x140951166, os("event:/SFX/GateAppear", 0x15)),
    row(18, 0x1409512bc, lp(7, None)),
    row(19, 0x1409513ec, os("event:/SFX/PlatformStairsAppear", 0x1f)),
    row(20, 0x14095119e, lp(9, None)),
    row(21, 0x140951210, os("event:/SFX/WallSplit", 0x14)),
    row(22, 0x1409513b4, os("event:/SFX/SewerGateAppear", 0x1a)),
    row(23, 0x140951424, lp(8, None)),
];

/// Lookup; kinds >= 24 are outside the jump table (A §9 notes 24+ out of range).
pub fn wall_kind(kind: u8) -> Option<&'static WallKindRow> {
    WALL_KINDS.get(kind as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_is_indexed_by_kind() {
        for (i, r) in WALL_KINDS.iter().enumerate() {
            assert_eq!(r.kind as usize, i);
        }
        assert!(wall_kind(24).is_none());
    }

    #[test]
    fn loop_names_lengths_match_doc() {
        for (n, l) in WALL_LOOP_NAMES {
            assert_eq!(n.len(), l, "{n}");
        }
    }

    #[test]
    fn oneshot_name_lengths_match_doc() {
        for r in WALL_KINDS {
            if let WallAction::OneShot { name, len } = r.action {
                assert_eq!(name.len(), len, "kind {} {name}", r.kind);
            }
        }
    }

    #[test]
    fn loop_kinds_cover_each_loop_once() {
        let mut seen = [0u8; 10];
        for r in WALL_KINDS {
            if let WallAction::Loop { loop_idx, .. } = r.action {
                seen[loop_idx] += 1;
            }
        }
        assert_eq!(seen, [1; 10]);
    }

    #[test]
    fn kinds_9_and_16_share_event() {
        assert_eq!(WALL_KINDS[9].action_name(), WALL_KINDS[16].action_name());
    }

    impl WallKindRow {
        fn action_name(&self) -> Option<&'static str> {
            match self.action {
                WallAction::OneShot { name, .. } => Some(name),
                _ => None,
            }
        }
    }
}
