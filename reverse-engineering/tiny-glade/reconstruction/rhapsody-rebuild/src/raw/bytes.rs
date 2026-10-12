//! 上传接口只接受已定义布局的字节切片，不猜测任意 Rust 类型的填充。
pub trait BufferBytes {
    fn byte_count(&self) -> usize;
    fn copy_to_slice(&self, output: &mut [u8]);
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
