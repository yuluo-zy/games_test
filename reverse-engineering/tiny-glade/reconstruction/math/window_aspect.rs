//! 从原作 0x140949c20 的完整 20 字节指令还原。
//! 仅还原已观察的两个 u32 值到 f32 的除法；不假定完整 WindowSize 布局。
#[cfg(not(feature = "library"))]
use std::io::{self, BufRead};

/// 对应原函数从 RCX+0、RCX+4 读取的数值。
/// 不添加原指令没有的零分母检查，保留 IEEE 754 结果。
#[inline(never)]
pub fn aspect_ratio_observed(field_0x00: u32, field_0x04: u32) -> f32 {
    (field_0x00 as f32) / (field_0x04 as f32)
}

#[cfg(not(feature = "library"))]
fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.expect("读取差分输入失败");
        let values: Vec<u32> = line.split_whitespace().map(|s| s.parse().expect("输入须为 u32")).collect();
        assert_eq!(values.len(), 2);
        println!("{:08x}", aspect_ratio_observed(values[0], values[1]).to_bits());
    }
}
