//! A/B/C 三版共享工具候选，只恢复本文件列出的观察函数。
//! 状态：implemented_unverified；按用户此前要求，构建/测试/原码执行继续暂停。
//! 此文件是原指令语义的 Rust 候选，不是取回的作者源码。
//! 非完整类型标为投影；未声称原 Rust 签名、ABI 或完整系统已还原。
//! 完整身份、指令、版本对应及消费者见本任务 work 根目录 mapping.json / CONTRACT.md。
pub struct ClutterVersion {
    pub observed_tag: u8,
}
impl ClutterVersion {
    // ORIGINAL: country_core::resources::walls::decorator::ClutterVersion::is_initial
    // 三版对应：A=0x1405971e0；B=0x140833230；C=0x1408fb520
    // 逻辑组：abf19a5c267455b3b3
    /// 原首字节为零时才是初始版本；不是任意字段全零判断。
    pub fn is_initial(&self) -> bool {
        self.observed_tag == 0
    }
}
