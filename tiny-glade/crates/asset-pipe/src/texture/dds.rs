use anyhow::{Context, Result};
use asset::texture::blob::TextureAssetBlob;
use std::path::Path;

/// Import all DDS layers without decoding or applying image-processing options.
pub fn process_dds_image(path: &Path) -> Result<TextureAssetBlob> {
    let file = std::fs::File::open(path).with_context(|| format!("Opening DDS {path:?}"))?;
    process_dds(ddsfile::Dds::read(file).with_context(|| format!("Reading DDS {path:?}"))?)
}
pub fn process_dds(dds: ddsfile::Dds) -> Result<TextureAssetBlob> {
    use ddsfile::{D3D10ResourceDimension as R, D3DFormat as L, DxgiFormat as D};
    let format = match dds.get_dxgi_format() {
        Some(D::R9G9B9E5_SharedExp) => 123,
        Some(D::BC1_UNorm_sRGB) => 134,
        Some(D::BC3_UNorm) => 137,
        Some(D::BC3_UNorm_sRGB) => 138,
        Some(D::BC5_UNorm) => 141,
        Some(D::BC5_SNorm) => 142,
        // Legacy FourCC formats carry the established fixed gamma policy.
        None => match dds.get_d3d_format() {
            Some(L::DXT1) => 134,
            Some(L::DXT5) => 138,
            _ => match dds.header.spf.fourcc {
                Some(ddsfile::FourCC(0x32495441)) => 141,
                _ => anyhow::bail!("Unsupported DDS format"),
            },
        },
        other => anyhow::bail!("Unsupported DDS format {other:?}"),
    };
    let dimensions = [dds.get_width(), dds.get_height(), dds.get_depth()];
    anyhow::ensure!(
        dimensions.iter().all(|&value| value > 0),
        "DDS dimensions must be nonzero"
    );
    anyhow::ensure!(
        !dds.header.caps2.contains(ddsfile::Caps2::CUBEMAP),
        "DDS cubemaps are not supported"
    );
    let layers =
        u16::try_from(dds.get_num_array_layers()).context("DDS array has too many layers")?;
    anyhow::ensure!(layers > 0, "DDS must contain at least one layer");
    let kind = if let Some(header) = &dds.header10 {
        anyhow::ensure!(
            !header.misc_flag.contains(ddsfile::MiscFlag::TEXTURECUBE),
            "DDS cubemaps are not supported"
        );
        match header.resource_dimension {
            R::Texture1D => {
                anyhow::ensure!(
                    dimensions[1] == 1 && dimensions[2] == 1,
                    "Invalid 1D DDS dimensions"
                );
                if layers == 1 { 0 } else { 1 }
            }
            R::Texture2D => {
                anyhow::ensure!(dimensions[2] == 1, "Invalid 2D DDS depth");
                if layers == 1 { 2 } else { 3 }
            }
            R::Texture3D => {
                anyhow::ensure!(layers == 1, "3D DDS arrays are not supported");
                4
            }
            R::Unknown | R::Buffer => anyhow::bail!("DDS dimension is not a texture"),
        }
    } else {
        anyhow::ensure!(dimensions[2] == 1, "Legacy DDS depth must be one");
        2
    };
    let mip_levels = dds.get_num_mipmap_levels();
    let full_chain = 32
        - dimensions
            .iter()
            .copied()
            .max()
            .unwrap_or(1)
            .leading_zeros();
    anyhow::ensure!(
        mip_levels > 0 && mip_levels <= full_chain,
        "Invalid DDS mip count {mip_levels}"
    );
    let byte_count = super::layout::texture_bytes(dimensions, mip_levels, layers, format)?;
    anyhow::ensure!(
        dds.data.len() >= byte_count,
        "DDS payload is truncated: expected {byte_count} bytes, found {}",
        dds.data.len()
    );
    // Move the payload instead of copying each layer. Ignore trailing non-texture bytes.
    let mut data = dds.data;
    data.truncate(byte_count);
    Ok(TextureAssetBlob {
        data,
        dimensions,
        mips: mip_levels as u16,
        layers,
        format,
        kind,
    })
}
