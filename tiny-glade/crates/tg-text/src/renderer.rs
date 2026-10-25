//! Text rasterization, bounded atlas allocation and frame batch construction.
mod atlas;
mod primitive;
use crate::color::tint_color;
use crate::shape_stream::encoding::{MAX_SHAPE_WORDS, ShapePacket};
use crate::validation::validate_primitive_geometry;
use crate::{RawShapeStream, ShapeStream, TextError, TextResult};
use atlas::AtlasCache;
pub use atlas::AtlasConfig;
use cosmic_text::{CacheKey, FontSystem, SwashCache, SwashContent};
use etagere::AllocId;
pub use primitive::{
    AtlasDescriptor, PendingGlyphUpload, Primitive, RenderBatch, TextRenderStream,
};
use std::{path::Path, sync::Arc};

/// A glyph allocation tied to one renderer and atlas revision.
///
/// Atlas coordinates and size are pixels; bearing offsets locate the bitmap relative
/// to its baseline. Resetting or growing the atlas invalidates every existing handle.
/// Use [`TextRenderer::validate_glyph_cache_allocs`] before reusing cached handles.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Glyph {
    /// Allocator handle; callers must not modify it before cache validation.
    pub allocation: AllocId,
    /// Top-left bitmap position in atlas pixels.
    pub atlas: [u32; 2],
    /// Bitmap width and height in pixels.
    pub size: [u32; 2],
    /// Horizontal and downward vertical offsets from the glyph baseline.
    pub bearing: [i32; 2],
    owner: u64,
    epoch: u64,
    nonce: u64,
}
/// A frame stamp and allocator generation value. Frame zero denotes unused storage.
#[derive(Clone, Copy, Debug, Default)]
pub struct AtlasAllocUsage {
    pub last_frame: u64,
    pub generation: u16,
}
/// Cumulative rasterization and atlas counters; counters saturate at `u64::MAX`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TextStats {
    /// Images returned by the rasterizer, including empty bitmaps.
    pub rasterized: u64,
    /// Positive and negative glyph-cache hits.
    pub cache_hits: u64,
    /// Expired atlas allocations reclaimed to satisfy new requests.
    pub evicted: u64,
    /// Successful atlas growth operations; explicit resets are not counted.
    pub atlas_rebuilds: u64,
}

/// Owns font data, reusable glyph allocations, pending uploads and one frame stream.
///
/// Build primitives, then [`Self::consume`] to submit a frame. Atlas growth returns
/// [`TextError::AtlasChanged`]: call [`Self::begin_frame`] and rebuild that frame.
/// [`TextError::AtlasFull`] means the configured budget cannot satisfy the request.
#[derive(bevy_ecs::prelude::Resource)]
pub struct TextRenderer {
    pub(crate) font_system: FontSystem,
    swash: SwashCache,
    atlas: AtlasCache,
    stream: TextRenderStream,
    consumed_primitive_count: u64,
    pixels_per_point: f32,
    frame_invalidated: bool,
    pub stats: TextStats,
}
impl Default for TextRenderer {
    fn default() -> Self {
        Self::new()
    }
}
/// Create an empty font database using the requested locale, the system locale,
/// or `en-US` when neither is available. Font files are loaded explicitly.
pub fn init_font_system(locale: Option<String>) -> FontSystem {
    FontSystem::new_with_locale_and_db(
        locale
            .or_else(sys_locale::get_locale)
            .unwrap_or_else(|| "en-US".into()),
        cosmic_text::fontdb::Database::new(),
    )
}
impl TextRenderer {
    /// Create the default 1024×1024 atlas with a 64 MiB growth budget.
    ///
    /// # Panics
    /// Panics if the process exhausts its unique `u64` atlas revision counter.
    /// Use [`Self::with_config`] to handle initialization errors as a result.
    pub fn new() -> Self {
        Self::with_config(AtlasConfig::default()).expect("default renderer initialization failed")
    }
    /// Create an atlas using explicit initial dimensions and growth limits.
    ///
    /// # Errors
    /// Returns [`TextError::InvalidAtlas`] for unsupported dimensions, insufficient
    /// byte budget, or exhaustion of the process-wide revision counter.
    pub fn with_config(config: AtlasConfig) -> TextResult<Self> {
        Ok(Self {
            font_system: init_font_system(None),
            swash: SwashCache::new(),
            atlas: AtlasCache::new(config)?,
            stream: TextRenderStream::default(),
            consumed_primitive_count: 0,
            pixels_per_point: 1.,
            frame_invalidated: false,
            stats: TextStats::default(),
        })
    }
    /// Create an atlas of `width`×`height`, retaining the default growth limits.
    ///
    /// # Errors
    /// Returns [`TextError::InvalidAtlas`] if dimensions are zero, exceed the
    /// default 4096×4096 limit, or a unique revision cannot be allocated.
    pub fn with_atlas_size(width: u32, height: u32) -> TextResult<Self> {
        Self::with_config(AtlasConfig {
            width,
            height,
            ..Default::default()
        })
    }
    /// Borrow the font system without allowing mutations that bypass invalidation.
    pub fn font_system(&self) -> &FontSystem {
        &self.font_system
    }
    /// Read a font file and load all readable faces, invalidating existing glyphs.
    ///
    /// # Errors
    /// Returns an I/O error, invalid font input, or revision exhaustion. Failed reads
    /// and unparseable input leave font data and queued rendering unchanged.
    pub fn load_font_from_file(&mut self, path: impl AsRef<Path>) -> TextResult<()> {
        let bytes = std::fs::read(path)?;
        self.load_font_data(bytes)
    }
    /// Load all readable faces from owned font bytes without duplicating the buffer.
    /// Successful loading clears queued primitives/uploads and changes the cache token.
    ///
    /// # Errors
    /// Returns [`TextError::InvalidInput`] when no faces can be read, or
    /// [`TextError::InvalidAtlas`] if a new revision cannot be allocated.
    pub fn load_font_data(&mut self, bytes: Vec<u8>) -> TextResult<()> {
        let font_data: Arc<dyn AsRef<[u8]> + Send + Sync> = Arc::new(bytes);
        let mut validation_database = cosmic_text::fontdb::Database::new();
        let loaded_faces = validation_database
            .load_font_source(cosmic_text::fontdb::Source::Binary(Arc::clone(&font_data)));
        if loaded_faces.is_empty() {
            return Err(TextError::InvalidInput(
                "font data contains no readable faces".into(),
            ));
        }
        self.reset_glyph_cache()?;
        self.font_system
            .db_mut()
            .load_font_source(cosmic_text::fontdb::Source::Binary(font_data));
        Ok(())
    }
    /// Iterate over the faces explicitly loaded into this renderer.
    pub fn font_faces(&self) -> impl Iterator<Item = &cosmic_text::fontdb::FaceInfo> {
        self.font_system.db().faces()
    }
    /// Return the positive device-pixel scale used for glyph rasterization.
    pub fn pixels_per_point(&self) -> f32 {
        self.pixels_per_point
    }
    /// Return the frame stamp, advanced only by a successful consume operation.
    pub fn current_frame(&self) -> u64 {
        self.atlas.frame
    }
    /// Return the current atlas revision, independent of the frame counter.
    pub fn invalidation_frame(&self) -> u64 {
        self.atlas.descriptor.generation
    }
    /// Return `(renderer identity, atlas revision)` for validating fragment caches.
    pub fn cache_token(&self) -> (u64, u64) {
        self.atlas.token()
    }
    /// Return dimensions and a process-unique revision for GPU atlas synchronization.
    pub fn atlas(&self) -> AtlasDescriptor {
        self.atlas.descriptor
    }
    /// Change device-pixel scale; a changed value invalidates queued glyph rendering.
    ///
    /// # Errors
    /// Returns [`TextError::InvalidValue`] unless `value` is finite and positive,
    /// or [`TextError::InvalidAtlas`] if invalidation exhausts revision storage.
    pub fn set_pixels_per_point(&mut self, value: f32) -> TextResult<()> {
        positive("pixels per point", value)?;
        if value != self.pixels_per_point {
            self.reset_glyph_cache()?;
            self.pixels_per_point = value;
        }
        Ok(())
    }
    /// Invalidate all glyph handles, pending uploads and queued primitives.
    /// The frame counter is preserved; an earlier growth failure still requires
    /// [`Self::begin_frame`] before further rendering.
    ///
    /// # Errors
    /// Returns [`TextError::InvalidAtlas`] on revision exhaustion without clearing data.
    pub fn reset_glyph_cache(&mut self) -> TextResult<()> {
        self.atlas.reset()?;
        self.swash = SwashCache::new();
        self.stream.clear();
        self.consumed_primitive_count = 0;
        Ok(())
    }
    /// Discard queued primitives and acknowledge any atlas rebuild failure.
    /// Pending valid glyph uploads and the frame stamp are preserved; beginning a
    /// frame alone does not make glyph allocations eligible for eviction.
    pub fn begin_frame(&mut self) {
        self.stream.clear();
        self.frame_invalidated = false;
    }
    /// Return the number of cached glyph keys, including cached missing glyphs.
    pub fn cache_len(&self) -> usize {
        self.atlas.cache.len()
    }
    /// Borrow glyph updates that have not yet been drained by upload or consume.
    pub fn pending_uploads(&self) -> &[PendingGlyphUpload] {
        &self.atlas.pending
    }
    /// Validate every handle before marking any allocation as recently used.
    /// Returns `false` for foreign, stale, evicted or modified handles without
    /// touching the valid prefix. Empty slices are valid; malformed IDs never panic.
    pub fn validate_glyph_cache_allocs(&mut self, glyphs: &[Glyph]) -> bool {
        self.atlas.validate(glyphs)
    }
    /// Rasterize or reuse a glyph with a finite font size in `(0, 4096]` pixels.
    /// `Ok(None)` denotes a missing glyph or an empty bitmap, not atlas exhaustion.
    ///
    /// # Errors
    /// Invalid keys or raster payloads return input errors. A successful atlas growth
    /// returns [`TextError::AtlasChanged`] and discards this frame; begin and rebuild
    /// it before continuing. [`TextError::AtlasFull`] indicates a dimension, byte or
    /// allocation-count budget that cannot satisfy the request. Revision or allocation
    /// counter exhaustion returns [`TextError::InvalidAtlas`].
    pub fn get_glyph(&mut self, key: CacheKey) -> TextResult<Option<Glyph>> {
        self.check_frame()?;
        let font_size = f32::from_bits(key.font_size_bits);
        positive("glyph font size", font_size)?;
        if font_size > 4096. {
            return Err(TextError::InvalidValue {
                field: "glyph font size",
                value: font_size,
            });
        }
        if let Some(entry) = self.atlas.hit(&key) {
            self.stats.cache_hits = self.stats.cache_hits.saturating_add(1);
            return Ok(entry);
        }
        let Some(image) = self.swash.get_image_uncached(&mut self.font_system, key) else {
            self.atlas.cache_empty(key);
            return Ok(None);
        };
        self.stats.rasterized = self.stats.rasterized.saturating_add(1);
        let width = image.placement.width;
        let height = image.placement.height;
        if width == 0 || height == 0 {
            self.atlas.cache_empty(key);
            return Ok(None);
        }
        let rgba = glyph_rgba(image.content, image.data, width, height)?;
        let glyph = match self.atlas.allocate(
            width,
            height,
            [
                image.placement.left,
                image
                    .placement
                    .top
                    .checked_neg()
                    .ok_or_else(|| TextError::InvalidInput("glyph bearing overflow".into()))?,
            ],
            &mut self.stats,
        ) {
            Ok(glyph) => glyph,
            Err(TextError::AtlasChanged) => {
                self.stream.clear();
                self.consumed_primitive_count = 0;
                self.frame_invalidated = true;
                return Err(TextError::AtlasChanged);
            }
            Err(error) => return Err(error),
        };
        self.atlas.pending.push(PendingGlyphUpload {
            x: glyph.atlas[0],
            y: glyph.atlas[1],
            width,
            height,
            rgba,
            generation: self.atlas.descriptor.generation,
        });
        self.atlas.cache.insert(key, Some(glyph));
        Ok(Some(glyph))
    }
    /// Drain pending RGBA updates without advancing the frame counter.
    /// The caller must upload them before drawing primitives that reference them.
    ///
    /// # Errors
    /// Returns [`TextError::AtlasChanged`] while a discarded frame awaits `begin_frame`.
    pub fn upload_pending_glyphs(&mut self) -> TextResult<Vec<PendingGlyphUpload>> {
        self.check_frame()?;
        Ok(std::mem::take(&mut self.atlas.pending))
    }
    /// Queue a white rectangle with finite opacity in `[0, 1]` and optional shapes.
    /// Positions must be finite and extents finite and nonnegative.
    ///
    /// # Errors
    /// Rejects invalid geometry, opacity or packed shapes without changing the stream;
    /// also reports an atlas-changed frame awaiting [`Self::begin_frame`].
    pub fn rect(
        &mut self,
        pos: [f32; 2],
        extent: [f32; 2],
        opacity: f32,
        shapes: &RawShapeStream,
    ) -> TextResult<()> {
        if !opacity.is_finite() || !(0.0..=1.0).contains(&opacity) {
            return Err(TextError::InvalidValue {
                field: "opacity",
                value: opacity,
            });
        }
        self.rect_with_color(
            pos,
            extent,
            crate::scale_u32_packed_srgba_color(u32::MAX, opacity),
            shapes,
        )
    }
    /// Queue a rectangle using an explicit packed color and validated shape words.
    /// An empty shape stream emits a solid rectangle. Geometry uses the same bounds
    /// as [`Self::rect`]; image indices are checked against bound images by the GPU.
    ///
    /// # Errors
    /// Rejects invalid geometry, shape instructions, oversized packets or an
    /// atlas-changed frame without modifying the queued stream.
    pub fn rect_with_color(
        &mut self,
        pos: [f32; 2],
        extent: [f32; 2],
        color: u32,
        shapes: &RawShapeStream,
    ) -> TextResult<()> {
        self.check_frame()?;
        let mut primitive = Primitive::rect(pos, extent, color);
        validate_primitive_geometry(&primitive)?;
        if shapes.0.len() > MAX_SHAPE_WORDS {
            return Err(TextError::ShapeStreamTooLong(shapes.0.len()));
        }
        crate::validation::validate_shape_words(&shapes.0, 65_536)?;
        if !shapes.0.is_empty() {
            primitive.stream = ShapePacket {
                word_offset: self.stream.shapes.0.len(),
                word_count: shapes.0.len(),
            }
            .pack()?;
        }
        self.stream.primitives.push(primitive);
        self.stream.shapes.0.extend_from_slice(&shapes.0);
        Ok(())
    }
    /// Encode named shapes using `resolver`, then queue a colored rectangle.
    /// Resolver callbacks run in source order and their side effects are not rolled back.
    ///
    /// # Errors
    /// Propagates shape encoding and [`Self::rect_with_color`] errors without adding
    /// a partial rectangle to the frame.
    pub fn shaped_rect(
        &mut self,
        pos: [f32; 2],
        extent: [f32; 2],
        color: u32,
        shapes: &ShapeStream,
        resolver: &mut impl FnMut(&str) -> u32,
    ) -> TextResult<()> {
        let mut raw = RawShapeStream::new();
        raw.append(shapes, resolver)?;
        self.rect_with_color(pos, extent, color, &raw)
    }
    /// Append local primitives with an offset and per-channel RGBA tint (white is identity).
    /// Glyph primitives must refer to handles already validated for this atlas.
    ///
    /// # Errors
    /// Nonfinite coordinates, negative extents, offset overflow or an atlas-changed
    /// frame leave the entire destination stream unchanged.
    pub fn append_primitives(
        &mut self,
        primitives: &[Primitive],
        offset: [f32; 2],
        tint: u32,
    ) -> TextResult<()> {
        self.check_frame()?;
        for value in offset {
            finite("primitive offset", value)?;
        }
        for primitive in primitives {
            let mut adjusted = *primitive;
            adjusted.pos_min[0] += offset[0];
            adjusted.pos_min[1] += offset[1];
            validate_primitive_geometry(&adjusted)?;
        }
        self.stream
            .primitives
            .extend(primitives.iter().map(|primitive| {
                let mut adjusted = *primitive;
                adjusted.pos_min[0] += offset[0];
                adjusted.pos_min[1] += offset[1];
                adjusted.color = tint_color(adjusted.color, tint);
                adjusted
            }));
        Ok(())
    }
    /// Borrow the current atlas and report the last successfully consumed primitive count.
    pub fn render(&self) -> (&AtlasDescriptor, u64) {
        (&self.atlas.descriptor, self.consumed_primitive_count)
    }
    /// Drain primitives, shapes and pending glyph uploads, then advance the frame stamp.
    /// Submit this batch against the current atlas before resetting or growing it.
    ///
    /// # Errors
    /// Returns [`TextError::AtlasChanged`] for a discarded frame, or an invalid-batch
    /// error if the frame counter is exhausted. Errors preserve queued data.
    pub fn consume(&mut self) -> TextResult<RenderBatch> {
        self.check_frame()?;
        let next_frame = self
            .atlas
            .frame
            .checked_add(1)
            .ok_or_else(|| TextError::InvalidBatch("frame counter exhausted".into()))?;
        self.consumed_primitive_count = self.stream.primitives.len() as u64;
        let mut batch = self.stream.consume();
        batch.glyph_uploads = std::mem::take(&mut self.atlas.pending);
        self.atlas.frame = next_frame;
        Ok(batch)
    }
    /// Borrow frame streams for an infallible consumer, then clear and advance them.
    /// A panic in `consume` leaves the frame intact. Use [`Self::try_consume_with`]
    /// when a normal callback error should retain the frame for retry.
    ///
    /// # Errors
    /// Reports an atlas-changed frame or frame-counter exhaustion before the callback.
    pub fn consume_with<T>(
        &mut self,
        consume: impl FnOnce(&[Primitive], &[u32]) -> T,
    ) -> TextResult<(T, Vec<PendingGlyphUpload>)> {
        self.try_consume_with(|primitives, shapes| Ok(consume(primitives, shapes)))
    }
    /// Borrow frame streams and commit only after the consumer returns success.
    /// The callback runs before pending uploads are drained. A returned error or panic
    /// preserves primitives, shape words, uploads, counts and the current frame stamp.
    ///
    /// # Errors
    /// Propagates callback errors and reports atlas invalidation or counter exhaustion.
    /// Caller-owned callback side effects cannot be rolled back.
    pub fn try_consume_with<T>(
        &mut self,
        consume: impl FnOnce(&[Primitive], &[u32]) -> TextResult<T>,
    ) -> TextResult<(T, Vec<PendingGlyphUpload>)> {
        self.check_frame()?;
        let next_frame = self
            .atlas
            .frame
            .checked_add(1)
            .ok_or_else(|| TextError::InvalidBatch("frame counter exhausted".into()))?;
        let primitive_count = self.stream.primitives.len() as u64;
        let result = consume(&self.stream.primitives, &self.stream.shapes.0)?;
        self.stream.clear();
        self.consumed_primitive_count = primitive_count;
        let uploads = std::mem::take(&mut self.atlas.pending);
        self.atlas.frame = next_frame;
        Ok((result, uploads))
    }
    fn check_frame(&self) -> TextResult<()> {
        if self.frame_invalidated {
            Err(TextError::AtlasChanged)
        } else {
            Ok(())
        }
    }
}
fn finite(field: &'static str, value: f32) -> TextResult<()> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(TextError::InvalidValue { field, value })
    }
}
fn positive(field: &'static str, value: f32) -> TextResult<()> {
    if value.is_finite() && value > 0. {
        Ok(())
    } else {
        Err(TextError::InvalidValue { field, value })
    }
}
fn glyph_rgba(
    content: SwashContent,
    data: Vec<u8>,
    width: u32,
    height: u32,
) -> TextResult<Vec<u8>> {
    let pixels = (width as usize)
        .checked_mul(height as usize)
        .ok_or(TextError::AtlasFull)?;
    let expected = match content {
        SwashContent::Mask => pixels,
        SwashContent::Color | SwashContent::SubpixelMask => {
            pixels.checked_mul(4).ok_or(TextError::AtlasFull)?
        }
    };
    if data.len() != expected {
        return Err(TextError::InvalidInput(
            "rasterizer returned an inconsistent glyph payload".into(),
        ));
    }
    Ok(match content {
        SwashContent::Mask => data
            .into_iter()
            .flat_map(|alpha| [255, 255, 255, alpha])
            .collect(),
        SwashContent::Color | SwashContent::SubpixelMask => data,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn validates_configuration_scaling_and_counter_exhaustion() {
        assert!(TextRenderer::with_atlas_size(0, 16).is_err());
        assert!(
            TextRenderer::with_config(AtlasConfig {
                width: 65536,
                height: 1,
                max_width: 65536,
                max_height: 1,
                max_bytes: u64::MAX
            })
            .is_err()
        );
        assert!(
            TextRenderer::with_config(AtlasConfig {
                width: 65535,
                height: 65535,
                max_width: 65535,
                max_height: 65535,
                max_bytes: u64::MAX
            })
            .is_err()
        );
        let mut renderer = TextRenderer::new();
        for scale in [0., -1., f32::NAN, f32::INFINITY] {
            assert!(renderer.set_pixels_per_point(scale).is_err());
        }
        let revision = renderer.atlas().generation;
        renderer.reset_glyph_cache().unwrap();
        assert!(renderer.atlas().generation > revision);
        let revision = renderer.atlas().generation;
        renderer.reset_glyph_cache().unwrap();
        assert!(renderer.atlas().generation > revision);
        renderer.atlas.frame = u64::MAX;
        assert!(renderer.consume().is_err());
        assert_eq!(renderer.current_frame(), u64::MAX);
    }
    #[test]
    fn rejected_callback_and_append_preserve_frame_data() {
        let mut renderer = TextRenderer::new();
        renderer
            .rect([0., 0.], [1., 1.], 1., &RawShapeStream::new())
            .unwrap();
        let primitives = [
            Primitive::rect([0., 0.], [1., 1.], 0),
            Primitive::rect([f32::MAX, 0.], [1., 1.], 0),
        ];
        assert!(
            renderer
                .append_primitives(&primitives, [f32::MAX, 0.], 0)
                .is_err()
        );
        let frame = renderer.current_frame();
        let failure: TextResult<((), Vec<PendingGlyphUpload>)> = renderer
            .try_consume_with(|_, _| Err(TextError::InvalidInput("consumer failed".into())));
        assert!(failure.is_err());
        assert_eq!(renderer.current_frame(), frame);
        assert_eq!(renderer.consume().unwrap().primitives.len(), 1);
        assert!(
            renderer
                .rect([0., 0.], [1., 1.], 1., &RawShapeStream(vec![0]))
                .is_err()
        );
        assert!(renderer.consume().unwrap().primitives.is_empty());
        let selection = Primitive::rect([0., 0.], [1., 1.], 0x80ff_ffff);
        renderer
            .append_primitives(&[selection], [0., 0.], u32::MAX)
            .unwrap();
        assert_eq!(
            renderer.consume().unwrap().primitives[0].color,
            selection.color
        );
        let colored = Primitive::rect([0., 0.], [1., 1.], 0x8040_2010);
        renderer
            .append_primitives(&[colored], [0., 0.], 0x8080_8080)
            .unwrap();
        assert_eq!(renderer.consume().unwrap().primitives[0].color, 0x4020_1008);
    }
    #[test]
    fn empty_shape_rect_is_solid_and_invalid_inputs_leave_the_stream_unchanged() {
        let mut renderer = TextRenderer::new();
        renderer
            .rect([0., 0.], [1., 1.], 1., &RawShapeStream::new())
            .unwrap();
        assert!(
            renderer
                .rect([f32::NAN, 0.], [1., 1.], 1., &RawShapeStream::new())
                .is_err()
        );
        assert!(
            renderer
                .rect([0., 0.], [-1., 1.], 1., &RawShapeStream::new())
                .is_err()
        );
        assert!(
            renderer
                .rect([0., 0.], [1., 1.], 1., &RawShapeStream(vec![0; 256]))
                .is_err()
        );
        let batch = renderer.consume().unwrap();
        assert_eq!(batch.primitives.len(), 1);
        assert_eq!(batch.primitives[0].stream, u32::MAX);
        assert_eq!(std::mem::size_of::<Primitive>(), 48);
        assert!(
            ShapePacket {
                word_offset: 0x00ff_ffff,
                word_count: 255
            }
            .pack()
            .is_err()
        );
        assert_eq!(
            ShapePacket {
                word_offset: 0x00ff_ffff,
                word_count: 254
            }
            .pack()
            .unwrap(),
            u32::MAX - 1
        );
    }
}
