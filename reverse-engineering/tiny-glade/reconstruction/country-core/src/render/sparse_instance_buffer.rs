//! `SparseInstanceBuffer<T>` / `SparseInstanceBufferWriter<T>` / `SparseInstanceBufferAllocation<T>`（layer A2）。
//!
//! 32 个实例为一个 chunk；每个 chunk 有“已写入数量”字节表、active / dirty 两张位图；释放的分配经
//! `Arc<Mutex<Vec<AllocationHandle>>>` 回收到 `take_out_waste`。
//!
//! 证据（O，逐条核对机器码，见 `evidence/country-core-audit/L0_NOTES.md` “A2 第一批”）：
//!  * `new_storage_buffer`        0x1412e4a90（RoofTile）/ 0x1412e5800（BrickTransform）等 4 个单态化
//!  * `take_out_waste`            0x1412e3d00    * `gpu_flush`  0x1412e2f10（BrickTransform）
//!  * `hide` 0x1412e3c00 / `show` 0x1412e3c80
//!  * `allocation_to_draw_multi_instanced` 0x1412e3a60 / `impl$3::to_draw_multi_instanced` 0x1412e4250
//!  * `SparseInstanceBufferWriter::push` 0x1412e4450 / `clear` 0x1412e4760
//!  * `drop_in_place<SparseInstanceBufferAllocation>` 0x1405de840、`drop_in_place<Writer>` 0x1405dec30
//!
//! 与旧版的主要差异：
//!  1. **布局**（0x90 B）：`free_chunks: RwLock<Vec<u32>>` @0、`byte_map: Box<[u8]>` @0x20、`active: Box<[AtomicUsize]>` @0x30、
//!     **CPU 影子 `Box<[T]>`** @0x40（零初始化，长度 = chunks*32）、`dirty: Box<[AtomicUsize]>` @0x50、
//!     `waste: Arc<Mutex<Vec<AllocationHandle>>>` @0x60、`TypedStorageBuffer<T>` @0x68。旧版的
//!     `pending_resource_handles`、`chunk_count`、`slot_meta` 都不存在；`gpu_flush` 的数据源是 CPU 影子，不是“映射指针”。
//!  2. `Allocation` 是 0x20 B 的 `Mutex<Vec<ChunkId>>`（Vec @0、RawMutex 字节 @0x18），装在 `Box` 里；
//!     `AllocationHandle` 就是 `Option<Box<Allocation>>`。旧版把 Vec 的 capacity 当“待释放指针”，是错的。
//!  3. `SparseInstanceBufferAllocation<T>` = `{ waste: Weak<…> @0, handle: AllocationHandle @8 }`；`Drop`：句柄非空且
//!     `Weak::upgrade` 成功 → 锁 waste，把句柄 `take` 后 push。`hide/show` 取的就是它（`+8` 为空 → `Option::unwrap` panic）。
//!  4. `Writer`（0x58 B）全是借用：`byte_map`、`active`、`MutexGuard<Vec<ChunkId>>`（该分配的 chunk 表）、`data: &mut [T]`、
//!     `&RwLock<Vec<u32>>`、`dirty`、`current_chunk`。`push` 返回 bool（**true = 没有空闲 chunk**），按值整块拷贝 `T`。
//!  5. `allocation_to_draw_multi_instanced` / `to_draw_multi_instanced` 返回 `Vec<(first_instance, instance_count)>`
//!     （8 B 一项），长度为 0 的 run 不输出；旧版返回 `(0,4,len)` 三元组，是编的。
//!  6. `gpu_flush` 无“runs 为空就返回”：原版无条件调用 `gpu_write_ranges_nonoverlapping`（空区间在其内部返回）。
//!     puffin 作用域（函数名 + 区间数）是库行为，省略。

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

/// 稀疏缓冲的元素类型。`CHUNK_COUNT` 为 O（RoofTile=0x2000，Brick/Plank 0x800 起，各类型不同），且应满足
/// `BUFFER_LEN == CHUNK_COUNT * 32`（O：影子数组长度 0x40000 = 0x2000*32）。
///
/// # Safety
/// 影子数组用全零字节初始化（O：`new_storage_buffer` 里的 memset 循环），实现者须保证全零是合法的 `T`。
pub unsafe trait SparseSsbo: StorageElement {
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
pub struct SparseInstanceBufferAllocation<T> {
    /// +0x00。`Weak::new()` 的哨兵（-1）在 `upgrade` 时直接失败。
    waste: Weak<Mutex<Vec<AllocationHandle>>>,
    /// +0x08。
    pub handle: AllocationHandle,
    _marker: PhantomData<fn() -> T>,
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

/// `SparseInstanceBuffer<T>` — 0x90 B（O）。
pub struct SparseInstanceBuffer<T: SparseSsbo> {
    /// +0x00：空闲 chunk 栈，初始降序 `[n-1 ..= 0]`（O：`psubd` 填充循环），栈顶 = 最小编号。
    pub free_chunks: RwLock<Vec<ChunkId>>,
    /// +0x20：每 chunk 的已写入数量；长度 n+1，`[n] = 0x20` 为哨兵（“永远是满的”）。
    pub byte_map: Box<[u8]>,
    /// +0x30：active 位图，n/64 个字。
    pub active_bitset: Box<[AtomicUsize]>,
    /// +0x40：CPU 影子，n*32 个 `T`，全零。`Writer` 写它，`gpu_flush` 上传它。
    pub data: Box<[T]>,
    /// +0x50：dirty 位图。
    pub dirty_bitset: Box<[AtomicUsize]>,
    /// +0x60：待回收的句柄。
    pub waste: Arc<Mutex<Vec<AllocationHandle>>>,
    /// +0x68。
    pub buffer: TypedStorageBuffer<T>,
}

fn zeroed_bitset(words: usize) -> Box<[AtomicUsize]> {
    (0..words).map(|_| AtomicUsize::new(0)).collect()
}

impl<T: SparseSsbo> SparseInstanceBuffer<T> {
    /// O — `impl$10::new_storage_buffer<T>`。第二参数必须为 `None`（`assertion failed: data.is_none()`）。
    pub fn new_storage_buffer(device: &dyn GraphicsDevice, data: Option<&[u8]>) -> Self {
        assert!(data.is_none(), "assertion failed: data.is_none()");
        let chunks = T::CHUNK_COUNT;
        debug_assert_eq!(T::BUFFER_LEN, chunks * SLOTS_PER_CHUNK);

        let mut byte_map = vec![0u8; chunks + 1].into_boxed_slice();
        byte_map[chunks] = SLOTS_PER_CHUNK as u8;
        // SAFETY: `SparseSsbo` 的契约——全零是合法值。
        let shadow: Box<[T]> = (0..chunks * SLOTS_PER_CHUNK).map(|_| unsafe { core::mem::zeroed::<T>() }).collect();
        let free: Vec<ChunkId> = (0..chunks as u32).rev().collect();
        let buffer = TypedStorageBuffer::new(device, None);
        let active_bitset = zeroed_bitset(chunks / usize::BITS as usize);
        let dirty_bitset = zeroed_bitset(chunks / usize::BITS as usize);

        Self {
            free_chunks: RwLock::new(free),
            byte_map,
            active_bitset,
            data: shadow,
            dirty_bitset,
            waste: Arc::new(Mutex::new(Vec::new())),
            buffer,
        }
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
    pub fn writer<'a>(&'a mut self, allocation: &'a SparseInstanceBufferAllocation<T>) -> SparseInstanceBufferWriter<'a, T> {
        let chunks = allocation.chunk_ids().chunk_ids.lock();
        let Self { free_chunks, byte_map, active_bitset, data, dirty_bitset, .. } = self;
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
        for &id in chunk_list(allocation.chunk_ids()) {
            self.active_bitset[id as usize >> 6].fetch_and(!(1usize << (id & 63)), Ordering::Relaxed);
        }
    }

    /// O — `show` 0x1412e3c80：置上该分配所有 chunk 的 active 位。
    pub fn show(&self, allocation: &SparseInstanceBufferAllocation<T>) {
        for &id in chunk_list(allocation.chunk_ids()) {
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
            for &id in ids {
                self.byte_map[id as usize] = 0;
                bitmap_unset(&self.active_bitset, id as usize);
            }
            self.free_chunks.write().extend(ids.iter().rev().copied());
        }
    }

    /// O — `gpu_flush` 0x1412e2f10：先回收，再把 dirty 的 chunk 合并成连续区间上传 CPU 影子。
    /// dirty 字被读后**非原子地**清零（`&mut self`）；区间以元素为单位 `[start*32, end*32)`。
    pub fn gpu_flush(&mut self, device: &dyn GraphicsDevice) {
        self.take_out_waste();
        let mut runs: Vec<core::ops::Range<usize>> = Vec::new();
        let (mut run_start, mut run_end) = (0u32, 0u32);
        for (w, word) in self.dirty_bitset.iter_mut().enumerate() {
            let mut bits = core::mem::take(word.get_mut());
            while bits != 0 {
                let chunk = (w * 64) as u32 + bits.trailing_zeros();
                if run_end == chunk {
                    run_end = chunk + 1;
                } else {
                    if run_start < run_end {
                        runs.push((run_start as usize) << 5..(run_end as usize) << 5);
                    }
                    run_start = chunk;
                    run_end = chunk + 1;
                }
                bits &= bits - 1;
            }
        }
        if run_start < run_end {
            runs.push((run_start as usize) << 5..(run_end as usize) << 5);
        }
        self.buffer.gpu_write_ranges_nonoverlapping(device, &self.data, &runs, 0);
    }

    /// O — `allocation_to_draw_multi_instanced` 0x1412e3a60：按该分配的 chunk 顺序合并绘制区间。
    pub fn allocation_to_draw_multi_instanced(&self, allocation: &SparseInstanceBufferAllocation<T>) -> Vec<DrawRange> {
        merge_draw_runs(chunk_list(allocation.chunk_ids()).iter().map(|&id| id as usize), &self.byte_map)
    }

    /// O — `impl$3::to_draw_multi_instanced` 0x1412e4250：对整张 active 位图做同样的合并。
    pub fn to_draw_multi_instanced(&self) -> Vec<DrawRange> {
        let mut chunks = Vec::new();
        for (w, word) in self.active_bitset.iter().enumerate() {
            let mut bits = word.load(Ordering::Relaxed);
            while bits != 0 {
                chunks.push(w * 64 + bits.trailing_zeros() as usize);
                bits &= bits - 1;
            }
        }
        merge_draw_runs(chunks.into_iter(), &self.byte_map)
    }
}

/// 不加锁读取 chunk 表（O：`hide/show/take_out_waste` 都直接读 `[alloc+8]/[alloc+0x10]`，无 RawMutex 操作）。
fn chunk_list(allocation: &Allocation) -> &[ChunkId] {
    // SAFETY: 调用方持有对分配的共享/独占引用；与原版一样不取锁。写入器存活期间不应同时调用。
    unsafe { &*allocation.chunk_ids.data_ptr() }
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
        self.current_chunk = self.byte_map.len().checked_sub(1).unwrap() as u32;
        for &id in self.allocation_chunks.iter() {
            self.byte_map[id as usize] = 0;
            bitmap_unset(self.active, id as usize);
        }
        let mut free = self.free_chunks.write();
        free.extend(self.allocation_chunks.drain(..).rev());
    }
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
    fn handle(&self) -> &ResourceHandle {
        // O — 5 字节桩 0x140005c50 返回内嵌缓冲（@0x68）的句柄；`Buffer` 首部即句柄字，类型转换 I。
        unsafe { &*(&self.buffer.buffer as *const rhapsody_boundary::Buffer as *const ResourceHandle) }
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

    #[derive(Clone, Copy, Debug, PartialEq)]
    struct Elem(u32);
    impl StorageElement for Elem {
        const NAME: &'static str = "Elem";
        const BUFFER_LEN: usize = 128 * SLOTS_PER_CHUNK;
    }
    unsafe impl SparseSsbo for Elem {
        const CHUNK_COUNT: usize = 128;
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
            Ok(Buffer { byte_size: desc.size as u64, ..Buffer::default() })
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
        assert!(b.dirty_bitset.iter().all(|w| w.load(Ordering::Relaxed) == 0));
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
}
