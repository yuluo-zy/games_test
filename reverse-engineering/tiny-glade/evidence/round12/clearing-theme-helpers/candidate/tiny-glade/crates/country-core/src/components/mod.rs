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

// 保持已有公共入口；权威定义分别属于 prefab 系统和树编辑资源。
pub use crate::resources::history::tree_edit::TreeCanopyMaxCullDistance;
pub use crate::systems::prefab::PrefabInstanceData;

/// Canonical material normal map used by startup and draw consumers.
pub use rendering::NormalMap;
