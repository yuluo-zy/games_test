//! 三版共享的优先建筑候选；implemented_unverified，验证暂停。
//! 原始函数/A/B/C地址见逐函数注释；完整身份与剩余边界见任务根mapping/CONTRACT。
//! 使用同一assembly crate的上批类型，原函数只保留一次实现；完整作者ABI未自动证明。
pub use crate::base_rng::init_rng_u64;
// ORIGINAL: utils::rng::init_rng
// 三版对应：A=0x140ceb430；B=0x140c96240；C=0x140de9bc0
// 逻辑组：738eff71f3565635f0
/// 原ECX低32位先零扩展，再做与init_rng_u64相同的游戏seed变换。
/// 两入口共用一份算法；输出是原64位状态，不再额外调用with_seed二次变换。
pub fn init_rng(seed: u32) -> u64 {
    init_rng_u64(u64::from(seed))
}
