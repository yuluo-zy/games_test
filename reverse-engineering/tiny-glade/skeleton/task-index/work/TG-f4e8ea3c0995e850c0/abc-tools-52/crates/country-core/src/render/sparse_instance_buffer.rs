//! A/B/C 三版共享工具候选，只恢复本文件列出的观察函数。
//! 状态：implemented_unverified；按用户此前要求，构建/测试/原码执行继续暂停。
//! 此文件是原指令语义的 Rust 候选，不是取回的作者源码。
//! 非完整类型标为投影；未声称原 Rust 签名、ABI 或完整系统已还原。
//! 完整身份、指令、版本对应及消费者见本任务 work 根目录 mapping.json / CONTRACT.md。
/// 本轮用户明确包含此后置文件中的工具函数；仅恢复访问器，不恢复 GPU。
pub struct AllocationHandle {
    pub observed_word: u64,
}
impl AllocationHandle {
    // ORIGINAL: country_core::render::sparse_instance_buffer::AllocationHandle::is_null
    // 三版对应：A=0x1406c0410；B=0x140985f30；C=0x140a9d9c0
    // 逻辑组：30d91f07f46165471e
    /// 原首 64 位为零即 null，不查询 allocation 表或 GPU 状态。
    pub fn is_null(&self) -> bool {
        self.observed_word == 0
    }
}
