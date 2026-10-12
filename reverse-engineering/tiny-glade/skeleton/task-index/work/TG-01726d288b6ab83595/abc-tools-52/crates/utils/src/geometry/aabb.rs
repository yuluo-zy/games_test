//! A/B/C 三版共享工具候选，只恢复本文件列出的观察函数。
//! 状态：implemented_unverified；按用户此前要求，构建/测试/原码执行继续暂停。
//! 此文件是原指令语义的 Rust 候选，不是取回的作者源码。
//! 非完整类型标为投影；未声称原 Rust 签名、ABI 或完整系统已还原。
//! 完整身份、指令、版本对应及消费者见本任务 work 根目录 mapping.json / CONTRACT.md。
use glam::{Vec2, Vec3};
/// 原生 Aabb2 的 min 位于 +0，max 位于 +8；使用原依赖向量，不重写 glam。
pub struct Aabb2 {
    pub min: Vec2,
    pub max: Vec2,
}
pub struct Aabb3 {
    pub min: Vec3,
    pub max: Vec3,
}
impl Aabb2 {
    // ORIGINAL: utils::geometry::aabb::Aabb2::area
    // 三版对应：A=0x140efbbc0；B=0x141240120；C=0x140deac10
    // 逻辑组：89fa35c8944c77586e
    /// 不排序端点、不取绝对值；原乘法的左操作数是 y 差，右操作数是 x 差。
    pub fn area(&self) -> f32 {
        let x = self.max.x - self.min.x;
        let y = self.max.y - self.min.y;
        y * x
    }
}
impl Aabb3 {
    // ORIGINAL: utils::geometry::aabb::Aabb3::extent
    // 三版对应：A=0x140efbf00；B=0x141240460；C=0x140deb110
    // 逻辑组：68a7b35e249b6b276a
    /// 返回 max-min，不除以二；这里 extent 指完整边长。
    pub fn extent(&self) -> Vec3 {
        self.max - self.min
    }
}
