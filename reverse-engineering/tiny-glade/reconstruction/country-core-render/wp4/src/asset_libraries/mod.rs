//! Core asset id types (WP4-owned).
//!
//! Skeleton bundle: `skeleton/B/crates/country-core/src/asset_libraries/mod.rs`
//! (source_sha256 `b86638af…`, 3 procs).

pub mod color_library;
pub mod mesh_library;
pub mod shader_library;
pub mod ssbo_library;
pub mod texture_library;

use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};

/// Global handle-id counter (O: `AssetMeshLibrary::add` /
/// `AssetShaderLibrary::add_*` increment `HANDLE_COUNT`).
static HANDLE_COUNT: AtomicU64 = AtomicU64::new(1);

/// Raw handle id (O: `HandleId::new` / `HandleId::from_raw`).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HandleId(u64);

impl HandleId {
    /// `new` — allocate the next global handle id (O).
    pub fn new() -> Self {
        HandleId(HANDLE_COUNT.fetch_add(1, Ordering::Relaxed))
    }

    /// `from_raw` (O name).
    pub fn from_raw(raw: u64) -> Self {
        HandleId(raw)
    }

    pub fn raw(self) -> u64 {
        self.0
    }
}

impl fmt::Debug for HandleId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HandleId({:#x})", self.0)
    }
}

/// Typed handle to an asset (O: `Handle<T>` wraps `HandleId`).
///
/// `PartialEq`/`Eq`/`Hash`/`Copy`/`Clone` are implemented without a `T: …`
/// bound (equality is by id only), so handles stay usable as hash-map keys for
/// any asset type.
pub struct Handle<T> {
    id: HandleId,
    _marker: core::marker::PhantomData<fn() -> T>,
}

impl<T> Handle<T> {
    /// `new` — allocate the next handle (O).
    pub fn new() -> Self {
        Handle {
            id: HandleId::new(),
            _marker: core::marker::PhantomData,
        }
    }

    /// `is_valid` (O name).
    pub fn is_valid(&self) -> bool {
        self.id.raw() != 0
    }

    pub fn id(&self) -> HandleId {
        self.id
    }
}

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        Handle {
            id: self.id,
            _marker: core::marker::PhantomData,
        }
    }
}
impl<T> Copy for Handle<T> {}
impl<T> PartialEq for Handle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl<T> Eq for Handle<T> {}
impl<T> PartialOrd for Handle<T> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<T> Ord for Handle<T> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}
impl<T> core::hash::Hash for Handle<T> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<T> fmt::Debug for Handle<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Handle").field(&self.id).finish()
    }
}

/// Named asset entry (O: `Asset { name, … }`, `Asset::name`, `Asset::new`).
#[derive(Clone, Debug)]
pub struct Asset {
    pub name: String,
}

impl Asset {
    pub fn new(name: String) -> Self {
        Asset { name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
