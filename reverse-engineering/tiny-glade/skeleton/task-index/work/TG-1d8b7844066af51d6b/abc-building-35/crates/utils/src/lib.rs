//! 三版共享的优先建筑候选；implemented_unverified，验证暂停。
//! 原始函数/A/B/C地址见逐函数注释；完整身份与剩余边界见任务根mapping/CONTRACT。
//! 使用同一assembly crate的上批类型，原函数只保留一次实现；完整作者ABI未自动证明。
pub use crate::base_math::{fit, intersects, inverse_lerp};
// ORIGINAL: utils::lerp
// 三版对应：A=0x1407d97b0；B=0x140c904d0；C=0x140de5ba0
// 逻辑组：8e3185720393a83945
/// 原 SUB/MUL/ADD 为 start+(end-start)*t；不融合乘加、不钳制 t。
/// 现有camera函数写 product+start，正常域可对应，但NaN操作数顺序尚无等价证明。
pub fn lerp(start: f32, end: f32, t: f32) -> f32 {
    let difference = end - start;
    let weighted = difference * t;
    start + weighted
}
