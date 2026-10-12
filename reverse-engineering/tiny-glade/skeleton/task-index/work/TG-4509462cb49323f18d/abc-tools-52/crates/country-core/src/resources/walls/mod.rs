//! A/B/C 三版共享工具候选，只恢复本文件列出的观察函数。
//! 状态：implemented_unverified；按用户此前要求，构建/测试/原码执行继续暂停。
//! 此文件是原指令语义的 Rust 候选，不是取回的作者源码。
//! 非完整类型标为投影；未声称原 Rust 签名、ABI 或完整系统已还原。
//! 完整身份、指令、版本对应及消费者见本任务 work 根目录 mapping.json / CONTRACT.md。
pub struct WallId(pub u64);
impl WallId {
    // ORIGINAL: country_core::resources::walls::WallId::is_valid
    // 三版对应：A=0x1405d43c0；B=0x140b34f20；C=0x140913ae0
    // 逻辑组：d0ee28bf51fd1b1835
    /// 原码只判断 ID 是否非零，不查询墙表，也不保证对象尚未删除。
    pub fn is_valid(&self) -> bool {
        self.0 != 0
    }
}
