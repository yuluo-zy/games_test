//! Packed text colors use RRGGBBAA, with alpha in the least significant byte.

/// Scale the alpha of an RRGGBBAA color, retaining its three sRGB channels.
///
/// Finite nonnegative scales are the intended input; the result rounds and
/// saturates to an 8-bit alpha. Negative scales and NaN produce zero alpha.
/// Renderer opacity entry points separately reject nonfinite/out-of-range input.
pub fn scale_u32_packed_srgba_color(color: u32, scale: f32) -> u32 {
    let alpha = (((color & 255) as f32 / 255. * scale * 255.) + 0.5).clamp(0., 255.) as u8;
    (color & 0xffff_ff00) | u32::from(alpha)
}

// Color's integer representation belongs to cosmic-text. Use its channel API
// at the boundary instead of allowing that representation into the shader ABI.
pub(crate) fn pack_text_color(color: cosmic_text::Color) -> u32 {
    u32::from_be_bytes(color.as_rgba())
}

// Multiplying all four channels preserves local editor colors and selection alpha.
pub(crate) fn tint_color(color: u32, tint: u32) -> u32 {
    let mut output = 0u32;
    for shift in [0, 8, 16, 24] {
        output |= ((((color >> shift) & 255) * ((tint >> shift) & 255)) / 255) << shift;
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn editor_colors_and_opacity_share_the_shader_channel_order() {
        let color = pack_text_color(cosmic_text::Color::rgba(0x12, 0x34, 0x56, 0x80));
        assert_eq!(color, 0x1234_5680);
        assert_eq!(scale_u32_packed_srgba_color(color, 0.5), 0x1234_5640);
        assert_eq!(scale_u32_packed_srgba_color(color, 4.), 0x1234_56ff);
        assert_eq!(scale_u32_packed_srgba_color(color, f32::NAN), 0x1234_5600);
    }
}
