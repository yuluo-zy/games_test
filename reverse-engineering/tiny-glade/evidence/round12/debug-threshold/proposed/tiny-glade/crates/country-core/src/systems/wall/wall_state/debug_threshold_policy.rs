//! Original140a8b330..494. The four arrays are zipped to their shortest length.
use crate::utils::debug_viz::{DebugColor, DebugSphere};
use glam::Vec3;

pub(super) fn threshold_spheres(
    positions: &[[f32; 3]],
    thresholds: &[f32],
    arch_heights: &[f32],
    third_heights: &[f32],
    mut emit: impl FnMut(DebugSphere),
) {
    for (((position, &threshold), &arch), &third) in positions
        .iter()
        .zip(thresholds)
        .zip(arch_heights)
        .zip(third_heights)
    {
        let [x, y, z] = *position;
        emit(DebugSphere {
            color: DebugColor::White,
            radius: f32::from_bits(0x3dcccccd),
            center: Vec3::new(x, y, z),
            alpha: 1.0,
            solid: false,
        });
        // Original SSE multiplies by zero. Do not replace these positions with
        // Vec3(x, y + height, z): NaN/Inf and signed-zero behavior would change.
        let threshold_zero = threshold * 0.0;
        emit(DebugSphere {
            color: DebugColor::Red,
            radius: f32::from_bits(0x3d4ccccd),
            center: Vec3::new(threshold_zero + x, threshold + y, z + threshold_zero),
            alpha: 2.0,
            solid: false,
        });
        let third_zero = third * 0.0;
        emit(DebugSphere {
            color: DebugColor::Red,
            radius: f32::from_bits(0x3e19999a),
            center: Vec3::new(third_zero + x, third + y, z + third_zero),
            alpha: 0.5,
            solid: false,
        });
        let delta = arch - y;
        let delta_zero = delta * 0.0;
        emit(DebugSphere {
            color: DebugColor::Blue,
            radius: f32::from_bits(0x3e19999a),
            center: Vec3::new(delta_zero + x, delta + y, z + delta_zero),
            alpha: 0.5,
            solid: false,
        });
    }
}
