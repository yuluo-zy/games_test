//! asset_libraries — 非渲染资产注册表（WP16 恢复）。
//!
//! 源 SHA256（B skeleton）`asset_libraries/{mod,meshwatch,configwatch,
//! texture_watch,vao_library,placement_data_library}.rs`（audio_trigger_library
//! 属 WP14）。PDB 实体：`Asset` / `Handle` / `HandleId` / `MeshWatch` /
//! `ConfigWatch` / `TextureWatch` / `AssetVAOLibrary` / `AssetPlacementDataLibrary`
//! / `InstancedDrawFromPlacementData`。
//!
//! `Asset` / `Handle<T>` / `HandleId` 为游戏自有注册表 id（wp4 已恢复渲染面；
//! 这里重述非渲染表面并按 wp4 接线）。

pub mod configwatch;
pub mod meshwatch;
pub mod placement_data_library;
pub mod texture_watch;
pub mod vao_library;

/// 原始句柄 id（O：`HandleId::new` 全局计数器分配；`HandleId` 实体）。
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct HandleId(u64);

impl HandleId {
    /// `new`（O：全局 HANDLE_COUNT 原子自增）。
    pub fn new() -> Self {
        static COUNT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
        HandleId(COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed))
    }
    pub fn from_raw(raw: u64) -> Self {
        HandleId(raw)
    }
    pub fn raw(self) -> u64 {
        self.0
    }
}

/// 命名资产（O：`Asset`；`Asset::new` / `Asset::name`）。
#[derive(Clone, Debug, Default)]
pub struct Asset {
    pub name: alloc::string::String,
}

impl Asset {
    pub fn new(name: alloc::string::String) -> Self {
        Asset { name }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// 类型化句柄（O：`Handle<T>`；按 HandleId 等值/哈希）。
#[derive(Clone, Copy)]
pub struct Handle<T> {
    id: HandleId,
    _marker: core::marker::PhantomData<fn() -> T>,
}

impl<T> Handle<T> {
    pub fn new() -> Self {
        Handle {
            id: HandleId::new(),
            _marker: core::marker::PhantomData,
        }
    }
    pub fn is_valid(&self) -> bool {
        self.id.raw() != 0
    }
    pub fn id(&self) -> HandleId {
        self.id
    }
}

impl<T> PartialEq for Handle<T> {
    fn eq(&self, o: &Self) -> bool {
        self.id == o.id
    }
}
impl<T> Eq for Handle<T> {}
impl<T> core::hash::Hash for Handle<T> {
    fn hash<H: core::hash::Hasher>(&self, h: &mut H) {
        self.id.hash(h);
    }
}
impl<T> core::fmt::Debug for Handle<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Handle").field(&self.id).finish()
    }
}
