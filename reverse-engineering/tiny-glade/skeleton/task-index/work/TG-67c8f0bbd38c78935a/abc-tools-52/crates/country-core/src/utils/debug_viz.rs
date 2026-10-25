//! A/B/C 三版共享工具候选，只恢复本文件列出的观察函数。
//! 状态：implemented_unverified；按用户此前要求，构建/测试/原码执行继续暂停。
//! 此文件是原指令语义的 Rust 候选，不是取回的作者源码。
//! 非完整类型标为投影；未声称原 Rust 签名、ABI 或完整系统已还原。
//! 完整身份、指令、版本对应及消费者见本任务 work 根目录 mapping.json / CONTRACT.md。
/// 颜色 16 字节的具体作者类型仍未知；构造器只写默认 tag=0，
/// 未初始化的枚举 payload 不补零，不从中构造 RGBA 值。
#[derive(Clone, Copy)]
pub enum DebugColor {
    DefaultTagZero,
    Encoded([u8; 16]),
}
pub struct DebugRectangle3d {
    pub color_value: DebugColor,
    pub width: f32,
    pub geometry: [u8; 32],
    pub solid: bool,
}
impl DebugRectangle3d {
    // ORIGINAL: country_core::utils::debug_viz::DebugRectangle3d::new
    // 三版对应：A=0x140ec4160；B=0x140b02e10；C=0x140a75bf0
    // 逻辑组：210e372d0b853fc7ea
    /// 原写入 +0 tag=0、+16 width=1、+20..52 原样几何、+52 solid=0。
    /// 此处是语义字段投影，不声称 Rust enum/struct 与原 56B ABI 相同。
    pub fn new(geometry: [u8; 32]) -> Self {
        Self {
            color_value: DebugColor::DefaultTagZero,
            width: 1.0,
            geometry,
            solid: false,
        }
    }
    // ORIGINAL: country_core::utils::debug_viz::DebugRectangle3d::color
    // 三版对应：A=0x140ec4190；B=0x140b02e40；C=0x140a75c30
    // 逻辑组：d545cd1269d491efed
    /// 原 builder 按值写 color 后返回整体，其他字段保持。
    pub fn color(mut self, encoded: [u8; 16]) -> Self {
        self.color_value = DebugColor::Encoded(encoded);
        self
    }
    // ORIGINAL: country_core::utils::debug_viz::DebugRectangle3d::line_width
    // 三版对应：A=0x140ec41c0；B=0x140b02e70；C=0x140a75b90
    // 逻辑组：472e54216575b614fc
    /// 无条件写宽度，不检查是否大于零；NaN 也原样保留。
    pub fn line_width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }
}
/// Sphere 的构造器不在本批；使用完整已初始化的原 40B 记录，保留未知字段和 padding。
pub struct DebugSphere {
    pub observed_record: [u8; 40],
}
impl DebugSphere {
    // ORIGINAL: country_core::utils::debug_viz::DebugSphere::solid
    // 三版对应：A=0x140ec38f0；B=0x140b02500；C=0x140a75490
    // 逻辑组：58cd0366efd714b7ea
    /// 只写 +36 的 solid=1，之后按值返回记录，不创建球体 mesh。
    pub fn solid(mut self) -> Self {
        self.observed_record[36] = 1;
        self
    }
    // ORIGINAL: country_core::utils::debug_viz::DebugSphere::color
    // 三版对应：A=0x140ec38b0；B=0x140b024c0；C=0x140a75470
    // 逻辑组：735e9e7167559b2205
    /// 只覆盖前 16B 颜色记录，余下 24B 原样保存。
    pub fn color(mut self, encoded: [u8; 16]) -> Self {
        self.observed_record[..16].copy_from_slice(&encoded);
        self
    }
}
