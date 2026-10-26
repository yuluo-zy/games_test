//! Device-bound successful upload submission payloads, never GPU completion.
//! Each source uses its own monotonic atomic; mapped CPU writes are excluded.
use std::sync::atomic::{AtomicU64, Ordering};

/// Which independently submitted producer is being observed. Graph-owned copies
/// and the TransferManager buffer queue have separate receipt/queue accounting.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextureUploadProbeKind {
    SetupRegions,
    HostDirectRegions,
    TransferTextures,
    BufferSetup,
}
/// Fixed Copy device snapshot. Sources are read separately; callers take these
/// at explicit owner phase boundaries, never infer a GPU completion from them.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize)]
pub struct TextureUploadProbeSnapshot {
    pub generation: u64,
    pub valid: bool,
    pub setup_submitted_payload_bytes: u64,
    pub direct_region_submitted_payload_bytes: u64,
    pub transfer_submitted_payload_bytes: u64,
    pub buffer_setup_submitted_payload_bytes: u64,
}
impl TextureUploadProbeSnapshot {
    /// Reject a different device, invalid/overflowed inputs and counter rollback.
    pub fn checked_delta(self, before: Self) -> Option<Self> {
        if !self.valid || !before.valid || self.generation == 0 || self.generation != before.generation {
            return None;
        }
        Some(Self { generation: self.generation, valid: true,
            setup_submitted_payload_bytes: self.setup_submitted_payload_bytes.checked_sub(before.setup_submitted_payload_bytes)?,
            direct_region_submitted_payload_bytes: self.direct_region_submitted_payload_bytes.checked_sub(before.direct_region_submitted_payload_bytes)?,
            transfer_submitted_payload_bytes: self.transfer_submitted_payload_bytes.checked_sub(before.transfer_submitted_payload_bytes)?,
            buffer_setup_submitted_payload_bytes: self.buffer_setup_submitted_payload_bytes.checked_sub(before.buffer_setup_submitted_payload_bytes)?,
        })
    }
}
#[derive(Default)]
pub(super) struct TextureUploadProbeCounters {
    setup_submitted_payload_bytes: AtomicU64,
    direct_region_submitted_payload_bytes: AtomicU64,
    transfer_submitted_payload_bytes: AtomicU64,
    buffer_setup_submitted_payload_bytes: AtomicU64,
}
impl TextureUploadProbeCounters {
    pub(super) fn record(&self, kind: TextureUploadProbeKind, payload: Option<u64>) {
        let counter = match kind {
            TextureUploadProbeKind::SetupRegions => &self.setup_submitted_payload_bytes,
            TextureUploadProbeKind::HostDirectRegions => &self.direct_region_submitted_payload_bytes,
            TextureUploadProbeKind::TransferTextures => &self.transfer_submitted_payload_bytes,
            TextureUploadProbeKind::BufferSetup => &self.buffer_setup_submitted_payload_bytes,
        };
        // MAX is reserved for invalid, including a mathematically exact MAX sum.
        // The update never wraps or repairs an invalid source. Probe overflow
        // cannot change whether the already successful real submit succeeded.
        let _ = counter.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |old| {
            Some(payload.and_then(|v| old.checked_add(v)).filter(|&sum| sum < u64::MAX).unwrap_or(u64::MAX))
        });
    }
    pub(super) fn snapshot(&self, generation: u64) -> TextureUploadProbeSnapshot {
        let setup_submitted_payload_bytes = self.setup_submitted_payload_bytes.load(Ordering::Relaxed);
        let direct_region_submitted_payload_bytes = self.direct_region_submitted_payload_bytes.load(Ordering::Relaxed);
        let transfer_submitted_payload_bytes = self.transfer_submitted_payload_bytes.load(Ordering::Relaxed);
        let buffer_setup_submitted_payload_bytes = self.buffer_setup_submitted_payload_bytes.load(Ordering::Relaxed);
        TextureUploadProbeSnapshot {
            generation,
            valid: generation != 0 && [setup_submitted_payload_bytes, direct_region_submitted_payload_bytes, transfer_submitted_payload_bytes, buffer_setup_submitted_payload_bytes].into_iter().all(|v| v != u64::MAX),
            setup_submitted_payload_bytes,
            direct_region_submitted_payload_bytes,
            transfer_submitted_payload_bytes,
            buffer_setup_submitted_payload_bytes,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn device_upload_probe_categories_delta_and_generation_are_distinct() {
        let counters = TextureUploadProbeCounters::default();
        let before = counters.snapshot(7);
        for (kind, bytes) in [(TextureUploadProbeKind::SetupRegions, 3),
            (TextureUploadProbeKind::HostDirectRegions, 5), (TextureUploadProbeKind::TransferTextures, 11),
            (TextureUploadProbeKind::BufferSetup, 17)] { counters.record(kind, Some(bytes)); }
        let after = counters.snapshot(7);
        assert_eq!(after.checked_delta(before), Some(after));
        assert_eq!(after.setup_submitted_payload_bytes, 3);
        assert_eq!(after.direct_region_submitted_payload_bytes, 5);
        assert_eq!(after.transfer_submitted_payload_bytes, 11);
        assert_eq!(after.buffer_setup_submitted_payload_bytes, 17);
        assert!(after.checked_delta(counters.snapshot(8)).is_none());
        assert!(before.checked_delta(after).is_none());
        assert!(after.checked_delta(TextureUploadProbeSnapshot::default()).is_none());
    }
    #[test]
    fn device_upload_probe_max_and_overflow_are_sticky_invalid() {
        for payload in [None, Some(u64::MAX)] {
            let counters = TextureUploadProbeCounters::default();
            let before = counters.snapshot(1);
            counters.record(TextureUploadProbeKind::SetupRegions, payload);
            counters.record(TextureUploadProbeKind::SetupRegions, Some(0));
            counters.record(TextureUploadProbeKind::SetupRegions, Some(7));
            let after = counters.snapshot(1);
            assert!(!after.valid);
            assert_eq!(after.setup_submitted_payload_bytes, u64::MAX);
            assert!(after.checked_delta(before).is_none());
        }
        let counters = TextureUploadProbeCounters::default();
        counters.record(TextureUploadProbeKind::BufferSetup, Some(u64::MAX - 1));
        assert!(counters.snapshot(1).valid);
        counters.record(TextureUploadProbeKind::BufferSetup, Some(1));
        assert!(!counters.snapshot(1).valid, "MAX is reserved even without arithmetic wrap");
    }
    #[test]
    fn concurrent_upload_probe_updates_do_not_lose_payload() {
        let counters = TextureUploadProbeCounters::default();
        std::thread::scope(|scope| { for _ in 0..4 { let counters = &counters;
            scope.spawn(move || for _ in 0..100 { counters.record(TextureUploadProbeKind::SetupRegions, Some(7)); });
        }});
        let snapshot = counters.snapshot(2);
        assert!(snapshot.valid);
        assert_eq!(snapshot.setup_submitted_payload_bytes, 2800);
        assert_eq!(snapshot.direct_region_submitted_payload_bytes, 0);
    }
}
