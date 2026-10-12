//! A/B/C 三版共享工具候选，只恢复本文件列出的观察函数。
//! 状态：implemented_unverified；按用户此前要求，构建/测试/原码执行继续暂停。
//! 此文件是原指令语义的 Rust 候选，不是取回的作者源码。
//! 非完整类型标为投影；未声称原 Rust 签名、ABI 或完整系统已还原。
//! 完整身份、指令、版本对应及消费者见本任务 work 根目录 mapping.json / CONTRACT.md。
/// 只命名已证谓词对应的 tag；不补造完整版本枚举。
pub struct GladeType {
    pub observed_tag: u8,
}
impl GladeType {
    // ORIGINAL: country_core::resources::session::GladeType::is_flowery
    // 三版对应：A=0x140549ff0；B=0x14092e6a0；C=0x1408f9b30
    // 逻辑组：44cb3204783ce3cf21
    /// 花草场景 tag=2。
    pub fn is_flowery(&self) -> bool {
        self.observed_tag == 2
    }
    // ORIGINAL: country_core::resources::session::GladeType::is_autumn
    // 三版对应：A=0x14054a000；B=0x14092e6c0；C=0x1408fb1c0
    // 逻辑组：9a231fc21fe8ec7a1c
    /// 秋季场景 tag=3。
    pub fn is_autumn(&self) -> bool {
        self.observed_tag == 3
    }
    // ORIGINAL: country_core::resources::session::GladeType::is_olden
    // 三版对应：A=0x140549fe0；B=0x140a9fa70；C=0x140b05d10
    // 逻辑组：b8c6f692715911ec7a
    /// 古旧场景 tag=1。
    pub fn is_olden(&self) -> bool {
        self.observed_tag == 1
    }
}
