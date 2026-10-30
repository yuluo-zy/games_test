//! 二维 UI 的范围、图像和绘制顺序。

use bevy_ecs::component::Component;
use glam::Vec2;

/// 屏幕空间中的中心位置与尺寸。B 大小 0x10，pos@0、size@8。
#[repr(C)]
#[derive(Clone, Component, Debug, Default, PartialEq)]
#[component(on_insert = crate::render::extract::record_component_change::<Self>,
    on_remove = crate::render::extract::record_component_change::<Self>)]
pub struct Ui2dExtent {
    pub pos: Vec2,
    pub size: Vec2,
}

/// Canonical B component identity. register_component @0x141e35eb0 records
/// four bytes, alignment four; ui_2d_pass consumes byte +2. The opaque payload
/// keeps that observed schema until each byte's producer is identified.
#[repr(C, align(4))]
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct UiMouseInteractable {
    pub bytes: [u8; 4],
}

const _: () = assert!(std::mem::size_of::<UiMouseInteractable>() == 4);

/// UI 纹理资源名和数组图层。
///
/// B 大小 0x20；String 占前 24 字节，layer@0x18。持有字符串，不能作为
/// Copy 数据或无载荷标记；ECS 克隆策略保持原作 Default。
#[repr(C)]
#[derive(Clone, Component, Debug, Default, PartialEq, Eq)]
#[component(on_insert = crate::render::extract::record_component_change::<Self>,
    on_remove = crate::render::extract::record_component_change::<Self>)]
pub struct Ui2dImage {
    pub name: String,
    pub layer: u32,
}

impl Ui2dImage {
    /// B 内联构造 Ui2dImage::new：移入名称，图层初值为零。
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            layer: 0,
        }
    }
}

/// 二维 UI 的 64 位排序键；缺少该组件的元素排在有排序键的元素之前。
///
/// B register 0x140850aa0 记录大小/对齐均为 8；绘制端按 u64 比较。
#[repr(transparent)]
#[derive(Clone, Component, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
#[component(on_insert = crate::render::extract::record_component_change::<Self>,
    on_remove = crate::render::extract::record_component_change::<Self>)]
pub struct UiDrawOrder(pub u64);

/// B register@0x14084e100: zero-sized marker, alignment 1. The render query
/// binds its presence to U_INACTIVE; there is no payload to reconstruct.
#[derive(Component, Clone, Copy, Debug, Default)]
#[component(on_insert = crate::render::extract::record_component_change::<Self>,
    on_remove = crate::render::extract::record_component_change::<Self>)]
pub struct UiInactive;

/// B register@0x14084eee0: size 0x14, align 4. ui_2d_pass reads extent@0
/// as Vec2 and world position@8 as Vec3 without sorting this query.
#[repr(C)]
#[derive(Component, Clone, Copy, Debug, Default)]
#[component(on_insert = crate::render::extract::record_component_change::<Self>,
    on_remove = crate::render::extract::record_component_change::<Self>)]
pub struct Ui2dWsExtent {
    pub extent: Vec2,
    pub position: glam::Vec3,
}

/// Canonical B UiInteractionState (register@0x14084fcc0): three bytes,
/// alignment 1. Toolbar writers and the UI raster query share this owner.
#[repr(C)]
#[derive(Component, Clone, Copy, Debug, Default)]
#[component(on_insert = crate::render::extract::record_component_change::<Self>,
    on_remove = crate::render::extract::record_component_change::<Self>)]
pub struct UiInteractionState {
    pub state: u8,
    pub keep: u8,
    pub selected: u8,
}

const _: () = assert!(std::mem::size_of::<UiInactive>() == 0);
const _: () = assert!(std::mem::size_of::<Ui2dWsExtent>() == 0x14);
const _: () = assert!(std::mem::size_of::<UiInteractionState>() == 3);

/// Original zero-sized query filter marker; no independent runtime owner.
#[derive(bevy_ecs::component::Component, Clone, Copy, Debug, Default)]
pub struct Ui2DFollowMouse;

impl crate::render::extract::JournaledComponent for Ui2dExtent {}
impl crate::render::extract::JournaledComponent for Ui2dImage {}
impl crate::render::extract::JournaledComponent for Ui2dWsExtent {}
impl crate::render::extract::JournaledComponent for UiDrawOrder {}
impl crate::render::extract::JournaledComponent for UiInactive {}
impl crate::render::extract::JournaledComponent for UiInteractionState {}



/// 单条导航链接记录（O：`UiNav` 每 16B 槽；fresh `link_up_nav` 0x141e4d880
/// 写面：`{tag: u64@+0, value: u64@+8}`）。
///
/// 观测 tag 值：`0`（未连接）与 `3`（已连接，邻居 id 在 value）。tag 3 的枚举
/// 语义（3 变体枚举的 niche None / 具体变体名）**U**；本实现保留原字节值。
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UiNavLink {
    /// +0 判别/标签（观测 0 / 3）。
    pub tag: u64,
    /// +8 邻居 GlobalId（tag==3 时有效）。
    pub value: u64,
}

const _UI_NAV_LINK_SIZE: () = assert!(std::mem::size_of::<UiNavLink>() == 0x10);

/// UiNav 组件（O：**`country_core::components::UiNav`**——PDB 路径见
/// `render_taffy_ui_to_widgets` Query 面与 `link_up_nav` insert 命令泛型；
/// 由 country-core 统一拥有）。
///
/// **B 布局（0x40 = 64B，fresh `register_component<UiNav>` 0x141e36510
/// `ComponentDescriptor` size 立即数 0x40）**：四槽 `UiNavLink`（各 16B）。
///
/// 写面（`ToolbarLayout::link_up_nav` 0x141e4d880，O）：
/// - 纵向（`in_DL==0`）：slot0/slot1 tag=0；slot2={3,next}、slot3={3,prev}；
/// - 横向（`in_DL!=0`）：slot0={3,prev}、slot1={3,next}；slot2/slot3 tag=0。
///
/// 槽语义为 left/right/down/up，已由 ui_nav_update 14233f537..58e 确认。
/// toolbar_setup 142346fb4..fc2 将 is_horizontal 的 AL 直接传为 DL；旧恢复
/// 误将 DL==0 标作横向，导致真实左右键绕过环链接。调用与写面联合裁决纠正。
///
/// **I 级残差**：`link_up_nav` 对“未连接”槽只显式写 tag qword = 0（横向 slot0/1、
/// 纵向 slot2/3），其 value qword 在原栈记录中未显式写（栈初值 U）。本实现取 0
/// （`Default`），为可复现的结构化选择，登记为 I。
#[derive(bevy_ecs::component::Component, Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(C)]
pub struct UiNav {
    /// 四条链接（布局 O；语义名 U）。
    pub links: [UiNavLink; 4],
}

const _UI_NAV_SIZE: () = assert!(std::mem::size_of::<UiNav>() == 0x40);

impl UiNav {
    /// `link_up_nav(in_DL==0)` 的纵向槽写面：down=next / up=prev。
    pub fn vertical(prev: u64, next: u64) -> Self {
        Self {
            links: [
                UiNavLink::default(),
                UiNavLink::default(),
                UiNavLink { tag: 3, value: next },
                UiNavLink { tag: 3, value: prev },
            ],
        }
    }

    /// `link_up_nav(in_DL!=0)` 的横向槽写面：left=prev / right=next。
    pub fn horizontal(prev: u64, next: u64) -> Self {
        Self {
            links: [
                UiNavLink { tag: 3, value: prev },
                UiNavLink { tag: 3, value: next },
                UiNavLink::default(),
                UiNavLink::default(),
            ],
        }
    }
}

