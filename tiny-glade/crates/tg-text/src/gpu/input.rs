//! Validate inputs and pack the shader ABI before allocating frame resources.
use super::*;
use crate::shape_stream::encoding::GLYPH_STREAM;
use shader_pipeline::reflection::ShaderReflection;

/// Validated borrowed inputs remain alive through preparation and submission.
/// Validation does not borrow the renderer, so resource preparation can mutate its caches.
pub(super) struct ValidatedFrame<'a> {
    pub(super) width: u32,
    pub(super) height: u32,
    pub(super) batch: &'a RenderBatch,
    pub(super) atlas: AtlasDescriptor,
    pub(super) uploads: &'a [PendingGlyphUpload],
    pub(super) images: &'a [Arc<Texture>],
    pub(super) readback_bytes: u64,
    pub(super) vertex_count: u32,
    pub(super) atlas_is_new: bool,
}
pub(super) struct Globals {
    pub(super) blue_noise_dimensions: [u32; 2],
    pub(super) window_pixels: [f32; 2],
    pub(super) atlas_dimensions: [u32; 2],
}
impl Globals {
    pub(super) const BYTE_LEN: usize = 24;
    pub(super) fn encode(&self) -> [u8; Self::BYTE_LEN] {
        let mut bytes = [0; Self::BYTE_LEN];
        for (destination, value) in bytes.as_chunks_mut::<4>().0.iter_mut().zip([
            self.blue_noise_dimensions[0].to_le_bytes(),
            self.blue_noise_dimensions[1].to_le_bytes(),
            self.window_pixels[0].to_le_bytes(),
            self.window_pixels[1].to_le_bytes(),
            self.atlas_dimensions[0].to_le_bytes(),
            self.atlas_dimensions[1].to_le_bytes(),
        ]) {
            destination.copy_from_slice(&value);
        }
        bytes
    }
    pub(super) fn validate_layout(
        reflection: &ShaderReflection,
        binding: BindingLocation,
    ) -> GpuResult<()> {
        let uniform = reflection
            .uniform_block
            .as_ref()
            .ok_or("text globals absent")?;
        const NOISE_OFFSET: u32 = 0;
        const WINDOW_OFFSET: u32 = 8;
        const ATLAS_OFFSET: u32 = 16;
        if uniform.binding != binding
            || uniform.byte_size as usize != Self::BYTE_LEN
            || ![
                ("blue_noise_texture_dims", NOISE_OFFSET),
                ("window_size_pixels", WINDOW_OFFSET),
                ("font_atlas_tex_dims", ATLAS_OFFSET),
            ]
            .iter()
            .all(|(name, offset)| {
                uniform
                    .name_offsets
                    .iter()
                    .any(|(actual_name, actual_offset)| {
                        actual_name == name && actual_offset == offset
                    })
            })
        {
            return Err("text globals layout differs from the shader contract".into());
        }
        Ok(())
    }
}
pub(super) struct PackedGlyphUploads {
    pub(super) bytes: Vec<u8>,
    pub(super) regions: Vec<vk::BufferImageCopy>,
}
impl PackedGlyphUploads {
    // validate_batch must accept uploads before this allocation/packing step.
    pub(super) fn new(uploads: &[PendingGlyphUpload]) -> GpuResult<Self> {
        let total_bytes = uploads.iter().try_fold(0usize, |total, upload| {
            total
                .checked_add(upload.rgba.len())
                .ok_or("upload size overflow")
        })?;
        let mut upload_data = Vec::with_capacity(total_bytes);
        let mut regions = Vec::with_capacity(uploads.len());
        for upload in uploads {
            regions.push(
                vk::BufferImageCopy::default()
                    .buffer_offset(upload_data.len() as u64)
                    .image_subresource(
                        vk::ImageSubresourceLayers::default()
                            .aspect_mask(vk::ImageAspectFlags::COLOR)
                            .layer_count(1),
                    )
                    .image_offset(vk::Offset3D {
                        x: upload.x.try_into()?,
                        y: upload.y.try_into()?,
                        z: 0,
                    })
                    .image_extent(vk::Extent3D {
                        width: upload.width,
                        height: upload.height,
                        depth: 1,
                    }),
            );
            upload_data.extend_from_slice(&upload.rgba);
        }
        Ok(Self {
            bytes: upload_data,
            regions,
        })
    }
}
impl GpuTextRenderer {
    pub(super) fn validate_inputs<'a>(
        &self,
        width: u32,
        height: u32,
        batch: &'a RenderBatch,
        atlas_desc: AtlasDescriptor,
        uploads: &'a [PendingGlyphUpload],
        images: &'a [Arc<Texture>],
    ) -> GpuResult<ValidatedFrame<'a>> {
        let length = validate_batch(
            width,
            height,
            batch,
            atlas_desc,
            uploads,
            images.len(),
            &self.limits,
        )?;
        let max_dimension = self.device.limits().max_image_dimension2_d;
        if [width, height, atlas_desc.width, atlas_desc.height]
            .into_iter()
            .any(|size| size > max_dimension)
        {
            return Err("text target or atlas exceeds device extent limit".into());
        }
        let image_limit = self
            .abi
            .slot(self.bindings.images)
            .ok_or("resolver array absent from text ABI")?
            .parameter_count;
        if images.len() > image_limit as usize {
            return Err("too many resolver images".into());
        }
        if images
            .iter()
            .any(|image| !Arc::ptr_eq(&image.device(), &self.device) || !sampled_2d(image))
        {
            return Err("resolver image uses another device or incompatible sampling type".into());
        }
        let storage_limit = u64::from(self.device.limits().max_storage_buffer_range);
        if batch.primitive_bytes().len() as u64 > storage_limit
            || (batch.shape_words.len() as u64) * 4 > storage_limit
        {
            return Err("text batch exceeds the device storage buffer range".into());
        }
        let fresh = self
            .atlas
            .as_ref()
            .is_none_or(|(descriptor, _)| *descriptor != atlas_desc);
        if fresh {
            // Most glyphs exactly match an upload. Memoize repeated primitives;
            // uncommon subrectangles use a bounded fallback containment search.
            let exact: std::collections::HashSet<_> = uploads
                .iter()
                .map(|upload| [upload.x, upload.y, upload.width, upload.height])
                .collect();
            let mut checked = std::collections::HashSet::new();
            let mut comparisons = 0usize;
            for primitive in batch
                .primitives
                .iter()
                .filter(|primitive| primitive.stream == GLYPH_STREAM)
            {
                let values = [
                    primitive.uv_min[0],
                    primitive.uv_min[1],
                    primitive.uv_extent[0],
                    primitive.uv_extent[1],
                ];
                if !checked.insert(values.map(f32::to_bits)) {
                    continue;
                }
                if values.iter().all(|coordinate| coordinate.fract() == 0.)
                    && exact.contains(&values.map(|coordinate| coordinate as u32))
                {
                    continue;
                }
                let mut covered = false;
                for upload in uploads {
                    comparisons = comparisons.checked_add(1).ok_or("coverage work overflow")?;
                    if comparisons > self.limits.max_coverage_checks {
                        return Err("glyph coverage exceeds validation work budget".into());
                    }
                    if f64::from(primitive.uv_min[0]) >= f64::from(upload.x)
                        && f64::from(primitive.uv_min[1]) >= f64::from(upload.y)
                        && f64::from(primitive.uv_min[0]) + f64::from(primitive.uv_extent[0])
                            <= f64::from(upload.x) + f64::from(upload.width)
                        && f64::from(primitive.uv_min[1]) + f64::from(primitive.uv_extent[1])
                            <= f64::from(upload.y) + f64::from(upload.height)
                    {
                        covered = true;
                        break;
                    }
                }
                if !covered {
                    return Err("fresh GPU atlas lacks cached glyph pixels; reset_glyph_cache then render again".into());
                }
            }
        }
        Ok(ValidatedFrame {
            width,
            height,
            batch,
            atlas: atlas_desc,
            uploads,
            images,
            readback_bytes: length,
            vertex_count: batch
                .primitives
                .len()
                .checked_mul(6)
                .ok_or("draw count overflow")?
                .try_into()?,
            atlas_is_new: fresh,
        })
    }
}
