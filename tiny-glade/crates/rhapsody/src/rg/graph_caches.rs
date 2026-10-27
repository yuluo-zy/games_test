//! Evictable graph caches: the idle resource pool and the transient alias
//! sets, registered with the device's [`EvictionRegistry`](crate::accounting::EvictionRegistry).
//!
//! # Ownership
//! A pooled resource is *parked* here when its frame completes: the
//! backend's owner table gives it up, so while parked it is reachable only
//! through this store (and the session pool's token). A frame that reuses a
//! pooled token *claims* the resource back first. A resource lent to an
//! in-flight frame is therefore never in this store and can never be evicted.
//!
//! # Eviction
//! Under memory pressure ([`Evictable::evict_idle`]) parked resources whose
//! recorded GPU uses all completed go first, least recently parked first,
//! then retired alias sets (least recently used first). The evictor runs from
//! inside allocations, possibly on another thread, and never blocks: when a
//! lock is held (the allocation was triggered from within the cache itself)
//! it releases nothing. Released tokens are queued so the owning session can
//! drop its stale pool entries ([`GraphCaches::take_evicted`]); a stale
//! entry that is claimed before that simply fails to match and is skipped.
//!
//! # Lifetime
//! The device holds only a `Weak` to this value. It is owned by the backend,
//! so it unregisters when the backend drops, and holds no `Arc<Device>`
//! itself (only the resources inside do, as every resource does).
use super::{
    pass_bindings::{ManagedPassResources, ManagedProjectionPool},
    transient_resources::AliasCache,
    vulkan::GpuResource,
};
use crate::sync::CompletedValues;
use crate::tracking::ResourceSharing;
use crate::accounting::Evictable;
use parking_lot::{Mutex, MutexGuard};
use std::{collections::BTreeMap, sync::Arc};

struct Parked {
    resource: GpuResource,
    /// Queue family recorded for the token when it was parked.
    family: Option<u32>,
    bytes: u64,
    /// Park order: smaller is older (least recently used).
    sequence: u64,
}
#[derive(Default)]
struct IdleStore {
    parked: BTreeMap<u64, Parked>,
    evicted: Vec<u64>,
    clock: u64,
}

/// The CPU projection of the last frame's physical resources: a pool of
/// reusable snapshots and the most recent one. Snapshots hold the actual
/// resources they project, so under pressure they are dropped first.
pub(crate) struct ProjectionCache {
    pub pool: ManagedProjectionPool,
    pub snapshot: Option<Arc<ManagedPassResources>>,
}
pub(crate) struct GraphCaches {
    idle: Mutex<IdleStore>,
    alias: Mutex<AliasCache>,
    projection: Mutex<ProjectionCache>,
}
/// Counters of the evictable graph caches.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GraphCacheStats {
    /// Idle pooled resources parked for reuse.
    pub parked_resources: usize,
    /// Native bytes of those resources.
    pub parked_bytes: u64,
    /// Retired alias sets kept for reuse.
    pub idle_alias_sets: usize,
}

impl GraphCaches {
    pub(crate) fn new(pool: ManagedProjectionPool) -> Self {
        Self {
            idle: Mutex::default(),
            alias: Mutex::default(),
            projection: Mutex::new(ProjectionCache {
                pool,
                snapshot: None,
            }),
        }
    }
    pub(crate) fn projection(&self) -> MutexGuard<'_, ProjectionCache> {
        self.projection.lock()
    }
    pub(crate) fn alias(&self) -> MutexGuard<'_, AliasCache> {
        self.alias.lock()
    }
    /// Take ownership of an idle pooled resource until it is claimed or evicted.
    pub(crate) fn park(&self, token: u64, resource: GpuResource, family: Option<u32>) {
        let bytes = resource.allocation_bytes();
        let mut idle = self.idle.lock();
        idle.clock += 1;
        let sequence = idle.clock;
        idle.parked.insert(
            token,
            Parked {
                resource,
                family,
                bytes,
                sequence,
            },
        );
    }
    /// Take a parked resource back for a frame, if it still exists and has
    /// the wanted sharing. A missing token (evicted) or another sharing
    /// leaves the store untouched.
    pub(crate) fn claim(
        &self,
        token: u64,
        sharing: ResourceSharing,
    ) -> Option<(GpuResource, Option<u32>)> {
        let mut idle = self.idle.lock();
        let matches = idle
            .parked
            .get(&token)
            .is_some_and(|entry| entry.resource.sharing_mode() == sharing);
        if !matches {
            return None;
        }
        let entry = idle.parked.remove(&token)?;
        Some((entry.resource, entry.family))
    }
    /// Drop a parked resource without recording an eviction (explicit
    /// cache clearing). Returns whether the token was parked.
    pub(crate) fn discard(&self, token: u64) -> bool {
        let removed = self.idle.lock().parked.remove(&token);
        removed.is_some()
    }
    /// Drop every parked resource; returns how many there were.
    pub(crate) fn clear_idle(&self) -> usize {
        let parked = std::mem::take(&mut self.idle.lock().parked);
        parked.len()
    }
    /// Parked (idle, evictable) pooled resources.
    pub(crate) fn parked_count(&self) -> usize {
        self.idle.lock().parked.len()
    }
    /// Tokens released by eviction since the last call, for the session to
    /// forget in its pool.
    pub(crate) fn take_evicted(&self) -> Vec<u64> {
        std::mem::take(&mut self.idle.lock().evicted)
    }
    pub(crate) fn stats(&self) -> GraphCacheStats {
        let idle = self.idle.lock();
        let (parked_resources, parked_bytes) = (
            idle.parked.len(),
            idle.parked.values().map(|entry| entry.bytes).sum(),
        );
        drop(idle);
        GraphCacheStats {
            parked_resources,
            parked_bytes,
            idle_alias_sets: self.alias.lock().available_count(),
        }
    }
}

impl Evictable for GraphCaches {
    fn evict_idle(&self, completed: &CompletedValues, target_bytes: u64) -> u64 {
        let mut released = 0u64;
        // Last frames' CPU snapshots hold the resources they project: drop
        // the ones nothing else references, so releasing a parked entry
        // below really frees its memory.
        if let Some(mut projection) = self.projection.try_lock() {
            projection.snapshot.take();
            projection.pool.clear_idle();
        }
        // Pooled resources first. Victims are collected under the lock and
        // dropped after it, so their retirement runs lock-free.
        let victims = match self.idle.try_lock() {
            Some(mut idle) => {
                let mut order = idle
                    .parked
                    .iter()
                    .filter(|(_, entry)| entry.resource.owner_cell().is_reached(completed))
                    .map(|(&token, entry)| (entry.sequence, token))
                    .collect::<Vec<_>>();
                order.sort_unstable();
                let mut victims = Vec::new();
                let mut bytes = 0u64;
                for (_, token) in order {
                    if bytes >= target_bytes {
                        break;
                    }
                    if idle.evicted.try_reserve(1).is_err() || victims.try_reserve(1).is_err() {
                        break;
                    }
                    if let Some(entry) = idle.parked.remove(&token) {
                        bytes = bytes.saturating_add(entry.bytes);
                        idle.evicted.push(token);
                        victims.push(entry);
                    }
                }
                released = bytes;
                victims
            }
            None => Vec::new(),
        };
        drop(victims);
        if released < target_bytes {
            let (bytes, sets) = match self.alias.try_lock() {
                Some(mut alias) => alias.evict_idle(completed, target_bytes - released),
                None => return released,
            };
            drop(sets);
            released = released.saturating_add(bytes);
        }
        released
    }
}
