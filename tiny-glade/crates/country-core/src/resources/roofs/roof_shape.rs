//! 原作 Roof 局部状态函数。字段名未知；不代表完整 Roof 类型或网格算法。

pub fn ty_observed(value_at_0x30: f32) -> bool {
    let threshold = f32::from_bits(0x3dcccccd);
    // UCOMISS + SETC: 无序比较（NaN）也设置 CF。
    threshold < value_at_0x30 || threshold.is_nan() || value_at_0x30.is_nan()
}

pub fn is_gable_observed(value_at_0x30: f32, value_at_0x34: f32) -> bool {
    // CMPNLEPS（含无序比较）与 CMPEQPS 的低 lane 按位相与。
    ty_observed(value_at_0x30) && value_at_0x34 == 1.0
}

// 原入口的八参数观察合同保持不变。
#[allow(clippy::too_many_arguments)]
pub fn new_observed(
    output: &mut [u8; 0x58],
    arg2: &[u8; 8],
    arg3: &[u8; 28],
    arg4: &[u8; 28],
    arg5: &[u8; 4],
    arg6: &[u8; 4],
    arg7: u8,
    arg8: &[u8; 12],
) {
    output[0..8].copy_from_slice(arg2);
    output[0x24..0x40].copy_from_slice(arg3);
    output[8..0x24].copy_from_slice(arg4);
    output[0x4c..0x50].copy_from_slice(arg5);
    output[0x50..0x54].copy_from_slice(arg6);
    output[0x54] = arg7;
    output[0x40..0x4c].copy_from_slice(arg8);
    // 原作没有写入 +0x55..+0x57。0x58 来自调用者组件步长，非完整 Rust 类型证明。
}
