//! A/B/C 三版共享工具候选，只恢复本文件列出的观察函数。
//! 状态：implemented_unverified；按用户此前要求，构建/测试/原码执行继续暂停。
//! 此文件是原指令语义的 Rust 候选，不是取回的作者源码。
//! 非完整类型标为投影；未声称原 Rust 签名、ABI 或完整系统已还原。
//! 完整身份、指令、版本对应及消费者见本任务 work 根目录 mapping.json / CONTRACT.md。
// ORIGINAL: utils::rng::init_rng_u64
// 三版对应：A=0x140ceb460；B=0x140c96270；C=0x140de9b90
// 逻辑组：e768a76ab5467b69d1
/// 游戏自己的 seed 变换，先 wrapping_mul，再 wrapping_add，最后 XOR。
/// 返回的是观察到的 64 位初始状态；RNG 后续生成算法继续复用原 fastrand。
/// 不擅自调用 Rng::with_seed，后者可能再次变换 seed。
pub fn init_rng_u64(seed: u64) -> u64 {
    seed.wrapping_mul(0x7598_39da_217f)
        .wrapping_add(0x0001_3c7c_a929_3765)
        ^ 0x4d59_5df4_d0f3_3173
}
