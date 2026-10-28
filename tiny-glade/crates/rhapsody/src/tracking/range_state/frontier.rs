//! Write frontiers of a range.
use crate::native::access_table::AccessScope;
use crate::sync::MAX_QUEUES;
use crate::sync::QueueId;
use crate::sync::SyncSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Frontier {
    pub scopes: [AccessScope; MAX_QUEUES],
    pub points: SyncSet,
    /// Host accesses complete synchronously and have no device timeline point.
    pub host: AccessScope,
}

impl Default for Frontier {
    fn default() -> Self {
        Self::EMPTY
    }
}

impl Frontier {
    pub const EMPTY: Self = Self {
        scopes: [AccessScope::EMPTY; MAX_QUEUES],
        points: SyncSet::EMPTY,
        host: AccessScope::EMPTY,
    };
    pub fn combined(&self) -> AccessScope {
        self.scopes
            .iter()
            .copied()
            .fold(self.host, AccessScope::union)
    }
    pub fn for_queue(&self, queue: QueueId) -> AccessScope {
        self.scopes[queue.index()].union(self.host)
    }
    pub fn add(&mut self, queue: QueueId, scope: AccessScope) {
        self.scopes[queue.index()] = self.scopes[queue.index()].union(scope);
    }
}
