//! A/B/C 三版共享工具候选，只恢复本文件列出的观察函数。
//! 状态：implemented_unverified；按用户此前要求，构建/测试/原码执行继续暂停。
//! 此文件是原指令语义的 Rust 候选，不是取回的作者源码。
//! 非完整类型标为投影；未声称原 Rust 签名、ABI 或完整系统已还原。
//! 完整身份、指令、版本对应及消费者见本任务 work 根目录 mapping.json / CONTRACT.md。
/// impl$0 作者类型与 tag 语义未证，此投影只保存原 +0 的 64 位载荷。
pub struct TagView {
    pub observed_word: u64,
}
impl TagView {
    // ORIGINAL: motomoto::impl$0::as_tag
    // 三版对应：A=0x14010da30；B=0x1400885d0；C=0x1400cb570
    // 逻辑组：167b3ed95821d822ee
    /// 返回原 word，不生成/转换 tag，不把可能的指针强行解引用。
    pub fn as_tag(&self) -> u64 {
        self.observed_word
    }
}
