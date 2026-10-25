//! decorator_outline.rs — 装饰描边（WP21）。
//!
//! O：`DecoratorCollisionOutline::display` 0xb33920（判别 Lines(Vec<Vec3>)/Sphere，
//! 顶点按 3×3 矩阵+平移变换后 DebugLine/Sphere::show）；`DecoratorOutline::display`
//! 0xb33b00（位置 +0x30/+0x34/+0x38、尺寸 +0x20/+0x24/+0x28，flag 时位置+=尺寸*0.18，
//! 格式化 Debug 串后按判别显示）。

use glam::{Mat3, Vec3};

use crate::typed_gap::Gap;

/// 装饰碰撞描边（O：判别枚举；display 0xb33920）。
#[derive(Clone, Debug, PartialEq)]
pub enum DecoratorCollisionOutline {
    /// 折线（顶点行，display 按 Mat3+平移变换）。
    Lines(Vec<Vec3>),
    /// 球。
    Sphere,
    /// 其他（U）。
    Other(Gap),
}

impl DecoratorCollisionOutline {
    /// `display`（O 0xb33920）：lines → `p' = m*p + t` 逐顶点，调试显示。
    pub fn display(&self, m: &Mat3, t: Vec3) {
        match self {
            DecoratorCollisionOutline::Lines(pts) => {
                let _ = pts.iter().map(|p| *m * *p + t).collect::<Vec<_>>();
            }
            DecoratorCollisionOutline::Sphere => {}
            DecoratorCollisionOutline::Other(_) => {}
        }
    }
}

/// 装饰描边（O：display 0xb33b00）。
#[derive(Clone, Debug, PartialEq)]
pub struct DecoratorOutline {
    /// 尺寸（+0x20/+0x24/+0x28，O）。
    pub dimensions: Vec3,
    /// 位置（+0x30/+0x34/+0x38，O）。
    pub position: Vec3,
    /// 判别（display 按此选择绘制；O）。
    pub kind: u8,
    /// 未映射区（U）。
    pub _gap: Gap,
}

impl DecoratorOutline {
    /// `display`（O 0xb33b00）：`position += dimensions * 0.18`（flag 时），调试显示。
    pub fn display(&self, offset: bool) {
        let mut p = self.position;
        if offset {
            p += self.dimensions * 0.18;
        }
        let _ = p;
    }
}
