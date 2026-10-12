//! 三版共享的优先建筑候选；implemented_unverified，验证暂停。
//! 原始函数/A/B/C地址见逐函数注释；完整身份与剩余边界见任务根mapping/CONTRACT。
//! 使用同一assembly crate的上批类型，原函数只保留一次实现；完整作者ABI未自动证明。
use glam::{Vec2, Vec3};
// ORIGINAL: utils::swizzlers::impl$1::xvy
// 三版对应：A=0x1407da780；B=0x140c9cde0；C=0x140df0580
// 逻辑组：f99ccfabf54b5b948f
/// 将原 Vec2 的两个分量放到三维 x/z，中间 y 使用调用方值。
/// 原指令只有三次32位复制，不归一化，也不做坐标缩放。
pub fn xvy(xz: Vec2, y: f32) -> Vec3 {
    Vec3::new(xz.x, y, xz.y)
}
