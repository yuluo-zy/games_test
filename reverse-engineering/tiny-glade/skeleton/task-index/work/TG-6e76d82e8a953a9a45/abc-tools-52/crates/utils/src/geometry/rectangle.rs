//! A/B/C 三版共享工具候选，只恢复本文件列出的观察函数。
//! 状态：implemented_unverified；按用户此前要求，构建/测试/原码执行继续暂停。
//! 此文件是原指令语义的 Rust 候选，不是取回的作者源码。
//! 非完整类型标为投影；未声称原 Rust 签名、ABI 或完整系统已还原。
//! 完整身份、指令、版本对应及消费者见本任务 work 根目录 mapping.json / CONTRACT.md。
use glam::Vec2;
/// 原 +0..16 几何数据未在本函数解释；原样保存，不猜旋转/轴的字段名。
pub struct Rectangle2d {
    pub geometry: [u8; 16],
    pub size: Vec2,
}
impl Rectangle2d {
    // ORIGINAL: utils::geometry::rectangle::Rectangle2d::with_expanded_size
    // 三版对应：A=0x1408d27a0；B=0x140c9a730；C=0x140dee1d0
    // 逻辑组：d5ffbe533785be3996
    /// 原码直接 size+amount；不乘二，也不钳制负尺寸。
    pub fn with_expanded_size(&self, amount: Vec2) -> Self {
        Self {
            geometry: self.geometry,
            size: amount + self.size,
        }
    }
}
