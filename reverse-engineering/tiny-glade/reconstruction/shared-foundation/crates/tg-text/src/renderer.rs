//! 字体、字形 atlas 与原 48 字节 primitive 流；通用整形/分配由原依赖执行。
use crate::{RawShapeStream, ShapeStream};
use cosmic_text::{CacheKey, FontSystem, SwashCache, SwashContent};
use etagere::{AllocId, AtlasAllocator, Size};
use std::{collections::HashMap, path::Path};

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
    pub allocation_generation: u64,
    pub allocation: AllocId,
    pub atlas: [u32; 2],
    pub size: [u32; 2],
    pub bearing: [i32; 2],
    pub generation: u64,
}
#[derive(Clone, Copy, Debug)]
pub struct AtlasAllocUsage {
    pub last_frame: u64,
    pub generation: u64,
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
    usage: HashMap<AllocId, AtlasAllocUsage>,
    pending: Vec<PendingGlyphUpload>,
    stream: TextRenderStream,
    pixels_per_point: f32,
    generation: u64,
    frame: u64,
    grow_requested: bool,
    allocation_generation: u64,
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
            usage: HashMap::new(),
            pending: Vec::new(),
            stream: TextRenderStream::default(),
            pixels_per_point: 1.,
            generation: 1,
            frame: 1,
            grow_requested: false,
            allocation_generation: 0,
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
    pub fn atlas(&self) -> AtlasDescriptor {
        let size = self.allocator.size();
        AtlasDescriptor {
            width: size.width as u32,
            height: size.height as u32,
            generation: self.generation,
        }
    }
    pub fn set_pixels_per_point(&mut self, value: f32) {
        if self.pixels_per_point != value {
            self.pixels_per_point = value;
            self.invalidate_atlas();
        }
    }
    /// 独立消费者重建设备/atlas后的恢复扩展；原公开接口不作此命名承诺。
    pub fn reset_glyph_cache(&mut self) {
        self.invalidate_atlas();
    }
    fn invalidate_atlas(&mut self) {
        self.generation = self.generation.wrapping_add(1);
        self.allocator.clear();
        self.glyph_cache.clear();
        self.usage.clear();
        self.pending.clear();
        self.grow_requested = false;
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
        if glyphs.iter().any(|g| {
            g.generation != self.generation
                || self
                    .usage
                    .get(&g.allocation)
                    .is_none_or(|u| u.generation != g.allocation_generation)
        }) {
            return false;
        }
        for glyph in glyphs {
            self.usage.get_mut(&glyph.allocation).unwrap().last_frame = self.frame;
        }
        true
    }
    fn allocate_atlas_space(&mut self, width: u32, height: u32) -> Option<etagere::Allocation> {
        let size = Size::new(i32::try_from(width).ok()?, i32::try_from(height).ok()?);
        if let Some(a) = self.allocator.allocate(size) {
            return Some(a);
        }
        // 原 get_glyph：回收 last_used + 1 < current_frame 的分配后再尝试。
        let expired: Vec<_> = self
            .usage
            .iter()
            .filter(|(_, u)| u.last_frame.saturating_add(1) < self.frame)
            .map(|(&id, _)| id)
            .collect();
        for id in expired {
            self.allocator.deallocate(id);
            self.usage.remove(&id);
            self.stats.evicted += 1;
        }
        self.glyph_cache
            .retain(|_, entry| entry.is_none_or(|g| self.usage.contains_key(&g.allocation)));
        self.allocator.allocate(size)
    }
    pub fn get_glyph(&mut self, key: CacheKey) -> Option<Glyph> {
        if let Some(entry) = self.glyph_cache.get(&key).copied() {
            self.stats.cache_hits += 1;
            if let Some(u) = entry.and_then(|g| self.usage.get_mut(&g.allocation)) {
                u.last_frame = self.frame;
            }
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
        let mut rgba = Vec::with_capacity(width as usize * height as usize * 4);
        match image.content {
            SwashContent::Mask => {
                for alpha in image.data {
                    rgba.extend([255, 255, 255, alpha]);
                }
            }
            SwashContent::Color => rgba = image.data,
            SwashContent::SubpixelMask => {
                for pixel in image.data.chunks_exact(3) {
                    rgba.extend([pixel[0], pixel[1], pixel[2], *pixel.iter().max().unwrap()]);
                }
            }
        }
        self.allocation_generation = self.allocation_generation.wrapping_add(1);
        let glyph = Glyph {
            allocation_generation: self.allocation_generation,
            allocation: allocation.id,
            atlas: [
                allocation.rectangle.min.x as u32,
                allocation.rectangle.min.y as u32,
            ],
            size: [width, height],
            bearing: [image.placement.left, -image.placement.top],
            generation: self.generation,
        };
        self.usage.insert(
            allocation.id,
            AtlasAllocUsage {
                last_frame: self.frame,
                generation: self.allocation_generation,
            },
        );
        self.pending.push(PendingGlyphUpload {
            x: glyph.atlas[0],
            y: glyph.atlas[1],
            width,
            height,
            rgba,
            generation: self.generation,
        });
        self.glyph_cache.insert(key, Some(glyph));
        Some(glyph)
    }
    /// 原 overflow 在 upload 阶段倍增 atlas、清缓存并废弃待上传；下一帧重新生成。
    pub fn upload_pending_glyphs(&mut self) -> Vec<PendingGlyphUpload> {
        if self.grow_requested {
            let size = self.allocator.size();
            let width = size.width.checked_mul(2).expect("atlas width overflow");
            let height = size.height.checked_mul(2).expect("atlas height overflow");
            self.invalidate_atlas();
            self.allocator = AtlasAllocator::new(Size::new(width, height));
            self.swash = SwashCache::new();
            self.stats.atlas_rebuilds += 1;
            Vec::new()
        } else {
            std::mem::take(&mut self.pending)
        }
    }
    pub fn rect(&mut self, pos: [f32; 2], extent: [f32; 2], color: u32, shapes: &RawShapeStream) {
        assert!(shapes.0.len() < 256, "shape stream exceeds packed length");
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
    pub fn rect_with_opacity(
        &mut self,
        pos: [f32; 2],
        extent: [f32; 2],
        opacity: f32,
        shapes: &RawShapeStream,
    ) {
        self.rect(
            pos,
            extent,
            crate::scale_u32_packed_srgba_color(u32::MAX, opacity),
            shapes,
        );
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
        self.rect(pos, extent, color, &raw);
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
    pub fn render(&self) -> AtlasDescriptor {
        self.atlas()
    }
    pub fn consume(&mut self) -> RenderBatch {
        let batch = self.stream.consume();
        self.frame = self.frame.wrapping_add(1);
        batch
    }
    pub fn consume_with<T>(&mut self, consume: impl FnOnce(&[Primitive], &[u32]) -> T) -> T {
        let result = self.stream.consume_with(consume);
        self.frame = self.frame.wrapping_add(1);
        result
    }
}
