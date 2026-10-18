//! 二维 UI 的范围、图像和绘制顺序。

use bevy_ecs::component::Component;
use glam::Vec2;

/// 屏幕空间中的中心位置与尺寸。B 大小 0x10，pos@0、size@8。
#[repr(C)]
#[derive(Component, Debug, Default, PartialEq)]
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
#[derive(Component, Debug, Default, PartialEq, Eq)]
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
#[derive(Component, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct UiDrawOrder(pub u64);
