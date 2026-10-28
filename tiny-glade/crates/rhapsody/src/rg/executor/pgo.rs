//! Profile-guided pass statistics.

/// Owned full-content PGO identity. Cloning a declaration's name shares the
/// existing allocation, and names disappear with the last plan or observation.
pub(super) type TokenHashBuilder = std::hash::BuildHasherDefault<crate::collections::token_map::TokenHasher>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PassPgoNameId(crate::resource::ResourceName);

impl PassPgoNameId {
    pub fn from_shared(name: std::sync::Arc<str>) -> Self {
        Self(name.into())
    }
    pub fn from_resource_name(name: crate::resource::ResourceName) -> Self {
        Self(name)
    }
    /// Cold-path convenience; production recording shares the declaration Arc.
    pub fn from_static(name: &'static str) -> Self {
        Self(crate::resource::ResourceName::new(name))
    }
    pub fn from_name(name: &str) -> Self {
        Self(crate::resource::ResourceName::new(name))
    }
}

impl AsRef<str> for PassPgoNameId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Clone, Debug)]
pub struct PassPgoDuration {
    pub name: PassPgoNameId,
    pub duration_ns: u64,
}

impl PassPgoDuration {
    pub fn new(name: PassPgoNameId, duration_ns: u64) -> Self {
        Self { name, duration_ns }
    }
}

/// One frame's observations. Repeated names keep their final duration; totals
/// include every recorded invocation. Reset releases names while reusing backing.
#[derive(Clone, Debug, Default)]
pub struct PassPgoStats {
    // An interned name hashes as one precomputed u64; SipHash on top of that
    // was pure overhead on every pass of every frame.
    counts: std::collections::HashMap<PassPgoNameId, u64, TokenHashBuilder>,
    durations: std::collections::HashMap<PassPgoNameId, u64, TokenHashBuilder>,
    total_ns: u64,
    max_ns: u64,
}

impl PassPgoStats {
    pub fn reset(&mut self) {
        self.counts.clear();
        self.durations.clear();
        self.total_ns = 0;
        self.max_ns = 0;
    }
    pub fn extend(&mut self, durations: impl IntoIterator<Item = PassPgoDuration>) {
        for duration in durations {
            self.record(duration);
        }
    }
    pub fn duration_ns(&self, id: impl AsRef<str>) -> u64 {
        self.get(id).unwrap_or(0)
    }
    pub fn get(&self, id: impl AsRef<str>) -> Option<u64> {
        // A name never interned was never recorded.
        let name = crate::resource::ResourceName::from_existing(id.as_ref())?;
        self.get_interned(&name)
    }
    /// [`Self::get`] for a name that is already interned: one pointer-hash lookup, no
    /// string hashing and no global intern table access.
    pub fn get_interned(&self, name: &crate::resource::ResourceName) -> Option<u64> {
        self.durations.get(&PassPgoNameId(*name)).copied()
    }
    pub fn call_count(&self, id: impl AsRef<str>) -> u64 {
        crate::resource::ResourceName::from_existing(id.as_ref())
            .and_then(|name| self.counts.get(&PassPgoNameId(name)).copied())
            .unwrap_or(0)
    }
    pub fn is_empty(&self) -> bool {
        self.durations.is_empty()
    }
    /// Read-only view for diagnostics: `(pass name, final duration ns, invocation count)`.
    pub fn iter(&self) -> impl Iterator<Item = (&str, u64, u64)> + '_ {
        self.durations.iter().map(|(name, duration)| {
            (
                name.0.as_str(),
                *duration,
                self.counts.get(name).copied().unwrap_or(0),
            )
        })
    }
    pub fn total_ns(&self) -> u64 {
        self.total_ns
    }
    pub fn max_ns(&self) -> u64 {
        self.max_ns
    }
    pub fn record(&mut self, duration: PassPgoDuration) {
        *self.counts.entry(duration.name.clone()).or_default() += 1;
        self.durations.insert(duration.name, duration.duration_ns);
        self.total_ns = self.total_ns.wrapping_add(duration.duration_ns);
        self.max_ns = self.max_ns.max(duration.duration_ns);
    }
}

/// Chunk planning reads the preceding frame. At frame end the buffers swap
/// and the new current observations are cleared, releasing obsolete names.
#[derive(Clone, Debug, Default)]
pub struct PassPgoFrames {
    pub previous: PassPgoStats,
    pub current: PassPgoStats,
}

impl PassPgoFrames {
    pub fn end_frame(&mut self) {
        std::mem::swap(&mut self.previous, &mut self.current);
        self.current.reset();
    }
}

/// Identity of a compiled graph for per-graph PGO statistics: the address of its
/// plan. Equal for every frame that hits the plan cache; a recycled address only
/// ever yields a poor chunk plan for a frame, never a wrong result.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PgoKey(pub u64);
