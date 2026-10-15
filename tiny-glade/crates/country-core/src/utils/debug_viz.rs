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
//! 颜色：`color` 不是 RGBA 向量，而是 16 B 的 6 变体枚举——`u32` 标签 @0，`Custom` 的 RGB 三个 f32 @4..16。
//! 构造器只写标签（`mov dword [rcx], 0` = `White`），其余 12 字节在 EXE 里未初始化。
//! 证据：`DebugSphere::show`（0x140b02520）用标签索引两张 6 项表（长度 0x142ae7200、串指针 0x142ae7230）选
//! `debug_sphere_{white,red,blue,green,grey,custom}`；`DebugLine::show`（0x140b04f70）按跳表 0x142ae731c 把标签映射为
//! RGB：白 (1,1,1)、红 (1,0,0)、蓝 (0,0,1)、绿 (0,1,0)、灰 (0.1,0.1,0.1)、自定义取 `+0x34` 起的三个 f32。
//! `DebugRectangle3d` 的标签同位置同写法，按同一枚举处理（`show` 0x140b02ed0 只粗读过，I）。
//! `DebugRectangle2d` 没有独立构造器（`show` 直接取 4 个 f32）。
//!
//! `show` 系列（0x140b02520 / 0x140b02ed0 / 0x140b04640 / 0x140b04f70 及其闭包）把一次性闭包
//! 压入 `Commands`，闭包体读 `AssetMeshLibrary` / `AssetShaderLibrary`，属于 ECS 胶水层，
//! 留给组件/ECS 包（H）。

use glam::Vec3;

/// 调试图元的颜色（O：标签顺序与串表一致）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DebugColor {
    White,
    Red,
    Blue,
    Green,
    Grey,
    /// 标签 5，RGB 在 `+4..+16`。
    Custom(Vec3),
}

impl Default for DebugColor {
    /// 构造器写入的标签 0。
    fn default() -> Self {
        Self::White
    }
}

impl DebugColor {
    /// 标签字节（O：`u32 @0`）。
    pub fn tag(self) -> u32 {
        match self {
            Self::White => 0,
            Self::Red => 1,
            Self::Blue => 2,
            Self::Green => 3,
            Self::Grey => 4,
            Self::Custom(_) => 5,
        }
    }

    /// `DebugLine::show` 的映射（O：0x140b0507d..0x140b05119）。
    pub fn rgb(self) -> Vec3 {
        match self {
            Self::White => Vec3::new(1.0, 1.0, 1.0),
            Self::Red => Vec3::new(1.0, 0.0, 0.0),
            Self::Blue => Vec3::new(0.0, 0.0, 1.0),
            Self::Green => Vec3::new(0.0, 1.0, 0.0),
            Self::Grey => Vec3::new(0.1, 0.1, 0.1),
            Self::Custom(rgb) => rgb,
        }
    }

    /// `DebugSphere::show` 选用的网格名（O：0x142ae7230 的 6 个串）。
    pub fn sphere_mesh_name(self) -> &'static str {
        match self {
            Self::White => "debug_sphere_white",
            Self::Red => "debug_sphere_red",
            Self::Blue => "debug_sphere_blue",
            Self::Green => "debug_sphere_green",
            Self::Grey => "debug_sphere_grey",
            Self::Custom(_) => "debug_sphere_custom",
        }
    }
}

/// 一次画一个球的请求（0x28 B）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DebugSphere {
    pub color: DebugColor,
    pub radius: f32,
    pub center: Vec3,
    pub alpha: f32,
    pub solid: bool,
}

impl DebugSphere {
    /// O 0x140b02490。
    pub fn new(center: Vec3) -> Self {
        Self {
            color: DebugColor::White,
            radius: 0.1,
            center,
            alpha: 1.0,
            solid: false,
        }
    }

    /// O 0x140b024c0。
    pub fn color(mut self, color: DebugColor) -> Self {
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
    pub color: DebugColor,
    pub line_width: f32,
    pub rect: [f32; 8],
    pub dashed: bool,
}

impl DebugRectangle3d {
    /// O 0x140b02e10。
    pub fn new(rect: [f32; 8]) -> Self {
        Self {
            color: DebugColor::White,
            line_width: 1.0,
            rect,
            dashed: false,
        }
    }

    /// O 0x140b02e40。
    pub fn color(mut self, color: DebugColor) -> Self {
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
    pub color: DebugColor,
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
            color: DebugColor::White,
            line_width: 1.0,
            ui_pass: true,
            dashed: false,
        }
    }

    /// O 0x140b04eb0。
    pub fn color(mut self, color: DebugColor) -> Self {
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
    fn colour_tags_follow_the_original_table_order() {
        let all = [
            DebugColor::White,
            DebugColor::Red,
            DebugColor::Blue,
            DebugColor::Green,
            DebugColor::Grey,
            DebugColor::Custom(Vec3::new(0.2, 0.4, 0.6)),
        ];
        assert_eq!(all.map(DebugColor::tag), [0, 1, 2, 3, 4, 5]);
        assert_eq!(DebugColor::default(), DebugColor::White);
        assert_eq!(DebugColor::Blue.rgb(), Vec3::new(0.0, 0.0, 1.0));
        assert_eq!(DebugColor::Green.rgb(), Vec3::new(0.0, 1.0, 0.0));
        assert_eq!(DebugColor::Grey.rgb(), Vec3::splat(0.1));
        assert_eq!(all[5].rgb(), Vec3::new(0.2, 0.4, 0.6));
    }

    #[test]
    fn sphere_mesh_names_have_the_original_lengths() {
        let lens = [0x12, 0x10, 0x11, 0x12, 0x11, 0x13];
        let colours = [
            DebugColor::White,
            DebugColor::Red,
            DebugColor::Blue,
            DebugColor::Green,
            DebugColor::Grey,
            DebugColor::Custom(Vec3::ZERO),
        ];
        for (colour, len) in colours.into_iter().zip(lens) {
            assert_eq!(colour.sphere_mesh_name().len(), len, "{colour:?}");
        }
    }

    #[test]
    fn sphere_defaults_match_the_constructor_bytes() {
        let sphere = DebugSphere::new(Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(sphere.radius.to_bits(), 0x3dcccccd);
        assert_eq!(sphere.alpha, 1.0);
        assert!(!sphere.solid);
        let sphere = sphere.radius(2.0).solid().color(DebugColor::Red);
        assert_eq!(
            (sphere.radius, sphere.solid, sphere.color),
            (2.0, true, DebugColor::Red)
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
