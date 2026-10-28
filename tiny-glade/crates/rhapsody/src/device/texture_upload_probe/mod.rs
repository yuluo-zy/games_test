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
mod tests;
