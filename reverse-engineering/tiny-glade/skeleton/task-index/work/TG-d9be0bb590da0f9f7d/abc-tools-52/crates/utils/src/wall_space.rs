//! A/B/C 三版共享工具候选，只恢复本文件列出的观察函数。
//! 状态：implemented_unverified；按用户此前要求，构建/测试/原码执行继续暂停。
//! 此文件是原指令语义的 Rust 候选，不是取回的作者源码。
//! 非完整类型标为投影；未声称原 Rust 签名、ABI 或完整系统已还原。
//! 完整身份、指令、版本对应及消费者见本任务 work 根目录 mapping.json / CONTRACT.md。
pub struct WallSpaceCoord {
    pub u: f32,
    pub y: f32,
}
impl WallSpaceCoord {
    // ORIGINAL: utils::wall_space::WallSpaceCoord::with_y_offset
    // 三版对应：A=0x140cebbc0；B=0x140c925d0；C=0x140de5e20
    // 逻辑组：1a7cb100060026f67e
    /// 原值传递 ABI 保留 xmm0 的 u，仅对 xmm1 的 y 加 offset。
    pub fn with_y_offset(self, offset: f32) -> Self {
        Self {
            u: self.u,
            y: self.y + offset,
        }
    }
    // ORIGINAL: utils::wall_space::WallSpaceCoord::with_y
    // 三版对应：A=0x140cebbd0；B=0x140c925e0；C=0x140de61d0
    // 逻辑组：7b6894c091c28e7cd4
    /// 只替换 y，不归一化 u。
    pub fn with_y(self, y: f32) -> Self {
        Self { u: self.u, y }
    }
}
