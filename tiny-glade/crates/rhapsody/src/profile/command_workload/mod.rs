//! Fixed-size, worker-local native API invocation counts. No GPU draw estimate.
use serde::Serialize;

/// Calls recorded by successful graph callbacks, not draws executed by the GPU.
/// In particular, one indirect/count-form API call counts once regardless of
/// draw_count, max_draw_count, instance_count or the GPU count-buffer contents.
/// `valid` is false after arithmetic overflow or incomplete graph execution.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct CommandWorkload {
    pub valid: bool,
    pub direct_draw_api_calls: u64,
    pub indexed_draw_api_calls: u64,
    pub indirect_draw_api_calls: u64,
    pub indirect_count_draw_api_calls: u64,
    pub indexed_indirect_draw_api_calls: u64,
    pub indexed_indirect_count_draw_api_calls: u64,
    pub direct_dispatch_api_calls: u64,
    pub indirect_dispatch_api_calls: u64,
    /// Host-recorded bytes in this graph; independent transfer/setup queues are separate.
    pub staging_h2d_copy_recorded_bytes: u64,
    pub immutable_producer_h2d_copy_recorded_bytes: u64,
    pub cached_d2d_copy_recorded_bytes: u64,
}
impl Default for CommandWorkload {
    fn default() -> Self {
        Self { valid: true,
            direct_draw_api_calls: 0,
            indexed_draw_api_calls: 0,
            indirect_draw_api_calls: 0,
            indirect_count_draw_api_calls: 0,
            indexed_indirect_draw_api_calls: 0,
            indexed_indirect_count_draw_api_calls: 0,
            direct_dispatch_api_calls: 0,
            indirect_dispatch_api_calls: 0,
            staging_h2d_copy_recorded_bytes: 0,
            immutable_producer_h2d_copy_recorded_bytes: 0,
            cached_d2d_copy_recorded_bytes: 0,
        }
    }
}
#[cfg(feature = "cpu-probe")]
#[derive(Clone, Copy)]
pub(crate) enum NativeCommandKind {
    Draw,
    DrawIndexed,
    DrawIndirect,
    DrawIndirectCount,
    DrawIndexedIndirect,
    DrawIndexedIndirectCount,
    DispatchGroups,
    DispatchIndirect,
}
impl CommandWorkload {
    pub fn invalid() -> Self {
        Self { valid: false, ..Self::default() }
    }
    pub fn invalidate(&mut self) { self.valid = false; }
    fn checked_add(target: &mut u64, value: u64) -> bool {
        match target.checked_add(value) {
            Some(sum) => { *target = sum; true }
            None => { *target = u64::MAX; false }
        }
    }
    /// Fold an exclusively owned callback/chunk value once after success.
    /// Overflow saturates the field and permanently invalidates the record.
    pub fn merge(&mut self, other: Self) {
        self.valid &= other.valid;
        self.valid &= Self::checked_add(&mut self.direct_draw_api_calls, other.direct_draw_api_calls);
        self.valid &= Self::checked_add(&mut self.indexed_draw_api_calls, other.indexed_draw_api_calls);
        self.valid &= Self::checked_add(&mut self.indirect_draw_api_calls, other.indirect_draw_api_calls);
        self.valid &= Self::checked_add(&mut self.indirect_count_draw_api_calls, other.indirect_count_draw_api_calls);
        self.valid &= Self::checked_add(&mut self.indexed_indirect_draw_api_calls, other.indexed_indirect_draw_api_calls);
        self.valid &= Self::checked_add(&mut self.indexed_indirect_count_draw_api_calls, other.indexed_indirect_count_draw_api_calls);
        self.valid &= Self::checked_add(&mut self.direct_dispatch_api_calls, other.direct_dispatch_api_calls);
        self.valid &= Self::checked_add(&mut self.indirect_dispatch_api_calls, other.indirect_dispatch_api_calls);
        self.valid &= Self::checked_add(&mut self.staging_h2d_copy_recorded_bytes, other.staging_h2d_copy_recorded_bytes);
        self.valid &= Self::checked_add(&mut self.immutable_producer_h2d_copy_recorded_bytes, other.immutable_producer_h2d_copy_recorded_bytes);
        self.valid &= Self::checked_add(&mut self.cached_d2d_copy_recorded_bytes, other.cached_d2d_copy_recorded_bytes);
    }
    #[cfg(feature = "cpu-probe")]
    pub(crate) fn recorded(&mut self, kind: Option<NativeCommandKind>) {
        let field = match kind {
            Some(NativeCommandKind::Draw) => &mut self.direct_draw_api_calls,
            Some(NativeCommandKind::DrawIndexed) => &mut self.indexed_draw_api_calls,
            Some(NativeCommandKind::DrawIndirect) => &mut self.indirect_draw_api_calls,
            Some(NativeCommandKind::DrawIndirectCount) => &mut self.indirect_count_draw_api_calls,
            Some(NativeCommandKind::DrawIndexedIndirect) => &mut self.indexed_indirect_draw_api_calls,
            Some(NativeCommandKind::DrawIndexedIndirectCount) => &mut self.indexed_indirect_count_draw_api_calls,
            Some(NativeCommandKind::DispatchGroups) => &mut self.direct_dispatch_api_calls,
            Some(NativeCommandKind::DispatchIndirect) => &mut self.indirect_dispatch_api_calls,
            None => return,
        };
        let good = Self::checked_add(field, 1);
        self.valid &= good;
    }
}

#[cfg(test)]
mod tests;
