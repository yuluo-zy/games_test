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

pub mod brush_previews;
pub use brush_previews::{SnappingToEndPreview, SnappingToSegmentPreview};

pub use ui::Ui2DFollowMouse;
pub use rendering::MeshFollowTerrain;


/// 原 BrickColor 的六个原字；同一组件由装饰物构建器写入并由 Nani 两种砖材质消费。
/// 保留末字全部位，不能在共享组件层把某一个 shader 的位选择规则套到所有消费者。
#[repr(transparent)]
#[derive(bevy_ecs::component::Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct BrickColor([u32; 6]);

impl BrickColor {
    pub const fn from_words(words: [u32; 6]) -> Self { Self(words) }
    pub const fn words(self) -> [u32; 6] { self.0 }

    /// 构建器原载荷保持小端；逐字搬运，不转换浮点或归一化标志。
    pub fn from_native_bytes(bytes: [u8; 24]) -> Self {
        Self(std::array::from_fn(|index| {
            let offset = index * 4;
            u32::from_le_bytes([bytes[offset], bytes[offset + 1], bytes[offset + 2], bytes[offset + 3]])
        }))
    }
}

/// 原 Gothic 主函数重排后的十五标量，不能直接使用 gethat 返回布局。
/// 数组字段保持原四字节对齐，避免 Vec4 改变该 60 字节组件的布局。
#[repr(C)]
#[derive(bevy_ecs::component::Component, Clone, Copy, Debug, PartialEq)]
pub struct GothicWindowBrickDeformation {
    pub side: [f32; 4],
    pub edge: [f32; 2],
    pub first: [f32; 3],
    pub center: [f32; 3],
    pub second: [f32; 3],
}

impl GothicWindowBrickDeformation {
    pub fn from_words(words: [u32; 15]) -> Self {
        let f = words.map(f32::from_bits);
        Self { side: [f[0], f[1], f[2], f[3]], edge: [f[4], f[5]],
            first: [f[6], f[7], f[8]], center: [f[9], f[10], f[11]],
            second: [f[12], f[13], f[14]] }
    }

    pub fn words(self) -> [u32; 15] {
        [self.side[0], self.side[1], self.side[2], self.side[3], self.edge[0], self.edge[1],
            self.first[0], self.first[1], self.first[2], self.center[0], self.center[1], self.center[2],
            self.second[0], self.second[1], self.second[2]].map(f32::to_bits)
    }

    pub fn from_native_bytes(bytes: [u8; 60]) -> Self {
        Self::from_words(std::array::from_fn(|index| {
            let offset = index * 4;
            u32::from_le_bytes([bytes[offset], bytes[offset + 1], bytes[offset + 2], bytes[offset + 3]])
        }))
    }
}

#[cfg(test)]
mod brick_component_tests {
    use super::{BrickColor, GothicWindowBrickDeformation};

    #[test]
    fn canonical_brick_components_keep_native_word_layout_and_nonfinite_bits() {
        assert_eq!(std::mem::size_of::<BrickColor>(), 24);
        assert_eq!(std::mem::align_of::<BrickColor>(), 4);
        assert_eq!(std::mem::size_of::<GothicWindowBrickDeformation>(), 60);
        assert_eq!(std::mem::align_of::<GothicWindowBrickDeformation>(), 4);
        let brick: [u32; 6] = [0x12345678, 0x87654321, 0x80000000, 0x7fc00041, 0xff800000, 0xaabbcc01];
        let bytes: [u8; 24] = std::array::from_fn(|i| brick[i / 4].to_le_bytes()[i % 4]);
        assert_eq!(BrickColor::from_native_bytes(bytes).words(), brick);
        let deformation: [u32; 15] = [0x80000000, 0x7fc00041, 0xffc00042, 0x7f800000, 0xff800000,
            0x3f800000, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let bytes: [u8; 60] = std::array::from_fn(|i| deformation[i / 4].to_le_bytes()[i % 4]);
        assert_eq!(GothicWindowBrickDeformation::from_native_bytes(bytes).words(), deformation);
    }

    #[test]
    fn gothic_main_layout_is_side_edge_then_three_points() {
        let words = std::array::from_fn(|i| ((i + 1) as f32).to_bits());
        let deformation = GothicWindowBrickDeformation::from_words(words);
        assert_eq!(deformation.side, [1., 2., 3., 4.]);
        assert_eq!(deformation.edge, [5., 6.]);
        assert_eq!(deformation.first, [7., 8., 9.]);
        assert_eq!(deformation.center, [10., 11., 12.]);
        assert_eq!(deformation.second, [13., 14., 15.]);
        assert_eq!(deformation.words(), words);
    }
}
