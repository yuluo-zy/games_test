//! 三版共享的优先建筑候选；implemented_unverified，验证暂停。
//! 原始函数/A/B/C地址见逐函数注释；完整身份与剩余边界见任务根mapping/CONTRACT。
//! 使用同一assembly crate的上批类型，原函数只保留一次实现；完整作者ABI未自动证明。
/// 原+24的长度输入投影；ReplayBuffer中其它数组/时间/owner仍未由此getter恢复。
pub struct ReplayBuffer {
    pub observed_len: usize,
}
impl ReplayBuffer {
    // ORIGINAL: country_core::resources::history::replay_cmd_buffer::ReplayBuffer::len
    // 三版对应：A=0x14069fb90；B=0x140949170；C=0x140a59330
    // 逻辑组：42798014f1c95e5c2c
    /// 返回原word长度，不实现容器或把同名len当LoopRange长度。
    pub fn len(&self) -> usize {
        self.observed_len
    }
}
