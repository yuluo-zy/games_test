//! structural_graph.rs — 楼梯支撑碰撞图（WP21）。
//!
//! O：`StairsStructuralGraph`（span↔collider 双向 map + 支撑高 + GraphMap<ColliderHandle,
//! StructuralGraphConnection>）。函数：clear 0x869a30、upsert_span 0x869d20、
//! upsert_support_source_collider 0x869f90、add_edge_by_span 0x869ff0、
//! add_edge_by_collider 0x86a260、remove_node_by_collider 0x86a320、
//! disconnect_node_by_collider 0x86a4c0。空判别 0x11(17)。

use std::collections::{HashMap, HashSet};

use bevy_ecs::resource::Resource;
use petgraph::graphmap::GraphMap;
use petgraph::Undirected;

use crate::resources::stairs::ColliderHandle;
use crate::resources::stairs::stairs_state::StairGraphEdge;
use crate::typed_gap::Gap;

/// `StructuralGraphConnection`（O：边权；u8 判别）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StructuralGraphConnection(pub u8);

/// `StairsStructuralGraph`（O：楼梯支撑的碰撞体图）。
#[derive(Resource, Clone, Debug, Default)]
pub struct StairsStructuralGraph {
    /// span→collider（O：`HashMap<(StairGraphEdge, u32), ColliderHandle>`，entry 0x18B）。
    pub span_to_collider: HashMap<(StairGraphEdge, u32), ColliderHandle>,
    /// collider→span（O 逆映射）。
    pub collider_to_span: HashMap<ColliderHandle, (StairGraphEdge, u32)>,
    /// collider→支撑高（O：`HashMap<ColliderHandle, f32>`）。
    pub collider_height: HashMap<ColliderHandle, f32>,
    /// visited 集合（O：`HashMap<ColliderHandle, ()>`）。
    pub visited: HashSet<ColliderHandle>,
    /// 碰撞体邻接图（O：GraphMap<ColliderHandle, StructuralGraphConnection>）。
    pub graph: GraphMap<ColliderHandle, StructuralGraphConnection, Undirected>,
    /// 边表（O：IndexMap<(h1,h2), StructuralGraphConnection>，0x18B 步长）。
    pub connections: HashMap<(ColliderHandle, ColliderHandle), StructuralGraphConnection>,
    /// 未映射区（U）。
    pub _gap: Gap,
}

impl StairsStructuralGraph {
    /// `clear`（O 0x869a30）：清 8 张表。
    pub fn clear(&mut self) {
        self.span_to_collider.clear();
        self.collider_to_span.clear();
        self.collider_height.clear();
        self.visited.clear();
        self.graph.clear();
        self.connections.clear();
    }
    /// `upsert_span`（O 0x869d20）：删旧 span 关联，加节点/边/高/visited。
    pub fn upsert_span(
        &mut self,
        span: (StairGraphEdge, u32),
        collider: ColliderHandle,
        height: f32,
    ) {
        self.span_to_collider.insert(span, collider);
        self.collider_to_span.insert(collider, span);
        self.collider_height.insert(collider, height);
        self.visited.insert(collider);
        self.graph.add_node(collider);
    }
    /// `upsert_support_source_collider`（O 0x869f90）：只加节点/高/visited。
    pub fn upsert_support_source_collider(&mut self, collider: ColliderHandle, height: f32) {
        self.collider_height.insert(collider, height);
        self.visited.insert(collider);
        self.graph.add_node(collider);
    }
    /// `add_edge_by_span`（O 0x869ff0）：两 span 碰撞体间加边（若已存在重写 0）。
    pub fn add_edge_by_span(
        &mut self,
        a_span: (StairGraphEdge, u32),
        b_span: (StairGraphEdge, u32),
    ) {
        let (a, b) = (self.span_to_collider[&a_span], self.span_to_collider[&b_span]);
        if let Some(conn) = self.graph.edge_weight_mut(a, b) {
            conn.0 = 0;
        } else {
            self.graph.add_edge(a, b, StructuralGraphConnection(0));
        }
        self.connections
            .insert((a, b), StructuralGraphConnection(0));
    }
    /// `add_edge_by_collider`（O 0x86a260）：直接两碰撞体加边 + IndexMap 写 0。
    pub fn add_edge_by_collider(&mut self, a: ColliderHandle, b: ColliderHandle) {
        if let Some(conn) = self.graph.edge_weight_mut(a, b) {
            conn.0 = 0;
        } else {
            self.graph.add_edge(a, b, StructuralGraphConnection(0));
        }
        self.connections
            .insert((a, b), StructuralGraphConnection(0));
    }
    /// `remove_node_by_collider`（O 0x86a320）：删邻边 + remove_node + 逆映射清理。
    pub fn remove_node_by_collider(&mut self, collider: ColliderHandle) {
        for n in self.graph.neighbors(collider).collect::<Vec<_>>() {
            self.connections.remove(&(collider, n));
            self.connections.remove(&(n, collider));
        }
        self.graph.remove_node(collider);
        self.visited.remove(&collider);
        self.collider_height.remove(&collider);
        if let Some(span) = self.collider_to_span.remove(&collider) {
            self.span_to_collider.remove(&span);
        }
    }
    /// `disconnect_node_by_collider`（O 0x86a4c0）：枚举边移除（保留节点）。
    pub fn disconnect_node_by_collider(&mut self, collider: ColliderHandle) {
        let edges = self
            .graph
            .edges(collider)
            .map(|(a, b, _)| (a, b))
            .collect::<Vec<_>>();
        for (a, b) in edges {
            self.graph.remove_edge(a, b);
            self.connections.remove(&(a, b));
            self.connections.remove(&(b, a));
        }
    }
}
