//! Plain CPU packets shared with the GPU adapter.
use crate::RawShapeStream;
// Older bytemuck derives generate a private layout helper.
#[allow(dead_code)]
mod packed {
    /// A 48-byte shader primitive. Positions/extents use logical UI units;
    /// the GPU adapter maps a logical height of 1000 to the target height. Glyph UV
    /// coordinates use atlas pixels. Colors are RRGGBBAA with sRGB color channels.
    /// Public fields permit packet construction; GPU submission validates their values.
    #[repr(C)]
    #[derive(Clone, Copy, Debug, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
    pub struct Primitive {
        pub pos_min: [f32; 2],
        pub pos_extent: [f32; 2],
        pub uv_min: [f32; 2],
        pub uv_extent: [f32; 2],
        pub color: u32,
        pub stream: u32,
        pub padding: [u32; 2],
    }
    impl Primitive {
        /// Construct an unchecked solid rectangle with zeroed reserved padding.
        /// Use the renderer's rectangle methods to validate geometry before queuing it.
        pub fn rect(pos_min: [f32; 2], pos_extent: [f32; 2], color: u32) -> Self {
            Self {
                pos_min,
                pos_extent,
                uv_min: [0.; 2],
                uv_extent: [1.; 2],
                color,
                stream: crate::shape_stream::encoding::SOLID_STREAM,
                padding: [0; 2],
            }
        }
    }
}
pub use packed::Primitive;
/// Atlas dimensions/revision used to invalidate GPU textures. Revisions issued by
/// TextRenderer are process-unique; manual descriptors must distinguish new pixel contents.
/// A changed revision requires replacing the GPU atlas and rebuilding glyph draws.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AtlasDescriptor {
    pub width: u32,
    pub height: u32,
    pub generation: u64,
}
/// One tightly packed RGBA bitmap update for a rectangle in a specific atlas revision.
/// Upload before drawing its glyph. Dimensions, byte length, bounds and generation
/// are validated by the GPU adapter before commands are recorded.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PendingGlyphUpload {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<u8>,
    pub generation: u64,
}
/// Owned frame packets. Glyph updates must reach the matching GPU atlas before drawing.
/// Consuming a renderer transfers these vectors; callers own submission and retry policy.
#[derive(Clone, Debug, Default)]
pub struct RenderBatch {
    pub primitives: Vec<Primitive>,
    pub shape_words: Vec<u32>,
    /// RGBA glyph updates that must be uploaded before drawing these primitives.
    pub glyph_uploads: Vec<PendingGlyphUpload>,
}
impl RenderBatch {
    /// Borrow the primitive vector as packed bytes without allocating or copying.
    pub fn primitive_bytes(&self) -> &[u8] {
        bytemuck::cast_slice(&self.primitives)
    }
}
/// Low-level primitive and shape buffers, independent of fonts and atlas ownership.
/// Direct field mutation is unchecked; the GPU adapter validates completed packets.
#[derive(Clone, Debug, Default)]
pub struct TextRenderStream {
    pub primitives: Vec<Primitive>,
    pub shapes: RawShapeStream,
}
impl TextRenderStream {
    /// Transfer both buffers into a batch without glyph uploads, leaving empty buffers.
    pub fn consume(&mut self) -> RenderBatch {
        RenderBatch {
            primitives: std::mem::take(&mut self.primitives),
            shape_words: std::mem::take(&mut self.shapes.0),
            glyph_uploads: Vec::new(),
        }
    }
    /// Borrow both buffers for an infallible callback, then clear them on return.
    /// A panic preserves the buffers; an ordinary callback return value, including
    /// a `Result::Err`, is treated as success. Renderer callers needing transactional
    /// errors should use `TextRenderer::try_consume_with` instead.
    pub fn consume_with<T>(&mut self, consume: impl FnOnce(&[Primitive], &[u32]) -> T) -> T {
        let result = consume(&self.primitives, &self.shapes.0);
        self.clear();
        result
    }
    /// Discard buffered packets while retaining their allocation capacity.
    pub fn clear(&mut self) {
        self.primitives.clear();
        self.shapes.clear();
    }
}
