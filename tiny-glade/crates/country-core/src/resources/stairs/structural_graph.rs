//! 楼梯支撑碰撞体图；原 B 140869a30..14086a4c0 的七个业务入口。
//! span 身份包含边和采样索引，连接的一位权重区分永久与临时支撑。
//! 原完整证据与字段合同见 TG-d1ffc27432f3d739f8/core-stair-structure-20261021。
use crate::{
    resources::stairs::stairs_state::StairGraphEdge,
    systems::collision::world::SubworldColliderHandle,
};
use bevy_ecs::resource::Resource;
use hashbrown::{HashMap, HashSet};
use petgraph::graphmap::UnGraphMap;
use std::{
    collections::hash_map::RandomState,
    hash::{Hash, Hasher},
};

/// 楼梯碰撞体句柄（= `SubworldColliderHandle` 三字段；排序键 = {slot, index, generation}，
/// O：`add_edge_by_collider` 0x14086a260 按 byte2/slot → u32[0]/index → u32[1]/generation）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StairColliderHandle {
    pub index: u32,
    pub generation: u32,
    pub slot: u8,
}

impl StairColliderHandle {
    pub fn new(index: u32, generation: u32, slot: u8) -> Self {
        Self {
            index,
            generation,
            slot,
        }
    }
    pub fn as_subworld(self) -> SubworldColliderHandle {
        SubworldColliderHandle::new(self.index, self.generation, self.slot)
    }
}

impl From<SubworldColliderHandle> for StairColliderHandle {
    fn from(handle: SubworldColliderHandle) -> Self {
        Self {
            index: handle.index,
            generation: handle.generation,
            slot: handle.slot,
        }
    }
}

impl PartialOrd for StairColliderHandle {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for StairColliderHandle {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.slot, self.index, self.generation).cmp(&(other.slot, other.index, other.generation))
    }
}
impl Hash for StairColliderHandle {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // 排序/哈希一致（机器码排序先 slot）。
        self.slot.hash(state);
        self.index.hash(state);
        self.generation.hash(state);
    }
}

/// 原 span 为 `(StairGraphEdge, u32)`；同一边的不同采样不可互相覆盖。
pub type StructuralStairSpan = (StairGraphEdge, u32);

/// 原连接仅有一位业务值；false 的永久支撑不会被临时连接覆盖。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StructuralGraphConnection {
    pub transient: bool,
}

impl StructuralGraphConnection {
    pub const fn new(transient: bool) -> Self {
        Self { transient }
    }
}

pub type StructuralGraphMap = UnGraphMap<StairColliderHandle, StructuralGraphConnection>;
pub type StructuralMap<K, V> = HashMap<K, V, RandomState>;
pub type StructuralColliderSet = HashSet<StairColliderHandle, RandomState>;

/// 图和六张业务表共用同一个 owner；没有额外的边索引镜像。
#[derive(Resource, Clone, Debug, Default)]
pub struct StairsStructuralGraph {
    graph: StructuralGraphMap,
    collider_to_span: StructuralMap<StairColliderHandle, StructuralStairSpan>,
    span_to_collider: StructuralMap<StructuralStairSpan, StairColliderHandle>,
    collider_to_height: StructuralMap<StairColliderHandle, f32>,
    working_heights: StructuralMap<StairColliderHandle, f32>,
    connections: StructuralColliderSet,
    segment_counts: StructuralMap<StairGraphEdge, u32>,
}

/// 装配阶段对同一 owner 分字段借用；不会克隆、分配或建立第二份图。
pub struct StairsStructuralGraphParts<'a> {
    pub graph: &'a mut StructuralGraphMap,
    pub collider_to_span: &'a mut StructuralMap<StairColliderHandle, StructuralStairSpan>,
    pub span_to_collider: &'a mut StructuralMap<StructuralStairSpan, StairColliderHandle>,
    pub collider_to_height: &'a mut StructuralMap<StairColliderHandle, f32>,
    pub working_heights: &'a mut StructuralMap<StairColliderHandle, f32>,
    pub connections: &'a mut StructuralColliderSet,
    pub segment_counts: &'a mut StructuralMap<StairGraphEdge, u32>,
}

impl StairsStructuralGraph {
    /// 原 140869a30：先清图，再按原字段顺序清六张表，保留容器容量。
    pub fn clear(&mut self) {
        self.graph.clear();
        self.collider_to_span.clear();
        self.span_to_collider.clear();
        self.collider_to_height.clear();
        self.working_heights.clear();
        self.connections.clear();
        self.segment_counts.clear();
    }

    /// 原 140869d20：先移除旧身份，再逐邻居标记；断言失败前的副作用必须保留。
    pub fn upsert_span(
        &mut self,
        span: StructuralStairSpan,
        collider: StairColliderHandle,
        height: f32,
    ) {
        if let Some(old) = self.span_to_collider.remove(&span) {
            self.collider_to_span.remove(&old);
            for neighbor in self.graph.neighbors(old) {
                self.connections.insert(neighbor);
            }
            assert!(
                self.graph.remove_node(old),
                "assertion failed: self.graph.remove_node(prev)"
            );
            self.collider_to_height.remove(&old);
        }
        self.graph.add_node(collider);
        self.collider_to_span.insert(collider, span);
        self.span_to_collider.insert(span, collider);
        self.collider_to_height.insert(collider, height);
        self.connections.insert(collider);
    }

    /// 原 140869f90：源支撑固定写入正零高度，不接收外部高度参数。
    pub fn upsert_support_source_collider(&mut self, collider: StairColliderHandle) {
        self.graph.add_node(collider);
        self.collider_to_height.insert(collider, 0.0);
        self.connections.insert(collider);
    }

    /// 原 140869ff0：任一 span 尚未注册时返回；永久连接保持永久属性。
    pub fn add_edge_by_span(
        &mut self,
        span_a: StructuralStairSpan,
        span_b: StructuralStairSpan,
        connection: StructuralGraphConnection,
    ) {
        let Some(&a) = self.span_to_collider.get(&span_a) else {
            return;
        };
        let Some(&b) = self.span_to_collider.get(&span_b) else {
            return;
        };
        self.insert_connection(a, b, connection);
    }

    /// 原 14086a260：使用 GraphMap 自有边表，不能要求消费者预置另一份索引。
    pub fn add_edge_by_collider(
        &mut self,
        a: StairColliderHandle,
        b: StairColliderHandle,
        connection: StructuralGraphConnection,
    ) {
        self.insert_connection(a, b, connection);
    }

    fn insert_connection(
        &mut self,
        a: StairColliderHandle,
        b: StairColliderHandle,
        connection: StructuralGraphConnection,
    ) {
        if let Some(previous) = self.graph.add_edge(a, b, connection) {
            if !previous.transient {
                self.graph
                    .edge_weight_mut(a, b)
                    .expect("edge index entry exists")
                    .transient = false;
            }
        }
    }

    /// 原 14086a320：先移除高度，再通知邻居、删节点，最后清双向 span 映射。
    pub fn remove_node_by_collider(&mut self, collider: StairColliderHandle) {
        if self.collider_to_height.remove(&collider).is_some() {
            for neighbor in self.graph.neighbors(collider) {
                self.connections.insert(neighbor);
            }
            assert!(
                self.graph.remove_node(collider),
                "assertion failed: self.graph.remove_node(collider)"
            );
        }
        if let Some(span) = self.collider_to_span.remove(&collider) {
            assert!(
                self.span_to_collider.remove(&span).is_some(),
                "assertion failed: self.span_to_collider_handle.remove(&span).is_some()"
            );
        }
    }

    /// 原 14086a4c0：只断临时边；每条边先通知邻接端，再增长待删向量。
    pub fn disconnect_node_by_collider(&mut self, collider: StairColliderHandle) {
        let mut incident = Vec::new();
        for (a, b, connection) in self.graph.edges(collider) {
            if connection.transient {
                self.connections.insert(b);
                incident.push((a, b));
            }
        }
        for (a, b) in incident {
            self.graph.remove_edge(a, b);
        }
    }

    pub fn graph(&self) -> &StructuralGraphMap {
        &self.graph
    }
    pub fn collider_to_span(&self) -> &StructuralMap<StairColliderHandle, StructuralStairSpan> {
        &self.collider_to_span
    }
    pub fn span_to_collider(&self) -> &StructuralMap<StructuralStairSpan, StairColliderHandle> {
        &self.span_to_collider
    }
    pub fn collider_to_height(&self) -> &StructuralMap<StairColliderHandle, f32> {
        &self.collider_to_height
    }
    pub fn working_heights(&self) -> &StructuralMap<StairColliderHandle, f32> {
        &self.working_heights
    }
    pub fn connections(&self) -> &StructuralColliderSet {
        &self.connections
    }
    pub fn segment_counts(&self) -> &StructuralMap<StairGraphEdge, u32> {
        &self.segment_counts
    }

    pub fn parts_mut(&mut self) -> StairsStructuralGraphParts<'_> {
        StairsStructuralGraphParts {
            graph: &mut self.graph,
            collider_to_span: &mut self.collider_to_span,
            span_to_collider: &mut self.span_to_collider,
            collider_to_height: &mut self.collider_to_height,
            working_heights: &mut self.working_heights,
            connections: &mut self.connections,
            segment_counts: &mut self.segment_counts,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic::{AssertUnwindSafe, catch_unwind};

    fn collider(index: u32) -> StairColliderHandle {
        StairColliderHandle::new(index, 1, 3)
    }
    fn span(a: u32, b: u32, sample: u32) -> StructuralStairSpan {
        (StairGraphEdge::new(a, b), sample)
    }
    fn connection(transient: bool) -> StructuralGraphConnection {
        StructuralGraphConnection::new(transient)
    }

    #[test]
    fn upsert_binds_maps_and_connections() {
        let mut graph = StairsStructuralGraph::default();
        let s = span(1, 2, 7);
        graph.upsert_span(s, collider(5), 2.5);
        assert_eq!(graph.span_to_collider()[&s], collider(5));
        assert_eq!(graph.collider_to_span()[&collider(5)], s);
        assert_eq!(
            graph.collider_to_height()[&collider(5)].to_bits(),
            2.5_f32.to_bits()
        );
        assert!(graph.connections().contains(&collider(5)));
        assert!(graph.graph().contains_node(collider(5)));
    }

    #[test]
    fn upsert_same_span_unbinds_old_collider() {
        let mut graph = StairsStructuralGraph::default();
        let s = span(1, 2, 7);
        graph.upsert_span(s, collider(5), 2.5);
        graph.add_edge_by_collider(collider(5), collider(6), connection(false));
        graph.connections.clear();
        graph.upsert_span(s, collider(9), 4.0);
        assert!(!graph.graph().contains_node(collider(5)));
        assert!(!graph.collider_to_span.contains_key(&collider(5)));
        assert!(!graph.collider_to_height.contains_key(&collider(5)));
        assert_eq!(graph.span_to_collider()[&s], collider(9));
        assert!(graph.connections().contains(&collider(6)));
        assert!(graph.connections().contains(&collider(9)));
    }

    #[test]
    fn different_samples_of_same_edge_remain_independent() {
        let mut graph = StairsStructuralGraph::default();
        graph.upsert_span(span(1, 2, 0), collider(1), -0.0);
        graph.upsert_span(span(1, 2, 1), collider(2), 7.0);
        assert_eq!(graph.span_to_collider.len(), 2);
        assert_eq!(
            graph.collider_to_height[&collider(1)].to_bits(),
            (-0.0_f32).to_bits()
        );
        graph.upsert_span(span(1, 2, 0), collider(3), f32::from_bits(0x7fc00041));
        assert!(graph.graph.contains_node(collider(2)));
        assert_eq!(graph.span_to_collider[&span(1, 2, 1)], collider(2));
        assert_eq!(graph.collider_to_height[&collider(3)].to_bits(), 0x7fc00041);
    }

    #[test]
    fn support_source_records_positive_zero_without_a_span() {
        let mut graph = StairsStructuralGraph::default();
        graph.upsert_support_source_collider(collider(1));
        assert_eq!(graph.collider_to_height[&collider(1)].to_bits(), 0);
        assert!(graph.collider_to_span.is_empty() && graph.span_to_collider.is_empty());
        assert!(graph.graph.contains_node(collider(1)) && graph.connections.contains(&collider(1)));
    }

    #[test]
    fn missing_span_returns_without_creating_edges() {
        let mut graph = StairsStructuralGraph::default();
        graph.upsert_span(span(1, 2, 0), collider(1), 1.0);
        graph.add_edge_by_span(span(3, 4, 0), span(1, 2, 0), connection(false));
        graph.add_edge_by_span(span(1, 2, 0), span(3, 4, 0), connection(false));
        assert_eq!(graph.graph.node_count(), 1);
        assert_eq!(graph.graph.edge_count(), 0);
    }

    #[test]
    fn new_edges_need_no_parallel_registry() {
        let mut graph = StairsStructuralGraph::default();
        graph.upsert_span(span(1, 2, 0), collider(1), 1.0);
        graph.upsert_span(span(3, 4, 0), collider(2), 2.0);
        graph.add_edge_by_span(span(1, 2, 0), span(3, 4, 0), connection(true));
        assert_eq!(
            graph.graph.edge_weight(collider(1), collider(2)),
            Some(&connection(true))
        );
        graph.add_edge_by_collider(collider(3), collider(4), connection(false));
        assert!(graph.graph.contains_edge(collider(3), collider(4)));
    }

    #[test]
    fn permanent_edge_is_sticky_through_both_entrypoints() {
        let mut graph = StairsStructuralGraph::default();
        graph.upsert_span(span(1, 2, 0), collider(1), 1.0);
        graph.upsert_span(span(3, 4, 0), collider(2), 2.0);
        graph.add_edge_by_span(span(1, 2, 0), span(3, 4, 0), connection(false));
        graph.add_edge_by_collider(collider(2), collider(1), connection(true));
        assert_eq!(
            graph.graph.edge_weight(collider(1), collider(2)),
            Some(&connection(false))
        );
        graph.add_edge_by_span(span(3, 4, 0), span(1, 2, 0), connection(true));
        assert_eq!(
            graph.graph.edge_weight(collider(2), collider(1)),
            Some(&connection(false))
        );
    }

    #[test]
    fn transient_edge_uses_new_weight_including_self_loop() {
        let mut graph = StairsStructuralGraph::default();
        graph.add_edge_by_collider(collider(1), collider(1), connection(true));
        graph.add_edge_by_collider(collider(1), collider(1), connection(false));
        graph.add_edge_by_collider(collider(1), collider(1), connection(true));
        assert_eq!(
            graph.graph.edge_weight(collider(1), collider(1)),
            Some(&connection(false))
        );
        graph.add_edge_by_collider(collider(2), collider(3), connection(true));
        graph.add_edge_by_collider(collider(2), collider(3), connection(true));
        assert_eq!(
            graph.graph.edge_weight(collider(2), collider(3)),
            Some(&connection(true))
        );
    }

    #[test]
    fn remove_node_cleans_reverse_maps() {
        let mut graph = StairsStructuralGraph::default();
        graph.upsert_span(span(1, 2, 0), collider(5), 2.5);
        graph.remove_node_by_collider(collider(5));
        assert!(!graph.graph.contains_node(collider(5)));
        assert!(
            graph.span_to_collider.is_empty()
                && graph.collider_to_span.is_empty()
                && graph.collider_to_height.is_empty()
        );
    }

    #[test]
    fn disconnect_keeps_node_and_permanent_edges_marks_only_transient_neighbor() {
        let mut graph = StairsStructuralGraph::default();
        graph.add_edge_by_collider(collider(1), collider(2), connection(true));
        graph.add_edge_by_collider(collider(1), collider(3), connection(false));
        graph.connections.clear();
        graph.disconnect_node_by_collider(collider(1));
        assert!(!graph.graph.contains_edge(collider(1), collider(2)));
        assert!(graph.graph.contains_edge(collider(1), collider(3)));
        assert!(graph.graph.contains_node(collider(1)) && graph.graph.contains_node(collider(2)));
        assert_eq!(
            graph.connections.iter().copied().collect::<Vec<_>>(),
            vec![collider(2)]
        );
    }

    #[test]
    fn upsert_failure_keeps_original_partial_unbinding_order() {
        let mut graph = StairsStructuralGraph::default();
        let s = span(1, 2, 0);
        graph.upsert_span(s, collider(1), 2.5);
        graph.graph.remove_node(collider(1));
        assert!(catch_unwind(AssertUnwindSafe(|| graph.upsert_span(s, collider(2), 3.0))).is_err());
        assert!(!graph.span_to_collider.contains_key(&s));
        assert!(!graph.collider_to_span.contains_key(&collider(1)));
        assert_eq!(graph.collider_to_height[&collider(1)], 2.5);
        assert!(!graph.graph.contains_node(collider(2)));
    }

    #[test]
    fn remove_failure_keeps_height_removal_before_required_node() {
        let mut graph = StairsStructuralGraph::default();
        let s = span(1, 2, 0);
        graph.upsert_span(s, collider(1), 2.5);
        graph.graph.remove_node(collider(1));
        assert!(
            catch_unwind(AssertUnwindSafe(
                || graph.remove_node_by_collider(collider(1))
            ))
            .is_err()
        );
        assert!(!graph.collider_to_height.contains_key(&collider(1)));
        assert_eq!(graph.collider_to_span[&collider(1)], s);
        assert_eq!(graph.span_to_collider[&s], collider(1));
    }

    #[test]
    fn missing_reverse_mapping_panics_after_node_and_primary_mapping_removal() {
        let mut graph = StairsStructuralGraph::default();
        graph.upsert_span(span(1, 2, 0), collider(1), 2.5);
        graph.span_to_collider.clear();
        assert!(
            catch_unwind(AssertUnwindSafe(
                || graph.remove_node_by_collider(collider(1))
            ))
            .is_err()
        );
        assert!(!graph.graph.contains_node(collider(1)));
        assert!(graph.collider_to_span.is_empty() && graph.collider_to_height.is_empty());
    }

    #[test]
    fn clear_resets_all_six_tables_and_graph() {
        let mut graph = StairsStructuralGraph::default();
        let edge = StairGraphEdge::new(1, 2);
        {
            let parts = graph.parts_mut();
            parts
                .graph
                .add_edge(collider(1), collider(2), connection(false));
            parts.collider_to_span.insert(collider(1), (edge, 5));
            parts.span_to_collider.insert((edge, 5), collider(1));
            parts.collider_to_height.insert(collider(1), 2.0);
            parts.working_heights.insert(collider(2), 3.0);
            parts.connections.insert(collider(2));
            parts.segment_counts.insert(edge, 7);
        }
        graph.clear();
        assert_eq!(graph.graph.node_count(), 0);
        assert_eq!(graph.graph.edge_count(), 0);
        assert!(graph.collider_to_span.is_empty() && graph.span_to_collider.is_empty());
        assert!(graph.collider_to_height.is_empty() && graph.working_heights.is_empty());
        assert!(graph.connections.is_empty() && graph.segment_counts.is_empty());
    }
}
