//! `country_core::utils::debug_viz` — 调试绘制的构造器（builder）部分。
//!
//! 布局与默认值逐字节取自 EXE（B 构建）：
//!
//! | 类型 / 函数 | 地址 | 布局 |
//! |---|---|---|
//! | `DebugSphere::new` | 0x140b02490 | 0x28 B：color@0（16 B，仅首 dword 置 0）、radius@0x10 = 0.1（`0x3dcccccd`）、center@0x14（12 B）、alpha@0x20 = 1.0、solid@0x24 = false |
//! | `DebugSphere::{color,radius,solid}` | 0x140b024c0 / 0x140b024e0 / 0x140b02500 | 就地改写对应字段，按值返回 |
//! | `DebugRectangle3d::new` | 0x140b02e10 | 0x38 B：color@0、line_width@0x10 = 1.0、rect@0x14（32 B，来自入参指针）、dashed@0x34 = false |
//! | `DebugRectangle3d::{color,line_width,dashed_line}` | 0x140b02e40 / 0x140b02e70 / 0x140b02ea0 | 同上 |
//! | `DebugLine::new` | 0x140b04e70 | 0x48 B：points `Vec<Vec3>`@0（24 B）、label `Option<String>`@0x18（None = `0x8000000000000000`）、color@0x30、line_width@0x40 = 1.0、ui_pass@0x44 = true、dashed@0x45 = false |
//! | `DebugLine::{color,set_ui_pass,dashed,line_width}` | 0x140b04eb0 / 0x140b04ee0 / 0x140b04f10 / 0x140b04f40 | 同上 |
//!
//! 颜色默认值：构造器只写 `color` 的首个 dword（= 0），其余 12 字节在 EXE 里未初始化，
//! 这里取 0（I）。`DebugRectangle2d` 没有独立构造器（`show` 直接取 4 个 f32）。
//!
//! `show` 系列（0x140b02520 / 0x140b02ed0 / 0x140b04640 / 0x140b04f70 及其闭包）把一次性闭包
//! 压入 `Commands`，闭包体读 `AssetMeshLibrary` / `AssetShaderLibrary`，属于 ECS 胶水层，
//! 留给组件/ECS 包（H）。

use glam::{Vec3, Vec4};

/// 一次画一个球的请求（0x28 B）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DebugSphere {
    pub color: Vec4,
    pub radius: f32,
    pub center: Vec3,
    pub alpha: f32,
    pub solid: bool,
}

impl DebugSphere {
    /// O 0x140b02490。
    pub fn new(center: Vec3) -> Self {
        Self {
            color: Vec4::ZERO,
            radius: 0.1,
            center,
            alpha: 1.0,
            solid: false,
        }
    }

    /// O 0x140b024c0。
    pub fn color(mut self, color: Vec4) -> Self {
        self.color = color;
        self
    }

    /// O 0x140b024e0。
    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }

    /// O 0x140b02500。
    pub fn solid(mut self) -> Self {
        self.solid = true;
        self
    }
}

/// 一次画一个 3D 矩形的请求（0x38 B）。`rect` 是入参里的 32 字节，布局 U（`show` 里用
/// `FUN_140c99600` 展开为 4 个角点）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DebugRectangle3d {
    pub color: Vec4,
    pub line_width: f32,
    pub rect: [f32; 8],
    pub dashed: bool,
}

impl DebugRectangle3d {
    /// O 0x140b02e10。
    pub fn new(rect: [f32; 8]) -> Self {
        Self {
            color: Vec4::ZERO,
            line_width: 1.0,
            rect,
            dashed: false,
        }
    }

    /// O 0x140b02e40。
    pub fn color(mut self, color: Vec4) -> Self {
        self.color = color;
        self
    }

    /// O 0x140b02e70。
    pub fn line_width(mut self, line_width: f32) -> Self {
        self.line_width = line_width;
        self
    }

    /// O 0x140b02ea0。
    pub fn dashed_line(mut self) -> Self {
        self.dashed = true;
        self
    }
}

/// 一次画一条折线的请求（0x48 B）。
#[derive(Clone, Debug, PartialEq)]
pub struct DebugLine {
    pub points: Vec<Vec3>,
    pub label: Option<String>,
    pub color: Vec4,
    pub line_width: f32,
    pub ui_pass: bool,
    pub dashed: bool,
}

impl DebugLine {
    /// O 0x140b04e70。
    pub fn new(points: Vec<Vec3>) -> Self {
        Self {
            points,
            label: None,
            color: Vec4::ZERO,
            line_width: 1.0,
            ui_pass: true,
            dashed: false,
        }
    }

    /// O 0x140b04eb0。
    pub fn color(mut self, color: Vec4) -> Self {
        self.color = color;
        self
    }

    /// O 0x140b04ee0：`r8b` 直接写入 +0x44。
    pub fn set_ui_pass(mut self, ui_pass: bool) -> Self {
        self.ui_pass = ui_pass;
        self
    }

    /// O 0x140b04f10。
    pub fn dashed(mut self) -> Self {
        self.dashed = true;
        self
    }

    /// O 0x140b04f40。
    pub fn line_width(mut self, line_width: f32) -> Self {
        self.line_width = line_width;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sphere_defaults_match_the_constructor_bytes() {
        let sphere = DebugSphere::new(Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(sphere.radius.to_bits(), 0x3dcccccd);
        assert_eq!(sphere.alpha, 1.0);
        assert!(!sphere.solid);
        let sphere = sphere.radius(2.0).solid().color(Vec4::ONE);
        assert_eq!(
            (sphere.radius, sphere.solid, sphere.color),
            (2.0, true, Vec4::ONE)
        );
    }

    #[test]
    fn rectangle_and_line_defaults() {
        let rect = DebugRectangle3d::new([0.0; 8]);
        assert_eq!((rect.line_width, rect.dashed), (1.0, false));
        assert!(rect.dashed_line().dashed);
        let line = DebugLine::new(vec![Vec3::ZERO]);
        assert_eq!(
            (line.line_width, line.ui_pass, line.dashed),
            (1.0, true, false)
        );
        assert!(line.label.is_none());
        let line = line.set_ui_pass(false).dashed().line_width(3.0);
        assert_eq!(
            (line.ui_pass, line.dashed, line.line_width),
            (false, true, 3.0)
        );
    }
}
