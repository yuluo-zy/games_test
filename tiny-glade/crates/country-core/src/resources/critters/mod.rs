//! 已恢复的小生物共享目标和物种资源。
pub mod butterfly;
pub mod sheep;
pub use butterfly::Butterfly;

/// Original shared zero-sized critter marker, registered at140b22db0 and
/// included in both original startup species bundles.
#[derive(bevy_ecs::component::Component, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Critter;

use bevy_ecs::component::Component;
use glam::Vec3;
use serde::{Deserialize, Serialize};

/// B CritterGoal语义是Option<Vec3>；原内存tag@+0、目标坐标@+4。
/// 这里保留现有类型和字段顺序，不声明原生ABI。它不持有小生物当前位置。
#[derive(Component, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct CritterGoal {
    pub position: Vec3,
    pub goal_tag: u32,
}

pub mod wren;
pub use wren::{Wren, WrenSpotState};

/// Original shared control resource (default140f85b90; updater140f7df84).
#[repr(C)]
#[derive(bevy_ecs::resource::Resource, Clone, Copy, Debug, PartialEq)]
pub struct CritterControl {
    pub intensity: f32,
    pub enabled: bool,
    pub birds_enabled: bool,
}
impl Default for CritterControl {
    fn default() -> Self {
        Self {
            intensity: 0.0,
            enabled: false,
            birds_enabled: true,
        }
    }
}
