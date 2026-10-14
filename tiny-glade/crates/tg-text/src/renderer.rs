//! 字体、字形 atlas 与原 48 字节 primitive 流；通用整形/分配由原依赖执行。
use crate::{RawShapeStream, ShapeStream};
use cosmic_text::{CacheKey, FontSystem, SwashCache, SwashContent};
use etagere::{AllocId, AtlasAllocator, Size};
use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

// 原锁 bytemuck_derive 1.4.1 在新版 Rust 生成未使用的静态布局检查 helper。
#[allow(dead_code)]
mod packed {
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
        pub fn rect(pos_min: [f32; 2], pos_extent: [f32; 2], color: u32) -> Self {
            Self {
                pos_min,
                pos_extent,
                uv_min: [0.; 2],
                uv_extent: [1.; 2],
                color,
                stream: u32::MAX,
                padding: [0; 2],
            }
        }
    }
}
pub use packed::Primitive;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AtlasDescriptor {
    pub width: u32,
    pub height: u32,
    pub generation: u64,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PendingGlyphUpload {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<u8>,
    pub generation: u64,
}
#[derive(Clone, Debug, Default)]
pub struct RenderBatch {
    pub primitives: Vec<Primitive>,
    pub shape_words: Vec<u32>,
    /// CPU-produced glyph payloads from the upload step before frame advancement.
    /// Native GPU submission is outside this module's current execution boundary.
    pub glyph_uploads: Vec<PendingGlyphUpload>,
}
impl RenderBatch {
    pub fn primitive_bytes(&self) -> &[u8] {
        bytemuck::cast_slice(&self.primitives)
    }
}
#[derive(Clone, Debug, Default)]
pub struct TextRenderStream {
    pub primitives: Vec<Primitive>,
    pub shapes: RawShapeStream,
}
impl TextRenderStream {
    pub fn consume(&mut self) -> RenderBatch {
        RenderBatch {
            primitives: std::mem::take(&mut self.primitives),
            shape_words: std::mem::take(&mut self.shapes.0),
            glyph_uploads: Vec::new(),
        }
    }
    pub fn consume_with<T>(&mut self, consume: impl FnOnce(&[Primitive], &[u32]) -> T) -> T {
        let result = consume(&self.primitives, &self.shapes.0);
        self.clear();
        result
    }
    pub fn clear(&mut self) {
        self.primitives.clear();
        self.shapes.clear();
    }
}
#[derive(Clone, Copy, Debug)]
pub struct Glyph {
    pub allocation: AllocId,
    pub atlas: [u32; 2],
    pub size: [u32; 2],
    pub bearing: [i32; 2],
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct AtlasAllocUsage {
    pub last_frame: u32,
    pub generation: u16,
}
impl AtlasAllocUsage {
    fn is_expired(self, frame: u32) -> bool {
        self.last_frame != 0 && self.last_frame.wrapping_add(1) < frame
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TextStats {
    pub rasterized: u64,
    pub cache_hits: u64,
    pub evicted: u64,
    pub atlas_rebuilds: u64,
}

#[derive(bevy_ecs::prelude::Resource)]
pub struct TextRenderer {
    pub font_system: FontSystem,
    swash: SwashCache,
    allocator: AtlasAllocator,
    glyph_cache: HashMap<CacheKey, Option<Glyph>>,
    usage: Vec<AtlasAllocUsage>,
    pending: Vec<PendingGlyphUpload>,
    stream: TextRenderStream,
    // 原 +0x440：consume 开始时保存图元数，供 render getter 使用。
    consumed_primitive_count: u64,
    atlas_view: AtlasDescriptor,
    pixels_per_point: f32,
    invalidation_frame: u32,
    frame: u32,
    grow_requested: bool,
    pub stats: TextStats,
}
impl Default for TextRenderer {
    fn default() -> Self {
        Self::new()
    }
}
pub fn init_font_system(locale: Option<String>) -> FontSystem {
    // 原 PDB 显示 new_with_locale_and_db；只加载调用方明确提供的发行字体。
    FontSystem::new_with_locale_and_db(
        locale
            .or_else(sys_locale::get_locale)
            .unwrap_or_else(|| "en-US".into()),
        cosmic_text::fontdb::Database::new(),
    )
}
impl TextRenderer {
    pub fn new() -> Self {
        Self::with_atlas_size(1024, 1024)
    }
    /// 测试/独立消费者可选择较小 atlas；原默认尺寸由 new 的 0x400 证实。
    pub fn with_atlas_size(width: u32, height: u32) -> Self {
        assert!(width > 0 && height > 0 && width <= i32::MAX as u32 && height <= i32::MAX as u32);
        Self {
            font_system: init_font_system(None),
            swash: SwashCache::new(),
            allocator: AtlasAllocator::new(Size::new(width as i32, height as i32)),
            glyph_cache: HashMap::new(),
            usage: Vec::new(),
            pending: Vec::new(),
            stream: TextRenderStream::default(),
            consumed_primitive_count: 0,
            atlas_view: AtlasDescriptor {
                width,
                height,
                generation: 1,
            },
            pixels_per_point: 1.,
            invalidation_frame: 1,
            frame: 1,
            grow_requested: false,
            stats: TextStats::default(),
        }
    }
    pub fn load_font_from_file(&mut self, path: impl AsRef<Path>) -> std::io::Result<()> {
        self.font_system.db_mut().load_font_file(path)
    }
    pub fn load_font_data(&mut self, bytes: Vec<u8>) {
        self.font_system.db_mut().load_font_data(bytes);
    }
    pub fn font_faces(&self) -> impl Iterator<Item = &cosmic_text::fontdb::FaceInfo> {
        self.font_system.db().faces()
    }
    pub fn pixels_per_point(&self) -> f32 {
        self.pixels_per_point
    }
    pub fn current_frame(&self) -> u32 {
        self.frame
    }
    pub fn invalidation_frame(&self) -> u32 {
        self.invalidation_frame
    }
    pub fn atlas(&self) -> AtlasDescriptor {
        self.atlas_view
    }
    pub fn set_pixels_per_point(&mut self, value: f32) {
        if self.pixels_per_point != value {
            self.pixels_per_point = value;
            self.clear_atlas_for_scale();
        }
    }
    /// 独立消费者重建设备/atlas后的恢复扩展；原公开接口不作此命名承诺。
    pub fn reset_glyph_cache(&mut self) {
        self.clear_atlas_for_scale();
    }
    fn clear_atlas_for_scale(&mut self) {
        // 0x140b8f3f0: stamp is the current frame, including frame zero;
        // same-frame invalidations do not invent a new epoch.
        self.invalidation_frame = self.frame;
        self.atlas_view.generation = u64::from(self.invalidation_frame);
        self.allocator.clear();
        self.glyph_cache.clear();
        self.usage.fill(AtlasAllocUsage::default());
        self.pending.clear();
    }
    pub fn begin_frame(&mut self) {
        self.stream.clear();
    }
    pub fn cache_len(&self) -> usize {
        self.glyph_cache.len()
    }
    pub fn pending_uploads(&self) -> &[PendingGlyphUpload] {
        &self.pending
    }
    pub fn validate_glyph_cache_allocs(&mut self, glyphs: &[Glyph]) -> bool {
        for glyph in glyphs {
            let id = glyph.allocation.serialize();
            // Original validation deliberately panics for an out-of-bounds slot.
            let usage = &mut self.usage[(id & 0xffff) as usize];
            if usage.generation != (id >> 16) as u16 || usage.last_frame == 0 {
                return false;
            }
            // A later invalid item does not roll back the already visited prefix.
            usage.last_frame = self.frame;
        }
        true
    }
    fn allocate_atlas_space(&mut self, width: u32, height: u32) -> Option<etagere::Allocation> {
        let size = Size::new(i32::try_from(width).ok()?, i32::try_from(height).ok()?);
        if let Some(a) = self.allocator.allocate(size) {
            return Some(a);
        }
        // 原 get_glyph：回收 last_used + 1 < current_frame 的分配后再尝试。
        let mut removed = HashSet::new();
        for (index, usage) in self.usage.iter_mut().enumerate() {
            if usage.is_expired(self.frame) {
                let id = AllocId::deserialize(index as u32 | (u32::from(usage.generation) << 16));
                self.allocator.deallocate(id);
                usage.last_frame = 0;
                removed.insert(id);
                self.stats.evicted += 1;
            }
        }
        self.glyph_cache
            .retain(|_, entry| entry.is_none_or(|g| !removed.contains(&g.allocation)));
        self.allocator.allocate(size)
    }
    pub fn get_glyph(&mut self, key: CacheKey) -> Option<Glyph> {
        if let Some(entry) = self.glyph_cache.get(&key).copied() {
            self.stats.cache_hits += 1;
            return entry;
        }
        let image = self.swash.get_image_uncached(&mut self.font_system, key)?;
        self.stats.rasterized += 1;
        let width = image.placement.width;
        let height = image.placement.height;
        if width == 0 || height == 0 {
            self.glyph_cache.insert(key, None);
            return None;
        }
        let Some(allocation) = self.allocate_atlas_space(width, height) else {
            self.grow_requested = true;
            return None;
        };
        let rgba = glyph_rgba(image.content, image.data);
        let glyph = Glyph {
            allocation: allocation.id,
            atlas: [
                allocation.rectangle.min.x as u32,
                allocation.rectangle.min.y as u32,
            ],
            size: [width, height],
            bearing: [image.placement.left, -image.placement.top],
        };
        self.track_allocation(allocation.id);
        self.pending.push(PendingGlyphUpload {
            x: glyph.atlas[0],
            y: glyph.atlas[1],
            width,
            height,
            rgba,
            generation: u64::from(self.invalidation_frame),
        });
        self.glyph_cache.insert(key, Some(glyph));
        Some(glyph)
    }
    fn track_allocation(&mut self, allocation: AllocId) {
        let id = allocation.serialize();
        let index = (id & 0xffff) as usize;
        if self.usage.len() <= index {
            self.usage.resize(index + 1, AtlasAllocUsage::default());
        }
        self.usage[index] = AtlasAllocUsage {
            last_frame: self.frame,
            generation: (id >> 16) as u16,
        };
    }
    /// 原 overflow 在 upload 阶段倍增 atlas、清缓存并废弃待上传；下一帧重新生成。
    pub fn upload_pending_glyphs(&mut self) -> Vec<PendingGlyphUpload> {
        if self.grow_requested {
            self.grow_requested = false;
            let size = self.allocator.size();
            let width = size.width.checked_mul(2).expect("atlas width overflow");
            let height = size.height.checked_mul(2).expect("atlas height overflow");
            self.allocator = AtlasAllocator::new(Size::new(width, height));
            self.atlas_view.width = width as u32;
            self.atlas_view.height = height as u32;
            self.swash = SwashCache::new();
            self.glyph_cache.clear();
            self.usage = Vec::new();
            self.pending.clear();
            self.invalidation_frame = self.frame;
            self.atlas_view.generation = u64::from(self.invalidation_frame);
            self.stats.atlas_rebuilds += 1;
            Vec::new()
        } else {
            std::mem::take(&mut self.pending)
        }
    }
    /// 原 TextRenderer::rect(pos, extent, opacity, shapes)：颜色固定白色 u32::MAX，
    /// alpha 经 utils::scale_u32_packed_srgba_color 按 opacity 缩放（RNE 0x140b8f548/0x140b8f550）。
    pub fn rect(&mut self, pos: [f32; 2], extent: [f32; 2], opacity: f32, shapes: &RawShapeStream) {
        assert!(shapes.0.len() < 256, "stream_len < 256");
        self.rect_with_color(
            pos,
            extent,
            crate::scale_u32_packed_srgba_color(u32::MAX, opacity),
            shapes,
        );
    }
    /// 重建扩展：候选显式颜色入口；原 rect 固定白色仅接受 opacity。
    pub fn rect_with_color(
        &mut self,
        pos: [f32; 2],
        extent: [f32; 2],
        color: u32,
        shapes: &RawShapeStream,
    ) {
        assert!(shapes.0.len() < 256, "stream_len < 256");
        let start = self.stream.shapes.0.len();
        assert!(
            start <= 0x00ff_ffff,
            "shape stream offset exceeds packed range"
        );
        let mut primitive = Primitive::rect(pos, extent, color);
        primitive.stream = ((start as u32) << 8) | shapes.0.len() as u32;
        self.stream.primitives.push(primitive);
        self.stream.shapes.0.extend_from_slice(&shapes.0);
    }
    pub fn shaped_rect(
        &mut self,
        pos: [f32; 2],
        extent: [f32; 2],
        color: u32,
        shapes: &ShapeStream,
        resolver: &mut impl FnMut(&str) -> u32,
    ) {
        let mut raw = RawShapeStream::new();
        raw.append(shapes, resolver);
        self.rect_with_color(pos, extent, color, &raw);
    }
    pub fn append_primitives(&mut self, primitives: &[Primitive], offset: [f32; 2], color: u32) {
        self.stream.primitives.extend(primitives.iter().map(|p| {
            let mut p = *p;
            p.pos_min[0] += offset[0];
            p.pos_min[1] += offset[1];
            p.color = color;
            p
        }));
    }
    /// 原 0x140b8f670：借用 atlas，并返回最近一次 consume 的图元数。
    /// 原返回 &rhapsody::raw::Texture；CPU 接口仅提供它的 descriptor，
    /// 真实 native texture 仍由可选 GPU 适配器持有。
    pub fn render(&self) -> (&AtlasDescriptor, u64) {
        (&self.atlas_view, self.consumed_primitive_count)
    }
    pub fn consume(&mut self) -> RenderBatch {
        self.consumed_primitive_count = self.stream.primitives.len() as u64;
        let mut batch = self.stream.consume();
        // Original consume clears both streams, uploads pending glyphs using
        // the old frame, then advances the u32 frame counter.
        batch.glyph_uploads = self.upload_pending_glyphs();
        self.frame = self.frame.wrapping_add(1);
        batch
    }
    pub fn consume_with<T>(
        &mut self,
        consume: impl FnOnce(&[Primitive], &[u32]) -> T,
    ) -> (T, Vec<PendingGlyphUpload>) {
        // 原 0x140fac810：先保存 count，再调用上传 closure；失败不清流、不推进 frame。
        self.consumed_primitive_count = self.stream.primitives.len() as u64;
        let result = self.stream.consume_with(consume);
        let uploads = self.upload_pending_glyphs();
        self.frame = self.frame.wrapping_add(1);
        (result, uploads)
    }
    #[cfg(test)]
    pub(crate) fn set_cache_fixture(&mut self, frame: u32, stamp: u32, usage: &[[u32; 2]]) {
        self.frame = frame;
        self.invalidation_frame = stamp;
        self.atlas_view.generation = u64::from(stamp);
        self.usage = usage
            .iter()
            .map(|u| AtlasAllocUsage {
                last_frame: u[0],
                generation: u[1] as u16,
            })
            .collect();
    }
    #[cfg(test)]
    pub(crate) fn usage_fixture(&self) -> Vec<[u32; 2]> {
        self.usage
            .iter()
            .map(|u| [u.last_frame, u32::from(u.generation)])
            .collect()
    }
}

fn glyph_rgba(content: SwashContent, data: Vec<u8>) -> Vec<u8> {
    match content {
        SwashContent::Mask => data
            .into_iter()
            .flat_map(|alpha| [255, 255, 255, alpha])
            .collect(),
        // 原 upload fold 0x14180585e：content tag 非零时直接使用原 bytes。
        // swash 0.1.16 的 Color/SubpixelMask 都是 RGBA32；当前 cosmic fork
        // 固定 Format::Alpha，通常不产生 SubpixelMask。
        SwashContent::Color | SwashContent::SubpixelMask => data,
    }
}

#[cfg(test)]
mod followup_tests {
    use super::*;

    #[test]
    fn upload_content_dispatch_preserves_rgba_and_expands_only_alpha() {
        let bytes = vec![1, 2, 3, 4, 5, 6, 7, 8];
        assert_eq!(glyph_rgba(SwashContent::Color, bytes.clone()), bytes);
        assert_eq!(glyph_rgba(SwashContent::SubpixelMask, bytes.clone()), bytes);
        assert_eq!(
            glyph_rgba(SwashContent::Mask, vec![0, 128, 255]),
            [255, 255, 255, 0, 255, 255, 255, 128, 255, 255, 255, 255]
        );
    }

    #[test]
    fn callback_unwind_does_not_advance_frame_and_scale_preserves_growth_request() {
        let mut renderer = TextRenderer::new();
        let frame = renderer.frame;
        renderer.rect([0.; 2], [1.; 2], 1., &RawShapeStream::new());
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            renderer.consume_with(|_, _| panic!("upload failure"));
        }));
        assert!(result.is_err());
        assert_eq!(renderer.frame, frame);
        renderer.consume();
        assert_eq!(renderer.frame, frame + 1);
        renderer.grow_requested = true;
        renderer.set_pixels_per_point(2.);
        assert!(renderer.grow_requested);
        renderer.upload_pending_glyphs();
        assert_eq!(renderer.atlas().width, 2048);
    }

    #[test]
    fn original_frame_stores_expiry_and_slot_writes() {
        let evidence: serde_json::Value = serde_json::from_str(include_str!(
            "../tests/fixtures/internal-native-cache-oracle.json"
        ))
        .unwrap();
        for case in evidence["expiry_cases"].as_array().unwrap() {
            let usage = AtlasAllocUsage {
                last_frame: case["last"].as_u64().unwrap() as u32,
                generation: 65535,
            };
            assert_eq!(
                usage.is_expired(case["frame"].as_u64().unwrap() as u32),
                case["expired"].as_bool().unwrap()
            );
        }
        for case in evidence["overflow_consume_store_cases"].as_array().unwrap() {
            let mut renderer = TextRenderer::with_atlas_size(8, 8);
            let frame = case["before_frame"].as_u64().unwrap() as u32;
            renderer.set_cache_fixture(frame, 7, &[[9, 65535]]);
            renderer.grow_requested = true;
            let batch = renderer.consume();
            assert!(batch.glyph_uploads.is_empty());
            assert_eq!(
                renderer.invalidation_frame,
                case["overflow_stamp"].as_u64().unwrap() as u32
            );
            assert_eq!(
                renderer.frame,
                case["after_consume_frame"].as_u64().unwrap() as u32
            );
            assert!(renderer.usage.is_empty());
            assert_eq!(renderer.atlas().width, 16);
        }
        for case in evidence["setter_prefix_cases"].as_array().unwrap() {
            let mut renderer = TextRenderer::with_atlas_size(8, 8);
            let frame = case["frame"].as_u64().unwrap() as u32;
            renderer.set_cache_fixture(frame, 7, &[[9, 65535]]);
            renderer.pixels_per_point = f32::from_bits(case["old_bits"].as_u64().unwrap() as u32);
            renderer.grow_requested = true;
            renderer
                .set_pixels_per_point(f32::from_bits(case["new_bits"].as_u64().unwrap() as u32));
            assert_eq!(
                renderer.invalidation_frame,
                case["stamp"].as_u64().unwrap() as u32
            );
            assert!(renderer.grow_requested);
            assert_eq!(renderer.usage.len(), 1);
            if case["changed"].as_u64().unwrap() != 0 {
                assert_eq!(renderer.usage_fixture(), [[0, 0]]);
            } else {
                assert_eq!(renderer.usage_fixture(), [[9, 65535]]);
            }
        }
        for case in evidence["usage_helper_cases"].as_array().unwrap() {
            let mut renderer = TextRenderer::with_atlas_size(8, 8);
            renderer.set_cache_fixture(case["frame"].as_u64().unwrap() as u32, 1, &[[9, 7]; 3]);
            let index = case["index"].as_u64().unwrap() as u32;
            let generation = case["generation"].as_u64().unwrap() as u32;
            renderer.track_allocation(AllocId::deserialize(index | (generation << 16)));
            assert_eq!(
                renderer.usage.len() as u64,
                case["vector_len"].as_u64().unwrap()
            );
            assert_eq!(
                renderer.usage_fixture()[index as usize],
                [renderer.frame, generation]
            );
        }
    }

    #[test]
    fn cache_hit_does_not_touch_usage_and_uploads_precede_frame_advance() {
        let mut renderer = TextRenderer::new();
        renderer.set_cache_fixture(9, 3, &[[4, 65535]]);
        let key = CacheKey::new(
            cosmic_text::fontdb::ID::dummy(),
            1,
            20.,
            (0., 0.),
            cosmic_text::CacheKeyFlags::empty(),
        )
        .0;
        let glyph = Glyph {
            allocation: AllocId::deserialize(0xffff0000),
            atlas: [0; 2],
            size: [1; 2],
            bearing: [0; 2],
        };
        renderer.glyph_cache.insert(key, Some(glyph));
        assert_eq!(
            renderer.get_glyph(key).unwrap().allocation,
            glyph.allocation
        );
        assert_eq!(renderer.usage_fixture(), [[4, 65535]]);
        renderer.pending.push(PendingGlyphUpload {
            x: 0,
            y: 0,
            width: 1,
            height: 1,
            rgba: vec![255; 4],
            generation: 3,
        });
        let batch = renderer.consume();
        assert_eq!(batch.glyph_uploads.len(), 1);
        assert!(renderer.pending.is_empty());
        assert_eq!(renderer.frame, 10);
        assert_eq!(renderer.invalidation_frame, 3);
    }
    #[test]
    fn original_usage_gap_initialization() {
        let evidence: serde_json::Value = serde_json::from_str(include_str!(
            "../tests/fixtures/internal-native-same-id.json"
        ))
        .unwrap();
        for case in evidence["gap_cases"].as_array().unwrap() {
            let mut renderer = TextRenderer::new();
            renderer.set_cache_fixture(
                case["frame"].as_u64().unwrap() as u32,
                1,
                &[[10, 0], [11, 1], [12, 2]],
            );
            renderer.usage.reserve(5);
            let before = renderer.usage.as_ptr();
            renderer.track_allocation(AllocId::deserialize(
                case["requested_index"].as_u64().unwrap() as u32,
            ));
            assert_eq!(renderer.usage.as_ptr(), before);
            let length = case["after_len"].as_u64().unwrap() as usize;
            assert_eq!(renderer.usage.len(), length);
            let original: Vec<[u32; 2]> =
                serde_json::from_value(case["after_usage"].clone()).unwrap();
            assert_eq!(renderer.usage_fixture(), original[..length]);
        }
    }
}
