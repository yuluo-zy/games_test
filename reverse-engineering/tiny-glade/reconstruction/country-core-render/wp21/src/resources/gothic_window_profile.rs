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
            Vec2::new(0.54, -0.148),
        ],
        2 => &[
            Vec2::new(0.0, 1.75),
            Vec2::new(0.32, 1.58),
            Vec2::new(0.58, 1.31),
            Vec2::new(0.8, 1.0),
            Vec2::new(0.8, -1.58),
        ],
        3 => &[
            Vec2::new(0.0, 1.9),
            Vec2::new(0.34, 1.75),
            Vec2::new(0.64, 1.53),
            Vec2::new(0.9, 1.26),
            Vec2::new(1.1, 0.59),
            Vec2::new(1.1, -0.6),
        ],
        _ => panic!("gothic door profile size"),
    };
    pts.to_vec()
}
