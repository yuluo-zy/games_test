//! Validate shader input before allocating GPU resources or recording commands.
use crate::shape_stream::encoding::{
    ARGUMENT_SHIFT, FILL_OPCODE, GLYPH_STREAM, IMAGE_INDEX_MASK, IMAGE_OPCODE, OPCODE_MASK,
    RADIUS_OPCODE, SCALING_SHIFT, SOLID_STREAM, ShapePacket,
};
use crate::{AtlasDescriptor, PendingGlyphUpload, Primitive, RenderBatch, TextError, TextResult};

/// Per-submission CPU work and GPU resource budgets. Zero forbids that resource.
/// Device capabilities may impose lower limits; defaults suit interface-sized frames.
#[derive(Clone, Debug)]
pub struct RenderLimits {
    /// Maximum RGBA8 offscreen/readback byte size.
    pub max_target_bytes: u64,
    /// Maximum number of primitives in a batch.
    pub max_primitives: usize,
    /// Maximum stored words and cumulative words parsed across distinct packets.
    pub max_shape_words: usize,
    /// Maximum combined RGBA glyph-update bytes.
    pub max_upload_bytes: usize,
    /// Maximum number of individual glyph-update rectangles.
    pub max_upload_regions: usize,
    /// Maximum allocated RGBA8 atlas byte size.
    pub max_atlas_bytes: u64,
    /// Maximum fallback containment comparisons when reconstructing a GPU atlas.
    pub max_coverage_checks: usize,
}
impl Default for RenderLimits {
    fn default() -> Self {
        Self {
            max_target_bytes: 256 * 1024 * 1024,
            max_primitives: 1_000_000,
            max_shape_words: 1_000_000,
            max_upload_bytes: 64 * 1024 * 1024,
            max_upload_regions: 16_384,
            max_atlas_bytes: 64 * 1024 * 1024,
            max_coverage_checks: 4_000_000,
        }
    }
}

fn invalid(message: &str) -> TextError {
    TextError::InvalidBatch(message.into())
}

/// Validate dimensions, geometry, packed programs and glyph updates without mutation.
/// Returns the required RGBA readback size. `uploads` is the explicit submission
/// slice; it may differ from `batch.glyph_uploads` for a custom consumer.
///
/// # Errors
/// Returns [`TextError::InvalidBatch`] for malformed/stale/out-of-bounds data or
/// exceeded resource/work budgets. Device-specific limits and fresh-atlas pixel
/// coverage are checked by the GPU adapter after this CPU-only validation.
pub fn validate_batch(
    width: u32,
    height: u32,
    batch: &RenderBatch,
    atlas: AtlasDescriptor,
    uploads: &[PendingGlyphUpload],
    image_count: usize,
    limits: &RenderLimits,
) -> TextResult<u64> {
    if width == 0 || height == 0 || atlas.width == 0 || atlas.height == 0 {
        return Err(invalid("zero target or atlas extent"));
    }
    let size = u64::from(width)
        .checked_mul(u64::from(height))
        .and_then(|n| n.checked_mul(4))
        .ok_or_else(|| invalid("target byte size overflow"))?;
    if size > limits.max_target_bytes || usize::try_from(size).is_err() {
        return Err(invalid("target exceeds readback budget"));
    }
    if batch.primitives.len() > limits.max_primitives
        || batch.shape_words.len() > limits.max_shape_words
        || batch
            .primitives
            .len()
            .checked_mul(6)
            .is_none_or(|n| n > u32::MAX as usize)
    {
        return Err(invalid("render batch exceeds primitive or shape budget"));
    }
    let atlas_bytes = u64::from(atlas.width)
        .checked_mul(u64::from(atlas.height))
        .and_then(|n| n.checked_mul(4));
    if atlas_bytes.is_none_or(|n| n > limits.max_atlas_bytes)
        || uploads.len() > limits.max_upload_regions
    {
        return Err(invalid("atlas or upload region count exceeds budget"));
    }
    let mut upload_bytes = 0usize;
    for upload in uploads {
        let expected = u64::from(upload.width)
            .checked_mul(u64::from(upload.height))
            .and_then(|n| n.checked_mul(4));
        if upload.width == 0
            || upload.height == 0
            || expected != Some(upload.rgba.len() as u64)
            || upload.generation != atlas.generation
            || upload
                .x
                .checked_add(upload.width)
                .is_none_or(|end| end > atlas.width)
            || upload
                .y
                .checked_add(upload.height)
                .is_none_or(|end| end > atlas.height)
        {
            return Err(invalid("stale, malformed or out-of-bounds glyph upload"));
        }
        upload_bytes = upload_bytes
            .checked_add(upload.rgba.len())
            .ok_or_else(|| invalid("upload size overflow"))?;
        if upload_bytes > limits.max_upload_bytes {
            return Err(invalid("glyph uploads exceed staging budget"));
        }
    }
    let mut validated_packets = std::collections::HashSet::new();
    let mut parsed_words = 0usize;
    for primitive in &batch.primitives {
        validate_primitive_geometry(primitive)?;
        match primitive.stream {
            SOLID_STREAM => {}
            GLYPH_STREAM => {
                if primitive.uv_min.iter().any(|v| *v < 0.)
                    || primitive.uv_extent.iter().any(|v| *v <= 0.)
                    || f64::from(primitive.uv_min[0]) + f64::from(primitive.uv_extent[0])
                        > f64::from(atlas.width)
                    || f64::from(primitive.uv_min[1]) + f64::from(primitive.uv_extent[1])
                        > f64::from(atlas.height)
                {
                    return Err(invalid("glyph UV rectangle is outside the atlas"));
                }
            }
            packet => {
                let words = batch
                    .shape_words
                    .get(ShapePacket::unpack(packet)?)
                    .ok_or_else(|| invalid("shape packet lies outside the batch"))?;
                if validated_packets.insert(packet) {
                    parsed_words = parsed_words
                        .checked_add(words.len())
                        .ok_or_else(|| invalid("shape validation work overflow"))?;
                    if parsed_words > limits.max_shape_words {
                        return Err(invalid("shape validation exceeds word budget"));
                    }
                    validate_shape_words(words, image_count)?;
                }
            }
        }
    }
    Ok(size)
}

/// Shared finite/nonnegative geometry rule for frame construction and submission.
pub(crate) fn validate_primitive_geometry(primitive: &Primitive) -> TextResult<()> {
    if primitive
        .pos_min
        .iter()
        .chain(&primitive.pos_extent)
        .chain(&primitive.uv_min)
        .chain(&primitive.uv_extent)
        .any(|coordinate| !coordinate.is_finite())
        || primitive
            .pos_extent
            .iter()
            .chain(&primitive.uv_extent)
            .any(|extent| *extent < 0.)
    {
        return Err(invalid(
            "primitive geometry must be finite and nonnegative in extent",
        ));
    }
    Ok(())
}

/// Validate one packed shape program; image_count is the resolver capacity.
pub(crate) fn validate_shape_words(words: &[u32], image_count: usize) -> TextResult<()> {
    let mut cursor = 0;
    while cursor < words.len() {
        let word = words[cursor];
        match word & OPCODE_MASK {
            FILL_OPCODE => {
                if word != FILL_OPCODE || cursor + 1 >= words.len() {
                    return Err(invalid("malformed fill operation"));
                }
                cursor += 2;
            }
            IMAGE_OPCODE => {
                if word >> SCALING_SHIFT > crate::ImageScaling::Nine as u32
                    || ((word >> ARGUMENT_SHIFT) & IMAGE_INDEX_MASK) as usize >= image_count
                {
                    return Err(invalid("invalid image scaling or resolver index"));
                }
                cursor += 1;
            }
            RADIUS_OPCODE => {
                if word >> (ARGUMENT_SHIFT * 2) != 0 {
                    return Err(invalid("invalid radius operation"));
                }
                cursor += 1;
            }
            _ => return Err(invalid("unknown shape opcode")),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Primitive;
    #[test]
    fn batch_validation_rejects_malformed_packets_uploads_and_unbounded_targets() {
        let atlas = AtlasDescriptor {
            width: 8,
            height: 8,
            generation: 3,
        };
        let limits = RenderLimits::default();
        let mut batch = RenderBatch::default();
        batch
            .primitives
            .push(Primitive::rect([0.; 2], [2.; 2], u32::MAX));
        assert_eq!(
            validate_batch(8, 8, &batch, atlas, &[], 0, &limits).unwrap(),
            256
        );
        for words in [
            vec![3],
            vec![0],
            vec![1 | (4 << 24)],
            vec![1],
            vec![2 | (1 << 16)],
        ] {
            batch.primitives[0].stream = words.len() as u32;
            batch.shape_words = words;
            assert!(validate_batch(8, 8, &batch, atlas, &[], 0, &limits).is_err());
        }
        batch.primitives[0].stream = u32::MAX;
        batch.primitives[0].pos_min[0] = f32::NAN;
        assert!(validate_batch(8, 8, &batch, atlas, &[], 0, &limits).is_err());
        batch.primitives[0].pos_min[0] = 0.;
        let mut upload = PendingGlyphUpload {
            x: 7,
            y: 0,
            width: 2,
            height: 1,
            rgba: vec![0; 8],
            generation: 3,
        };
        assert!(
            validate_batch(
                8,
                8,
                &batch,
                atlas,
                std::slice::from_ref(&upload),
                0,
                &limits
            )
            .is_err()
        );
        upload.x = 0;
        assert!(validate_batch(8, 8, &batch, atlas, &[upload], 0, &limits).is_ok());
        let tiny = RenderLimits {
            max_shape_words: 3,
            ..Default::default()
        };
        batch.shape_words = vec![2; 3];
        batch.primitives[0].stream = 3;
        batch.primitives.push(batch.primitives[0]);
        assert!(
            validate_batch(8, 8, &batch, atlas, &[], 0, &tiny).is_ok(),
            "identical packets are validated once"
        );
        batch.primitives[1].stream = (1 << 8) | 2;
        assert!(
            validate_batch(8, 8, &batch, atlas, &[], 0, &tiny).is_err(),
            "overlapping packets still consume work budget"
        );
        let no_regions = RenderLimits {
            max_upload_regions: 0,
            ..Default::default()
        };
        let update = PendingGlyphUpload {
            x: 0,
            y: 0,
            width: 1,
            height: 1,
            rgba: vec![0; 4],
            generation: 3,
        };
        assert!(validate_batch(8, 8, &batch, atlas, &[update], 0, &no_regions).is_err());
        let tiny_atlas = RenderLimits {
            max_atlas_bytes: 8,
            ..Default::default()
        };
        assert!(validate_batch(8, 8, &batch, atlas, &[], 0, &tiny_atlas).is_err());
        assert!(validate_batch(u32::MAX, u32::MAX, &batch, atlas, &[], 0, &limits).is_err());
    }
}
