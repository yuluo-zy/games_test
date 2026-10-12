//! A/B/C 三版共享工具候选，只恢复本文件列出的观察函数。
//! 状态：implemented_unverified；按用户此前要求，构建/测试/原码执行继续暂停。
//! 此文件是原指令语义的 Rust 候选，不是取回的作者源码。
//! 非完整类型标为投影；未声称原 Rust 签名、ABI 或完整系统已还原。
//! 完整身份、指令、版本对应及消费者见本任务 work 根目录 mapping.json / CONTRACT.md。
pub struct LazyTile(pub u32);
impl LazyTile {
    // ORIGINAL: country_core::LazyTile::from_index
    // 三版对应：A=0x140593250；B=0x140833250；C=0x1408fb460
    // 逻辑组：3f3b290f44e658d846
    /// 原 ecx->eax 恒等封装，不做 tile 解码、范围限制或资源查找。
    pub fn from_index(index: u32) -> Self {
        Self(index)
    }
}
