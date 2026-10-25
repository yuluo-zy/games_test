//! A/B/C 三版共享工具候选，只恢复本文件列出的观察函数。
//! 状态：implemented_unverified；按用户此前要求，构建/测试/原码执行继续暂停。
//! 此文件是原指令语义的 Rust 候选，不是取回的作者源码。
//! 非完整类型标为投影；未声称原 Rust 签名、ABI 或完整系统已还原。
//! 完整身份、指令、版本对应及消费者见本任务 work 根目录 mapping.json / CONTRACT.md。
pub struct WelcomeScreenState {
    pub observed_tag: u32,
}
impl WelcomeScreenState {
    // ORIGINAL: country_core::resources::onboarding::WelcomeScreenState::is_closed
    // 三版对应：A=0x14061eea0；B=0x1409a9140；C=0x1409f5810
    // 逻辑组：767e88b568473d68f6
    /// 注意这里比较 32 位 tag=2，而非 UI 的 8 位 tag。
    pub fn is_closed(&self) -> bool {
        self.observed_tag == 2
    }
}
