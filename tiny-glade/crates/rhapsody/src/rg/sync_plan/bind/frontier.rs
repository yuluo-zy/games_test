//! Pending cross-queue frontiers and their owners.
use crate::native::access_table::AccessScope;
use crate::rg::sync_plan::DependencyRef;
use crate::sync::QueueId;
use crate::tracking::range_state::RangeTransfer;
use crate::sync::MAX_QUEUES;
use crate::tracking::range_state::Frontier;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct PendingFrontier {
    scope: [AccessScope; MAX_QUEUES],
    host: AccessScope,
    pub(super) points: [Option<DependencyRef>; MAX_QUEUES],
}

impl PendingFrontier {
    pub(super) const EMPTY: Self = Self {
        scope: [AccessScope::EMPTY; MAX_QUEUES],
        host: AccessScope::EMPTY,
        points: [None; MAX_QUEUES],
    };
    pub(super) fn from_native(native: Frontier) -> Self {
        let mut result = Self {
            scope: native.scopes,
            host: native.host,
            ..Self::EMPTY
        };
        for point in native.points.points() {
            result.points[point.queue.index()] = Some(DependencyRef::Point(point));
        }
        result
    }
    pub(super) fn source(self, queue: QueueId) -> AccessScope {
        self.scope[queue.index()].union(self.host)
    }
    /// Some prerequisite executes on a queue other than `queue`, so it is
    /// reached through a semaphore wait rather than command order.
    pub(super) fn waits_other_queue(&self, queue: QueueId) -> bool {
        self.points
            .iter()
            .enumerate()
            .any(|(index, point)| point.is_some() && index != queue.index())
    }
    pub(super) fn add(&mut self, queue: QueueId, scope: AccessScope, batch: usize) {
        self.scope[queue.index()] = self.scope[queue.index()].union(scope);
        self.points[queue.index()] = Some(DependencyRef::Batch(batch));
    }
    /// Union of two frontiers. Per queue, the dependency that executes later
    /// covers the earlier one (`later` decides using the actual binding order).
    pub(super) fn merge(
        &mut self,
        other: Self,
        later: impl Fn(DependencyRef, DependencyRef) -> DependencyRef,
    ) {
        for queue in 0..MAX_QUEUES {
            self.scope[queue] = self.scope[queue].union(other.scope[queue]);
            self.points[queue] = match (self.points[queue], other.points[queue]) {
                (Some(a), Some(b)) => Some(later(a, b)),
                (a, b) => a.or(b),
            };
        }
        self.host = self.host.union(other.host);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum PendingOwner {
    Unclaimed,
    Owned(u32),
    Released {
        transfer: RangeTransfer,
        release: DependencyRef,
    },
}
