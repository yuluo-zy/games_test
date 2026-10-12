//! A/B/C 三版共享工具候选，只恢复本文件列出的观察函数。
//! 状态：implemented_unverified；按用户此前要求，构建/测试/原码执行继续暂停。
//! 此文件是原指令语义的 Rust 候选，不是取回的作者源码。
//! 非完整类型标为投影；未声称原 Rust 签名、ABI 或完整系统已还原。
//! 完整身份、指令、版本对应及消费者见本任务 work 根目录 mapping.json / CONTRACT.md。
/// 借用实际已初始化原记录；不重造完整 WallState、shape 或 ECS 布局。
pub struct WallState<'a> {
    pub observed_record: &'a [u8],
}
impl<'a> WallState<'a> {
    // ORIGINAL: country_core::resources::walls::inner_wall_state::WallState::get_enclosure_params
    // 三版对应：A=0x14051aa10；B=0x1408c56b0；C=0x140c557c0
    // 逻辑组：f8c5f5972307bf53ab
    /// 输入至少 60B。原 u32 tag 位于 +0x38，tag<2 返回 +0x38 本身，
    /// 包含 tag，并非 +0x3c 的 payload；否则 None。
    /// payload 长度/作者类型未知，借用尾部字节，不解读未取证字段。
    pub fn get_enclosure_params(&self) -> Option<&'a [u8]> {
        let tag = u32::from_le_bytes(self.observed_record[0x38..0x3c].try_into().unwrap());
        if tag < 2 {
            Some(&self.observed_record[0x38..])
        } else {
            None
        }
    }
}
