//! 三版共享的优先建筑候选；implemented_unverified，验证暂停。
//! 原始函数/A/B/C地址见逐函数注释；完整身份与剩余边界见任务根mapping/CONTRACT。
//! 使用同一assembly crate的上批类型，原函数只保留一次实现；完整作者ABI未自动证明。
/// 原32位命令编码投影；完整版本枚举和owner分发不由单个常量入口恢复。
pub struct HistoryCmd {
    pub encoded_word: u32,
}
impl HistoryCmd {
    // ORIGINAL: country_core::systems::history::history_process::HistoryCmd::interrupt
    // 三版对应：A=0x14068cac0；B=0x140af5b20；C=0x140c21ed0
    // 逻辑组：c3cf0d9f2e0d86185b
    /// 返回0x02000200；这是构造命令，不直接清空队列或强制commit事务。
    pub fn interrupt() -> Self {
        Self {
            encoded_word: 0x0200_0200,
        }
    }
}
