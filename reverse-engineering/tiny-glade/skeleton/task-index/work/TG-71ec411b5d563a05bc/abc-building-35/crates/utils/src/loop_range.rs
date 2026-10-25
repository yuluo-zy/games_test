//! 三版共享的优先建筑候选；implemented_unverified，验证暂停。
//! 原始函数/A/B/C地址见逐函数注释；完整身份与剩余边界见任务根mapping/CONTRACT。
//! 使用同一assembly crate的上批类型，原函数只保留一次实现；完整作者ABI未自动证明。
pub use crate::base_loop_range::{LoopIdx, LoopRange};
impl LoopRange {
    // ORIGINAL: utils::loop_range::LoopRange::len
    // 三版对应：A=0x140cb3c10；B=0x141200350；C=0x141405260
    // 逻辑组：1bea3154d1bcb2f43d
    /// 原end-start在32位回绕后符号扩展；逆序可返回负数，不转换成usize或abs。
    pub fn len(&self) -> i64 {
        self.end_idx.wrapping_sub(self.start_idx) as i64
    }
}
