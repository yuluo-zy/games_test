//! Bounded immutable shader-layout templates. An App/device owns its cache;
//! shader replacement publishes a new handle. No graph IDs, asset bindings,
//! dynamic values or upload leases are retained here.
use country_core::render::pass_builder::draw_stream::ShaderReflection;
use std::{collections::HashMap, mem::size_of, sync::Arc};

type Key = (u64, u8);
struct Entry {
    reflection: Arc<ShaderReflection>,
    bytes: usize,
    used: u64,
}

pub(crate) struct ReflectionCache {
    entries: HashMap<Key, Entry>,
    value_bytes: usize,
    budget: usize,
    clock: u64,
}

/// Hash-table capacity excludes control bytes and spare buckets. Charge a
/// conservative allocation bound, including their padding, rather than len.
fn table_bytes(capacity: usize, entry_size: usize) -> usize {
    if capacity == 0 { return 0; }
    (capacity + 1).next_power_of_two().saturating_mul(entry_size + 1).saturating_add(16)
}

fn reflection_bytes(value: &ShaderReflection) -> usize {
    size_of::<ShaderReflection>() + 4 * size_of::<usize>()
        + table_bytes(value.bindings.capacity(), size_of::<(u64, (u32, u32))>())
        + table_bytes(value.uniform_offsets.capacity(), size_of::<(u64, u32)>())
}

impl Default for ReflectionCache {
    fn default() -> Self { Self::new(32 << 20) }
}

impl ReflectionCache {
    fn new(budget: usize) -> Self {
        Self { entries: HashMap::new(), value_bytes: 0, budget, clock: 0 }
    }

    #[cfg(feature = "cpu-probe")]
    pub(crate) fn diagnostic_len(&self) -> usize { self.entries.len() }

    pub(crate) fn get(&mut self, key: &Key) -> Option<Arc<ShaderReflection>> {
        self.clock = self.clock.saturating_add(1);
        let entry = self.entries.get_mut(key)?;
        entry.used = self.clock;
        Some(entry.reflection.clone())
    }

    pub(crate) fn insert(&mut self, key: Key, value: Arc<ShaderReflection>) {
        let bytes = reflection_bytes(&value);
        if bytes > self.budget { return; } // An oversized template remains uncached.
        if let Some(old) = self.entries.remove(&key) { self.value_bytes -= old.bytes; }
        loop {
            let capacity = if self.entries.len() == self.entries.capacity() {
                self.entries.capacity().saturating_mul(2).max(3)
            } else { self.entries.capacity() };
            let projected = table_bytes(capacity, size_of::<(Key, Entry)>())
                .saturating_add(self.value_bytes).saturating_add(bytes);
            if projected <= self.budget { break; }
            let Some(oldest) = self.entries.iter().min_by_key(|(_, entry)| entry.used).map(|(&key, _)| key) else {
                // Retained table capacity counts even when every value is gone.
                self.entries = HashMap::new();
                if table_bytes(3, size_of::<(Key, Entry)>()).saturating_add(bytes) > self.budget { return; }
                continue;
            };
            self.value_bytes -= self.entries.remove(&oldest).unwrap().bytes;
        }
        if self.entries.try_reserve(1).is_err() { return; }
        // Verify the actual capacity as well; do not rely solely on the growth estimate.
        if table_bytes(self.entries.capacity(), size_of::<(Key, Entry)>())
            .saturating_add(self.value_bytes).saturating_add(bytes) > self.budget { return; }
        self.clock = self.clock.saturating_add(1);
        self.value_bytes += bytes;
        self.entries.insert(key, Entry { reflection: value, bytes, used: self.clock });
    }

    #[cfg(test)]
    fn charged_bytes(&self) -> usize {
        self.value_bytes + table_bytes(self.entries.capacity(), size_of::<(Key, Entry)>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cache_evicts_least_recent_layout_and_charges_reserved_capacity() {
        let template = Arc::new(ShaderReflection::default());
        let per_value = reflection_bytes(&template);
        let mut cache = ReflectionCache::new(table_bytes(3, size_of::<(Key, Entry)>()) + per_value * 2);
        cache.insert((1, 0), template.clone());
        cache.insert((2, 0), template.clone());
        assert!(cache.get(&(1, 0)).is_some());
        cache.insert((3, 0), template);
        assert!(cache.get(&(2, 0)).is_none());
        assert!(cache.get(&(1, 0)).is_some());
        assert!(cache.get(&(3, 0)).is_some());
        assert!(cache.charged_bytes() <= cache.budget);
    }

    #[test]
    fn reserved_layout_maps_are_budgeted_and_shader_variants_do_not_alias() {
        let mut cache = ReflectionCache::new(4096);
        cache.insert((1, 0), Arc::new(ShaderReflection::default()));
        let mut large = ShaderReflection::default();
        large.uniform_offsets.reserve(4096); // len=0 still consumes memory.
        cache.insert((1, 1), Arc::new(large));
        assert!(cache.get(&(1, 0)).is_some());
        assert!(cache.get(&(1, 1)).is_none());
        for id in 2..500 { cache.insert((id, 0), Arc::new(ShaderReflection::default())); }
        assert!(cache.charged_bytes() <= cache.budget);
        assert!(cache.get(&(499, 0)).is_some());
    }
}

/// Queried layouts for one owner declaration scope. Admission populates this
/// owner; Collect captures its immutable Arc and performs no LRU mutation.
/// Physical reflection bytes are conservatively charged even when another
/// cache owns the same Arc. Budget refusal affects caching, never drawing.
pub(crate) struct ReflectionScope {
    entries: HashMap<Key, Arc<ShaderReflection>>,
    value_bytes: usize,
    budget: usize,
    // Declared last: table/retained reflection owners are released before
    // returning conservative credit, including shared values pinned by us.
    storage: rhapsody::device::CpuStorageReservation,
}

#[cfg(test)]
impl Default for ReflectionScope {
    fn default() -> Self {
        Self::with_budget(&Arc::new(rhapsody::device::AllocatorStats::default()), 32 << 20).unwrap()
    }
}

impl ReflectionScope {
    pub(crate) fn try_new(stats: &Arc<rhapsody::device::AllocatorStats>) -> rhapsody::Result<Self> {
        Self::with_budget(stats, 32 << 20)
    }

    fn metadata_bytes() -> u64 {
        let (layout, _) = std::alloc::Layout::new::<[usize; 2]>()
            .extend(std::alloc::Layout::new::<Self>()).expect("scope metadata layout");
        layout.pad_to_align().size() as u64
    }

    fn with_budget(stats: &Arc<rhapsody::device::AllocatorStats>, budget: usize) -> rhapsody::Result<Self> {
        Ok(Self { entries: HashMap::new(), value_bytes: 0, budget,
            storage: stats.reserve_cpu_storage(Self::metadata_bytes())? })
    }

    pub(crate) fn get(&self, key: &Key) -> Option<Arc<ShaderReflection>> {
        self.entries.get(key).cloned()
    }

    pub(crate) fn insert(&mut self, key: Key, value: Arc<ShaderReflection>) {
        if self.entries.contains_key(&key) { return; }
        let bytes = reflection_bytes(&value);
        let growing = self.entries.len() == self.entries.capacity();
        let capacity = if growing {
            self.entries.capacity().saturating_mul(2).max(3)
        } else { self.entries.capacity() };
        if table_bytes(capacity, size_of::<(Key, Arc<ShaderReflection>)>())
            .saturating_add(self.value_bytes).saturating_add(bytes) > self.budget {
            return;
        }
        let predicted_table = table_bytes(capacity, size_of::<(Key, Arc<ShaderReflection>)>());
        // HashMap growth temporarily owns both old and new backing. Keep its
        // previous table admitted until try_reserve has finished rehashing.
        let growing_old_table = if growing {
            table_bytes(self.entries.capacity(), size_of::<(Key, Arc<ShaderReflection>)>())
        } else { 0 };
        if self.storage.resize(Self::metadata_bytes().saturating_add(predicted_table as u64)
            .saturating_add(growing_old_table as u64).saturating_add(self.value_bytes as u64)
            .saturating_add(bytes as u64)).is_err() { return; }
        if self.entries.try_reserve(1).is_err() {
            let actual_table = table_bytes(self.entries.capacity(), size_of::<(Key, Arc<ShaderReflection>)>());
            self.storage.resize(Self::metadata_bytes().saturating_add(actual_table as u64)
                .saturating_add(self.value_bytes as u64))
                .expect("shrinking scope storage credit");
            return;
        }
        // Never retain an unexpected implementation-specific growth beyond
        // our hard bound, even when the conservative prediction was lower.
        let table = table_bytes(self.entries.capacity(), size_of::<(Key, Arc<ShaderReflection>)>());
        if table.saturating_add(self.value_bytes).saturating_add(bytes) > self.budget
            || self.storage.resize(Self::metadata_bytes().saturating_add(table as u64)
                .saturating_add(self.value_bytes as u64).saturating_add(bytes as u64)).is_err() {
            self.entries = HashMap::new();
            self.value_bytes = 0;
            self.storage.resize(Self::metadata_bytes()).expect("releasing scope table credit");
            return;
        }
        self.entries.insert(key, value);
        self.value_bytes += bytes;
    }
}

#[cfg(test)]
mod scope_tests {
    use super::*;

    #[test]
    fn admission_resolves_each_unique_layout_once_and_worker_shares_it() {
        let mut scope = Arc::new(ReflectionScope::default());
        let value = Arc::new(ShaderReflection::default());
        let weak = Arc::downgrade(&value);
        let mut resolves = 0;
        for key in [(1, 0), (1, 0), (2, 1), (1, 0), (2, 1)] {
            if scope.get(&key).is_none() {
                resolves += 1;
                Arc::get_mut(&mut scope).unwrap().insert(key, value.clone());
            }
        }
        assert_eq!(resolves, 2);
        let worker = scope.clone();
        drop(value);
        std::thread::spawn(move || {
            for _ in 0..1000 {
                assert!(worker.get(&(1, 0)).is_some());
                assert!(worker.get(&(2, 1)).is_some());
                assert!(worker.get(&(1, 1)).is_none());
            }
        }).join().unwrap();
        assert!(weak.upgrade().is_some());
        drop(scope);
        assert!(weak.upgrade().is_none());
    }

    #[test]
    fn scope_budget_includes_reserved_capacity_and_large_reflection_maps() {
        let mut scope = ReflectionScope::with_budget(&Arc::new(rhapsody::device::AllocatorStats::default()), 4096).unwrap();
        let mut large = ShaderReflection::default();
        large.uniform_offsets.reserve(4096);
        scope.insert((1, 0), Arc::new(large));
        assert!(scope.get(&(1, 0)).is_none());
        for id in 2..200 {
            scope.insert((id, 0), Arc::new(ShaderReflection::default()));
        }
        assert!(scope.get(&(2, 0)).is_some());
        assert!(scope.entries.len() < 198);
        assert!(table_bytes(scope.entries.capacity(), size_of::<(Key, Arc<ShaderReflection>)>())
            + scope.value_bytes <= scope.budget);
    }

    #[test]
    fn exported_scope_remains_immutable_when_owner_prepares_another_variant() {
        let mut owner = Arc::new(ReflectionScope::default());
        let first = Arc::new(ShaderReflection { uniform_block_size: 16, ..Default::default() });
        Arc::get_mut(&mut owner).unwrap().insert((10, 0), first.clone());
        let prior = owner.clone();
        assert!(Arc::get_mut(&mut owner).is_none(), "published scopes never copy their credited map");
        owner = Arc::new(ReflectionScope::default());
        Arc::get_mut(&mut owner).unwrap().insert((10, 0), first.clone());
        Arc::get_mut(&mut owner).unwrap().insert((10, 1),
            Arc::new(ShaderReflection { uniform_block_size: 32, ..Default::default() }));
        Arc::get_mut(&mut owner).unwrap().insert((11, 0),
            Arc::new(ShaderReflection { uniform_block_size: 64, ..Default::default() }));
        assert!(Arc::ptr_eq(&first, &prior.get(&(10, 0)).unwrap()));
        assert!(prior.get(&(10, 1)).is_none());
        assert_eq!(owner.get(&(10, 1)).unwrap().uniform_block_size, 32);
        assert_eq!(owner.get(&(11, 0)).unwrap().uniform_block_size, 64);
        assert_eq!(owner.get(&(10, 0)).unwrap().uniform_block_size, 16);
    }

    #[test]
    fn denied_cpu_budget_preserves_draw_fallback_and_scope_drop_returns_credit() {
        let stats = Arc::new(rhapsody::device::AllocatorStats::default());
        let mut scope = ReflectionScope::try_new(&stats).unwrap();
        assert_eq!(stats.cpu_memory_usage().live, scope.storage.bytes());
        stats.set_host_budget(scope.storage.bytes()).unwrap();
        scope.insert((1, 0), Arc::new(ShaderReflection::default()));
        assert!(scope.get(&(1, 0)).is_none(), "caller can still use the original resolver");
        assert_eq!(stats.cpu_memory_usage().live, ReflectionScope::metadata_bytes());
        drop(scope);
        assert_eq!(stats.cpu_memory_usage().total(), 0);
        stats.set_host_budget(ReflectionScope::metadata_bytes() - 1).unwrap();
        assert!(ReflectionScope::try_new(&stats).is_err());
    }

    #[test]
    fn retaining_second_layout_requires_credit_even_when_the_map_does_not_grow() {
        let stats = Arc::new(rhapsody::device::AllocatorStats::default());
        let mut scope = ReflectionScope::try_new(&stats).unwrap();
        let first = Arc::new(ShaderReflection::default());
        scope.insert((1, 0), first.clone());
        assert!(Arc::ptr_eq(&scope.get(&(1, 0)).unwrap(), &first));
        let capacity = scope.entries.capacity();
        assert!(capacity > 1);
        let before = scope.storage.bytes();
        stats.set_host_budget(before).unwrap();
        let mut second = ShaderReflection::default();
        second.uniform_offsets.reserve(128);
        scope.insert((2, 0), Arc::new(second));
        assert!(scope.get(&(2, 0)).is_none(), "uncached result uses the existing draw resolver");
        assert!(Arc::ptr_eq(&scope.get(&(1, 0)).unwrap(), &first));
        assert_eq!(scope.entries.capacity(), capacity);
        assert_eq!(scope.storage.bytes(), before);
        assert_eq!(stats.cpu_memory_usage().total(), before);
        drop(scope);
        assert_eq!(stats.cpu_memory_usage().total(), 0);
    }

    #[test]
    fn map_growth_reserves_the_old_and_new_table_peak_before_allocating() {
        let stats = Arc::new(rhapsody::device::AllocatorStats::default());
        let mut scope = ReflectionScope::try_new(&stats).unwrap();
        let value = Arc::new(ShaderReflection::default());
        scope.insert((1, 0), value.clone());
        let old_capacity = scope.entries.capacity();
        for key in 2..=old_capacity { scope.insert((key as u64, 0), value.clone()); }
        assert_eq!(scope.entries.len(), old_capacity);
        let before = scope.storage.bytes();
        let next_table = table_bytes(old_capacity.saturating_mul(2).max(3),
            size_of::<(Key, Arc<ShaderReflection>)>());
        let final_only = ReflectionScope::metadata_bytes() + next_table as u64
            + scope.value_bytes as u64 + reflection_bytes(&value) as u64;
        stats.set_host_budget(final_only).unwrap();
        let next = old_capacity as u64 + 1;
        scope.insert((next, 0), value);
        assert!(scope.get(&(next, 0)).is_none(), "final-size-only budget cannot admit rehash peak");
        assert_eq!(scope.entries.capacity(), old_capacity);
        assert_eq!(scope.storage.bytes(), before);
        assert_eq!(stats.cpu_memory_usage().total(), before);
    }
}
