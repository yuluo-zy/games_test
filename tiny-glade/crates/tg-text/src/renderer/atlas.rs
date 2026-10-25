use super::{AtlasDescriptor, Glyph, PendingGlyphUpload, TextStats};
use crate::{TextError, TextResult};
use cosmic_text::CacheKey;
use etagere::{AllocId, Allocation, AtlasAllocator, Size};
use std::{
    collections::HashMap,
    sync::atomic::{AtomicU64, Ordering},
};

// Leave headroom for allocator split nodes within its 16-bit slot index space.
const MAX_ALLOCATIONS: usize = 16_000;

static REVISION: AtomicU64 = AtomicU64::new(1);
fn revision() -> TextResult<u64> {
    REVISION
        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |value| {
            value.checked_add(1)
        })
        .map_err(|_| TextError::InvalidAtlas("revision space exhausted"))
}

/// Initial RGBA atlas dimensions and hard growth limits, measured in pixels and bytes.
///
/// [`super::TextRenderer::with_config`] validates positive initial dimensions, maxima
/// no greater than 65535, and an initial pixel area representable by `i32`. Each
/// allocation is four bytes per pixel and must fit `max_bytes`; growth uses the
/// smallest permitted doubled dimensions that satisfy the pending request.
#[derive(Clone, Copy, Debug)]
pub struct AtlasConfig {
    /// Initial atlas width in pixels.
    pub width: u32,
    /// Initial atlas height in pixels.
    pub height: u32,
    /// Maximum width after growth; must be at least `width`.
    pub max_width: u32,
    /// Maximum height after growth; must be at least `height`.
    pub max_height: u32,
    /// Maximum RGBA atlas byte size, including four channels for every pixel.
    pub max_bytes: u64,
}
impl Default for AtlasConfig {
    fn default() -> Self {
        Self {
            width: 1024,
            height: 1024,
            max_width: 4096,
            max_height: 4096,
            max_bytes: 64 * 1024 * 1024,
        }
    }
}
impl AtlasConfig {
    fn validate(self) -> TextResult<()> {
        if self.width == 0
            || self.height == 0
            || self.width > self.max_width
            || self.height > self.max_height
            || self.max_width > u16::MAX as u32
            || self.max_height > u16::MAX as u32
            || !self.fits(self.width, self.height)
        {
            return Err(TextError::InvalidAtlas(
                "dimensions must be positive and fit the configured bounds",
            ));
        }
        Ok(())
    }
    fn fits(self, width: u32, height: u32) -> bool {
        u64::from(width)
            .checked_mul(u64::from(height))
            .filter(|pixels| *pixels <= i32::MAX as u64)
            .and_then(|pixels| pixels.checked_mul(4))
            .is_some_and(|bytes| bytes <= self.max_bytes)
    }
}

#[derive(Clone, Copy)]
struct Usage {
    glyph: Glyph,
    frame: u64,
}
pub(super) struct AtlasCache {
    pub descriptor: AtlasDescriptor,
    pub owner: u64,
    pub frame: u64,
    config: AtlasConfig,
    allocator: AtlasAllocator,
    pub cache: HashMap<CacheKey, Option<Glyph>>,
    usage: HashMap<AllocId, Usage>,
    next_allocation: u64,
    pub pending: Vec<PendingGlyphUpload>,
}
impl AtlasCache {
    pub fn new(config: AtlasConfig) -> TextResult<Self> {
        config.validate()?;
        let generation = revision()?;
        Ok(Self {
            descriptor: AtlasDescriptor {
                width: config.width,
                height: config.height,
                generation,
            },
            owner: generation,
            frame: 1,
            allocator: AtlasAllocator::new(Size::new(config.width as i32, config.height as i32)),
            config,
            cache: HashMap::new(),
            usage: HashMap::new(),
            next_allocation: 1,
            pending: Vec::new(),
        })
    }
    pub fn token(&self) -> (u64, u64) {
        (self.owner, self.descriptor.generation)
    }
    pub fn reset(&mut self) -> TextResult<()> {
        let generation = revision()?;
        self.descriptor.generation = generation;
        self.allocator.clear();
        self.cache.clear();
        self.usage.clear();
        self.pending.clear();
        Ok(())
    }
    pub fn validate(&mut self, glyphs: &[Glyph]) -> bool {
        if !glyphs.iter().all(|glyph| {
            glyph.owner == self.owner
                && glyph.epoch == self.descriptor.generation
                && self
                    .usage
                    .get(&glyph.allocation)
                    .is_some_and(|usage| usage.glyph == *glyph)
        }) {
            return false;
        }
        for glyph in glyphs {
            if let Some(usage) = self.usage.get_mut(&glyph.allocation) {
                usage.frame = self.frame;
            }
        }
        true
    }
    pub fn cache_empty(&mut self, key: CacheKey) {
        // Empty/missing glyphs consume no atlas area, so cap their metadata separately.
        if self.cache.len() >= 65_536 {
            self.cache.retain(|_, glyph| glyph.is_some());
        }
        if self.cache.len() < 65_536 {
            self.cache.insert(key, None);
        }
    }
    pub fn hit(&mut self, key: &CacheKey) -> Option<Option<Glyph>> {
        let entry = self.cache.get(key).copied()?;
        if let Some(glyph) = entry
            && !self.validate(&[glyph])
        {
            self.cache.remove(key);
            return None;
        }
        Some(entry)
    }
    fn reserve(
        &mut self,
        width: u32,
        height: u32,
        stats: &mut TextStats,
    ) -> TextResult<Allocation> {
        let requested_size = Size::new(
            i32::try_from(width).map_err(|_| TextError::AtlasFull)?,
            i32::try_from(height).map_err(|_| TextError::AtlasFull)?,
        );
        if self.usage.len() < MAX_ALLOCATIONS
            && let Some(allocation) = self.allocator.allocate(requested_size)
        {
            return Ok(allocation);
        }
        let expired_allocations: Vec<_> = self
            .usage
            .iter()
            .filter_map(|(allocation_id, usage)| {
                (self.frame.saturating_sub(usage.frame) > 1).then_some(*allocation_id)
            })
            .collect();
        for allocation_id in expired_allocations {
            self.allocator.deallocate(allocation_id);
            self.usage.remove(&allocation_id);
            stats.evicted = stats.evicted.saturating_add(1);
        }
        self.cache.retain(|_, entry| {
            entry.is_none_or(|glyph| self.usage.contains_key(&glyph.allocation))
        });
        if self.usage.len() >= MAX_ALLOCATIONS {
            return Err(TextError::AtlasFull);
        }
        if let Some(allocation) = self.allocator.allocate(requested_size) {
            return Ok(allocation);
        }
        let widths = growth_steps(self.descriptor.width, self.config.max_width);
        let heights = growth_steps(self.descriptor.height, self.config.max_height);
        let growth_dimensions = widths
            .into_iter()
            .flat_map(|candidate_width| {
                heights
                    .iter()
                    .copied()
                    .map(move |candidate_height| (candidate_width, candidate_height))
            })
            .filter(|&(candidate_width, candidate_height)| {
                (candidate_width, candidate_height)
                    != (self.descriptor.width, self.descriptor.height)
                    && candidate_width >= width
                    && candidate_height >= height
                    && self.config.fits(candidate_width, candidate_height)
            })
            .min_by_key(|&(candidate_width, candidate_height)| {
                (
                    u64::from(candidate_width) * u64::from(candidate_height),
                    candidate_width.max(candidate_height),
                )
            });
        let Some((new_width, new_height)) = growth_dimensions else {
            return Err(TextError::AtlasFull);
        };
        self.reset()?;
        self.allocator = AtlasAllocator::new(Size::new(new_width as i32, new_height as i32));
        self.descriptor.width = new_width;
        self.descriptor.height = new_height;
        stats.atlas_rebuilds = stats.atlas_rebuilds.saturating_add(1);
        Err(TextError::AtlasChanged)
    }
    pub fn allocate(
        &mut self,
        width: u32,
        height: u32,
        bearing: [i32; 2],
        stats: &mut TextStats,
    ) -> TextResult<Glyph> {
        let allocation_nonce = self.next_allocation;
        let next_allocation_nonce = allocation_nonce
            .checked_add(1)
            .ok_or(TextError::InvalidAtlas("allocation space exhausted"))?;
        let allocation = self.reserve(width, height, stats)?;
        self.next_allocation = next_allocation_nonce;
        let glyph = Glyph {
            allocation: allocation.id,
            atlas: [
                allocation.rectangle.min.x as u32,
                allocation.rectangle.min.y as u32,
            ],
            size: [width, height],
            bearing,
            owner: self.owner,
            epoch: self.descriptor.generation,
            nonce: allocation_nonce,
        };
        self.usage.insert(
            allocation.id,
            Usage {
                glyph,
                frame: self.frame,
            },
        );
        Ok(glyph)
    }
}

fn growth_steps(mut dimension: u32, maximum_dimension: u32) -> Vec<u32> {
    let mut steps = vec![dimension];
    while dimension < maximum_dimension {
        dimension = dimension.saturating_mul(2).min(maximum_dimension);
        steps.push(dimension);
    }
    steps
}

#[cfg(test)]
mod tests {
    use super::*;
    fn small() -> AtlasCache {
        AtlasCache::new(AtlasConfig {
            width: 8,
            height: 8,
            max_width: 16,
            max_height: 16,
            max_bytes: 1024,
        })
        .unwrap()
    }
    #[test]
    fn rejects_foreign_stale_and_modified_handles_without_touching_valid_prefix() {
        let mut atlas = small();
        let mut other_atlas = small();
        let mut stats = TextStats::default();
        let valid_glyph = atlas.allocate(4, 4, [0, 0], &mut stats).unwrap();
        let foreign_glyph = other_atlas.allocate(4, 4, [0, 0], &mut stats).unwrap();
        atlas.frame = 9;
        assert!(!atlas.validate(&[valid_glyph, foreign_glyph]));
        assert_eq!(atlas.usage[&valid_glyph.allocation].frame, 1);
        let mut modified = valid_glyph;
        modified.allocation = AllocId::deserialize(u32::MAX);
        assert!(!atlas.validate(&[modified]));
        atlas.reset().unwrap();
        assert!(!atlas.validate(&[valid_glyph]));
        let replacement = atlas.allocate(4, 4, [0, 0], &mut stats).unwrap();
        assert!(!atlas.validate(&[valid_glyph]));
        assert!(atlas.validate(&[replacement]));
    }
    #[test]
    fn grows_within_budget_then_reports_exhaustion() {
        let mut atlas = small();
        let mut stats = TextStats::default();
        assert!(matches!(
            atlas.allocate(12, 12, [0, 0], &mut stats),
            Err(TextError::AtlasChanged)
        ));
        assert_eq!((atlas.descriptor.width, atlas.descriptor.height), (16, 16));
        let glyph = atlas.allocate(12, 12, [0, 0], &mut stats).unwrap();
        assert!(matches!(
            atlas.allocate(12, 12, [0, 0], &mut stats),
            Err(TextError::AtlasFull)
        ));
        assert!(atlas.validate(&[glyph]));
        assert_eq!(stats.atlas_rebuilds, 1);
        let mut rectangular = AtlasCache::new(AtlasConfig {
            width: 8,
            height: 8,
            max_width: 32,
            max_height: 32,
            max_bytes: 1024,
        })
        .unwrap();
        assert!(matches!(
            rectangular.allocate(24, 4, [0, 0], &mut stats),
            Err(TextError::AtlasChanged)
        ));
        assert_eq!(
            (rectangular.descriptor.width, rectangular.descriptor.height),
            (32, 8)
        );
        assert!(rectangular.allocate(24, 4, [0, 0], &mut stats).is_ok());
    }
    #[test]
    fn eviction_rejects_reused_allocator_ids_and_u64_frames_do_not_wrap() {
        let mut atlas = small();
        let mut stats = TextStats::default();
        let evicted_glyph = atlas.allocate(8, 8, [0, 0], &mut stats).unwrap();
        atlas.frame = u32::MAX as u64 + 10;
        let replacement = atlas.allocate(8, 8, [0, 0], &mut stats).unwrap();
        assert!(!atlas.validate(&[evicted_glyph]));
        assert!(atlas.validate(&[replacement]));
        atlas.frame = u64::MAX;
        assert!(atlas.validate(&[replacement]));
    }
}
