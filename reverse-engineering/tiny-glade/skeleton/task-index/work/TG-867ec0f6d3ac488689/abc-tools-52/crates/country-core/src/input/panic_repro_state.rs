//! A/B/C 三版共享工具候选，只恢复本文件列出的观察函数。
//! 状态：implemented_unverified；按用户此前要求，构建/测试/原码执行继续暂停。
//! 此文件是原指令语义的 Rust 候选，不是取回的作者源码。
//! 非完整类型标为投影；未声称原 Rust 签名、ABI 或完整系统已还原。
//! 完整身份、指令、版本对应及消费者见本任务 work 根目录 mapping.json / CONTRACT.md。
pub struct PanicReproState {
    pub observed_tag: u8,
}
impl PanicReproState {
    // ORIGINAL: country_core::input::panic_repro_state::PanicReproState::is_inactive
    // 三版对应：A=0x1405971e0；B=0x140833230；C=0x1408fb520
    // 逻辑组：5969e04c7d5a73ef2b
    /// 原首字节等于零为 inactive，不推断其他复现状态的名称。
    pub fn is_inactive(&self) -> bool {
        self.observed_tag == 0
    }
}
