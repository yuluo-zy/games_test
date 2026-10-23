//! 按 32 个实例分块管理的共享缓冲。
//!
//! 分配对象持有 chunk 列表，析构时进入延迟回收队列；批量替换先检查容量，
//! GPU 上传只提交合并后的脏区间，成功后清除标记。原作布局证据保存在历史报告。
use core::any::Any;
use core::sync::atomic::{AtomicUsize, Ordering};
use std::marker::PhantomData;
use std::sync::{Arc, Weak};

use parking_lot::{Mutex, MutexGuard, RwLock};

use crate::asset_libraries::ssbo_library::Ssbo;
use crate::render::ssbo::rhapsody_boundary::{self, GraphicsDevice, ResourceHandle};
use crate::render::ssbo::{StorageElement, TypedStorageBuffer};

/// 每个 chunk 的实例数（O：`shl 5` / 哨兵字节 0x20）。
pub const SLOTS_PER_CHUNK: usize = 32;

/// 稀疏缓冲的元素类型。`CHUNK_COUNT` 为 O（RoofTile=0x2000，Brick=0x10000，Sparkle=0x300，各类型不同），且应满足
/// `BUFFER_LEN == CHUNK_COUNT * 32`（O：Brick 影子数组 0x200000 = 0x10000*32）。
///
/// 影子数组用 `vec![T::default(); CHUNK_COUNT*32]` 填充（O：`new_storage_buffer<BrickTransformSsbo>`
/// 0x1412e5800 的元素初值循环写 `deformation=0、surface=[-100;3]、flags=0`，即 `BrickTransformSsbo::default()`
/// 的 0x60 B 模式；PDB inline 记录含 `alloc::vec::from_elem`/`Vec::extend_with` → `vec![elem; n]`）。
/// 因此实现者必须提供正确的 `Default`——它不是“零值”的同义词。
///
/// # Safety
/// 元素不得包含填充或未初始化字节，所有字段必须可按字节上传；容量须等于 chunk 数乘 32。
pub unsafe trait SparseSsbo: StorageElement + Default {
    const CHUNK_COUNT: usize;
}

/// O — chunk 编号以 u32 存放。
pub type ChunkId = u32;

/// `(first_instance, instance_count)`（O：8 B 一项，由 `DrawBuilder::draw_multi_instanced` 消费）。
pub type DrawRange = (u32, u32);

/// 一次分配拥有的 chunk 表（O：0x20 B；`Vec` @0、`RawMutex` @0x18）。
#[derive(Debug, Default)]
pub struct Allocation {
    pub chunk_ids: Mutex<Vec<ChunkId>>,
}

/// O — `AllocationHandle::is_null` 0x140985f30：首个 qword 为 0。即 `Option<Box<Allocation>>`。
#[derive(Debug, Default)]
pub struct AllocationHandle(pub Option<Box<Allocation>>);

impl AllocationHandle {
    pub fn is_null(&self) -> bool {
        self.0.is_none()
    }
}

/// 释放时把句柄还给缓冲的 `waste` 列表（O：`drop_in_place` 0x1405de840）。
#[derive(Debug)]
pub struct SparseInstanceBufferAllocation<T> {
    /// +0x00。`Weak::new()` 的哨兵（-1）在 `upgrade` 时直接失败。
    waste: Weak<Mutex<Vec<AllocationHandle>>>,
    /// +0x08。
    pub handle: AllocationHandle,
    _marker: PhantomData<fn() -> T>,
}

/// A non-owning copy (render-thread snapshots): it names the same chunks but
/// never returns them to the waste queue.
impl<T> Clone for SparseInstanceBufferAllocation<T> {
    fn clone(&self) -> Self {
        let handle = AllocationHandle(self.handle.0.as_ref().map(|allocation| {
            Box::new(Allocation { chunk_ids: Mutex::new(allocation.chunk_ids.lock().clone()) })
        }));
        Self { waste: Weak::new(), handle, _marker: PhantomData }
    }
}

impl<T> Drop for SparseInstanceBufferAllocation<T> {
    fn drop(&mut self) {
        if self.handle.is_null() {
            return;
        }
        if let Some(waste) = self.waste.upgrade() {
            waste.lock().push(core::mem::take(&mut self.handle));
        }
    }
}

impl<T> SparseInstanceBufferAllocation<T> {
    /// 句柄内的 chunk 表。O：`hide/show/allocation_to_draw` 对 `+8` 为空做 `Option::unwrap`。
    fn chunk_ids(&self) -> &Allocation {
        self.handle.0.as_ref().unwrap()
    }
}

/// 类型擦除的“分离分配” = 原生 `MagicSupports` 的 `+0x20/+0x28`（第一写面）、
/// `+0x30/+0x38`（第二写面）以及 `StairsEdgeVisualState` 每边两个槽的存储形态。
///
/// fresh 0x140aa8210（`MagicSupports::clear`）逐指令：
///  * 槽为空 → `Arc::downgrade(*(buffer+0x60))`（= `SparseInstanceBuffer::waste` 的 Weak，
///    原子增 `[ArcInner+8]`）存 `+0x20`；另 `__rust_alloc(0x20,8)` 得新块
///    `{0,4,0,0}`（Vec 空 = cap 0 / dangling ptr 4 / len 0，RawMutex @+0x18）存 `+0x28`；
///  * 非空槽直接 `lock [alloc+0x18]`；用 `buffer` 的 `+0x20..+0x58`（byte_map/active/data/
///    dirty 的 ptr+len）与分配指针拼出 `SparseInstanceBufferWriter<BrickTransformSsbo>`，
///    调 `clear<BrickTransformSsbo>`。元素类型不进入分配表——故此处以 `Allocation` 擦除。
#[derive(Debug, Default)]
pub struct MeshAtlasAllocation {
    /// 原生 `+0x20`：`Arc::downgrade(&buffer.waste)`。
    pub waste: Weak<Mutex<Vec<AllocationHandle>>>,
    /// 原生 `+0x28`：0x20 B `Allocation`（`Mutex<Vec<ChunkId>>`）。`Option` 仅为 Drop 搬出。
    allocation: Option<Box<Allocation>>,
}

impl MeshAtlasAllocation {
    /// 原生懒初始化：`Arc::downgrade(buffer.waste)` + `Box::new(Allocation::default())`。
    pub fn new<T: SparseSsbo>(buffer: &SparseInstanceBuffer<T>) -> Self {
        Self {
            waste: Arc::downgrade(&buffer.waste),
            allocation: Some(Box::new(Allocation::default())),
        }
    }

    /// 供 `SparseInstanceBuffer::writer_for` 使用（原生直接解引用 `+0x28`）。
    pub fn allocation(&self) -> &Allocation {
        self.allocation
            .as_deref()
            .expect("separated allocation handle is null")
    }
}

/// O：`SparseInstanceBufferAllocation::drop` 0x1405de840 的同一回收语义——句柄非空且
/// `Weak::upgrade` 成功 → 把 `Box<Allocation>` 推进 `waste` 列表。
impl Drop for MeshAtlasAllocation {
    fn drop(&mut self) {
        let Some(allocation) = self.allocation.take() else {
            return;
        };
        if let Some(waste) = self.waste.upgrade() {
            waste.lock().push(AllocationHandle(Some(allocation)));
        }
    }
}

/// 独占设备分配；句柄副本只用于查询，不延长资源寿命。
/// 宿主负责让尚在执行的 GPU 命令持有底层资源，释放时无需阻塞等待设备。
struct OwnedSparseBuffer {
    device: Arc<dyn GraphicsDevice + Send + Sync>,
    buffer: rhapsody_boundary::Buffer,
}

impl Drop for OwnedSparseBuffer {
    fn drop(&mut self) {
        self.device.release_buffer(&self.buffer);
    }
}

/// 共享稀疏实例存储。生产构造使用 `try_new_owned` 闭合设备缓冲的生命周期。
pub struct SparseInstanceBuffer<T: SparseSsbo> {
    /// +0x00：空闲 chunk 栈，初始降序 `[n-1 ..= 0]`（O：`psubd` 填充循环），栈顶 = 最小编号。
    pub free_chunks: RwLock<Vec<ChunkId>>,
    /// +0x20：每 chunk 的已写入数量；长度 n+1，`[n] = 0x20` 为哨兵（“永远是满的”）。
    pub byte_map: Box<[u8]>,
    /// +0x30：active 位图，n/64 个字。
    pub active_bitset: Box<[AtomicUsize]>,
    /// CPU 影子，按 `T::default()` 初始化；仅上传 dirty chunk。
    pub data: Box<[T]>,
    /// +0x50：dirty 位图。
    pub dirty_bitset: Box<[AtomicUsize]>,
    /// +0x60：待回收的句柄。
    pub waste: Arc<Mutex<Vec<AllocationHandle>>>,
    /// +0x68。
    pub buffer: TypedStorageBuffer<T>,
    // 保留创建宿主及原始代际句柄，避免依赖 ECS 资源的析构顺序。
    _buffer_owner: Option<Arc<OwnedSparseBuffer>>,
}

fn zeroed_bitset(words: usize) -> Box<[AtomicUsize]> {
    (0..words).map(|_| AtomicUsize::new(0)).collect()
}

impl<T: SparseSsbo> SparseInstanceBuffer<T> {
    /// O — `impl$10::new_storage_buffer<T>`。第二参数必须为 `None`（`assertion failed: data.is_none()`）。
    pub fn new_storage_buffer(device: &dyn GraphicsDevice, data: Option<&[u8]>) -> Self {
        assert!(data.is_none(), "assertion failed: data.is_none()");
        Self::try_new(device).expect("创建稀疏实例缓冲失败")
    }

    /// 借用设备的兼容入口；调用方负责最终释放设备缓冲。
    /// 生产资源应使用 `try_new_owned`，让存储自动归还宿主句柄。
    pub fn try_new(device: &dyn GraphicsDevice) -> Result<Self, rhapsody_boundary::BackendError> {
        Self::try_new_with_owner(device, None)
    }

    /// 独占设备缓冲，销毁存储即归还宿主；创建失败不产生释放操作。
    /// 未使用的 GPU 槽不初始化，也不会进入绘制区间。
    pub fn try_new_owned(
        device: Arc<dyn GraphicsDevice + Send + Sync>,
    ) -> Result<Self, rhapsody_boundary::BackendError> {
        Self::try_new_with_owner(device.as_ref(), Some(device.clone()))
    }

    fn try_new_with_owner(
        device: &dyn GraphicsDevice,
        owner: Option<Arc<dyn GraphicsDevice + Send + Sync>>,
    ) -> Result<Self, rhapsody_boundary::BackendError> {
        let chunks = T::CHUNK_COUNT;
        if chunks == 0
            || chunks > u32::MAX as usize / SLOTS_PER_CHUNK
            || chunks.checked_mul(SLOTS_PER_CHUNK) != Some(T::BUFFER_LEN)
        {
            return Err(rhapsody_boundary::BackendError::invalid_usage(
                "稀疏缓冲容量与 chunk 数不匹配",
            ));
        }

        let buffer = TypedStorageBuffer::try_new_uninitialized(device)?;
        // 先建立释放守卫，后续 CPU 初始化即使发生 unwind 也不会泄漏设备分配。
        let buffer_owner = owner.map(|device| OwnedSparseBuffer {
            device,
            buffer: buffer.buffer,
        });
        let mut byte_map = vec![0u8; chunks + 1].into_boxed_slice();
        byte_map[chunks] = SLOTS_PER_CHUNK as u8;
        // O：影子按 `T::default()` 逐元素填充（`vec![elem; n]`；Brick 的默认 surface=-100）。
        let shadow: Box<[T]> = vec![T::default(); chunks * SLOTS_PER_CHUNK].into_boxed_slice();
        let free: Vec<ChunkId> = (0..chunks as u32).rev().collect();
        let active_bitset = zeroed_bitset(chunks.div_ceil(usize::BITS as usize));
        let dirty_bitset = zeroed_bitset(chunks.div_ceil(usize::BITS as usize));

        Ok(Self {
            free_chunks: RwLock::new(free),
            byte_map,
            active_bitset,
            data: shadow,
            dirty_bitset,
            waste: Arc::new(Mutex::new(Vec::new())),
            buffer,
            _buffer_owner: buffer_owner.map(Arc::new),
        })
    }

    /// O — 5 字节桩 0x140005c50：返回内嵌的 `TypedStorageBuffer`（@0x68）。
    pub fn buffer(&self) -> &TypedStorageBuffer<T> {
        &self.buffer
    }

    /// I — 原版内联（无独立符号）：新建一个空分配，`waste` 为本缓冲的弱引用。
    pub fn new_allocation(&self) -> SparseInstanceBufferAllocation<T> {
        SparseInstanceBufferAllocation {
            waste: Arc::downgrade(&self.waste),
            handle: AllocationHandle(Some(Box::new(Allocation::default()))),
            _marker: PhantomData,
        }
    }

    /// I — 原版内联：为 `allocation` 建立写入器（借用本缓冲的各表）。起始 chunk 为哨兵（视为已满，首次
    /// `push` 必先领取新 chunk）。
    pub fn writer<'a>(
        &'a mut self,
        allocation: &'a SparseInstanceBufferAllocation<T>,
    ) -> SparseInstanceBufferWriter<'a, T> {
        assert!(self.owns_allocation(allocation), "分配不属于当前稀疏缓冲");
        self.writer_for(allocation.chunk_ids())
    }

    /// Resume an existing source owner's tail chunk. Native constructor
    /// callers choose the allocation's last chunk (or the full sentinel when
    /// empty); generic `writer` retains its existing sentinel-first contract.
    /// All allocation, push, clear and batch-replace algorithms stay shared.
    pub fn append_writer<'a>(
        &'a mut self,
        allocation: &'a SparseInstanceBufferAllocation<T>,
    ) -> SparseInstanceBufferWriter<'a, T> {
        let mut writer = self.writer(allocation);
        if let Some(&chunk) = writer.allocation_chunks.last() {
            writer.current_chunk = chunk;
        }
        writer
    }

    /// 防止相同元素类型的两个缓冲误用彼此的 chunk 编号。
    pub fn owns_allocation(&self, allocation: &SparseInstanceBufferAllocation<T>) -> bool {
        Weak::ptr_eq(&allocation.waste, &Arc::downgrade(&self.waste))
    }

    /// 原生 `SparseInstanceBufferWriter` 的构造面：`MagicSupports::clear`
    /// （0x140aa8210）与 `StairsVisualState::reset_edge`（0x140aa8a60）都**不**持有
    /// `SparseInstanceBufferAllocation<T>`，而是直接从资源里的裸 `Allocation`
    /// （`Mutex<Vec<ChunkId>>`，块 `{0,4,0,0}` 语义 = 空 chunk 表 + RawMutex @+0x18）
    /// 拼出 writer：byte_map/active/data/free/dirty 取自缓冲、`+0x20` 直接存分配指针。
    /// 起始 chunk = 哨兵（`byte_map.len()-1`），与 `clear` 入口写回的值一致。
    pub fn writer_for<'a>(
        &'a mut self,
        allocation: &'a Allocation,
    ) -> SparseInstanceBufferWriter<'a, T> {
        let chunks = allocation.chunk_ids.lock();
        self.writer_for_locked(chunks)
    }

    /// 原楼梯 reset 的写域适配：只清理并持有分配锁，缓冲借用在返回前结束。
    /// 调用方可继续清理同缓冲的另一分配，再将原 guard 移入写入器；不重新加锁。
    pub fn clear_and_lock_allocation<'alloc>(
        &mut self,
        allocation: &'alloc Allocation,
    ) -> MutexGuard<'alloc, Vec<ChunkId>> {
        let mut chunks = allocation.chunk_ids.lock();
        let mut current_chunk = 0;
        clear_parts(
            &mut current_chunk,
            &mut self.byte_map,
            &self.active_bitset,
            &mut chunks,
            &self.free_chunks,
        );
        chunks
    }

    /// 消费已持有的同分配 guard；组装原写入器，不重锁、不清理或分配。
    /// chunk 哨兵的初始化算术与 writer_for 原实现一致。
    pub fn writer_for_locked<'a>(
        &'a mut self,
        chunks: MutexGuard<'a, Vec<ChunkId>>,
    ) -> SparseInstanceBufferWriter<'a, T> {
        let Self {
            free_chunks,
            byte_map,
            active_bitset,
            data,
            dirty_bitset,
            ..
        } = self;
        SparseInstanceBufferWriter {
            current_chunk: (byte_map.len() - 1) as u32,
            byte_map,
            active: active_bitset,
            allocation_chunks: chunks,
            data,
            free_chunks,
            dirty: dirty_bitset,
        }
    }

    /// O — `hide` 0x1412e3c00：清掉该分配所有 chunk 的 active 位。
    pub fn hide(&self, allocation: &SparseInstanceBufferAllocation<T>) {
        for &id in chunk_list(allocation.chunk_ids()).iter() {
            self.active_bitset[id as usize >> 6]
                .fetch_and(!(1usize << (id & 63)), Ordering::Relaxed);
        }
    }

    /// O — `show` 0x1412e3c80：置上该分配所有 chunk 的 active 位。
    pub fn show(&self, allocation: &SparseInstanceBufferAllocation<T>) {
        for &id in chunk_list(allocation.chunk_ids()).iter() {
            self.active_bitset[id as usize >> 6].fetch_or(1usize << (id & 63), Ordering::Relaxed);
        }
    }

    /// O — `take_out_waste` 0x1412e3d00：回收 `waste` 里的分配。
    /// 对每个句柄：`byte_map[id]=0`、清 active 位；随后把 chunk 表**逆序**压回空闲栈；`Box<Allocation>` 释放。
    /// 句柄为空 → `Option::unwrap` panic。`waste` 的 Mutex 整个循环持有，空闲栈每个分配各加一次写锁。
    pub fn take_out_waste(&mut self) {
        let waste = Arc::clone(&self.waste);
        let mut waste = waste.lock();
        for handle in waste.drain(..) {
            let allocation = handle.0.as_ref().unwrap();
            let ids = chunk_list(allocation);
            for &id in ids.iter() {
                self.byte_map[id as usize] = 0;
                bitmap_unset(&self.active_bitset, id as usize);
            }
            self.free_chunks.write().extend(ids.iter().rev().copied());
        }
    }

    /// 将脏 chunk 合并后批量上传；失败保留 dirty 标记，下一帧可重试。
    pub fn try_gpu_flush(
        &mut self,
        device: &dyn GraphicsDevice,
    ) -> Result<(), rhapsody_boundary::BackendError> {
        self.take_out_waste();
        let element_size = core::mem::size_of::<T>();
        let mut byte_ranges: Vec<core::ops::Range<usize>> = Vec::new();
        for (word_index, word) in self.dirty_bitset.iter().enumerate() {
            let mut dirty_chunks = word.load(Ordering::Relaxed);
            while dirty_chunks != 0 {
                let chunk_index =
                    word_index * usize::BITS as usize + dirty_chunks.trailing_zeros() as usize;
                let start = chunk_index * SLOTS_PER_CHUNK * element_size;
                let end = start + SLOTS_PER_CHUNK * element_size;
                if let Some(previous) = byte_ranges.last_mut().filter(|range| range.end == start) {
                    previous.end = end;
                } else {
                    byte_ranges.push(start..end);
                }
                dirty_chunks &= dirty_chunks - 1;
            }
        }
        if byte_ranges.is_empty() {
            return Ok(());
        }
        // SparseSsbo 的安全合同保证元素没有填充和未初始化字节。
        device.update_buffer_bytes(
            &self.buffer.buffer,
            rhapsody_boundary::slice_as_bytes(&self.data),
            &byte_ranges,
            0,
        )?;
        for word in &mut self.dirty_bitset {
            *word.get_mut() = 0;
        }
        Ok(())
    }

    /// 兼容旧调用方；需要恢复失败的运行期代码应使用 try_gpu_flush。
    pub fn gpu_flush(&mut self, device: &dyn GraphicsDevice) {
        self.try_gpu_flush(device).expect("上传稀疏实例缓冲失败");
    }

    /// O — `allocation_to_draw_multi_instanced` 0x1412e3a60：按该分配的 chunk 顺序合并绘制区间。
    pub fn allocation_to_draw_multi_instanced(
        &self,
        allocation: &SparseInstanceBufferAllocation<T>,
    ) -> Vec<DrawRange> {
        merge_draw_runs(
            chunk_list(allocation.chunk_ids())
                .iter()
                .map(|&id| id as usize),
            &self.byte_map,
        )
    }

    /// O — `impl$3::to_draw_multi_instanced` 0x1412e4250：对整张 active 位图做同样的合并。
    pub fn to_draw_multi_instanced(&self) -> Vec<DrawRange> {
        let chunks = self
            .active_bitset
            .iter()
            .enumerate()
            .flat_map(|(word_index, word)| {
                let mut active_chunks = word.load(Ordering::Relaxed);
                std::iter::from_fn(move || {
                    if active_chunks == 0 {
                        return None;
                    }
                    let chunk_index =
                        word_index * usize::BITS as usize + active_chunks.trailing_zeros() as usize;
                    active_chunks &= active_chunks - 1;
                    Some(chunk_index)
                })
            });
        merge_draw_runs(chunks, &self.byte_map)
    }
}

/// 通过库提供的锁守卫读取分配表，避免共享引用下读取可变 Vec。
fn chunk_list(allocation: &Allocation) -> MutexGuard<'_, Vec<ChunkId>> {
    allocation.chunk_ids.lock()
}

/// 合并连续 chunk：`prev_end == start` 则延伸，否则收尾上一段（长度为 0 不输出）并开新段；
/// `prev_end = start + byte_map[id]`，所以未写满的 chunk 会截断 run。`byte_map[id]` 越界 → bounds panic。
fn merge_draw_runs(chunks: impl Iterator<Item = usize>, byte_map: &[u8]) -> Vec<DrawRange> {
    let mut runs = Vec::new();
    let (mut run_start, mut prev_end) = (0u32, 0u32);
    for id in chunks {
        let count = byte_map[id] as u32;
        let start = (id as u32) << 5;
        if prev_end != start {
            if prev_end > run_start {
                runs.push((run_start, prev_end - run_start));
            }
            run_start = start;
        }
        prev_end = start + count;
    }
    if prev_end > run_start {
        runs.push((run_start, prev_end - run_start));
    }
    runs
}

/// `push` 失败：空闲 chunk 栈为空（O：返回 bool，true = 失败）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NoFreeChunks;

/// `SparseInstanceBufferWriter<T>` — 0x58 B（O）。全部是借用，持有该分配 chunk 表的 `MutexGuard`（Drop 即解锁）。
pub struct SparseInstanceBufferWriter<'a, T: SparseSsbo> {
    /// +0x00。
    byte_map: &'a mut [u8],
    /// +0x10。
    active: &'a [AtomicUsize],
    /// +0x20。
    allocation_chunks: MutexGuard<'a, Vec<ChunkId>>,
    /// +0x28：CPU 影子。
    data: &'a mut [T],
    /// +0x38。
    free_chunks: &'a RwLock<Vec<ChunkId>>,
    /// +0x40。
    dirty: &'a [AtomicUsize],
    /// +0x50。
    current_chunk: u32,
}

impl<T: SparseSsbo> SparseInstanceBufferWriter<'_, T> {
    /// 整批替换。容量不足时不修改原分配；成功时复用旧 chunk，只领取增长部分。
    pub fn replace_all(&mut self, instances: &[T]) -> Result<(), NoFreeChunks> {
        let required_chunks = instances.len().div_ceil(SLOTS_PER_CHUNK);
        let mut free_chunks = self.free_chunks.write();
        if required_chunks > self.allocation_chunks.len() + free_chunks.len() {
            return Err(NoFreeChunks);
        }
        if required_chunks > self.allocation_chunks.len() {
            let additional = required_chunks - self.allocation_chunks.len();
            self.allocation_chunks.reserve(additional);
            while self.allocation_chunks.len() < required_chunks {
                self.allocation_chunks
                    .push(free_chunks.pop().expect("已检查空闲 chunk 数"));
            }
        }
        for chunk_id in self.allocation_chunks.drain(required_chunks..).rev() {
            self.byte_map[chunk_id as usize] = 0;
            bitmap_unset(self.active, chunk_id as usize);
            free_chunks.push(chunk_id);
        }
        drop(free_chunks);
        for (&chunk_id, values) in self
            .allocation_chunks
            .iter()
            .zip(instances.chunks(SLOTS_PER_CHUNK))
        {
            let chunk_index = chunk_id as usize;
            let first_slot = chunk_index * SLOTS_PER_CHUNK;
            self.data[first_slot..first_slot + values.len()].copy_from_slice(values);
            self.byte_map[chunk_index] = values.len() as u8;
            bitmap_set(self.active, chunk_index);
            bitmap_set(self.dirty, chunk_index);
        }
        self.current_chunk = self
            .allocation_chunks
            .last()
            .copied()
            .unwrap_or((self.byte_map.len() - 1) as u32);
        Ok(())
    }

    /// O — `push` 0x1412e4450。当前 chunk 未满：槽位 `chunk<<5 | count`，计数 +1，写入并置 dirty；
    /// 已满：写锁下弹出空闲栈顶（空 → `Err`），记入分配的 chunk 表，`byte_map[id]=1`，写槽 `id<<5`，
    /// 置 active + dirty，`current_chunk=id`。所有下标均带边界检查（panic 文案为 `index out of bounds`）。
    pub fn push(&mut self, value: &T) -> Result<(), NoFreeChunks> {
        let chunk = self.current_chunk as usize;
        let count = self.byte_map[chunk] as usize;
        if count < SLOTS_PER_CHUNK {
            let slot = chunk << 5 | count;
            self.byte_map[chunk] = (count + 1) as u8;
            self.data[slot] = *value;
            self.dirty[chunk >> 6].fetch_or(1usize << (chunk & 63), Ordering::Relaxed);
            return Ok(());
        }
        let mut free = self.free_chunks.write();
        let Some(id) = free.pop() else {
            return Err(NoFreeChunks);
        };
        let chunk = id as usize;
        self.allocation_chunks.push(id);
        self.byte_map[chunk] = 1;
        self.data[chunk << 5] = *value;
        self.active[chunk >> 6].fetch_or(1usize << (chunk & 63), Ordering::Relaxed);
        self.dirty[chunk >> 6].fetch_or(1usize << (chunk & 63), Ordering::Relaxed);
        self.current_chunk = id;
        Ok(())
    }

    /// O — `clear` 0x1412e4760：当前 chunk 复位到哨兵（`len-1`，必然“已满”，下次 `push` 领新 chunk）；
    /// 该分配的每个 chunk：`byte_map=0`、清 active 位；随后把 chunk 表**逆序**压回空闲栈并清空。
    /// dirty 位不清——之后的 `gpu_flush` 仍会上传这些 chunk。`byte_map` 为空 → `Option::unwrap` panic。
    pub fn clear(&mut self) {
        clear_parts(
            &mut self.current_chunk,
            self.byte_map,
            self.active,
            &mut self.allocation_chunks,
            self.free_chunks,
        );
    }
}

/// 两个公开清理入口共用原业务体，保留哨兵、逐块清理与逆序归还的副作用顺序。
fn clear_parts(
    current_chunk: &mut u32,
    byte_map: &mut [u8],
    active: &[AtomicUsize],
    allocation_chunks: &mut Vec<ChunkId>,
    free_chunks: &RwLock<Vec<ChunkId>>,
) {
    *current_chunk = byte_map.len().checked_sub(1).unwrap() as u32;
    for &id in allocation_chunks.iter() {
        byte_map[id as usize] = 0;
        bitmap_unset(active, id as usize);
    }
    let mut free = free_chunks.write();
    free.extend(allocation_chunks.drain(..).rev());
}

impl<T: SparseSsbo + Send + Sync + 'static> Ssbo for SparseInstanceBuffer<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn buffer(&self) -> &rhapsody_boundary::Buffer {
        &self.buffer.buffer
    }
    fn handle(&self) -> ResourceHandle {
        ResourceHandle(self.buffer.buffer.id, self.buffer.buffer.generation)
    }
    /// The render copy shares the GPU buffer owner and the waste queue, and
    /// copies the chunk maps and CPU shadow as of the hand-over.
    fn snapshot(&self) -> Box<dyn Ssbo> {
        let atomics = |bits: &[AtomicUsize]| -> Box<[AtomicUsize]> {
            bits.iter().map(|bit| AtomicUsize::new(bit.load(Ordering::Relaxed))).collect()
        };
        Box::new(SparseInstanceBuffer::<T> {
            free_chunks: RwLock::new(self.free_chunks.read().clone()),
            byte_map: self.byte_map.clone(),
            active_bitset: atomics(&self.active_bitset),
            data: self.data.clone(),
            dirty_bitset: atomics(&self.dirty_bitset),
            waste: self.waste.clone(),
            buffer: self.buffer,
            _buffer_owner: self._buffer_owner.clone(),
        })
    }
}

/// 原子置位（`lock or`）。
pub fn bitmap_set(bits: &[AtomicUsize], index: usize) {
    bits[index >> 6].fetch_or(1usize << (index & 63), Ordering::Relaxed);
}

/// 原子清位（`lock and`，掩码 `rol(-2, cl)`）。
pub fn bitmap_unset(bits: &[AtomicUsize], index: usize) {
    bits[index >> 6].fetch_and(!(1usize << (index & 63)), Ordering::Relaxed);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::ssbo::rhapsody_boundary::{Buffer, BufferDesc};
    use std::cell::RefCell;
    use std::ops::Range;

    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    struct Elem(u32);
    impl StorageElement for Elem {
        const NAME: &'static str = "Elem";
        const BUFFER_LEN: usize = 128 * SLOTS_PER_CHUNK;
    }
    unsafe impl SparseSsbo for Elem {
        const CHUNK_COUNT: usize = 128;
    }

    /// 非零默认值类型：验证影子不是 `zeroed::<T>()`，而取 `T::default()`。
    #[derive(Clone, Copy, Debug, PartialEq)]
    struct NonZero(u32, f32);
    impl Default for NonZero {
        fn default() -> Self {
            Self(7, -100.0)
        }
    }
    impl StorageElement for NonZero {
        const NAME: &'static str = "NonZero";
        const BUFFER_LEN: usize = 2 * SLOTS_PER_CHUNK;
    }
    unsafe impl SparseSsbo for NonZero {
        const CHUNK_COUNT: usize = 2;
    }

    #[derive(Default)]
    struct Rec {
        writes: RefCell<Vec<Vec<Range<usize>>>>,
    }
    impl GraphicsDevice for Rec {
        fn create_buffer(
            &self,
            desc: &BufferDesc,
            _n: &str,
            _d: Option<&[u8]>,
        ) -> Result<Buffer, rhapsody_boundary::BackendError> {
            Ok(Buffer {
                byte_size: desc.size as u64,
                ..Buffer::default()
            })
        }
        fn update_buffer_bytes(
            &self,
            _b: &Buffer,
            _d: &[u8],
            r: &[Range<usize>],
            _o: usize,
        ) -> Result<(), rhapsody_boundary::BackendError> {
            self.writes.borrow_mut().push(r.to_vec());
            Ok(())
        }
    }

    fn new_buf() -> (Rec, SparseInstanceBuffer<Elem>) {
        let dev = Rec::default();
        let b = SparseInstanceBuffer::<Elem>::new_storage_buffer(&dev, None);
        (dev, b)
    }

    #[test]
    fn held_allocation_lock_allows_same_buffer_magic_clear_before_writer_reassembly() {
        let (_device, mut buffer) = new_buf();
        let state = MeshAtlasAllocation::new(&buffer);
        let magic = MeshAtlasAllocation::new(&buffer);
        buffer.writer_for(state.allocation()).push(&Elem(11)).unwrap();
        buffer.writer_for(magic.allocation()).push(&Elem(22)).unwrap();
        let state_chunk = state.allocation().chunk_ids.lock()[0];
        let magic_chunk = magic.allocation().chunk_ids.lock()[0];
        assert_ne!(state_chunk, magic_chunk);

        // guard 仅借分配，同 buffer 的后续清理不制造重叠可变引用。
        let state_lock = buffer.clear_and_lock_allocation(state.allocation());
        assert!(state.allocation().chunk_ids.try_lock().is_none());
        let magic_lock = buffer.clear_and_lock_allocation(magic.allocation());
        assert!(state_lock.is_empty());
        assert!(magic_lock.is_empty());
        assert!(magic.allocation().chunk_ids.try_lock().is_none());
        assert_eq!(buffer.byte_map[state_chunk as usize], 0);
        assert_eq!(buffer.byte_map[magic_chunk as usize], 0);
        drop(magic_lock);
        assert!(state.allocation().chunk_ids.try_lock().is_none());

        let mut writer = buffer.writer_for_locked(state_lock);
        assert_eq!(writer.current_chunk, Elem::CHUNK_COUNT as u32);
        writer.push(&Elem(99)).unwrap();
        // 原哨兵领新 chunk；Magic 刚归还的尾块可由同 free 栈重新领取。
        assert_eq!(&*writer.allocation_chunks, &[magic_chunk]);
        drop(writer);
        assert!(state.allocation().chunk_ids.try_lock().is_some());
        assert_eq!(buffer.data[(magic_chunk as usize) << 5], Elem(99));
        assert_eq!(buffer.byte_map[state_chunk as usize], 0);
        assert_eq!(buffer.byte_map[magic_chunk as usize], 1);
        assert_eq!(buffer.active_bitset[0].load(Ordering::Relaxed) & (1 << magic_chunk), 1 << magic_chunk);
        assert_ne!(buffer.dirty_bitset[0].load(Ordering::Relaxed) & (1 << state_chunk), 0);
    }

    #[test]
    fn locked_clear_and_existing_writer_clear_share_multi_chunk_order_and_dirty_state() {
        let (_first_device, mut first) = new_buf();
        let (_second_device, mut second) = new_buf();
        let first_slot = MeshAtlasAllocation::new(&first);
        let second_slot = MeshAtlasAllocation::new(&second);
        {
            let mut first_writer = first.writer_for(first_slot.allocation());
            let mut second_writer = second.writer_for(second_slot.allocation());
            for value in 0..70 {
                first_writer.push(&Elem(value)).unwrap();
                second_writer.push(&Elem(value)).unwrap();
            }
        }
        let second_capacity = second_slot.allocation().chunk_ids.lock().capacity();
        first.writer_for(first_slot.allocation()).clear();
        let second_guard = second.clear_and_lock_allocation(second_slot.allocation());
        assert!(second_guard.is_empty());
        assert_eq!(second_guard.capacity(), second_capacity);
        assert_eq!(first.byte_map, second.byte_map);
        assert_eq!(*first.free_chunks.read(), *second.free_chunks.read());
        assert_eq!(first.active_bitset.iter().map(|v| v.load(Ordering::Relaxed)).collect::<Vec<_>>(),
            second.active_bitset.iter().map(|v| v.load(Ordering::Relaxed)).collect::<Vec<_>>());
        assert_eq!(first.dirty_bitset.iter().map(|v| v.load(Ordering::Relaxed)).collect::<Vec<_>>(),
            second.dirty_bitset.iter().map(|v| v.load(Ordering::Relaxed)).collect::<Vec<_>>());
        assert_ne!(second.dirty_bitset[0].load(Ordering::Relaxed), 0);
        drop(second_guard);
    }

    #[test]
    fn initial_state_matches_binary() {
        let (_d, b) = new_buf();
        assert_eq!(b.byte_map.len(), 129);
        assert_eq!(b.byte_map[128], 0x20);
        assert!(b.byte_map[..128].iter().all(|&x| x == 0));
        assert_eq!(b.active_bitset.len(), 2);
        assert_eq!(b.dirty_bitset.len(), 2);
        assert_eq!(b.data.len(), 128 * 32);
        let free = b.free_chunks.read();
        assert_eq!(free.len(), 128);
        assert_eq!((free[0], free[127]), (127, 0)); // 降序，栈顶 = 0
    }

    #[test]
    fn shadow_is_filled_with_default_values_not_zeroes() {
        // O：`new_storage_buffer<BrickTransformSsbo>` 逐元素写 Default 模式（surface=-100），
        // 不是 memset 0 —— 因此影子必须是 `T::default()`。
        let dev = Rec::default();
        let b = SparseInstanceBuffer::<NonZero>::new_storage_buffer(&dev, None);
        assert_eq!(b.data.len(), 2 * SLOTS_PER_CHUNK);
        assert!(b.data.iter().all(|&v| v == NonZero::default()));
        assert_eq!(b.data[0], NonZero(7, -100.0));
    }

    #[test]
    #[should_panic(expected = "assertion failed: data.is_none()")]
    fn initial_data_is_rejected() {
        let dev = Rec::default();
        SparseInstanceBuffer::<Elem>::new_storage_buffer(&dev, Some(&[0u8]));
    }

    #[test]
    fn push_claims_chunks_in_order_and_fills_32_per_chunk() {
        let (_d, mut b) = new_buf();
        let a = b.new_allocation();
        {
            let mut w = b.writer(&a);
            for i in 0..33 {
                w.push(&Elem(i)).unwrap();
            }
            assert_eq!(w.allocation_chunks.as_slice(), &[0, 1]);
        }
        assert_eq!(b.byte_map[0], 32);
        assert_eq!(b.byte_map[1], 1);
        assert_eq!(b.data[0], Elem(0));
        assert_eq!(b.data[31], Elem(31));
        assert_eq!(b.data[32], Elem(32));
        assert_eq!(b.active_bitset[0].load(Ordering::Relaxed), 0b11);
        assert_eq!(b.dirty_bitset[0].load(Ordering::Relaxed), 0b11);
    }

    #[test]
    fn push_reports_exhaustion_with_err() {
        let (_d, mut b) = new_buf();
        b.free_chunks.write().clear();
        let a = b.new_allocation();
        let mut w = b.writer(&a);
        assert_eq!(w.push(&Elem(1)), Err(NoFreeChunks));
    }

    #[test]
    fn writer_clear_returns_chunks_in_reverse_and_keeps_dirty() {
        let (_d, mut b) = new_buf();
        let a = b.new_allocation();
        {
            let mut w = b.writer(&a);
            for i in 0..40 {
                w.push(&Elem(i)).unwrap();
            }
            w.clear();
            assert!(w.allocation_chunks.is_empty());
        }
        assert_eq!(b.byte_map[0], 0);
        assert_eq!(b.byte_map[1], 0);
        assert_eq!(b.active_bitset[0].load(Ordering::Relaxed), 0);
        assert_eq!(b.dirty_bitset[0].load(Ordering::Relaxed), 0b11); // 不清 dirty
        let free = b.free_chunks.read();
        // 弹出时是 [0,1]，逆序压回 → 先 1 后 0，栈顶 = 0。
        assert_eq!(&free[free.len() - 2..], &[1, 0]);
    }

    #[test]
    fn dropping_allocation_feeds_waste_and_take_out_waste_reclaims() {
        let (_d, mut b) = new_buf();
        let a = b.new_allocation();
        {
            let mut w = b.writer(&a);
            for i in 0..33 {
                w.push(&Elem(i)).unwrap();
            }
        }
        drop(a);
        assert_eq!(b.waste.lock().len(), 1);
        b.take_out_waste();
        assert!(b.waste.lock().is_empty());
        assert_eq!((b.byte_map[0], b.byte_map[1]), (0, 0));
        assert_eq!(b.active_bitset[0].load(Ordering::Relaxed), 0);
        let free = b.free_chunks.read();
        assert_eq!(free.len(), 128);
        assert_eq!(*free.last().unwrap(), 0); // 逆序压回后栈顶是原来的第一个 chunk
    }

    #[test]
    fn dropping_allocation_after_buffer_is_a_noop() {
        let (_d, b) = new_buf();
        let a = b.new_allocation();
        drop(b); // Weak::upgrade 失败
        drop(a);
    }

    #[test]
    fn hide_and_show_toggle_active_bits() {
        let (_d, mut b) = new_buf();
        let a = b.new_allocation();
        {
            let mut w = b.writer(&a);
            for i in 0..33 {
                w.push(&Elem(i)).unwrap();
            }
        }
        b.hide(&a);
        assert_eq!(b.active_bitset[0].load(Ordering::Relaxed), 0);
        b.show(&a);
        assert_eq!(b.active_bitset[0].load(Ordering::Relaxed), 0b11);
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn hide_with_null_handle_panics() {
        let (_d, b) = new_buf();
        let mut a = b.new_allocation();
        a.handle = AllocationHandle(None);
        b.hide(&a);
    }

    #[test]
    fn draw_runs_merge_only_full_adjacent_chunks_and_skip_empty() {
        let (_d, mut b) = new_buf();
        let a = b.new_allocation();
        {
            let mut w = b.writer(&a);
            for i in 0..70 {
                w.push(&Elem(i)).unwrap(); // chunk 0:32, 1:32, 2:6
            }
        }
        // 0、1 已满且相邻 → 合并为 (0,64)；chunk 2 紧接其后（start=64==prev_end）→ 同一 run → (0,70)。
        assert_eq!(b.allocation_to_draw_multi_instanced(&a), vec![(0, 70)]);
        assert_eq!(b.to_draw_multi_instanced(), vec![(0, 70)]);
        // 未写满的 chunk 截断 run：把 chunk 1 改成 5 个。
        b.byte_map[1] = 5;
        assert_eq!(b.to_draw_multi_instanced(), vec![(0, 37), (64, 6)]);
        // 空 chunk（计数 0）产生长度 0 的 run，不输出。
        b.byte_map[2] = 0;
        assert_eq!(b.to_draw_multi_instanced(), vec![(0, 37)]);
    }

    #[test]
    fn gpu_flush_uploads_merged_dirty_runs_and_clears_them() {
        let (d, mut b) = new_buf();
        let a = b.new_allocation();
        {
            let mut w = b.writer(&a);
            for i in 0..70 {
                w.push(&Elem(i)).unwrap(); // chunk 0,1,2 dirty
            }
        }
        // 另外手工标记 chunk 5、70（跨 64 位字）为 dirty。
        bitmap_set(&b.dirty_bitset, 5);
        bitmap_set(&b.dirty_bitset, 70);
        b.gpu_flush(&d);
        // 元素区间 [0,96) [160,192) [2240,2272)，设备收到的是 ×size_of::<Elem>() = ×4 的字节区间。
        assert_eq!(*d.writes.borrow(), vec![vec![0..384, 640..768, 8960..9088]]);
        assert!(
            b.dirty_bitset
                .iter()
                .all(|w| w.load(Ordering::Relaxed) == 0)
        );
        // 无 dirty 时原版仍然调用，区间为空（由 gpu_write_ranges_nonoverlapping 内部返回，不触发写）。
        b.gpu_flush(&d);
        assert_eq!(d.writes.borrow().len(), 1);
    }

    #[test]
    fn gpu_flush_reclaims_waste_before_uploading() {
        let (d, mut b) = new_buf();
        let a = b.new_allocation();
        {
            let mut w = b.writer(&a);
            w.push(&Elem(1)).unwrap();
        }
        drop(a);
        b.gpu_flush(&d);
        assert!(b.waste.lock().is_empty());
        assert_eq!(b.free_chunks.read().len(), 128);
    }

    #[test]
    fn is_null_follows_option() {
        assert!(AllocationHandle(None).is_null());
        assert!(!AllocationHandle(Some(Box::default())).is_null());
    }

    #[test]
    fn writer_for_uses_a_detached_allocation_like_magic_supports() {
        // 原生 0x140aa8210 / 0x140aa8a60 从资源里的裸 Allocation 直接拼 writer，
        // 起始 chunk = byte_map.len()-1；clear 后 chunk 逆序回空闲栈。
        let (_d, mut b) = new_buf();
        let detached = MeshAtlasAllocation::new(&b);
        {
            let mut w = b.writer_for(detached.allocation());
            assert_eq!(w.current_chunk, 128);
            w.push(&Elem(7)).unwrap();
            assert_eq!(w.allocation_chunks.as_slice(), &[0]);
            w.clear();
        }
        assert_eq!(b.byte_map[0], 0);
        assert_eq!(b.active_bitset[0].load(Ordering::Relaxed), 0);
        assert_eq!(b.free_chunks.read().len(), 128);
    }

    #[test]
    fn detached_allocation_drop_returns_chunks_to_waste() {
        let (_d, mut b) = new_buf();
        let detached = MeshAtlasAllocation::new(&b);
        {
            let mut w = b.writer_for(detached.allocation());
            w.push(&Elem(1)).unwrap();
        }
        drop(detached);
        assert_eq!(b.waste.lock().len(), 1);
        b.take_out_waste();
        assert!(b.waste.lock().is_empty());
        assert_eq!(b.free_chunks.read().len(), 128);
    }
}
