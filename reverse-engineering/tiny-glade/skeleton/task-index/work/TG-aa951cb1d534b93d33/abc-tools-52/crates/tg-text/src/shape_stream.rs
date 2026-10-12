//! A/B/C 三版共享工具候选，只恢复本文件列出的观察函数。
//! 状态：implemented_unverified；按用户此前要求，构建/测试/原码执行继续暂停。
//! 此文件是原指令语义的 Rust 候选，不是取回的作者源码。
//! 非完整类型标为投影；未声称原 Rust 签名、ABI 或完整系统已还原。
//! 完整身份、指令、版本对应及消费者见本任务 work 根目录 mapping.json / CONTRACT.md。
/// 原 +16 的长度由真实 ShapeStream owner 提供，其余 text/shape 字段尚未恢复。
pub struct ShapeStream {
    pub observed_len: usize,
}
impl ShapeStream {
    // ORIGINAL: tg_text::shape_stream::ShapeStream::is_empty
    // 三版对应：A=0x140a67b20；B=0x140b92210；C=0x140cb3030
    // 逻辑组：e176d7b2471b5f6857
    /// 只比较长度是否为零，不重写文字 shaping 或 Vec 存储。
    pub fn is_empty(&self) -> bool {
        self.observed_len == 0
    }
}
