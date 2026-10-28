//! Native layouts, access and sync state of a range.
use ash::vk;
use crate::native::access_table::AccessScope;
use crate::sync::QueueId;
use crate::sync::SyncSet;
use crate::tracking::OwnershipRange;
use super::frontier::Frontier;
use super::owner::RangeOwner;

pub type NativeRange = OwnershipRange;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NativeState {
    pub layout: vk::ImageLayout,
    pub layout_known: bool,
    pub writer: Frontier,
    pub readers: Frontier,
    pub owner: RangeOwner,
    pub revision: u64,
}

impl Default for NativeState {
    fn default() -> Self {
        Self {
            layout: vk::ImageLayout::UNDEFINED,
            layout_known: true,
            writer: Frontier::EMPTY,
            readers: Frontier::EMPTY,
            owner: RangeOwner::Unclaimed,
            revision: 0,
        }
    }
}

impl NativeState {
    /// Whether the image `range` holds no defined contents (known UNDEFINED
    /// layout, e.g. a discarded swapchain import or a freshly aliased
    /// member). Its queue-family ownership carries nothing to preserve, so
    /// the first access may come from any family without a release/acquire
    /// pair. Buffers have no layout and are never discarded.
    pub fn contents_discarded(&self, range: NativeRange) -> bool {
        matches!(range, NativeRange::Image { .. })
            && self.layout_known
            && self.layout == vk::ImageLayout::UNDEFINED
    }
    pub fn dependencies(&self, write_or_layout_change: bool) -> SyncSet {
        let mut set = self.writer.points;
        if write_or_layout_change {
            set.merge(&self.readers.points);
        }
        if let RangeOwner::Released { point, .. } = self.owner {
            set.insert(point);
        }
        set
    }
    pub fn source_scope(&self, queue: QueueId, write_or_layout_change: bool) -> AccessScope {
        let scope = self.writer.for_queue(queue);
        if write_or_layout_change {
            scope.union(self.readers.for_queue(queue))
        } else {
            scope
        }
    }
}
