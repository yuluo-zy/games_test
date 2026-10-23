//! 类型化存储缓冲注册表。短名定位、完整类型名校验，按图导入周期缓存句柄。
//! 原作来源见项目 evidence；当前接口、行为变化与验证范围见 README.md。

#![allow(clippy::doc_lazy_continuation, clippy::redundant_closure)]
use core::any::{Any, type_name};
use std::sync::{Arc, atomic::{AtomicU64, Ordering}};

use parking_lot::RwLock;

use crate::frozen_map::FrozenMap;
use crate::name_hash::NameHash;
use crate::render::ssbo::rhapsody_boundary::{Buffer, GraphicsDevice, ResourceHandle};
use crate::render::ssbo::{
    InstanceBuffer, InstanceCounted, MappedStorageBuffer, StorageElement, TypedStorageBuffer,
};
use crate::render::sparse_instance_buffer::SparseDrawView;
use crate::render::extract::{ChangeCursor, ChangeJournal};
use crate::rhapsody_types::{ImportedBuffer, RenderGraph};

pub const RG_BUFFER_ACCESS: u8 = 0x37;

/// 存储对象的公共访问面；句柄按值返回，不依赖内存布局强转。
pub trait Ssbo: Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn buffer(&self) -> &Buffer;
    fn handle(&self) -> ResourceHandle;
    /// 受 NoUninit 约束的四字节元素视图，由 bytemuck 检查转换。
    fn cpu_shadow_words(&self) -> Option<&[u32]> {
        None
    }
    fn instance_count(&self) -> Option<usize> {
        None
    }
    /// Declaration-only sparse metadata; never exposes CPU instance storage
    /// or allocation/reclamation authority to a render snapshot.
    fn sparse_draw_view(&self) -> Option<SparseDrawView<'_>> {
        None
    }
    /// The render thread's declaration state at hand-over. A snapshot may
    /// expose a different concrete type with no CPU mutation authority.
    fn snapshot(&self) -> Box<dyn Ssbo>;
    /// Interior-mutating storage (such as sparse visibility) publishes into
    /// the same library journal even when its caller holds only `&Storage`.
    /// Render snapshots must not inherit a producer's notification callback.
    fn set_change_notifier(&mut self, _notify: Arc<dyn Fn() + Send + Sync>) {}
}

impl<T: StorageElement + bytemuck::NoUninit + Send + Sync + 'static> Ssbo
    for crate::render::shadowed_buffer::ShadowedBuffer<T>
{
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn buffer(&self) -> &Buffer {
        &self.buffer.buffer
    }
    fn handle(&self) -> ResourceHandle {
        handle_of(&self.buffer.buffer)
    }
    fn cpu_shadow_words(&self) -> Option<&[u32]> {
        if core::mem::size_of::<T>() != 4 || core::mem::align_of::<T>() != 4 {
            return None;
        }
        // NoUninit 排除填充和未初始化字节，转换由既有依赖检查对齐。
        bytemuck::try_cast_slice(&self.data).ok()
    }
    fn snapshot(&self) -> Box<dyn Ssbo> {
        // Declaration has no CPU-shadow consumer. Keep its typed GPU metadata,
        // while the game retains the only shadow and mutation authority.
        Box::new(self.buffer)
    }
}

fn handle_of(buffer: &Buffer) -> ResourceHandle {
    ResourceHandle(buffer.id, buffer.generation)
}

impl<T: StorageElement + Send + Sync + 'static> Ssbo for TypedStorageBuffer<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn buffer(&self) -> &Buffer {
        &self.buffer
    }
    fn handle(&self) -> ResourceHandle {
        handle_of(&self.buffer)
    }
    fn snapshot(&self) -> Box<dyn Ssbo> {
        Box::new(*self)
    }
}

impl<T: StorageElement + Send + Sync + 'static> Ssbo for InstanceBuffer<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn buffer(&self) -> &Buffer {
        &self.buffer.buffer
    }
    fn handle(&self) -> ResourceHandle {
        handle_of(&self.buffer.buffer)
    }
    fn instance_count(&self) -> Option<usize> {
        Some(InstanceCounted::instance_count(self))
    }
    fn snapshot(&self) -> Box<dyn Ssbo> {
        Box::new(*self)
    }
}

impl<T: StorageElement + Send + Sync + 'static> Ssbo for MappedStorageBuffer<'static, T> {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn buffer(&self) -> &Buffer {
        &self.buffer
    }
    fn handle(&self) -> ResourceHandle {
        handle_of(&self.buffer)
    }
    /// The mapping stays with the game-side owner (two `&mut` views of one
    /// mapping would alias); the render copy binds the same buffer only.
    fn snapshot(&self) -> Box<dyn Ssbo> {
        Box::new(MappedStorageBuffer::<'static, T>::from_mapping(&mut [], self.buffer))
    }
}

/// 元素到具体存储类型的关联，由该类型决定创建方式。
pub trait SsboType: StorageElement + 'static {
    type Storage: Ssbo + 'static;
    fn new_storage_buffer(device: &dyn GraphicsDevice, initial: Option<&[u8]>) -> Self::Storage;
}

/// 元素短名的哈希键；完整类型名另存，用于发现名字冲突。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SsboId(pub NameHash);

impl SsboId {
    pub fn of<T: StorageElement>() -> Self {
        SsboId(NameHash::compute(T::NAME))
    }
}

/// 已注册存储与当前导入周期的图句柄。
pub struct BufferState {
    data: Box<dyn Ssbo>,
    name: &'static str,
    render_graph: Arc<RwLock<Option<ImportedBuffer>>>,
}

impl BufferState {
    /// The render-thread copy; the graph-local import cache is shared because
    /// only declaration uses it.
    fn snapshot(&self) -> Self {
        Self { data: self.data.snapshot(), name: self.name, render_graph: self.render_graph.clone() }
    }
}

impl BufferState {
    /// 创建初始状态。
    pub fn new(data: Box<dyn Ssbo>, name: &'static str) -> Self {
        BufferState {
            data,
            name,
            render_graph: Arc::new(RwLock::new(None)),
        }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn data(&self) -> &dyn Ssbo {
        &*self.data
    }

    pub fn rg_handle(&self) -> Option<ImportedBuffer> {
        *self.render_graph.read()
    }

    pub fn instance_count(&self) -> Option<usize> {
        self.data.instance_count()
    }
}

/// An owned declaration-time view of one actual registered storage entry.
/// It shares the library's existing import cache; taking a snapshot does not
/// import any buffer, create a parallel library or extend an old graph scope.
#[derive(Clone)]
pub struct SsboGraphLease {
    pub id: SsboId,
    pub canonical_type_name: &'static str,
    buffer: Buffer,
    instance_count: Option<usize>,
    render_graph: Arc<RwLock<Option<ImportedBuffer>>>,
}
impl SsboGraphLease {
    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }
    pub fn instance_count(&self) -> Option<usize> {
        self.instance_count
    }
    pub fn get_rg_buffer(
        &self,
        graph: &mut RenderGraph,
        expected_type: &'static str,
    ) -> ImportedBuffer {
        assert_eq!(self.canonical_type_name, expected_type);
        import_buffer(&self.render_graph, self.buffer.id, graph)
    }
    pub fn rg_handle(&self) -> Option<ImportedBuffer> {
        *self.render_graph.read()
    }
}

fn import_buffer(
    cache: &RwLock<Option<ImportedBuffer>>,
    buffer: u64,
    graph: &mut RenderGraph,
) -> ImportedBuffer {
    if let Some(handle) = *cache.read() {
        return handle;
    }
    let mut cached = cache.write();
    if let Some(handle) = *cached {
        return handle;
    }
    let imported = graph.import_unowned(buffer, RG_BUFFER_ACCESS);
    *cached = Some(imported);
    imported
}

/// 类型擦除后的实例数与底层缓冲。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InstanceInfo {
    pub instance_count: usize,
    /// 内嵌缓冲（`InstanceBuffer` `+8`）；`handle` 是它的首部。
    pub buffer: Buffer,
}

/// 保存类型擦除的存储对象，读取时核对具体元素类型。
#[derive(bevy_ecs::resource::Resource)]
pub struct AssetSsboLibrary {
    buffers: FrozenMap<SsboId, BufferState>,
    // register() takes &self, so ECS resource ticks do not observe additions.
    registration_revision: AtomicU64,
    changes: Arc<ChangeJournal<SsboId>>,
    source_cursor: Option<ChangeCursor>,
}

/// A snapshot for the render world: same GPU buffers and declaration metadata.
/// Sparse instances keep only draw metadata; their CPU shadows stay in game.
impl Clone for AssetSsboLibrary {
    fn clone(&self) -> Self {
        // Capture before reading entries. A concurrent shared registration
        // after this point must remain replayable on the next hand-over.
        let source_cursor = Some(self.changes.cursor());
        AssetSsboLibrary {
            buffers: self.buffers.snapshot_with(BufferState::snapshot),
            registration_revision: AtomicU64::new(self.registration_revision()),
            changes: Arc::new(ChangeJournal::default()),
            source_cursor,
        }
    }
}

impl Default for AssetSsboLibrary {
    fn default() -> Self {
        AssetSsboLibrary {
            buffers: FrozenMap::new(),
            registration_revision: AtomicU64::new(0),
            changes: Arc::new(ChangeJournal::default()),
            source_cursor: None,
        }
    }
}

impl AssetSsboLibrary {
    pub fn new() -> Self {
        Self::default()
    }

    /// Additions made through shared resource references need this token in
    /// addition to the ECS tick. Storage edits still use exclusive get_mut.
    pub fn registration_revision(&self) -> u64 {
        self.registration_revision.load(Ordering::Acquire)
    }

    pub fn snapshot_revision(&self) -> u64 { self.changes.revision() }

    /// Replay actual registration/storage writes into the existing render
    /// table. Unchanged sparse draw metadata keeps its allocation and owner.
    pub fn update_render_snapshot(&self, destination: &mut Self) {
        let Some((cursor, changed)) = self.changes.since(destination.source_cursor) else {
            *destination = self.clone();
            return;
        };
        let mut seen = std::collections::HashSet::with_capacity(changed.len());
        for id in changed {
            if !seen.insert(id) { continue; }
            let source = self.buffers.get(&id).expect("journal entry must name a registered SSBO");
            if let Some(entry) = destination.buffers.get_mut(&id) {
                *entry = source.snapshot();
            } else {
                assert!(destination.buffers.insert(id, Box::new(source.snapshot())).is_ok());
            }
            destination.changes.record(id);
        }
        destination.registration_revision.store(self.registration_revision(), Ordering::Release);
        destination.source_cursor = Some(cursor);
    }

    pub fn add_new<T: SsboType>(&self, device: &dyn GraphicsDevice) {
        let storage = T::new_storage_buffer(device, None);
        self.register::<T>(storage);
    }

    /// 上传 typed 数据必须排除未初始化填充；Copy 本身不保证逐字节读取安全。
    ///
    /// ```compile_fail
    /// use country_core::{asset_libraries::ssbo_library::{AssetSsboLibrary, SsboType},
    ///     render::ssbo::{StorageElement, TypedStorageBuffer, rhapsody_boundary::GraphicsDevice}};
    /// #[repr(C, align(4))]
    /// #[derive(Clone, Copy)]
    /// struct Padded(u8);
    /// impl StorageElement for Padded { const NAME: &'static str = "Padded"; }
    /// impl SsboType for Padded {
    ///     type Storage = TypedStorageBuffer<Self>;
    ///     fn new_storage_buffer(device: &dyn GraphicsDevice, data: Option<&[u8]>) -> Self::Storage {
    ///         TypedStorageBuffer::new_storage_buffer(device, data)
    ///     }
    /// }
    /// fn upload(library: &AssetSsboLibrary, device: &dyn GraphicsDevice) {
    ///     library.add_new_with_data(device, &[Padded(1)]);
    /// }
    /// ```
    pub fn add_new_with_data<T: SsboType + bytemuck::NoUninit>(
        &self,
        device: &dyn GraphicsDevice,
        data: &[T],
    ) {
        let storage = T::new_storage_buffer(device, Some(bytemuck::cast_slice(data)));
        self.register::<T>(storage);
    }

    /// 注册已成功创建的存储，允许调用方先处理设备创建失败。
    pub fn register<T: SsboType>(&self, mut storage: T::Storage) {
        let full_type_name = type_name::<T>();
        let changes = Arc::clone(&self.changes);
        storage.set_change_notifier(Arc::new(move || changes.record(SsboId::of::<T>())));
        let state = BufferState::new(Box::new(storage), full_type_name);
        if let Err(existing) = self.buffers.insert(SsboId::of::<T>(), Box::new(state)) {
            if existing.name == full_type_name {
                panic!(
                    "Tried to register the same storage buffer twice: {} (type: {})",
                    T::NAME,
                    full_type_name
                );
            }
            panic!(
                "Tried to register two storage buffers with the same `NAME`: {}\nPreviously registered: {}\nType of the duplicate: {}",
                T::NAME,
                existing.name,
                full_type_name
            );
        }
        self.registration_revision.fetch_update(Ordering::Release, Ordering::Relaxed,
            |revision| revision.checked_add(1)).expect("SSBO registration revision exhausted");
        self.changes.record(SsboId::of::<T>());
    }

    /// 同一导入周期内复用图句柄，导出后允许重新导入。
    pub fn get_rg_buffer<T: SsboType>(&self, render_graph: &mut RenderGraph) -> ImportedBuffer {
        let id = SsboId::of::<T>();
        let state = self.buffers.get(&id).unwrap_or_else(|| {
            panic!(
                "Buffer {} (type: {}) is not available in the library. Forgot to register it?",
                T::NAME,
                type_name::<T>()
            )
        });
        // 即使命中图缓存，也必须检查完整类型，不能让同 NAME 的其他类型绕过校验。
        assert_eq!(state.name, type_name::<T>());
        import_buffer(&state.render_graph, state.data.handle().0, render_graph)
    }

    /// Publish only metadata for real registered owners. Raster callbacks
    /// subsequently call get_rg_buffer on the selected lease, lazily, with
    /// the caller's exact generic type name, using this same shared cache.
    pub fn snapshot_graph_leases(&self) -> Vec<SsboGraphLease> {
        let mut leases = Vec::with_capacity(self.buffers.len());
        self.buffers.for_each_entry(|id, state| {
            leases.push(SsboGraphLease {
                id: *id,
                canonical_type_name: state.name,
                buffer: *state.data.buffer(),
                instance_count: state.data.instance_count(),
                render_graph: state.render_graph.clone(),
            })
        });
        leases
    }

    /// 按完整类型校验并取得存储的可变引用；紧凑渲染快照没有 CPU 存储可写。
    pub fn get_mut<T: SsboType>(&mut self) -> Option<&mut T::Storage> {
        let id = SsboId::of::<T>();
        let state = self.buffers.get_mut(&id)?;
        assert_eq!(state.name, type_name::<T>());
        let storage = state.data.as_any_mut().downcast_mut::<T::Storage>()?;
        self.changes.record(id);
        Some(storage)
    }

    pub fn get_buffer<T: SsboType>(&self) -> Option<&Buffer> {
        let state = self.buffers.get(&SsboId::of::<T>())?;
        assert_eq!(state.name, type_name::<T>());
        Some(state.data.buffer())
    }

    pub fn get_cpu_shadow_words<T: SsboType>(&self) -> Option<&[u32]> {
        let state = self.buffers.get(&SsboId::of::<T>())?;
        assert_eq!(state.name, type_name::<T>());
        state.data.cpu_shadow_words()
    }

    /// Read the concrete storage owner. Render-only sparse snapshots instead
    /// expose `sparse_draw_view`, and return None here.
    pub fn get<T: SsboType>(&self) -> Option<&T::Storage> {
        let state = self.buffers.get(&SsboId::of::<T>())?;
        assert_eq!(state.name, type_name::<T>());
        state.data.as_any().downcast_ref::<T::Storage>()
    }

    /// Works for both the game-owned storage and its compact render snapshot.
    /// Sparse render snapshots intentionally do not implement `T::Storage`:
    /// use this view for draw ranges, and `get_buffer` for GPU metadata.
    pub fn sparse_draw_view<T: SsboType>(&self) -> Option<SparseDrawView<'_>> {
        let state = self.buffers.get(&SsboId::of::<T>())?;
        assert_eq!(state.name, type_name::<T>());
        state.data.sparse_draw_view()
    }

    /// 该类型是否已注册（`FrozenMap::get` 非空；不做名字断言）。
    pub fn contains<T: SsboType>(&self) -> bool {
        self.buffers.get(&SsboId::of::<T>()).is_some()
    }

    pub fn instance_count<T: SsboType>(&self) -> usize
    where
        T::Storage: InstanceCounted,
    {
        self.get::<T>().map_or(0, InstanceCounted::instance_count)
    }

    /// 类型擦除的查询（不需要 `T`）：按 `SsboId` 取 `(state.name, 实例缓冲信息)`；存储不是 `InstanceBuffer` → 内层 `None`。
    /// 供按槽名哈希工作的调用方（`draw_shaders::ssbo_tags::SsboInstanceView`）使用——该 trait 的实现是
    /// `lookup(NameHash(id))` 再把 `InstanceInfo::buffer` 映射成它的 `ManagedHandle`。
    pub fn lookup(&self, id: SsboId) -> Option<(&'static str, Option<InstanceInfo>)> {
        let state = self.buffers.get(&id)?;
        let info = state
            .data
            .instance_count()
            .map(|instance_count| InstanceInfo {
                instance_count,
                buffer: *state.data.buffer(),
            });
        Some((state.name, info))
    }

    /// 只查询底层缓冲，不要求普通实例计数；适用于稀疏实例和间接绘制。
    pub fn lookup_buffer(&self, id: SsboId) -> Option<(&'static str, ResourceHandle)> {
        let state = self.buffers.get(&id)?;
        Some((state.name, state.data.handle()))
    }

    /// 导出所有已导入缓冲并清除本周期缓存。
    pub fn export_from_rg(&self, render_graph: &mut RenderGraph) {
        // Render snapshots have independent hash seeds. Export order is part
        // of the graph plan identity, so use logical import order rather than
        // hash iteration; otherwise every extracted frame invalidates PGO too.
        let mut handles = Vec::new();
        self.buffers.for_each(|state| {
            if let Some(handle) = state.render_graph.write().take() { handles.push(handle); }
        });
        handles.sort_unstable_by_key(|handle| (handle.resource, handle.generation));
        for handle in handles { render_graph.export(handle, RG_BUFFER_ACCESS); }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::ssbo::rhapsody_boundary::slice_as_bytes;
    use crate::render::test_device::TestDevice as Device;

    #[test]
    fn raster_metadata_lease_shares_the_library_cache_and_full_type_assertion() {
        let device = Device::default();
        let library = AssetSsboLibrary::new();
        library.add_new_with_data(&device, &[Alpha(0)]);
        library.add_new::<Beta>(&device);
        let leases = library.snapshot_graph_leases();
        assert!(leases.iter().all(|lease| lease.rg_handle().is_none()));
        let alpha = leases
            .iter()
            .find(|lease| lease.id == SsboId::of::<Alpha>())
            .unwrap();
        assert_eq!(alpha.buffer(), library.get_buffer::<Alpha>().unwrap());
        let mut graph = RenderGraph::default();
        let imported = alpha.get_rg_buffer(&mut graph, type_name::<Alpha>());
        assert_eq!(library.get_rg_buffer::<Alpha>(&mut graph), imported);
        assert!(
            leases
                .iter()
                .find(|lease| lease.id == SsboId::of::<Beta>())
                .unwrap()
                .rg_handle()
                .is_none()
        );
        // Even a cached NAME hit must not accept another canonical Rust type.
        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(
                || alpha.get_rg_buffer(&mut graph, type_name::<Beta>())
            ))
            .is_err()
        );
        library.export_from_rg(&mut graph);
        assert!(alpha.rg_handle().is_none());
    }

    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, PartialEq, bytemuck::NoUninit)]
    struct Alpha(u32);
    impl StorageElement for Alpha {
        const NAME: &'static str = "AlphaSsbo";
    }
    impl SsboType for Alpha {
        type Storage = TypedStorageBuffer<Alpha>;
        fn new_storage_buffer(
            device: &dyn GraphicsDevice,
            initial: Option<&[u8]>,
        ) -> Self::Storage {
            TypedStorageBuffer::new_storage_buffer(device, initial)
        }
    }

    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, PartialEq, bytemuck::NoUninit)]
    struct Beta(u32);
    impl StorageElement for Beta {
        const NAME: &'static str = "BetaSsbo";
        const BUFFER_LEN: usize = 4;
    }
    impl SsboType for Beta {
        type Storage = InstanceBuffer<Beta>;
        fn new_storage_buffer(
            device: &dyn GraphicsDevice,
            initial: Option<&[u8]>,
        ) -> Self::Storage {
            InstanceBuffer::new_storage_buffer(device, initial)
        }
    }

    /// 与 `Alpha` 同短名、不同类型：触发 “same `NAME`” 分支。
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, PartialEq, bytemuck::NoUninit)]
    struct AlphaImpostor(u32);
    impl StorageElement for AlphaImpostor {
        const NAME: &'static str = "AlphaSsbo";
    }
    impl SsboType for AlphaImpostor {
        type Storage = TypedStorageBuffer<AlphaImpostor>;
        fn new_storage_buffer(
            device: &dyn GraphicsDevice,
            initial: Option<&[u8]>,
        ) -> Self::Storage {
            TypedStorageBuffer::new_storage_buffer(device, initial)
        }
    }

    fn lib_with_alpha() -> (AssetSsboLibrary, Device) {
        let dev = Device(1);
        let lib = AssetSsboLibrary::new();
        lib.add_new_with_data(&dev, &[Alpha(1), Alpha(2)]);
        (lib, dev)
    }

    #[test]
    fn fresh_state_has_no_rg_handle() {
        let (lib, _) = lib_with_alpha();
        let s = lib.buffers.get(&SsboId::of::<Alpha>()).unwrap();
        assert_eq!(s.rg_handle(), None);
        assert_eq!(s.name(), type_name::<Alpha>());
    }

    #[test]
    fn incremental_snapshot_reuses_unmodified_storage_and_recovers_expired_history() {
        let (mut source, device) = lib_with_alpha();
        source.add_new::<Beta>(&device);
        let mut render = source.clone();
        let untouched = render.get::<Beta>().unwrap() as *const InstanceBuffer<Beta>;
        source.get_mut::<Alpha>().unwrap().buffer.generation = 19;
        source.update_render_snapshot(&mut render);
        assert_eq!(render.get_buffer::<Alpha>().unwrap().generation, 19);
        assert_eq!(render.get::<Beta>().unwrap() as *const InstanceBuffer<Beta>, untouched,
            "editing Alpha must not snapshot Beta or replace its box");
        for generation in 20..4200 {
            source.get_mut::<Alpha>().unwrap().buffer.generation = generation;
        }
        source.update_render_snapshot(&mut render);
        assert_eq!(render.get_buffer::<Alpha>().unwrap().generation, 4199);
        assert!(render.contains::<Beta>());
        let replacement = AssetSsboLibrary::new();
        replacement.add_new::<Beta>(&device);
        replacement.update_render_snapshot(&mut render);
        assert!(!render.contains::<Alpha>(), "a new source cannot reuse an old cursor");
    }

    #[test]
    fn get_rg_buffer_imports_once_then_reuses_the_handle() {
        let (lib, _) = lib_with_alpha();
        let mut rg = RenderGraph::default();
        let a = lib.get_rg_buffer::<Alpha>(&mut rg);
        let b = lib.get_rg_buffer::<Alpha>(&mut rg);
        assert_eq!(a, b);
        assert_eq!(rg.imports.len(), 1, "第二次命中缓存，不再导入");
        assert_eq!(rg.imports[0].1, RG_BUFFER_ACCESS);
    }

    #[test]
    fn cached_rg_buffer_still_rejects_a_colliding_element_type() {
        let (library, _) = lib_with_alpha();
        let mut graph = RenderGraph::default();
        let original = library.get_rg_buffer::<Alpha>(&mut graph);
        let collision = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            library.get_rg_buffer::<AlphaImpostor>(&mut graph)
        }));
        assert!(collision.is_err(), "图缓存不能绕过元素类型校验");
        assert_eq!(library.get_rg_buffer::<Alpha>(&mut graph), original);
        assert_eq!(graph.imports.len(), 1);
    }

    #[test]
    fn export_from_rg_exports_and_forces_reimport_next_frame() {
        let (lib, _) = lib_with_alpha();
        let mut rg = RenderGraph::default();
        let first = lib.get_rg_buffer::<Alpha>(&mut rg);
        lib.export_from_rg(&mut rg);
        assert_eq!(rg.exports, vec![(first, RG_BUFFER_ACCESS)]);
        assert_eq!(
            lib.buffers.get(&SsboId::of::<Alpha>()).unwrap().rg_handle(),
            None
        );
        // 再次导出：没有句柄可交，不产生新的 export。
        lib.export_from_rg(&mut rg);
        assert_eq!(rg.exports.len(), 1);
        let second = lib.get_rg_buffer::<Alpha>(&mut rg);
        assert_eq!(rg.imports.len(), 2);
        assert_ne!(first, second);
    }

    #[test]
    fn repeated_render_snapshots_export_in_stable_logical_order() {
        let device = Device::default();
        let library = AssetSsboLibrary::new();
        library.register::<Alpha>(Alpha::new_storage_buffer(&device, Some(&[0; 16])));
        library.register::<Beta>(Beta::new_storage_buffer(&device, Some(&[0; 16])));
        for _ in 0..64 {
            let snapshot = library.clone();
            let mut graph = RenderGraph::default();
            let first = snapshot.get_rg_buffer::<Beta>(&mut graph);
            let second = snapshot.get_rg_buffer::<Alpha>(&mut graph);
            snapshot.export_from_rg(&mut graph);
            assert_eq!(graph.exports, [(first, RG_BUFFER_ACCESS), (second, RG_BUFFER_ACCESS)]);
        }
    }

    #[test]
    fn export_skips_buffers_never_imported() {
        let (lib, _) = lib_with_alpha();
        let mut rg = RenderGraph::default();
        lib.export_from_rg(&mut rg);
        assert!(rg.exports.is_empty());
    }

    #[test]
    #[should_panic(expected = "Buffer BetaSsbo (type: ")]
    fn get_rg_buffer_of_unregistered_type_panics_with_forgot_to_register() {
        let (lib, _) = lib_with_alpha();
        lib.get_rg_buffer::<Beta>(&mut RenderGraph::default());
    }

    #[test]
    #[should_panic(expected = "is not available in the library. Forgot to register it?")]
    fn missing_message_tail() {
        let lib = AssetSsboLibrary::new();
        lib.get_rg_buffer::<Alpha>(&mut RenderGraph::default());
    }

    #[test]
    #[should_panic(expected = "Tried to register the same storage buffer twice: AlphaSsbo (type: ")]
    fn registering_the_same_type_twice_panics() {
        let (lib, dev) = lib_with_alpha();
        lib.add_new_with_data(&dev, &[Alpha(9)]);
    }

    #[test]
    #[should_panic(
        expected = "Tried to register two storage buffers with the same `NAME`: AlphaSsbo\nPreviously registered: "
    )]
    fn same_short_name_different_type_reports_both() {
        let (lib, dev) = lib_with_alpha();
        lib.add_new_with_data(&dev, &[AlphaImpostor(9)]);
    }

    #[test]
    fn get_mut_is_none_when_missing_and_downcasts_when_present() {
        let mut empty = AssetSsboLibrary::new();
        assert!(empty.get_mut::<Alpha>().is_none());

        let (mut lib, _) = lib_with_alpha();
        let buf = lib.get_mut::<Alpha>().expect("registered");
        // 初始数据 2 个 u32 元素 → 8 字节容量（占位设备按 desc.size 回填）。
        assert_eq!(buf.byte_size(), 8);
        assert!(lib.get_mut::<Beta>().is_none());
    }

    #[test]
    fn ssbo_impls_expose_buffer_and_handle_for_each_storage_kind() {
        let dev = Device(1);
        let t = TypedStorageBuffer::<Alpha>::new_storage_buffer(
            &dev,
            Some(slice_as_bytes(&[Alpha(1)])),
        );
        assert_eq!(t.handle().0, t.buffer().id);
        let i = InstanceBuffer::<Beta>::new_storage_buffer(&dev, Some(slice_as_bytes(&[Beta(1)])));
        assert_eq!(Ssbo::buffer(&i).byte_size, i.buffer.buffer.byte_size);
    }

    #[test]
    fn add_new_without_data_uses_default_creation_path() {
        let dev = Device(1);
        let lib = AssetSsboLibrary::new();
        lib.add_new::<Beta>(&dev); // BUFFER_LEN = 4 → 无初始数据也能创建
        assert!(lib.buffers.get(&SsboId::of::<Beta>()).is_some());
    }

    #[test]
    fn ssbo_id_depends_only_on_short_name() {
        assert_eq!(SsboId::of::<Alpha>(), SsboId::of::<AlphaImpostor>());
        assert_ne!(SsboId::of::<Alpha>(), SsboId::of::<Beta>());
    }

    /// 映射缓冲元素（`ButterflySsbo` 的同构：32 B、`BUFFER_LEN` 24、缺省元素非零）。
    #[derive(Clone, Copy, Debug, PartialEq)]
    #[repr(C)]
    struct Fly {
        position: [f32; 3],
        anim_t: f32,
        forward: [f32; 3],
        pad: f32,
    }
    impl StorageElement for Fly {
        const NAME: &'static str = "FlyProbe Buffer";
        const BUFFER_LEN: usize = 24;
        fn default_value() -> Option<Self> {
            Some(Fly {
                position: [0.0; 3],
                anim_t: 0.0,
                forward: [-0.0, -0.0, -1.0],
                pad: 0.0,
            })
        }
    }
    impl SsboType for Fly {
        type Storage = MappedStorageBuffer<'static, Fly>;
        fn new_storage_buffer(
            device: &dyn GraphicsDevice,
            initial: Option<&[u8]>,
        ) -> Self::Storage {
            MappedStorageBuffer::new_storage_buffer(device, initial)
        }
    }

    #[test]
    fn mapped_buffers_can_be_library_ssbos_and_mutated_through_get_mut() {
        let dev = Device(1);
        let mut lib = AssetSsboLibrary::new();
        lib.add_new::<Fly>(&dev);
        // 缺省元素已写入初始数据；映射区长度 = BUFFER_LEN 个元素。
        assert_eq!(lib.get::<Fly>().unwrap().mapped.len(), 24);
        let one = Fly {
            position: [1.0, 2.0, 3.0],
            anim_t: 0.5,
            forward: [0.0, 0.0, 1.0],
            pad: 0.0,
        };
        lib.get_mut::<Fly>().unwrap().cpu_write(&[one]);
        assert_eq!(lib.get::<Fly>().unwrap().mapped[0], one);
        // Ssbo 虚表：buffer/handle 指向缓冲（+0x10），不是实例缓冲。
        let state = lib.buffers.get(&SsboId::of::<Fly>()).unwrap();
        assert_eq!(state.data().buffer().byte_size, 24 * 32);
        assert_eq!(state.instance_count(), None);
    }

    #[test]
    fn instance_count_matches_the_instance_buffer_and_defaults_to_zero() {
        let dev = Device(1);
        let lib = AssetSsboLibrary::new();
        // 未注册 → 0（`xor esi, esi`），不 panic。
        assert_eq!(lib.instance_count::<Beta>(), 0);
        assert!(!lib.contains::<Beta>());
        lib.add_new_with_data(&dev, &[Beta(1), Beta(2), Beta(3)]);
        assert!(lib.contains::<Beta>());
        assert_eq!(lib.instance_count::<Beta>(), 3);
        assert_eq!(lib.get::<Beta>().unwrap().instance_count, 3);
    }

    #[test]
    fn lookup_is_type_erased_and_reports_instance_info_only_for_instance_buffers() {
        let dev = Device(1);
        let lib = AssetSsboLibrary::new();
        lib.add_new_with_data(&dev, &[Alpha(1)]);
        lib.add_new_with_data(&dev, &[Beta(1), Beta(2)]);
        assert!(lib.lookup(SsboId::of::<Fly>()).is_none());
        let (name, info) = lib.lookup(SsboId::of::<Alpha>()).unwrap();
        assert_eq!(name, type_name::<Alpha>());
        assert_eq!(info, None, "TypedStorageBuffer 不是实例缓冲");
        let (name, info) = lib.lookup(SsboId::of::<Beta>()).unwrap();
        assert_eq!(name, type_name::<Beta>());
        let info = info.unwrap();
        assert_eq!(info.instance_count, 2);
        assert_eq!(info.buffer.byte_size, 4 * 4, "BUFFER_LEN = 4 个 4 B 元素");
    }

    #[test]
    fn get_with_a_colliding_short_name_asserts_on_the_full_name() {
        let dev = Device(1);
        let lib = AssetSsboLibrary::new();
        lib.add_new_with_data(&dev, &[Alpha(1)]);
        // AlphaImpostor 与 Alpha 同短名 → 同 SsboId → 名字断言失败（`assert_failed<&str,&str>`）。
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            lib.get::<AlphaImpostor>().is_some()
        }));
        assert!(r.is_err());
    }
}
