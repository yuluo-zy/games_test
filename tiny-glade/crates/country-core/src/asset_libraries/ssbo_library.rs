//! `AssetSsboLibrary` — 类型化存储缓冲（SSBO）注册表（layer A3）。
//!
//! 证据（O，逐条核对机器码，见 `evidence/country-core-audit/L0_NOTES.md` 的 "A3 ssbo_library"）：
//!  * 库本体就是一张 `frozen::FrozenMap<SsboId, Box<BufferState>>`（`impl$6::default` 0x14091eb60：空表，
//!    `[+8]` 的“借用中”字节为 0）。所有入口都取 `&self`（`FrozenMap::insert` 经内部锁），`get_mut` 除外。
//!  * `SsboId = NameHash(xxh3_64(T::NAME))`——**短名**（`StorageElement::NAME`，如 "Cobblestone3RasterStagingSsbo"），
//!    不是完整类型名。`tools/country-core-audit/verify_ssbo_id.py` 用原 EXE 的 xxh3 secret 复算了
//!    `Cobblestone3RasterStagingSsbo → 0x802ff23dee42ab47`、`NaniNextVisibilityBitmaskSsbo → 0x69cdc275a605bad6`，
//!    与两处指令立即数一致。
//!  * `BufferState`（0x40 B，`BufferState::new` 0x14091eb30）：`data: Box<dyn Ssbo>` @0x00（胖指针）、
//!    `name: &'static str` @0x10（**完整** `type_name::<T>()`）、`rg: RwLock<Option<ImportedBuffer>>` @0x20
//!    （锁字 @0x20，20 B 句柄 @0x28，`+0x36` 的字节 = 5 表示 None）。旧版把 5 当成 “已导出”，实际是 “未导入”。
//!  * `add_new<T>(&self, device)`（0x1412bc470 起 ~80 个单态化）：`T::new_storage_buffer(device, None)`；
//!    `add_new_with_data<T>(&self, device, &[T])`（0x1412badf0 起）：同上但带 `len * size_of::<T>()` 字节的初始数据。
//!    之后 `Box<dyn Ssbo>` → `BufferState::new` → `FrozenMap::insert`。键已存在时：
//!      - 已有 `name` 与新类型完整名相同 → `panic!("Tried to register the same storage buffer twice: {NAME} (type: {full})")`；
//!      - 否则（短名哈希撞键）→ `panic!("Tried to register two storage buffers with the same `NAME`: {NAME}\nPreviously
//!        registered: {existing.name}\nType of the duplicate: {full}")`。
//!    缓冲在查重**之前**就已创建。
//!  * `get_rg_buffer<T>(&self, rg) -> ImportedBuffer`（0x1402b31a0 起 ~82 个）：
//!     1. 查表；缺失 → `panic!("Buffer {NAME} (type: {full}) is not available in the library. Forgot to register it?")`；
//!     2. 读锁；`rg` 为 Some → 直接返回该句柄；
//!     3. 为 None：释放读锁，**再查一次表**（`Option::unwrap`），`assert_eq!(state.name, type_name::<T>())`，
//!        `rg.import_unowned(state.data.handle(), 0x37)`，写锁存入 Some，返回新句柄。
//!  * `get_mut<T>(&mut self) -> Option<&mut T::Storage>`（0x1412098a0 起）：表空 / 键缺失 → `None`；
//!    命中后同样 `assert_eq!(name, type_name::<T>())`，再经 vtable 的 `as_any_mut` 取回具体缓冲。
//!  * `export_from_rg(&self, rg)`（0x14091ebb0）：对每个 `BufferState` 取写锁，`rg.take()`；原先为 Some 才
//!    `rg.export(handle, 0x37)`。表的“借用中”字节非 0 → `Result::unwrap` panic（占位 `FrozenMap` 不建模）。
//!  * `Ssbo` trait 的四个方法（B 阶段 i_api 逐个核对虚表，O）：虚表 = `[drop, size, align, +0x18 as_any, +0x20 as_any_mut,
//!    +0x28 buffer, +0x30 handle]`，**四组实现**（78 个虚表：`impl$0` ×38 / `impl$1` ×21 / `impl$2` ×19 + `sparse_instance_buffer::impl$12`）：
//!    | 实现 | 具体类型（size） | `as_any`/`as_any_mut` | `buffer`/`handle`（ICF 后同一个桩） |
//!    |---|---|---|---|
//!    | `impl$0` | `TypedStorageBuffer<T>`（0x28） | `mov rax, rcx; lea rdx, [Any 虚表]` | `mov rax, rcx`（缓冲在 +0） |
//!    | `impl$2` | `InstanceBuffer<T>`（0x30） | 同上 | `lea rax, [rcx+8]` |
//!    | `impl$1` | `MappedStorageBuffer<T>`（0x38） | 同上 | `lea rax, [rcx+0x10]` |
//!    | `impl$12`（sparse_instance_buffer） | `SparseInstanceBuffer<T>`（0x90） | 同上 | `lea rax, [rcx+0x68]`（0x140005c50） |
//!    `buffer()` 与 `handle()` 在每组里指向**同一个**桩（返回内嵌 `Buffer` 的地址；`ResourceHandle` 是 `Buffer` 的首部）。
//!    **纠正 d_shaders/B_i 之前的说法**：虚表槽 +0x18 就是 `as_any`（返回 `(自身指针, Any 虚表)`），不是“取实例缓冲”。
//!    `ivy_leaf_shader`/`roof_spire`/`roof_support_beam` 对该槽返回的**具体对象指针**直接 `mov rsi, [rax]`：对 `InstanceBuffer<T>`
//!    `+0` 是实例数；`indirect_draw_bricks` 对 `SparseInstanceBuffer<BrickTransformSsbo>` 取 `+0x68`（其内嵌 `Buffer`，即 `handle()`）当
//!    `&ManagedHandle` 传给 `bind_external_storage_buffer`。原版调用点没有再比对 `TypeId`（只有名字比对：`state.name == type_name::<T>()`，
//!    失败 → `assert_failed<&str,&str>`），这里用安全的 `downcast_ref` 代替。
//!    `ivy_leaf_shader` 的调用序列（`FrozenMap::get` → 名字断言 → `as_any` → `[rax]`；未注册 → 实例数 0）即 [`AssetSsboLibrary::instance_count`]。
//! 偏离：`frozen` 为外部库，用 `crate::frozen_map::FrozenMap` 占位；`RenderGraph` 同样是占位（记录调用）。

// Preserve the original instruction-level expression order/NaN policy, API call shapes, and evidence/test transcription. See README.md.
#![allow(clippy::doc_lazy_continuation, clippy::redundant_closure)]
use core::any::{Any, type_name};

use parking_lot::RwLock;

use crate::frozen_map::FrozenMap;
use crate::name_hash::NameHash;
use crate::render::ssbo::rhapsody_boundary::{
    Buffer, GraphicsDevice, ResourceHandle, slice_as_bytes,
};
use crate::render::ssbo::{
    InstanceBuffer, InstanceCounted, MappedStorageBuffer, StorageElement, TypedStorageBuffer,
};
use crate::rhapsody_types::{ImportedBuffer, RenderGraph};

/// `import_unowned` / `export` 的第三参数（O：`mov r9b, 0x37` / `mov r8b, 0x37`）。
pub const RG_BUFFER_ACCESS: u8 = 0x37;

/// 存储缓冲的类型擦除接口（虚表槽序全部为 O，B 阶段 i_api 核对：+0x18 `as_any`、+0x20 `as_any_mut`、+0x28 `buffer`、+0x30 `handle`）。
pub trait Ssbo: Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn buffer(&self) -> &Buffer;
    fn handle(&self) -> &ResourceHandle;
    /// CPU 影子（O：`nani_update_preplaced_flower` 在 `BufferState.data` 的 vtable `+0x18` 取回一个
    /// `(ptr, len)` 的 u32 数组，`NULL` → `Option::unwrap` panic）。只有带 CPU 影子的缓冲
    /// （`ShadowedBuffer<T>`，4 字节元素）返回 `Some`；其余为 `None`。槽位序号为 I。
    fn cpu_shadow_words(&self) -> Option<&[u32]> {
        None
    }
    /// 实例数（B 阶段追加，i_api）：原版没有该虚方法——调用点经 `as_any`（+0x18）读具体 `InstanceBuffer<T>` 的 `+0`。
    /// 类型擦除的库查询（[`AssetSsboLibrary::lookup`]）需要它；只有 `InstanceBuffer<T>` 返回 `Some`。
    fn instance_count(&self) -> Option<usize> {
        None
    }
}

impl<T: StorageElement + Send + Sync + 'static> Ssbo
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
    fn handle(&self) -> &ResourceHandle {
        handle_of(&self.buffer.buffer)
    }
    fn cpu_shadow_words(&self) -> Option<&[u32]> {
        if core::mem::size_of::<T>() != 4 || core::mem::align_of::<T>() != 4 {
            return None;
        }
        // SAFETY: 元素为 4 字节、4 对齐的 `Copy` 位模式类型（上面检查），`data` 是连续的 `Box<[T]>`。
        Some(unsafe {
            core::slice::from_raw_parts(self.data.as_ptr() as *const u32, self.data.len())
        })
    }
}

/// 缓冲首字即句柄（O：`impl$0::handle` = `mov rax, rcx`；其余偏移见模块注释）。
fn handle_of(buffer: &Buffer) -> &ResourceHandle {
    // SAFETY: `Buffer` 首部两个 u64 与 `ResourceHandle(u64, u64)` 同布局（既有占位约定，I）。
    unsafe { &*(buffer as *const Buffer as *const ResourceHandle) }
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
    fn handle(&self) -> &ResourceHandle {
        handle_of(&self.buffer)
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
    fn handle(&self) -> &ResourceHandle {
        handle_of(&self.buffer.buffer)
    }
    fn instance_count(&self) -> Option<usize> {
        Some(InstanceCounted::instance_count(self))
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
    fn handle(&self) -> &ResourceHandle {
        handle_of(&self.buffer)
    }
}

/// 可注册的存储缓冲类型（O：`add_new<T>` / `get_rg_buffer<T>` 的泛型参数；`T` 是元素类型，`NAME` 为其短名）。
/// `Storage` 是实际缓冲（`TypedStorageBuffer<T>` / `InstanceBuffer<T>` / `MappedStorageBuffer<T>` 三选一）。
pub trait SsboType: StorageElement + 'static {
    type Storage: Ssbo + 'static;
    fn new_storage_buffer(device: &dyn GraphicsDevice, initial: Option<&[u8]>) -> Self::Storage;
}

/// `FrozenMap` 的键（O：`SsboId` 包一个 `NameHash`）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SsboId(pub NameHash);

impl SsboId {
    /// O：`NameHash::compute(T::NAME)`（编译期折叠成立即数）。
    pub fn of<T: StorageElement>() -> Self {
        SsboId(NameHash::compute(T::NAME))
    }
}

/// 一个已注册的缓冲（0x40 B，O）。
pub struct BufferState {
    data: Box<dyn Ssbo>,
    name: &'static str,
    /// 当前帧导入渲染图后的句柄；`None` = 未导入（O：`+0x36 == 5`）。
    rg: RwLock<Option<ImportedBuffer>>,
}

impl BufferState {
    /// O 0x14091eb30：锁字清零、句柄置“未导入”。
    pub fn new(data: Box<dyn Ssbo>, name: &'static str) -> Self {
        BufferState {
            data,
            name,
            rg: RwLock::new(None),
        }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn data(&self) -> &dyn Ssbo {
        &*self.data
    }

    pub fn rg_handle(&self) -> Option<ImportedBuffer> {
        *self.rg.read()
    }

    /// 实例数：存储是 `InstanceBuffer<T>` → `Some(count)`，其余 `None`（O：`[as_any()]` 读 `+0`）。
    pub fn instance_count(&self) -> Option<usize> {
        self.data.instance_count()
    }
}

/// [`AssetSsboLibrary::lookup`] 的类型擦除结果：`(state.name, 实例缓冲)`。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InstanceInfo {
    pub instance_count: usize,
    /// 内嵌缓冲（`InstanceBuffer` `+8`）；`handle` 是它的首部。
    pub buffer: Buffer,
}

/// 存储缓冲注册表。
#[derive(bevy_ecs::resource::Resource)]
pub struct AssetSsboLibrary {
    buffers: FrozenMap<SsboId, BufferState>,
}

impl Default for AssetSsboLibrary {
    /// O 0x14091eb60。
    fn default() -> Self {
        AssetSsboLibrary {
            buffers: FrozenMap::new(),
        }
    }
}

impl AssetSsboLibrary {
    pub fn new() -> Self {
        Self::default()
    }

    /// O（0x1412bc470 起）：不带初始数据。
    pub fn add_new<T: SsboType>(&self, device: &dyn GraphicsDevice) {
        let storage = T::new_storage_buffer(device, None);
        self.register::<T>(storage);
    }

    /// O（0x1412badf0 起）：`len * size_of::<T>()` 字节的初始数据。
    pub fn add_new_with_data<T: SsboType>(&self, device: &dyn GraphicsDevice, data: &[T]) {
        let storage = T::new_storage_buffer(device, Some(slice_as_bytes(data)));
        self.register::<T>(storage);
    }

    fn register<T: SsboType>(&self, storage: T::Storage) {
        let full = type_name::<T>();
        let state = BufferState::new(Box::new(storage), full);
        if let Err(existing) = self.buffers.insert(SsboId::of::<T>(), Box::new(state)) {
            if existing.name == full {
                panic!(
                    "Tried to register the same storage buffer twice: {} (type: {})",
                    T::NAME,
                    full
                );
            }
            panic!(
                "Tried to register two storage buffers with the same `NAME`: {}\nPreviously registered: {}\nType of the duplicate: {}",
                T::NAME,
                existing.name,
                full
            );
        }
    }

    /// O（0x1402b31a0 起）：见模块注释。
    pub fn get_rg_buffer<T: SsboType>(&self, rg: &mut RenderGraph) -> ImportedBuffer {
        let id = SsboId::of::<T>();
        let state = self.buffers.get(&id).unwrap_or_else(|| {
            panic!(
                "Buffer {} (type: {}) is not available in the library. Forgot to register it?",
                T::NAME,
                type_name::<T>()
            )
        });
        if let Some(handle) = *state.rg.read() {
            return handle;
        }
        let state = self.buffers.get(&id).unwrap();
        assert_eq!(state.name, type_name::<T>());
        let imported = rg.import_unowned(state.data.handle().0, RG_BUFFER_ACCESS);
        *state.rg.write() = Some(imported);
        imported
    }

    /// O（0x1412098a0 起）：未注册 → `None`。
    pub fn get_mut<T: SsboType>(&mut self) -> Option<&mut T::Storage> {
        let id = SsboId::of::<T>();
        let state = self.buffers.get_mut(&id)?;
        assert_eq!(state.name, type_name::<T>());
        state.data.as_any_mut().downcast_mut::<T::Storage>()
    }

    /// O（`update_color_library` 内联，源码 ssbo_library.rs 行 210）：只读取缓冲。
    /// 未注册 → `None`；命中后 `assert_eq!(state.name, type_name::<T>())`，再经 vtable +0x18 取 `Buffer`。
    /// （A3 color 组新增；`buffers` 是私有字段，调用方没有别的只读入口。）
    pub fn get_buffer<T: SsboType>(&self) -> Option<&Buffer> {
        let state = self.buffers.get(&SsboId::of::<T>())?;
        assert_eq!(state.name, type_name::<T>());
        Some(state.data.buffer())
    }

    /// O（`nani_update_preplaced_flower` 内联）：`FrozenMap::get(SsboId)`；未注册 → `None`；命中后
    /// `assert_eq!(state.name, type_name::<T>())`（memcmp 0x3f 字节），再经 `Ssbo` vtable `+0x18`
    /// 取 CPU 影子 u32 数组（无影子 → `None`，原版 `unwrap` panic 的同一路径）。
    pub fn get_cpu_shadow_words<T: SsboType>(&self) -> Option<&[u32]> {
        let state = self.buffers.get(&SsboId::of::<T>())?;
        assert_eq!(state.name, type_name::<T>());
        state.data.cpu_shadow_words()
    }

    /// 取出 `T` 的具体存储（B 阶段追加，i_api；O：`ivy_leaf_shader` 0x1408dc1c0、`indirect_draw_bricks` 0x1408a53d0 等的序列）：
    /// `FrozenMap::get(SsboId::of::<T>())` → 未注册 `None`；`assert_eq!(state.name, type_name::<T>())`
    /// （`assert_failed<&str,&str>`）→ `as_any`（虚表 +0x18）→ 具体对象。原版不再比对 `TypeId`；这里 `downcast_ref`，类型不符 → `None`。
    pub fn get<T: SsboType>(&self) -> Option<&T::Storage> {
        let state = self.buffers.get(&SsboId::of::<T>())?;
        assert_eq!(state.name, type_name::<T>());
        state.data.as_any().downcast_ref::<T::Storage>()
    }

    /// 该类型是否已注册（`FrozenMap::get` 非空；不做名字断言）。
    pub fn contains<T: SsboType>(&self) -> bool {
        self.buffers.get(&SsboId::of::<T>()).is_some()
    }

    /// 实例数（O：`ivy_leaf_shader` / `roof_spire` / `roof_support_beam`）：未注册 → 0（`xor esi, esi`），否则读 `InstanceBuffer` 的 `+0`。
    /// `Storage` 不是实例缓冲的类型在编译期被 `InstanceCounted` 约束排除。
    pub fn instance_count<T: SsboType>(&self) -> usize
    where
        T::Storage: InstanceCounted,
    {
        self.get::<T>()
            .map_or(0, |s| InstanceCounted::instance_count(s))
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

    /// O 0x14091ebb0：把当前持有的渲染图导入全部导出，并回到“未导入”。
    pub fn export_from_rg(&self, rg: &mut RenderGraph) {
        self.buffers.for_each(|state| {
            if let Some(handle) = state.rg.write().take() {
                rg.export(handle, RG_BUFFER_ACCESS);
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rhapsody_contract::GraphicsDevice as Device;

    #[derive(Clone, Copy, Debug, PartialEq)]
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

    #[derive(Clone, Copy, Debug, PartialEq)]
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
    #[derive(Clone, Copy, Debug, PartialEq)]
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
