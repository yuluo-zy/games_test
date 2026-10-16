//! radial_menu/mod.rs — 径向调色菜单（B 0x1425f2680/0x1425f26f0/0x1425f27c0/
//! 0x1425f3d60，O；re-analyst-A 契约 §2/§4）。
//!
//! - `IconInfo`：{String 名, u8 kind@+0x18, u32 @+0x24}；kind 5 = separator
//!   （'white' 串 0x1425f2680）、8 = 类型图标（freehand_ty_constant(31B)/
//!   freehand_ty_terrain(30B)——**PDB 名 new_flat_roof_rim_ty 与串 freehand_ty
//!   名实错位**，按 freehand_ty 语义落码（coordinator/机器码为准））。
//! - `ColorRadialMenu::create`（0x1425f27c0，13 参 float 族 = 中心/半径/尺寸）：
//!   AngleRange::from_min_and_len（0x1425f2460）+ 图标实体生成。
//! - `register_required_components`（0x1425f3d60，胶水）。

use bevy_ecs::prelude::*;
use glam::Vec2;

use country_core::resources::color_icon::CustomizationIcon;

/// 图标信息（O：{String 名, u8 kind@+0x18, u32 @+0x24}；re-analyst §2）。
#[derive(Clone, Debug, Default)]
pub struct IconInfo {
    /// 图标网格路径名（O：图标串全表，re-analyst §2）。
    pub name: String,
    /// kind（O：5=separator、8=类型图标；其余 U）。
    pub kind: u8,
    /// @+0x24 载荷（O 存在，语义 U）。
    pub extra: u32,
}

impl IconInfo {
    /// `IconInfo::new_separator` @0x1425f2680（O：'white' 串，kind=5）。
    pub fn new_separator() -> Self {
        Self {
            name: "white".to_string(),
            kind: 5,
            extra: 0,
        }
    }

    /// `IconInfo::new_flat_roof_rim_ty` @0x1425f26f0（**PDB 名与串错位**——
    /// 实为 freehand 类型图标：'freehand_ty_constant'(31B) / 'freehand_ty_terrain'
    /// (30B)，kind=8）。
    pub fn new_freehand_ty(terrain: bool) -> Self {
        Self {
            name: if terrain {
                "freehand_ty_terrain".to_string()
            } else {
                "freehand_ty_constant".to_string()
            },
            kind: 8,
            extra: 0,
        }
    }

    /// `IconInfo::new_color`（行 93，内联）：颜色图标。
    pub fn new_color(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            kind: 1,
            extra: 0,
        }
    }

    /// `IconInfo::new_color_array`（行 101，内联）：颜色图标组。
    pub fn new_color_array(names: &[&str]) -> Vec<Self> {
        names.iter().map(|n| Self::new_color(*n)).collect()
    }

    /// `IconInfo::new_style`（行 109，内联）：样式图标。
    pub fn new_style(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            kind: 2,
            extra: 0,
        }
    }

    /// `IconInfo::new_plaster_peel`（行 117，内联）：plaster_peel_{low,medium,high}。
    pub fn new_plaster_peel(level: &str) -> Self {
        Self::new_style(format!("plaster_peel_{level}"))
    }

    /// `IconInfo::new_flatroof_style`（行 125，内联）：平顶样式图标。
    pub fn new_flatroof_style(name: impl Into<String>) -> Self {
        Self::new_style(name)
    }

    /// `IconInfo::new_flag_connection`（行 141，内联）：flag_{no_}connection。
    pub fn new_flag_connection(connected: bool) -> Self {
        Self::new_style(if connected {
            "flag_connection"
        } else {
            "flag_no_connection"
        })
    }

    /// `IconInfo::chimney_smoke_toggle`（行 153，内联）：chimney_smoke_{inactive,active}。
    pub fn chimney_smoke_toggle(active: bool) -> Self {
        Self::new_style(if active {
            "chimney_smoke_active"
        } else {
            "chimney_smoke_inactive"
        })
    }

    /// `IconInfo::stair_railing`（行 165，内联）：railing_1..4。
    pub fn stair_railing(index: u8) -> Self {
        Self::new_style(format!("railing_{index}"))
    }

    /// `IconInfo::new_brightness`（行 202，内联）：亮度图标（0.25/1.0/4.0）。
    pub fn new_brightness(level: &str) -> Self {
        Self::new_style(level)
    }

    /// `IconInfo::magic_color`（行 210，内联）。
    pub fn magic_color() -> Self {
        Self::new_style("magic_color")
    }
}

/// 径向颜色菜单组件（O：`ColorRadialMenu`；color_menu 0x142291230 的
/// `Query<(Entity, &mut ColorRadialMenu)>` 面；字段 U——中心/半径/图标表，
/// re-analyst §2 的 AngleRange 布局 I）。
#[derive(Component, Clone, Debug, Default)]
pub struct ColorRadialMenu {
    /// 菜单中心（I）。
    pub center: Vec2,
    /// 菜单半径（I）。
    pub radius: f32,
    /// 图标列表（I）。
    pub icons: Vec<IconInfo>,
}

impl ColorRadialMenu {
    /// `ColorRadialMenu::create` @0x1425f27c0（O 面：中心/半径/尺寸 float 族 +
    /// AngleRange::from_min_and_len；图标实体生成细节 U——Commands::spawn ×2 +
    /// 0x140ec30e0/0x140ec4530/0x141c5a430 UI 辅助）。
    pub fn create(center: Vec2, radius: f32, icons: Vec<IconInfo>) -> Self {
        Self {
            center,
            radius,
            icons,
        }
    }
}

/// `register_required_components` @0x1425f3d60（O：bevy required-components 递归
/// 注册；CustomizationIcon 组件面）。
pub fn register_required_components(world: &mut World) {
    world.register_component::<CustomizationIcon>();
}
