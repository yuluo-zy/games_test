//! A/B/C 三版共享工具候选，只恢复本文件列出的观察函数。
//! 状态：implemented_unverified；按用户此前要求，构建/测试/原码执行继续暂停。
//! 此文件是原指令语义的 Rust 候选，不是取回的作者源码。
//! 非完整类型标为投影；未声称原 Rust 签名、ABI 或完整系统已还原。
//! 完整身份、指令、版本对应及消费者见本任务 work 根目录 mapping.json / CONTRACT.md。
pub struct GableRoofStyle {
    pub observed_tag: u8,
}
impl GableRoofStyle {
    // ORIGINAL: country_core::resources::roofs::roof_shape::GableRoofStyle::is_overhang
    // 三版对应：A=0x14056dd60；B=0x1408c5680；C=0x1408f9b20
    // 逻辑组：a9126dedeb3f59e0f3
    /// 原码直接读取 byte，没有 compare；保留 u8，合法 bool tag 范围待消费者核对。
    pub fn is_overhang(&self) -> u8 {
        self.observed_tag
    }
}
