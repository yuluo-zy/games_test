//! `AssetSsboLibrary` — typed storage-buffer (SSBO) registry (WP4-owned,
//! render-facing surface).
//!
//! Skeleton bundle: `skeleton/B/crates/country-core/src/asset_libraries/ssbo_library.rs`
//! (source_sha256 `d783c195…`, 336 procedures — heavily monomorphized).
//! Decompiles: `_analysis/decomp/C_0x14091eb30.c`, `C_0x14091eb60.c`,
//! `C_0x14091ebb0.c`, `C_0x1402b31a0.c`, `C_0x1412098a0.c`, `C_0x1412bc470.c`,
//! `C_0x1412badf0.c`.
//!
//! `SsboId` = u64 type-name hash; entries are `BufferState` boxes keyed in a
//! FrozenMap with `NoHashHasher` (reused `frozen` crate). Storage-buffer
//! descriptors carry a vtable (+0x18 get-ptr, +0x20 get-mut, +0x30 get-value)
//! and the type-name string for asserts.
//!
//! Mutability model (O): the map itself is write-once (FrozenMap, immutable for
//! reads); each `BufferState` carries its own lock at +0x20 and a state byte at
//! +0x36, so `get`/`get_mut`/`get_rg_buffer` work through `&self` exactly like
//! the original.

use std::any::type_name;
use std::collections::HashMap;
use std::hash::{BuildHasherDefault, Hasher};
use std::marker::PhantomData;
use std::sync::atomic::{AtomicU8, Ordering};

use parking_lot::RwLock;

/// u64 type id for a storage buffer (O: `SsboId`).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct SsboId(u64);

impl SsboId {
    /// O: id derives from the type name (`utils::name_hash`); computed here from
    /// `std::any::type_name` so game logic stays faithful without recovering the
    /// exact FNV variant.
    pub fn of<T: ?Sized>() -> Self {
        let mut h = NoHashHasher(0xcbf29ce484222325);
        h.write(type_name::<T>().as_bytes());
        SsboId(h.finish())
    }
}

/// No-op hasher for pre-hashed ids (matches observed NoHashHasher).
#[derive(Default, Clone, Copy)]
pub struct NoHashHasher(u64);
impl Hasher for NoHashHasher {
    fn finish(&self) -> u64 {
        self.0
    }
    fn write(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.0 = self.0.wrapping_mul(0x100000001b3) ^ u64::from(b);
        }
    }
    fn write_u64(&mut self, i: u64) {
        self.0 = i;
    }
}
pub type NoHashBuildHasher = BuildHasherDefault<NoHashHasher>;

/// O: +0x36 discriminant byte; 5 = exported to CPU.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BufferStateKind {
    Gpu,
    Exported,
    /// U: other states.
    Other(u8),
}

/// Runtime state of one storage buffer (0x38+ bytes, O):
/// +0x00 handle/ptr, +0x08 size, +0x10 data ptr, +0x18 len, +0x20 RwLock state,
/// +0x28/+0x30 RG handle fields, +0x36 state byte (5 = exported to CPU).
///
/// The RG handle and state are behind interior locks so `get_rg_buffer` can
/// re-import an exported buffer through `&self` (as the original does by locking
/// +0x20).
#[derive(Debug)]
pub struct BufferState {
    pub buffer_ptr: *mut u8,
    pub size: usize,
    pub data_ptr: *const u8,
    pub len: usize,
    /// +0x20 per-buffer lock (O).
    rg_handle: RwLock<crate::rhapsody_contract::ResourceHandle>,
    /// +0x36 state byte (O).
    state: AtomicU8,
}

// SAFETY: `AssetSsboLibrary` is a game `Resource` (`Res<AssetSsboLibrary>`, O), so
// `BufferState` must be `Send + Sync`. The raw pointers are accessed only behind the
// per-buffer `RwLock` / the library's own locking, matching the original's ownership.
unsafe impl Send for BufferState {}
unsafe impl Sync for BufferState {}

/// `BufferState::new(ptr, size, data, len)` (O: 0x14091eb30, 36B).
impl BufferState {
    pub fn new(buffer_ptr: *mut u8, size: usize, data_ptr: *const u8, len: usize) -> Self {
        BufferState {
            buffer_ptr,
            size,
            data_ptr,
            len,
            rg_handle: RwLock::new(crate::rhapsody_contract::ResourceHandle::NONE),
            state: AtomicU8::new(Self::GPU),
        }
    }

    const GPU: u8 = 0;
    const EXPORTED: u8 = 5;

    pub fn state(&self) -> BufferStateKind {
        match self.state.load(Ordering::Relaxed) {
            Self::EXPORTED => BufferStateKind::Exported,
            _ => BufferStateKind::Gpu,
        }
    }

    pub fn rg_handle(&self) -> crate::rhapsody_contract::ResourceHandle {
        *self.rg_handle.read()
    }

    /// O: export_from_rg sets the state byte to 5 and exports via
    /// `import_export::export`.
    fn mark_exported(&self) {
        self.state.store(Self::EXPORTED, Ordering::Relaxed);
    }
    fn mark_gpu(&self) {
        self.state.store(Self::GPU, Ordering::Relaxed);
    }

    /// Re-import a CPU-exported buffer into the render graph and refresh the RG
    /// handle (O: `get_rg_buffer` path).
    fn reimport(&self) -> crate::rhapsody_contract::ResourceHandle {
        let handle = crate::rhapsody_contract::import_unowned(self.data_ptr, self.size);
        *self.rg_handle.write() = handle;
        self.mark_gpu();
        handle
    }
}

/// Storage-buffer payload type trait (game-owned wrapper; data upload handled by
/// `new_storage_buffer<T>` in `render::ssbo`, which wraps rhapsody buffers).
pub trait StorageBuffer: Sized {
    /// Type-name string (used for duplicate-registration asserts and debug
    /// labels, O).
    const NAME: &'static str;
    /// `new_storage_buffer<T>` — construct the initial payload (I; the original
    /// wraps a `MappedStorageBuffer<T>`).
    fn new() -> Self;
}

/// `StorageBufferType` (O name; `default_value` observed). Enum of buffer data
/// kinds, U arms.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageBufferType {
    Default,
    /// U: additional kinds.
    Other(u32),
}

impl StorageBufferType {
    pub fn default_value() -> Self {
        StorageBufferType::Default
    }
}

/// Storage-buffer registry (O: FrozenMap<SsboId, Box<BufferState>> + descriptor
/// map keyed by SsboId). The buffer map is write-once after startup (matching the
/// immutable FrozenMap reads): `register`/`add_*` mutate through `&mut self`,
/// readers hand out `&BufferState` through `&self`, and per-buffer GPU state
/// mutation goes through the buffer's own interior lock (+0x20).
/// `impl Resource`: `nani_update_preplaced_flower` 以 `Res<AssetSsboLibrary>` 使用（O）。
#[derive(bevy_ecs::resource::Resource)]
pub struct AssetSsboLibrary {
    buffers: HashMap<SsboId, Box<BufferState>, NoHashBuildHasher>,
    descriptors: HashMap<SsboId, Box<dyn crate::rhapsody_contract::StorageBufferDescriptor>>,
    #[allow(dead_code)] // +0x?? disposed flag; observed in other libraries, unused here.
    disposed: bool,
}

impl AssetSsboLibrary {
    /// `new` / `Default` (O: 0x14091eb60).
    pub fn new() -> Self {
        AssetSsboLibrary {
            buffers: HashMap::default(),
            descriptors: HashMap::default(),
            disposed: false,
        }
    }

    /// `add_new<T>()` (O: 0x1412bc470 rep): `new_storage_buffer<T>()`, build a
    /// `BufferState`, insert by SsboId. Duplicate → panic
    /// `"Tried to register the same storage buffer …"` / `"…two storage buffers
    /// with the same id"`.
    pub fn add_new<T: StorageBuffer + 'static>(&mut self) {
        let data = T::new();
        self.register::<T>(data, StorageBufferType::default_value());
    }

    /// `add_new_with_data<T>(data)` (O: 0x1412badf0 rep).
    pub fn add_new_with_data<T: StorageBuffer + 'static>(&mut self, data: T) {
        self.register::<T>(data, StorageBufferType::default_value());
    }

    fn register<T: StorageBuffer + 'static>(&mut self, data: T, ty: StorageBufferType) {
        let id = SsboId::of::<T>();
        if self.buffers.contains_key(&id) {
            panic!(
                "Tried to register the same storage buffer {:?} twice",
                T::NAME
            );
        }
        // O: descriptor carries type-name + vtable.
        let desc = crate::rhapsody_contract::StorageBufferDescriptorImpl::<T>::new();
        self.descriptors.insert(id, Box::new(desc));
        let data_ptr = crate::rhapsody_contract::StorageBufferDescriptorImpl::<T>::data_ptr(&data);
        let state = BufferState::new(
            data_ptr,
            core::mem::size_of::<T>(),
            &data as *const T as *const u8,
            1,
        );
        let _ = ty;
        self.buffers.insert(id, Box::new(state));
    }

    /// `export_from_rg(&mut self)` (O: 0x14091ebb0): export every buffer from the
    /// render graph (`import_export::export`), marking state `Exported`.
    pub fn export_from_rg(&mut self) {
        for state in self.buffers.values() {
            if state.state() != BufferStateKind::Exported {
                crate::rhapsody_contract::import_export_export(state.rg_handle());
                state.mark_exported();
            }
        }
    }

    /// `get<T>()` (O name; FrozenMap lookup, panic on missing). Returns a shared
    /// `&BufferState` — safe because the map is write-once and per-buffer
    /// mutation is behind the buffer's own lock (+0x20).
    pub fn get<T: 'static>(&self) -> &BufferState {
        let id = SsboId::of::<T>();
        self.buffers
            .get(&id)
            .map(|b| &**b)
            .unwrap_or_else(|| panic!("SSBO {:?} not registered", type_name::<T>()))
    }

    /// `get_mut<T>()` (O: 0x1412098a0 rep; asserts type-name and calls the
    /// descriptor vtable +0x20 to fetch the mutable data). Like `get`, returns a
    /// shared `&BufferState`; the game mutates the payload through the buffer's
    /// own lock, so `&self` suffices (I).
    pub fn get_mut<T: 'static>(&self) -> &BufferState {
        self.get::<T>()
    }

    /// `get_rg_buffer<T>() -> ResourceHandle` (O: 0x1402b31a0 rep) — the
    /// render-facing entry used by `pass_resource_access::*` / DrawBuilder /
    /// ComputePass. If the buffer was exported to CPU, re-import it
    /// (`import_export::import_unowned`) and refresh the RG handle.
    pub fn get_rg_buffer<T: 'static>(&self) -> crate::rhapsody_contract::ResourceHandle {
        let state = self.get::<T>();
        if state.state() == BufferStateKind::Exported {
            state.reimport()
        } else {
            state.rg_handle()
        }
    }

    /// `get_dyn` / `get_mut_dyn` (O names; erased-type variants).
    pub fn get_dyn(&self, id: SsboId) -> Option<&BufferState> {
        self.buffers.get(&id).map(|b| &**b)
    }

    /// `add_existing` (O name; U body — registers an externally-created buffer).
    pub fn add_existing(&mut self, _state: BufferState) {
        // U
    }
}

impl Default for AssetSsboLibrary {
    fn default() -> Self {
        Self::new()
    }
}

/// Private marker used by `StorageBuffer` implementors (declared here to keep
/// the contract cohesive; concrete buffers live in `render::ssbo`).
pub struct MappedStorageBuffer<T> {
    _data: T,
    _marker: PhantomData<T>,
}
