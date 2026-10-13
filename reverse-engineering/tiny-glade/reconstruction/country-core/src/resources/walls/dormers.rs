//! dormers.rs — 老虎窗洞 SSBO（WP21）。
//!
//! O：`DormerHoleSsbo` 48B 构造器（0x141286660 dummy / 0x141286680 gothic /
//! 0x1412866e0 cottage）。`DormerHoleCountPerCellSsbo` 计数 SSBO。

use crate::typed_gap::Gap;

/// 老虎窗洞 GPU 记录（O：48 字节；gothic/cottage 构造写满）。
///
/// 布局 O（gothic 0x141286680）：
/// `{ +0x0: Vec3, +0xc: f32, +0x10: Vec3, +0x1c: f32=0, +0x20..0x30: 4×f32 }`。
/// cottage 同 gothic 但 `+0x28=1.0`、`+0x2c=-50.0`（0xc24800003f800000）。字段名 I/U。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DormerHoleSsbo {
    /// +0x0 起 12B（Vec3；center/位置）。
    pub a: glam::Vec3,
    /// +0xc f32（尺寸/偏移）。
    pub b: f32,
    /// +0x10 起 12B（Vec3）。
    pub c: glam::Vec3,
    /// +0x1c f32 = 0（gothic/cottage 均清零）。
    pub d: f32,
    /// +0x20..0x30 4×f32（cottage 末两项 1.0 / -50.0）。
    pub e: [f32; 4],
    /// 未映射区（U）。
    pub _gap: Gap,
}

impl DormerHoleSsbo {
    /// `dummy`（O 0x141286660）：全零 48B。
    pub fn dummy() -> Self {
        Self::default()
    }
    /// `gothic`（O 0x141286680）：两个 Vec3 + 尺寸/常量。
    pub fn gothic(a: glam::Vec3, b: f32, c: glam::Vec3) -> Self {
        Self {
            a,
            b,
            c,
            d: 0.0,
            e: [0.0; 4],
            _gap: Gap::default(),
        }
    }
    /// `cottage`（O 0x1412866e0）：gothic 形，`e[2]=1.0`、`e[3]=-50.0`。
    pub fn cottage(a: glam::Vec3, b: f32, c: glam::Vec3) -> Self {
        Self {
            a,
            b,
            c,
            d: 0.0,
            e: [0.0, 0.0, 1.0, -50.0],
            _gap: Gap::default(),
        }
    }
}

/// 每格老虎窗洞计数 SSBO（O 名；布局 U）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DormerHoleCountPerCellSsbo(pub Gap);
