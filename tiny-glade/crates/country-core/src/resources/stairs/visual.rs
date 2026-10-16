//! `visual.rs` — 楼梯可视化资源（B `resources/stairs/visual.rs`，4 业务 + 6 胶水）。
//!
//! 章节：`chapters/resources/stairs/visual.rs.md`（O）。
//! - `MagicSupports::clear`（0x140aa8210）：清空魔法支撑集合（重建/失效时调用）；
//! - `StairsVisualState::remove_edge`（0x140aa8620）：删除指定边的视觉状态
//!   （invalidate 系统经 `contains_edge_or_stub` 判定后调用）；
//! - `StairsVisualState::reset_edge`（0x140aa8a60）：边视觉重置（v1 force_rebuild 名错）。
//!
//! **登记待接线（胶水）**：`remove_edge`/`reset_edge` 的 RtWorld RtMesh 删除、
//! mesh atlas 句柄释放与 DirtyCow 交互属 render/rt 域（库内未建 `resources/rt`），
//! 本文件只实现边视觉存储的增删核心 + 脏字节（+0x30）。
use crate::resources::stairs::stairs_state::StairGraphEdge;
use bevy_ecs::{component::Component, resource::Resource};
use hashbrown::{HashMap, HashSet};

/// `MagicSupports`（魔法支撑集合；清空在重建/失效时调用）。
#[derive(Resource, Clone, Debug, Default)]
pub struct MagicSupports {
    /// 支撑条目（句柄语义 U：render 域消费；仅存储核心）。
    entries: HashSet<StairGraphEdge>,
}

impl MagicSupports {
    /// `MagicSupports::clear`（0x140aa8210，O 存储核心）：清空魔法支撑集合。
    pub fn clear(&mut self) {
        self.entries.clear();
    }
    pub fn insert(&mut self, edge: StairGraphEdge) {
        self.entries.insert(edge);
    }
    pub fn remove(&mut self, edge: &StairGraphEdge) -> bool {
        self.entries.remove(edge)
    }
    pub fn contains(&self, edge: &StairGraphEdge) -> bool {
        self.entries.contains(edge)
    }
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    pub fn len(&self) -> usize {
        self.entries.len()
    }
}

/// 边视觉条目（RtWorld/mesh 句柄域；字段语义 U，待 render 域接线）。
#[derive(Clone, Debug, Default)]
pub struct StairsEdgeVisual {
    /// 占位——实际为 RtMesh 句柄等（U）。
    pub(crate) _opaque: (),
}

/// `StairsVisualState`（边 → 视觉状态 + 脏字节）。
#[derive(Resource, Clone, Debug, Default)]
pub struct StairsVisualState {
    /// +0x30：脏字节（O：remove_edge 处理末置 1）。
    dirty: bool,
    /// 边视觉状态表。
    edges: HashMap<StairGraphEdge, StairsEdgeVisual>,
}

impl StairsVisualState {
    /// `StairsVisualState::remove_edge`（0x140aa8620，O 存储核心）：删除指定边的视觉状态
    /// （RtWorld/mesh 撤除胶水登记），并置脏字节。
    pub fn remove_edge(&mut self, edge: StairGraphEdge) {
        self.edges.remove(&edge);
        self.dirty = true;
    }

    /// `StairsVisualState::reset_edge`（0x140aa8a60，O 存储核心）：边视觉重置
    /// （旧名 force_rebuild；重建/重新插入条目），并置脏字节。
    pub fn reset_edge(&mut self, edge: StairGraphEdge) {
        self.edges.insert(edge, StairsEdgeVisual::default());
        self.dirty = true;
    }

    /// `force_rebuild`（内联，行 132）：标记边重建（= reset_edge 存储语义）。
    pub fn force_rebuild(&mut self, edge: StairGraphEdge) {
        self.reset_edge(edge);
    }

    pub fn get(&self, edge: &StairGraphEdge) -> Option<&StairsEdgeVisual> {
        self.edges.get(edge)
    }
    pub fn contains(&self, edge: &StairGraphEdge) -> bool {
        self.edges.contains_key(edge)
    }
    pub fn edges(&self) -> &HashMap<StairGraphEdge, StairsEdgeVisual> {
        &self.edges
    }
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }
    pub fn clear_dirty(&mut self) {
        self.dirty = false;
    }
}

/// `StairsVisualComponent`（B `impl$6::register_required_components` 注册胶水的组件面）。
#[derive(Component, Clone, Debug, Default)]
pub struct StairsVisualComponent {
    pub(crate) _marker: (),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn magic_supports_clear_empties() {
        let mut supports = MagicSupports::default();
        supports.insert(StairGraphEdge::new(1, 2));
        supports.insert(StairGraphEdge::new(3, 4));
        assert_eq!(supports.len(), 2);
        supports.clear();
        assert!(supports.is_empty());
    }

    #[test]
    fn visual_state_remove_and_reset() {
        let mut state = StairsVisualState::default();
        let edge = StairGraphEdge::new(1, 2);
        state.reset_edge(edge);
        assert!(state.contains(&edge));
        assert!(state.is_dirty());
        state.clear_dirty();
        state.remove_edge(edge);
        assert!(!state.contains(&edge));
        assert!(state.is_dirty());
    }
}
