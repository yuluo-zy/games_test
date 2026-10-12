//! 上传接口只接受已定义布局的字节切片，不猜测任意 Rust 类型的填充。
pub trait BufferBytes {
    fn byte_count(&self) -> usize;
    fn copy_to_slice(&self, output: &mut [u8]);
}

/// 任意类型重解释为字节切片；调用方保证类型不含 padding 歧义。
/// # Safety
/// `value` 必须保持存活且无内部引用；结果切片的生命周期与借用一致。
pub unsafe fn slice_as_bytes<T>(value: &T) -> &[u8] {
    unsafe {
        std::slice::from_raw_parts(
            (value as *const T).cast::<u8>(),
            std::mem::size_of::<T>(),
        )
    }
}

/// 字节切片重解释为类型引用；未验证对齐或内容布局。
/// # Safety
/// `bytes` 的长度必须与 `T` 大小一致且满足 `T` 的对齐要求。
pub unsafe fn bytes_as_slice<T>(bytes: &[u8]) -> &T {
    debug_assert_eq!(bytes.len(), std::mem::size_of::<T>());
    unsafe { &*bytes.as_ptr().cast::<T>() }
}

impl BufferBytes for [u8] {
    fn byte_count(&self) -> usize {
        self.len()
    }
    fn copy_to_slice(&self, output: &mut [u8]) {
        output.copy_from_slice(self);
    }
}
impl<const N: usize> BufferBytes for [u8; N] {
    fn byte_count(&self) -> usize {
        N
    }
    fn copy_to_slice(&self, output: &mut [u8]) {
        output.copy_from_slice(self);
    }
}
