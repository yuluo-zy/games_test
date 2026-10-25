//! `ShadowedBuffer<T>` — CPU 影子数组 + 脏区间上传（layer A1）。
//!
//!  * `ShadowedBuffer::new<T>`      0x14168f870
//!  * `ShadowedBuffer::gpu_flush<T>` 0x141691b30（22 个 / 5 个单态化，仅元素大小不同）
//! 布局（O）：`dirty` @0x00（start,end 各一个 usize）、影子数组 `Box<[T]>`(ptr@0x10,len@0x18)、
//! `TypedStorageBuffer<T>` @0x20（其 `byte_size` 在 +0x38 被 `gpu_flush` 读取）。共 0x48 字节。

use crate::render::dirty_index_range::DirtyIndexRange;
use crate::render::ssbo::rhapsody_boundary::{slice_as_bytes, GraphicsDevice};
use crate::render::ssbo::{StorageElement, TypedStorageBuffer};

#[repr(C)]
#[derive(Debug)]
pub struct ShadowedBuffer<T> {
    /// O：闭区间脏窗口；空 = (usize::MAX, 0)。
    pub dirty: DirtyIndexRange,
    /// O：CPU 影子（定长，创建后不再增长）。
    pub data: Box<[T]>,
    /// O：GPU 缓冲。
    pub buffer: TypedStorageBuffer<T>,
}

impl<T: StorageElement> ShadowedBuffer<T> {
    /// `ShadowedBuffer::new<T>`（O：0x14168f870，入参为 `(device, Vec<T>)`，按值取得 Vec）：
    /// 先用该 Vec 的切片调用 `TypedStorageBuffer::new` 创建 GPU 缓冲，再把 Vec `shrink_to_fit`
    /// （容量 > 长度时 realloc）成 `Box<[T]>` 作为影子，脏窗口置空 (usize::MAX, 0)。
    /// （旧实现把数据拷进 `with_capacity` 的 Vec 却没设 `len`，再 `shrink_to_fit`，得到空影子数组——已修复。）
    pub fn new(device: &dyn GraphicsDevice, initial: Vec<T>) -> Self {
        let buffer = TypedStorageBuffer::new(device, Some(&initial));
        Self { dirty: DirtyIndexRange::empty(), data: initial.into_boxed_slice(), buffer }
    }

    /// I：整块可变视图，标记整个影子为脏（空数组不标记）。
    pub fn get_mut(&mut self) -> &mut [T] {
        self.dirty.add(0..self.data.len());
        &mut self.data
    }

    /// I：影子内部拷贝 `count` 个元素 `src → dst`，并标脏目的区间（重叠安全）。
    pub fn copy(&mut self, dst: usize, src: usize, count: usize) {
        self.dirty.add(dst..dst + count);
        self.data.copy_within(src..src + count, dst);
    }

    /// I：写入影子 `[index, index+data.len())` 并标脏。
    pub fn write(&mut self, index: usize, data: &[T]) {
        self.dirty.add(index..index + data.len());
        self.data[index..index + data.len()].copy_from_slice(data);
    }

    /// `gpu_flush<T>`（O）：脏窗口为空（start > end）直接返回；否则取 `[start, end+1)`：
    ///  * `end+1 < start` → slice_index_order_fail；`end+1 > data.len()` → slice_end_index_len_fail；
    ///  * `(end+1)*es > buffer.byte_size` → `Result::unwrap` panic；
    ///  * 一次 `update_buffer_bytes(buffer, &data[start..end+1] 的字节, &[0..n*es], start*es)`；
    ///  * 成功后窗口重置为 (usize::MAX, 0)。
    /// 原版另有 puffin profile scope（作用域名 `...ShadowedBuffer::gpu_flush<T>`，数据为 `dirty` 的 Debug），
    /// 属外部库行为，这里不复现。
    pub fn gpu_flush(&mut self, device: &dyn GraphicsDevice) {
        use core::sync::atomic::Ordering::Relaxed;
        let (start, end) = (self.dirty.start.load(Relaxed), self.dirty.end.load(Relaxed));
        if start > end {
            return;
        }
        let end_excl = end + 1;
        let es = core::mem::size_of::<T>();
        let slice = &self.data[start..end_excl]; // 越界时 panic，与原版两个 fail 函数等价
        let n_bytes = slice.len() * es;
        if n_bytes != 0 {
            assert!(
                end_excl * es <= self.buffer.byte_size(),
                "called `Result::unwrap()` on an `Err` value: gpu_flush range exceeds buffer"
            );
            device
                .update_buffer_bytes(&self.buffer.buffer, slice_as_bytes(slice), &[0..n_bytes], start * es)
                .expect("called `Result::unwrap()` on an `Err` value: update_buffer_bytes");
        }
        self.dirty.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::ssbo::tests::{Dyn, MockDevice};
    use core::sync::atomic::Ordering::Relaxed;

    fn rows(n: usize) -> Vec<Dyn> {
        (0..n).map(|i| Dyn([i as f32; 4])).collect()
    }

    #[test]
    fn new_keeps_the_initial_data_in_the_shadow() {
        let dev = MockDevice::default();
        let b = ShadowedBuffer::new(&dev, rows(5));
        assert_eq!(b.data.len(), 5);
        assert_eq!(b.data[3], Dyn([3.0; 4]));
        assert!(b.dirty.is_empty());
    }

    #[test]
    fn flush_without_dirty_is_noop() {
        let dev = MockDevice::default();
        let mut b = ShadowedBuffer::new(&dev, rows(4));
        b.gpu_flush(&dev);
        assert!(dev.writes.borrow().is_empty());
    }

    #[test]
    fn write_then_flush_uploads_only_the_dirty_window() {
        let dev = MockDevice::default();
        let mut b = ShadowedBuffer::new(&dev, rows(8));
        b.write(2, &[Dyn([20.0; 4]), Dyn([30.0; 4])]);
        assert_eq!((b.dirty.start.load(Relaxed), b.dirty.end.load(Relaxed)), (2, 3));
        b.gpu_flush(&dev);
        let w = dev.writes.borrow();
        assert_eq!(w.len(), 1);
        assert_eq!(w[0], (vec![0..32], 2 * 16, 32)); // 范围相对于子切片，偏移为 start*es
        let m = dev.mem.borrow();
        assert_eq!(&m[32..36], &20.0f32.to_le_bytes());
        assert_eq!(&m[48..52], &30.0f32.to_le_bytes());
        assert_eq!(&m[16..20], &1.0f32.to_le_bytes(), "未脏元素保持创建时的值");
        assert!(b.dirty.is_empty());
    }

    #[test]
    fn get_mut_marks_everything_and_flush_does_not_overrun() {
        let dev = MockDevice::default();
        let mut b = ShadowedBuffer::new(&dev, rows(4));
        b.get_mut()[0] = Dyn([42.0; 4]);
        b.gpu_flush(&dev); // 旧实现：end 记为 len（排他），+1 后越界 panic
        assert_eq!(&dev.mem.borrow()[0..4], &42.0f32.to_le_bytes());
    }

    #[test]
    fn copy_within_shadow_is_marked_dirty() {
        let dev = MockDevice::default();
        let mut b = ShadowedBuffer::new(&dev, rows(6));
        b.copy(4, 0, 2);
        assert_eq!(b.data[4], Dyn([0.0; 4]));
        assert_eq!(b.data[5], Dyn([1.0; 4]));
        b.gpu_flush(&dev);
        assert_eq!(&dev.mem.borrow()[5 * 16..5 * 16 + 4], &1.0f32.to_le_bytes());
    }
}
