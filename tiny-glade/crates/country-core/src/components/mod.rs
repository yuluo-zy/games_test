//! 场景实体的共享 ECS 数据：可绘制对象、树和二维 UI。
//!
//! 组件保存数据或标记，更新逻辑由系统消费；Bundle 显式组合创建所需的组件。
//! [`registry`] 是原作 B 的逆向证据清单，不负责运行时注册。
//! 字段证据和本轮修正见 crate 的 `recovery/components-2026-10-18.md`。

pub mod drawable;
pub mod registry;
pub mod rendering;
pub mod trees;
pub mod ui;

pub use rendering::{
    FollowMouse,
    BackgroundTerrain, DashedLineLength, DontReceiveContactShadows, FlatLighting,
    OverwriteRoughness, SemiTransparent, StairsStep,
};
pub use trees::{
    ConvertedPreplacedTree, TreeAnimation, TreeComponent, TreeMarkedForDeletion, TreeMoveBend,
};
pub use ui::{
    Ui2dExtent, Ui2dImage, Ui2dWsExtent, UiDrawOrder, UiInactive, UiInteractionState,
    UiMouseInteractable,
};

/// B KeepOnLevelUnload, original component registration @0x140852670:
/// size 0, alignment 1, no drop; animated-logo startup retains this entity.
#[derive(bevy_ecs::component::Component, Clone, Copy, Debug, Default)]
pub struct KeepOnLevelUnload;

/// B `components::TestCube`（`register_required_components` 0x140845610，registry.rs
/// 第 2 项）。`ui_adjust_wall_height_mode` / `history_record_adjust_wall_height` 的
/// `Query<(&TestCube, &mut Transform)>` 形参只占位、函数体均不读写；字段未取证，
/// 故只建标记组件。
#[derive(bevy_ecs::component::Component, Clone, Copy, Debug, Default)]
pub struct TestCube;

// 保持已有公共入口；权威定义分别属于 prefab 系统和树编辑资源。
pub use crate::resources::history::tree_edit::TreeCanopyMaxCullDistance;
pub use crate::systems::prefab::PrefabInstanceData;

/// Canonical material normal map used by startup and draw consumers.
pub use rendering::NormalMap;

/// 原WoodenGate的完整业务值：constructor写入、clutter与描边查询和绘制共用。
/// B原56B包含顶边点、墙ID、两端方向、seed和版本，不再拆成互不相通的组件。
#[derive(bevy_ecs::component::Component, Clone, Debug, PartialEq)]
#[component(on_insert = crate::render::extract::record_component_change::<Self>,
    on_remove = crate::render::extract::record_component_change::<Self>)]
pub struct WoodenGate {
    pub top_positions: Vec<[f32; 3]>,
    pub wall: crate::resources::walls::WallId,
    pub endpoint_directions: [[f32; 2]; 2],
    pub seed: u32,
    pub clutter_version: u8,
}

impl crate::render::extract::JournaledComponent for WoodenGate {}

/// 原Smoke注册140854230：size16、align8；builder1412ab270写0/8/12/13，
/// 厚度系统140818df0读key与enabled并回写thickness，Nani14207cd60读kind/id/厚度。
/// 尾部两字节是repr(C)对齐，不是额外业务载荷，不复用旧14B旁路组件。
#[repr(C)]
#[derive(bevy_ecs::component::Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct Smoke {
    pub key: usize,
    pub thickness: f32,
    pub kind: u8,
    pub enabled: u8,
}

const _: () = assert!(core::mem::size_of::<Smoke>() == 16);
const _: () = assert!(core::mem::align_of::<Smoke>() == 8);

/// 原BrickColor的24B活载荷；前两word为整数，+8/+12/+16为浮点，+20为标志字节。
/// 与窗口builder和两种砖实例Query共用TypeId，不声明原Rust ABI。
#[derive(bevy_ecs::component::Component, Clone, Copy, Debug)]
pub struct BrickColor(pub [u8; 24]);

impl BrickColor {
    pub fn words(&self) -> [u32; 6] {
        core::array::from_fn(|index| u32::from_le_bytes(self.0[index * 4..index * 4 + 4].try_into().unwrap()))
    }
}

/// 原GothicWindowBrickDeformation完整60B；六个边向量值和三组真实世界点由原producer写入。
#[derive(bevy_ecs::component::Component, Clone, Copy, Debug)]
pub struct GothicWindowBrickDeformation(pub [u8; 60]);

impl GothicWindowBrickDeformation {
    pub fn words(&self) -> [u32; 15] {
        core::array::from_fn(|index| u32::from_le_bytes(self.0[index * 4..index * 4 + 4].try_into().unwrap()))
    }
}

pub mod brush_previews;
pub use brush_previews::{SnappingToEndPreview, SnappingToSegmentPreview};

pub use ui::Ui2DFollowMouse;
pub use rendering::MeshFollowTerrain;
