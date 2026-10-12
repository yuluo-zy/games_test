//! A/B/C 三版共享工具候选，只恢复本文件列出的观察函数。
//! 状态：implemented_unverified；按用户此前要求，构建/测试/原码执行继续暂停。
//! 此文件是原指令语义的 Rust 候选，不是取回的作者源码。
//! 非完整类型标为投影；未声称原 Rust 签名、ABI 或完整系统已还原。
//! 完整身份、指令、版本对应及消费者见本任务 work 根目录 mapping.json / CONTRACT.md。
/// 原生值传递的首字节 tag；其他 attachment payload 不在本批定义。
pub struct WallCornerAttachment {
    pub observed_tag: u8,
}
impl WallCornerAttachment {
    // ORIGINAL: country_core::resources::walls::wall_attachment::WallCornerAttachment::is_attached
    // 三版对应：A=0x14066a720；B=0x140b256b0；C=0x140af95c0
    // 逻辑组：6cf5605e8800fdc6f3
    /// test cl;setne，仅 tag 非零即 attached。
    pub fn is_attached(self) -> bool {
        self.observed_tag != 0
    }
}
