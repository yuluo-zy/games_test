//! 原作 Roof 局部状态函数。字段名未知；不代表完整 Roof 类型或网格算法。
#[cfg(not(feature = "library"))]
use std::io::{self, BufRead};

pub fn ty_observed(value_at_0x30: f32) -> bool {
    let threshold = f32::from_bits(0x3dcccccd);
    // UCOMISS + SETC: 无序比较（NaN）也设置 CF。
    threshold < value_at_0x30 || threshold.is_nan() || value_at_0x30.is_nan()
}

pub fn is_gable_observed(value_at_0x30: f32, value_at_0x34: f32) -> bool {
    // CMPNLEPS（含无序比较）与 CMPEQPS 的低 lane 按位相与。
    ty_observed(value_at_0x30) && value_at_0x34 == 1.0
}

pub fn new_observed(output: &mut [u8; 0x58], arg2: &[u8; 8], arg3: &[u8; 28],
                    arg4: &[u8; 28], arg5: &[u8; 4], arg6: &[u8; 4], arg7: u8,
                    arg8: &[u8; 12]) {
    output[0..8].copy_from_slice(arg2);
    output[0x24..0x40].copy_from_slice(arg3);
    output[8..0x24].copy_from_slice(arg4);
    output[0x4c..0x50].copy_from_slice(arg5);
    output[0x50..0x54].copy_from_slice(arg6);
    output[0x54] = arg7;
    output[0x40..0x4c].copy_from_slice(arg8);
    // 原作没有写入 +0x55..+0x57。0x58 来自调用者组件步长，非完整 Rust 类型证明。
}

#[cfg(not(feature = "library"))]
fn decode<const N: usize>(s: &str) -> [u8; N] {
    assert_eq!(s.len(), N * 2);
    std::array::from_fn(|i| u8::from_str_radix(&s[i*2..i*2+2],16).unwrap())
}

#[cfg(not(feature = "library"))]
fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let p: Vec<_> = line.split_whitespace().collect();
        match p[0] {
            "pred" => {
                let v: Vec<f32> = p[1..].iter().map(|s| f32::from_bits(u32::from_str_radix(s,16).unwrap())).collect();
                println!("{} {}", ty_observed(v[0]) as u8, is_gable_observed(v[0],v[1]) as u8);
            }
            "new" => {
                let mut out = decode::<0x58>(p[1]);
                new_observed(&mut out, &decode(p[2]), &decode(p[3]), &decode(p[4]),
                             &decode(p[5]), &decode(p[6]), u8::from_str_radix(p[7],16).unwrap(), &decode(p[8]));
                println!("{}", out.iter().map(|b| format!("{b:02x}")).collect::<String>());
            }
            _ => panic!("unknown test mode"),
        }
    }
}
