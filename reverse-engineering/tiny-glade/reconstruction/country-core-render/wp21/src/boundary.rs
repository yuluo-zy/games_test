//! boundary.rs — 本包骨架引用的共享/域侧类型面。
//!
//! 集成 owner 用整合 crate 的真实类型替换（walls::WallId、decorator::DecoratorId、
//! utils::geometry::{aabb::Aabb2, rectangle::Rectangle3d}、collision world 等）。
//! 这些是游戏自有 utils 或已恢复类型，不重新实现。

use glam::{Vec2, Vec3};

/// `WallId`（O：u64 newtype；`get_inner` 0x140885d0 / `is_valid` u64!=0）。本包
/// 只是复用名；整合 crate 已恢复（walls::WallId）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct WallId(pub u64);

impl WallId {
    #[inline]
    pub fn get_inner(self) -> u64 {
        self.0
    }
    #[inline]
    pub fn is_valid(self) -> bool {
        self.0 != 0
    }
}

/// `DecoratorId`（O：u32 newtype；stairs 图节点值类型）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DecoratorId(pub u32);

/// 游戏自有 `utils::geometry::aabb::Aabb2`（16B：min/max Vec2）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Aabb2 {
    pub min: Vec2,
    pub max: Vec2,
}

impl Aabb2 {
    /// `Aabb2::from_center_dims`（O：wall_holes::finalize 调用，utils 游戏自有）。
    pub fn from_center_dims(center: Vec2, dims: Vec2) -> Self {
        Self {
            min: center - dims * 0.5,
            max: center + dims * 0.5,
        }
    }
}

/// `utils::geometry::rectangle::Rectangle3d`（decorator_collision 引用；U 布局）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Rectangle3d(pub [f32; 6]);

/// 碰撞世界句柄（`systems::collision::world::SubworldColliderHandle`；枚举，17=空）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ColliderHandle {
    Empty,
    Handle(u64),
}

impl ColliderHandle {
    /// O：空判别字节 0x11。
    pub const EMPTY: u8 = 0x11;
}

/// `systems::collision::world::RaycastMeshData`（Arc 负载；U 布局，复用边界）。
#[derive(Clone, Debug, Default)]
pub struct RaycastMeshData(pub crate::typed_gap::Gap);

/// 楼梯可视化状态（`stairs::visual::StairsVisualState`；域侧类型，U）。
#[derive(Clone, Debug, Default)]
pub struct StairsVisualState(pub crate::typed_gap::Gap);

impl StairsVisualState {
    /// `StairsVisualState::remove_edge`（O 名；invalidate 调用）。
    pub fn remove_edge(&mut self, _edge: &super::resources::stairs::stairs_state::StairGraphEdge) {}
}
