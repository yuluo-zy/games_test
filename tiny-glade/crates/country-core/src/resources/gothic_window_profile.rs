//! gothic_window_profile.rs — Gothic 窗/门拱剖面曲线（WP21）。
//!
//! O：`gothic_half_dormer_profile_curve`（0x1409a4cc0）与 `gothic_door_profile_curve`
//! （0x1409a4e70）：size 1/2/3 → Vec<Vec2> 剖面点（x=归一化横向，y=高）。点列逐 f32
//! 转录（O 机器码常量表）。

use glam::Vec2;

/// `gothic_half_dormer_profile_curve(size)`（O 0x1409a4cc0）。
pub fn gothic_half_dormer_profile_curve(size: u8) -> Vec<Vec2> {
    let pts: &[Vec2] = match size {
        1 => &[
            Vec2::new(0.0, 1.26),
            Vec2::new(0.2, 1.09),
            Vec2::new(0.35, 0.87),
            Vec2::new(0.44, 0.62),
            Vec2::new(0.48, 0.36),
            Vec2::new(0.48, -0.95),
        ],
        2 => &[
            Vec2::new(0.0, 1.7),
            Vec2::new(0.21, 1.6),
            Vec2::new(0.39, 1.46),
            Vec2::new(0.54, 1.29),
            Vec2::new(0.65, 1.09),
            Vec2::new(0.72, 0.88),
            Vec2::new(0.75, 0.65),
            Vec2::new(0.75, -0.95),
        ],
        3 => &[
            Vec2::new(0.0, 2.25),
            Vec2::new(0.21, 2.14),
            Vec2::new(0.4, 2.0),
            Vec2::new(0.57, 1.82),
            Vec2::new(0.71, 1.63),
            Vec2::new(0.82, 1.42),
            Vec2::new(0.9, 1.19),
            Vec2::new(0.95, 0.96),
            Vec2::new(0.96, 0.72),
            Vec2::new(0.96, -0.95),
        ],
        _ => panic!("gothic half dormer profile size"),
    };
    pts.to_vec()
}

/// `gothic_door_profile_curve(size)`（O 0x1409a4e70）。
pub fn gothic_door_profile_curve(size: u8) -> Vec<Vec2> {
    let pts: &[Vec2] = match size {
        1 => &[
            Vec2::new(0.0, 1.68),
            Vec2::new(0.32, 1.47),
            Vec2::new(0.5, 1.17),
            Vec2::new(0.5, 0.82),
            Vec2::new(0.54, 0.44),
            Vec2::new(0.54, -1.48),
        ],
        2 => &[
            Vec2::new(0.0, 1.75),
            Vec2::new(0.32, 1.57),
            Vec2::new(0.58, 1.31),
            Vec2::new(0.8, 1.0),
            Vec2::new(0.8, -1.57),
        ],
        3 => &[
            Vec2::new(0.0, 1.9),
            Vec2::new(0.34, 1.75),
            Vec2::new(0.64, 1.53),
            Vec2::new(0.9, 1.26),
            Vec2::new(1.1, 0.59),
            Vec2::new(1.1, -1.65),
        ],
        _ => panic!("gothic door profile size"),
    };
    pts.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Independent literal words from the saved native decompilation in
    // wp21/evidence/g8-misc-2.result.json, entries 0x1409a4cc0/0x1409a4e70.
    // The low dword is X and the high dword Y. These fixtures catch the old
    // door-profile transcription errors (-0.148 and -0.6).
    fn assert_words(points: Vec<Vec2>, words: &[u64]) {
        let actual: Vec<_> = points
            .into_iter()
            .map(|point| u64::from(point.x.to_bits()) | (u64::from(point.y.to_bits()) << 32))
            .collect();
        assert_eq!(actual, words);
    }

    #[test]
    fn half_dormer_points_match_observed_native_words() {
        assert_words(
            gothic_half_dormer_profile_curve(1),
            &[
                0x3fa147ae00000000,
                0x3f8b851f3e4ccccd,
                0x3f5eb8523eb33333,
                0x3f1eb8523ee147ae,
                0x3eb851ec3ef5c28f,
                0xbf7333333ef5c28f,
            ],
        );
        assert_words(
            gothic_half_dormer_profile_curve(2),
            &[
                0x3fd9999a00000000,
                0x3fcccccd3e570a3d,
                0x3fbae1483ec7ae14,
                0x3fa51eb83f0a3d71,
                0x3f8b851f3f266666,
                0x3f6147ae3f3851ec,
                0x3f2666663f400000,
                0xbf7333333f400000,
            ],
        );
        assert_words(
            gothic_half_dormer_profile_curve(3),
            &[
                0x4010000000000000,
                0x4008f5c33e570a3d,
                0x400000003ecccccd,
                0x3fe8f5c33f11eb85,
                0x3fd0a3d73f35c28f,
                0x3fb5c28f3f51eb85,
                0x3f9851ec3f666666,
                0x3f75c28f3f733333,
                0x3f3851ec3f75c28f,
                0xbf7333333f75c28f,
            ],
        );
    }

    #[test]
    fn door_points_match_observed_native_words() {
        assert_words(
            gothic_door_profile_curve(1),
            &[
                0x3fd70a3d00000000,
                0x3fbc28f63ea3d70a,
                0x3f95c28f3f000000,
                0x3f51eb853f000000,
                0x3ee147ae3f0a3d71,
                0xbfbd70a43f0a3d71,
            ],
        );
        assert_words(
            gothic_door_profile_curve(2),
            &[
                0x3fe0000000000000,
                0x3fc8f5c33ea3d70a,
                0x3fa7ae143f147ae1,
                0x3f8000003f4ccccd,
                0xbfc8f5c33f4ccccd,
            ],
        );
        assert_words(
            gothic_door_profile_curve(3),
            &[
                0x3ff3333300000000,
                0x3fe000003eae147b,
                0x3fc3d70a3f23d70a,
                0x3fa147ae3f666666,
                0x3f170a3d3f8ccccd,
                0xbfd333333f8ccccd,
            ],
        );
    }

    #[test]
    fn unsupported_sizes_follow_observed_panic_branches() {
        for size in [0, 4, u8::MAX] {
            assert!(std::panic::catch_unwind(|| gothic_half_dormer_profile_curve(size)).is_err());
            assert!(std::panic::catch_unwind(|| gothic_door_profile_curve(size)).is_err());
        }
    }
}
