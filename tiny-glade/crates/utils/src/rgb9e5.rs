//! RGB9E5 shared-exponent packing. Float additions implement round-to-even
//! through the exponent bias; do not substitute floor(x+0.5).
use crate::scalar::{max_f32, min_f32};
use glam::Vec3;
pub fn pack_rgb9e5(rgb: Vec3) -> u32 {
    let r = min_f32(65408.0, max_f32(0.0, rgb.x));
    let g = min_f32(65408.0, max_f32(0.0, rgb.y));
    let b = min_f32(65408.0, max_f32(0.0, rgb.z));
    let max_rg = r.max(g);
    let max = max_f32(max_rg, max_f32(b, f32::from_bits(0x3780_0000)));
    let bias_bits = max.to_bits().wrapping_add(0x0780_4000) & 0x7f80_0000;
    let bias = f32::from_bits(bias_bits);
    let exponent = (bias_bits << 4).wrapping_add(0x1000_0000);
    ((r + bias).to_bits() & 0x1ff)
        | ((g + bias).to_bits() << 9)
        | ((b + bias).to_bits() << 18)
        | exponent
}

#[cfg(test)]
#[path = "oracle/rgb9e5_native_oracle.rs"]
mod native_oracle;
