//! A/B/C 三版共享工具候选，只恢复本文件列出的观察函数。
//! 状态：implemented_unverified；按用户此前要求，构建/测试/原码执行继续暂停。
//! 此文件是原指令语义的 Rust 候选，不是取回的作者源码。
//! 非完整类型标为投影；未声称原 Rust 签名、ABI 或完整系统已还原。
//! 完整身份、指令、版本对应及消费者见本任务 work 根目录 mapping.json / CONTRACT.md。
pub struct TimelapseState {
    pub observed_tag: u8,
}
impl TimelapseState {
    // ORIGINAL: country_core::systems::timelapse_replay::TimelapseState::is_active
    // 三版对应：A=0x140597160；B=0x14092e6b0；C=0x1408fb1e0
    // 逻辑组：cf0aaf19e2bca01983
    /// 原 tag!=2 即 active，不把 tag=0 默认解释为 inactive。
    pub fn is_active(&self) -> bool {
        self.observed_tag != 2
    }
}
