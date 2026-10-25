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


