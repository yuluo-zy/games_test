//! 已恢复的小生物共享目标和物种资源。
pub mod sheep;

use bevy_ecs::component::Component;
use glam::Vec3;
use serde::{Deserialize, Serialize};

/// B CritterGoal是Option<Vec3>：tag@+0、目标坐标@+4（16B）。
/// 它不持有小生物当前位置；当前位置来自Transform.translation。
#[repr(C)]
#[derive(Component, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct CritterGoal {
    pub goal_tag: u32,
    pub position: Vec3,
}
