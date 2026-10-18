//! 树的身份、生长/树冠动画及移动弯曲状态。
//!
//! B 的注册描述符和创建/读取指令共同确定载荷；这里的字段名是恢复命名。
//! 组件的 ECS 克隆策略保持 Default，不从载荷的可复制性推导 Clone。

use asset::prefab::PrefabName;
use bevy_ecs::component::Component;
use glam::Vec2;

/// 树与 TreeStorage 条目的关联。B：id@0、is_preplaced@8，大小 0x10。
#[repr(C)]
#[derive(Component, Debug, PartialEq, Eq)]
pub struct TreeComponent {
    pub id: u64,
    pub is_preplaced: bool,
}

/// 预置树转换时使用的原 prefab 名；持有共享名称的所有权。
///
/// B 注册大小为 0x10，spawn_preplaced_trees 克隆 Arc，转换时读取名称。
#[repr(transparent)]
#[derive(Component, Debug, PartialEq, Eq)]
pub struct ConvertedPreplacedTree(pub PrefabName);

/// 删除请求标记。生长进度衰减至严格小于 0.01 时才回收实体。
#[derive(Component, Debug, Default, PartialEq, Eq)]
pub struct TreeMarkedForDeletion;

/// 手放树的生长与树冠可见性，B 载荷为 0x10 字节、对齐 4。
///
/// process_tree_cmds 初始化时长和零进度；动画系统更新后两项并清除隐藏请求。
/// despawn_trees 读取 progress@4，而不是 duration@0。
#[repr(C)]
#[derive(Component, Debug, PartialEq)]
pub struct TreeAnimation {
    /// 生长耗时，秒。
    pub duration: f32,
    /// 生长/收缩进度；删除系统读取此值。
    pub progress: f32,
    /// 树冠可见性的线性进度；渲染打包前再做二次缓动。
    pub canopy_visibility: f32,
    /// 本帧隐藏请求，消费后清零。B 偏移 +0x0c。
    pub hidden_this_frame: bool,
}

impl TreeAnimation {
    /// 新放置的树从零进度开始；调用方提供原作计算的生长时长。
    pub fn new(duration: f32) -> Self {
        Self {
            duration,
            progress: 0.0,
            canopy_visibility: 0.0,
            hidden_this_frame: false,
        }
    }

    /// B spawn_preplaced_trees 写入的初始值：0.75 / 1 / 1 / false。
    pub fn preplaced() -> Self {
        Self {
            duration: 0.75,
            progress: 1.0,
            canopy_visibility: 1.0,
            hidden_this_frame: false,
        }
    }

    /// 推进原作逐树动画，返回生长或树冠可见性是否发生变化。
    ///
    /// B 0x142279cdf–0x142279dc9。隐藏/删除以 -0.4 秒收缩；
    /// 生长完成且允许显示时淡入。无变化也会消费本帧隐藏标志。
    pub fn advance(&mut self, delta: f32, marked: bool, show_canopy: bool) -> bool {
        let old_progress = self.progress;
        let old_visibility = self.canopy_visibility;
        let step = delta
            / if marked || self.hidden_this_frame {
                -0.4
            } else {
                self.duration
            };
        self.progress = (self.progress + step).clamp(0.0, 1.0);
        self.canopy_visibility = if show_canopy && self.progress == 1.0 {
            (self.canopy_visibility + delta).min(1.0)
        } else {
            (self.canopy_visibility - delta).clamp(0.0, 1.0)
        };
        self.hidden_this_frame = false;
        self.progress != old_progress || self.canopy_visibility != old_visibility
    }
}

/// 树移动时追随目标位置的 XZ 缓动位置，供弯曲系统计算位移。
///
/// B tree_move_bend 0x1422689c0 按两个 f32 读写，大小 8、对齐 4。
#[repr(transparent)]
#[derive(Component, Debug, Default, PartialEq)]
pub struct TreeMoveBend {
    pub position: Vec2,
}

// 兼容旧入口；新代码从 components 或各权威模块导入。
pub use super::ui::{Ui2dExtent, Ui2dImage, UiDrawOrder};
pub use crate::resources::history::tree_edit::TreeCanopyMaxCullDistance;
pub use crate::systems::prefab::PrefabInstanceData;
