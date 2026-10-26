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
