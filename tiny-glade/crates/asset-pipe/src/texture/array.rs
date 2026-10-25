use super::{process_dds_image, process_regular_image};
use anyhow::{Context, Result};
use asset::texture::{blob::TextureAssetBlob, desc::TextureAssetDesc};

/// Assemble matching single-layer 2D textures in layer-major order.
pub fn assemble_array(layers: Vec<TextureAssetBlob>) -> Result<TextureAssetBlob> {
    let capacity = layers
        .iter()
        .try_fold(0usize, |sum, layer| sum.checked_add(layer.data.len()))
        .context("Texture array byte size overflows usize")?;
    assemble_layers(layers.into_iter().map(Ok), Some(capacity))
}

fn assemble_layers(
    mut layers: impl Iterator<Item = Result<TextureAssetBlob>>,
    capacity: Option<usize>,
) -> Result<TextureAssetBlob> {
    let mut result = layers.next().context("Texture array has no layers")??;
    validate_layer(&result)?;
    if let Some(capacity) = capacity {
        result
            .data
            .try_reserve_exact(capacity - result.data.len())?;
    }
    let mut count = 1u16;
    for layer in layers {
        let layer = layer?;
        validate_layer(&layer)?;
        anyhow::ensure!(
            layer.dimensions == result.dimensions,
            "Texture array dimensions mismatch"
        );
        anyhow::ensure!(
            layer.mips == result.mips,
            "Texture array mip count mismatch"
        );
        anyhow::ensure!(
            layer.format == result.format,
            "Texture array format mismatch"
        );
        count = count
            .checked_add(1)
            .context("Texture array has too many layers")?;
        result.data.try_reserve(layer.data.len())?;
        result.data.extend_from_slice(&layer.data);
    }
    result.layers = count;
    result.kind = 3;
    Ok(result)
}

fn validate_layer(layer: &TextureAssetBlob) -> Result<()> {
    anyhow::ensure!(
        layer.layers == 1 && layer.kind == 2 && layer.dimensions[2] == 1,
        "Texture arrays require single-layer 2D sources"
    );
    let expected =
        super::layout::texture_bytes(layer.dimensions, u32::from(layer.mips), 1, layer.format)?;
    anyhow::ensure!(
        layer.data.len() == expected,
        "Texture array layer has {} bytes; expected {expected}",
        layer.data.len()
    );
    Ok(())
}

/// Dispatch every source through the same PNG/JPEG/DDS route, including array layers.
pub fn process_texture_asset(descriptor: &TextureAssetDesc) -> Result<TextureAssetBlob> {
    if !descriptor.layers.is_empty() {
        // Decode one layer at a time instead of retaining all decoded outputs before assembly.
        assemble_layers(descriptor.layers.iter().map(process_texture_asset), None)
    } else if descriptor
        .path
        .extension()
        .is_some_and(|extension| extension == "dds")
    {
        process_dds_image(&descriptor.path)
    } else {
        process_regular_image(descriptor)
    }
}
