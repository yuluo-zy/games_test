//! Checked byte layouts for the formats produced by this pipeline.
use anyhow::{Context, Result};

pub(super) fn texture_bytes(
    dimensions: [u32; 3],
    mip_levels: u32,
    layers: u16,
    format: u8,
) -> Result<usize> {
    anyhow::ensure!(
        dimensions.iter().all(|&value| value > 0),
        "Texture dimensions must be nonzero"
    );
    anyhow::ensure!(layers > 0, "Texture must contain at least one layer");
    let full_chain = 32
        - dimensions
            .iter()
            .copied()
            .max()
            .unwrap_or(1)
            .leading_zeros();
    anyhow::ensure!(
        mip_levels > 0 && mip_levels <= full_chain,
        "Invalid texture mip count {mip_levels}"
    );
    let (block_side, unit_bytes) = match format {
        37 | 43 | 123 => (1, 4),                      // RGBA8 or packed RGB9E5.
        134 | 139 => (4, 8),                          // BC1 or BC4.
        137 | 138 | 141 | 142 | 145 | 146 => (4, 16), // BC3, BC5 or BC7.
        _ => anyhow::bail!("Unsupported texture format {format}"),
    };
    let mut layer_bytes = 0u64;
    for mip in 0..mip_levels {
        let [width, height, depth] = dimensions.map(|value| u64::from((value >> mip).max(1)));
        let bytes = width
            .div_ceil(block_side)
            .checked_mul(height.div_ceil(block_side))
            .and_then(|size| size.checked_mul(depth))
            .and_then(|size| size.checked_mul(unit_bytes))
            .context("Texture mip byte size overflows")?;
        layer_bytes = layer_bytes
            .checked_add(bytes)
            .context("Texture layer byte size overflows")?;
    }
    let total = layer_bytes
        .checked_mul(u64::from(layers))
        .context("Texture payload byte size overflows")?;
    usize::try_from(total).context("Texture payload does not fit in memory")
}

pub(super) fn image_elements(
    width: u32,
    height: u32,
    channels: u8,
    element_bytes: usize,
) -> Result<usize> {
    anyhow::ensure!(width > 0 && height > 0, "Image dimensions must be nonzero");
    let elements = (width as usize)
        .checked_mul(height as usize)
        .and_then(|pixels| pixels.checked_mul(usize::from(channels)))
        .context("Image element count overflows usize")?;
    let bytes = elements
        .checked_mul(element_bytes)
        .context("Image byte size overflows usize")?;
    anyhow::ensure!(
        bytes <= isize::MAX as usize,
        "Image byte size exceeds the addressable allocation range"
    );
    Ok(elements)
}
