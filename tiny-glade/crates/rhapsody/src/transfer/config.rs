//! Transfer configuration and statistics.

#[derive(Clone, Copy, Debug)]
pub struct TransferConfig {
    pub slots: u32,
    pub chunk_bytes: u64,
    /// Includes both upload and readback backing allocations, using their real
    /// memory requirement sizes rather than the requested payload size.
    pub max_bytes: u64,
}

impl Default for TransferConfig {
    fn default() -> Self {
        Self {
            slots: 4,
            chunk_bytes: 4 * 1024 * 1024,
            max_bytes: 32 * 1024 * 1024,
        }
    }
}

impl TransferConfig {
    pub(super) fn validate(self) -> crate::Result<()> {
        if self.slots == 0
            || self.slots > 64
            || self.chunk_bytes == 0
            || self.max_bytes == 0
            || self
                .chunk_bytes
                .checked_mul(u64::from(self.slots))
                .and_then(|n| n.checked_mul(2))
                .is_none_or(|n| n > self.max_bytes)
        {
            return Err(crate::Error::invalid_usage(
                "invalid transfer slot count, capacity or byte budget",
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TransferStats {
    pub committed_bytes: u64,
    pub pending_uploads: u32,
    pub pending_readbacks: u32,
    pub leased_readbacks: u32,
    pub capacity_stalls: u64,
    /// Idle staging slots released under device memory pressure.
    pub evicted_slots: u64,
}
