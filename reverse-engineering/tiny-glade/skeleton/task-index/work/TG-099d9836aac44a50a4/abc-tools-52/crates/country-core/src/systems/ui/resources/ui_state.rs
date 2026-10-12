//! A/B/C 三版共享工具候选，只恢复本文件列出的观察函数。
//! 状态：implemented_unverified；按用户此前要求，构建/测试/原码执行继续暂停。
//! 此文件是原指令语义的 Rust 候选，不是取回的作者源码。
//! 非完整类型标为投影；未声称原 Rust 签名、ABI 或完整系统已还原。
//! 完整身份、指令、版本对应及消费者见本任务 work 根目录 mapping.json / CONTRACT.md。
pub struct DragState {
    pub observed_tag: u8,
}
impl DragState {
    // ORIGINAL: country_core::systems::ui::resources::ui_state::DragState::is_finished
    // 三版对应：A=0x140686410；B=0x14092e6d0；C=0x1408fb1d0
    // 逻辑组：c887f684cec61c9ddf
    /// 原码是无符号 tag<2，不能按名字写成 tag==Finished。
    pub fn is_finished(&self) -> bool {
        self.observed_tag < 2
    }
    // ORIGINAL: country_core::systems::ui::resources::ui_state::DragState::is_started
    // 三版对应：A=0x14054a000；B=0x14092e6c0；C=0x1408fb1c0
    // 逻辑组：4c109e30d9389a6a18
    /// 开始状态的原 tag=3。
    pub fn is_started(&self) -> bool {
        self.observed_tag == 3
    }
}
pub struct HoverState {
    pub observed_tag: u8,
}
impl HoverState {
    // ORIGINAL: country_core::systems::ui::resources::ui_state::HoverState::is_some
    // 三版对应：A=0x14056dd60；B=0x1408c5680；C=0x1408f9b20
    // 逻辑组：eb46167ff732ec333b
    /// 原体 movzx 直接返回 byte，没有非零归一化。
    /// 在已证明输入只有 0/1 前保留 u8 返回投影；完整 Rust bool ABI 待调用方核对。
    pub fn is_some(&self) -> u8 {
        self.observed_tag
    }
}
