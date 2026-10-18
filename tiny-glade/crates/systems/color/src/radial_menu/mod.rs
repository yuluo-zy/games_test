//! radial_menu/mod.rs — 径向调色菜单（B 0x1425f2680 new_separator /
//! 0x1425f26f0 new_flat_roof_rim_ty / 0x1425f27c0 create，fresh 反编译；
//! restorer-state-color 2026-10-17）。
//!
//! - `IconInfo`：40B（0x28）布局（O，new_separator/new_flat_roof_rim_ty 写面）：
//!   `String{len,ptr,cap}`@+0x00、`kind u8`@+0x18、`u8`@+0x19、`u16`@+0x1a、
//!   `u32`@+0x1c、`u32`@+0x20、`u32`@+0x24。
//! - `new_separator`（0x1425f2680，106B）：构造 `{name:"white", kind=5}`。判等。
//! - `new_flat_roof_rim_ty`（0x1425f26f0，201B）：按 `ClutterVersion::is_initial()`
//!   选 `"color_icon/freehand_ty_terrain"`(30B) / `"color_icon/freehand_ty_constant"`
//!   (31B)，`kind=8`。判等。
//! - `ColorRadialMenu::create`（0x1425f27c0）：**未实现**——UI 实体 spawn 依赖
//!   country_core 的 `Ui2dExtent`/`Ui2dImage`/`CustomizationIcon`（当前为 typed-gap
//!   占位），`Commands::spawn<tuple<...>>` 无法按原布局编译（分歧点见 RESTORE.md）。

use bevy_ecs::prelude::*;

use country_core::resources::walls::decorator::ClutterVersion;

/// 图标信息（O：40B 布局；`new_separator`/`new_flat_roof_rim_ty` 逐字节写面）。
#[derive(Clone, Debug, Default)]
pub struct IconInfo {
    /// 图标网格路径名（String：len@+0 / ptr@+8 / cap@+0x10）。
    pub name: String,
    /// kind（O：5=separator、8=类型图标；其余值见 create 的分派面）。
    pub kind: u8,
    /// +0x19（部分构造器使用）。
    pub b19: u8,
    /// +0x1a。
    pub u1a: u16,
    /// +0x1c：u32/f32 值（颜色 id 或开关值；demo 分支写 0x309=777）。
    pub v1c: u32,
    /// +0x20。
    pub v20: u32,
    /// +0x24（new_* 均置 0）。
    pub v24: u32,
}

/// `IconInfo::new_separator` @0x1425f2680（106B，判等）。
///
/// fresh 反编译写面：`"white"`(len 5)、`kind=5`、`+0x24=0`。
pub fn new_separator() -> IconInfo {
    IconInfo {
        name: "white".to_string(),
        kind: 5,
        ..IconInfo::default()
    }
}

/// `IconInfo::new_flat_roof_rim_ty` @0x1425f26f0（201B，判等）。
///
/// fresh 反编译：读 `ClutterVersion::is_initial()`（0x140833230）；
/// true → `"color_icon/freehand_ty_terrain"`(0x1e=30B)；false →
/// `"color_icon/freehand_ty_constant"`(0x1f=31B)。`kind=8`、`+0x24=0`。
/// PDB 符号名 `new_flat_roof_rim_ty` 与图标串 `freehand_ty_*` 错位（登记，不发明
/// 改名）。
pub fn new_flat_roof_rim_ty() -> IconInfo {
    let initial = ClutterVersion::is_initial(ClutterVersion::default());
    IconInfo {
        name: if initial {
            "color_icon/freehand_ty_terrain".to_string()
        } else {
            "color_icon/freehand_ty_constant".to_string()
        },
        kind: 8,
        ..IconInfo::default()
    }
}

/// 径向颜色菜单组件（O：`ColorRadialMenu`；color_menu 0x142291230 的
/// `Query<(Entity, &mut ColorRadialMenu)>` 面）。
#[derive(Component, Clone, Debug, Default)]
pub struct ColorRadialMenu {
    /// 菜单中心（create 的 float 族输入）。
    pub center: glam::Vec2,
    /// 半径。
    pub radius: f32,
    /// 图标列表（stride 0x28）。
    pub icons: Vec<IconInfo>,
}

impl ColorRadialMenu {
    /// `ColorRadialMenu::create` @0x1425f27c0（1947B）。
    ///
    /// **未实现**。fresh 反编译已解出：先统计非分隔符图标数（tag!=5 计数）、
    /// `step=(start+span)*(count-1)/denom`、`AngleRange::from_min_and_len`
    /// (0x1425f2460)、`cosf/sinf` 极坐标定位（半径偏移 size*0.75/0.25、角步
    /// π/4）、`Commands::spawn<tuple<Ui2dExtent,Ui2dImage,CustomizationIcon,
    /// UiDrawOrder>>`、按 tag-3 分派插入图标组件（tag 3..14 全表）。
    /// 分歧点：country_core 的 `Ui2dExtent`/`Ui2dImage`/`CustomizationIcon` 为
    /// typed-gap 占位，无法按原字段布局 spawn；AngleRange 输入来源（start/span/
    /// denom 参数）在反编译中被内联丢失。本实现保留数据组装面，spawn 面标注未知。
    pub fn create(center: glam::Vec2, radius: f32, icons: Vec<IconInfo>) -> Self {
        let _ = (center, radius);
        // O：非分隔符图标计数（tag!=5）。
        let count = icons.iter().filter(|i| i.kind != 5).count();
        let _ = count;
        Self {
            center,
            radius,
            icons,
        }
    }
}

/// `register_required_components` @0x1425f3d60（bevy required-components 递归注册
/// 胶水；`CustomizationIcon` 组件面）。
pub fn register_required_components(world: &mut World) {
    world.register_component::<country_core::resources::color_icon::CustomizationIcon>();
}
