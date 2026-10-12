//! A/B/C 三版共享工具候选，只恢复本文件列出的观察函数。
//! 状态：implemented_unverified；按用户此前要求，构建/测试/原码执行继续暂停。
//! 此文件是原指令语义的 Rust 候选，不是取回的作者源码。
//! 非完整类型标为投影；未声称原 Rust 签名、ABI 或完整系统已还原。
//! 完整身份、指令、版本对应及消费者见本任务 work 根目录 mapping.json / CONTRACT.md。
// ORIGINAL: utils::inverse_lerp
// 三版对应：A=0x1407d9780；B=0x140c904a0；C=0x140de3df0
// 逻辑组：25dac2c59db32ac7b4
/// 顺序为 (value - start) / (end - start)，不钳制，不替换零分母。
pub fn inverse_lerp(start: f32, end: f32, value: f32) -> f32 {
    (value - start) / (end - start)
}
// ORIGINAL: utils::fit
// 三版对应：A=0x1407da6e0；B=0x140c918b0；C=0x140de5b80
// 逻辑组：adacba940fb8f10423
/// 按原 SUB/DIV/SUB/MUL/ADD 顺序映射，保留 NaN、无穷大和区间外值。
/// 不使用 mul_add，避免将原本分开的乘法和加法融合。
pub fn fit(value: f32, src_start: f32, src_end: f32, dst_start: f32, dst_end: f32) -> f32 {
    let t = (value - src_start) / (src_end - src_start);
    let span = dst_end - dst_start;
    t * span + dst_start
}
// ORIGINAL: utils::impl$5::intersects
// 三版对应：A=0x14177fc20；B=0x141762b70；C=0x14199edb0
// 逻辑组：466f5a839acbd65c29
/// 两个 f32 端点记录的闭区间相交；端点相等算相交，NaN 返回 false。
/// impl$5 的作者类型名尚未知，此处只表达已观察的两个端点。
pub fn intersects(a: [f32; 2], b: [f32; 2]) -> bool {
    a[1] >= b[0] && b[1] >= a[0]
}
