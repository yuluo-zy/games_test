//! `structural_graph.rs` — 楼梯支撑碰撞体图（B `resources/stairs/structural_graph.rs`，
//! 7 业务 + 1 胶水）。
//!
//! 证据：`chapters/resources/stairs/structural_graph.rs.md` + `collision-eco/ALGORITHMS.md`
//! §3.4（机器码 O）+ `collision-eco/decomp/decomp-0x140869{a30,d20,f90,ff0}a{260,320,4c0}.txt`。
//! 表分布（复核）：+0x90 collider→span、+0xc0 span→collider、+0xf0 collider→height、
//! +0x150 connections；GraphMap 节点 = 碰撞体句柄。
//! 未决（U）：+0x48 边索引表与 +0x120/+0x180 两附加表语义、碰撞体句柄在 upsert/remove
//! 面的字节宽（decomp 8B 读 vs add_edge_by_collider 12B 排序）——本实现统一用
//! [`StairColliderHandle`]（= `SubworldColliderHandle` 的 {index, generation, slot}）。
use crate::{
    resources::stairs::stairs_state::StairGraphEdge,
    systems::collision::world::SubworldColliderHandle,
};
use bevy_ecs::resource::Resource;
use hashbrown::{HashMap, HashSet};
use petgraph::graphmap::UnGraphMap;
use std::hash::{Hash, Hasher};

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

/// `StairsStructuralGraph`（B 0x140869a30..0x14086a4c0；Resource）。
#[derive(Resource, Clone, Debug, Default)]
pub struct StairsStructuralGraph {
    /// GraphMap（节点 = 碰撞体句柄）。
    graph: UnGraphMap<StairColliderHandle, ()>,
    /// +0x90：collider → span。
    collider_to_span: HashMap<StairColliderHandle, StairGraphEdge>,
    /// +0xc0：span → collider。
    span_to_collider: HashMap<StairGraphEdge, StairColliderHandle>,
    /// +0xf0：collider → 支撑高。
    collider_to_height: HashMap<StairColliderHandle, f32>,
    /// +0x150：连接集合（装配的支撑结构）。
    connections: HashSet<StairColliderHandle>,
    /// +0x48：边索引表（装配侧"写 0"标记；精确类型 IndexMap 语义 U）。
    edge_index: HashMap<(StairColliderHandle, StairColliderHandle), u8>,
}

impl StairsStructuralGraph {
    /// `clear`（0x140869a30，fresh 2026-10-17）：清空整张结构图。
    ///
    /// **地址判定**：0x140869a30 的 Ghidra 符号=`StairsStructuralGraph::clear`，函数体=
    /// 8 张 hashbrown 表的 `memset(ctrl,0xff)+items/growth 归零`（表头
    /// +0x18/+0x60/+0x90/+0xc0/+0xf0/+0x120/+0x150/+0x180）+ 一个 0x30-stride 条目 Vec
    /// （+0x08 data / +0x10 len，逐条 `dealloc(size<<4)`）+ 标量 +0x58=0。
    /// （LOGIC §5 / 任务说明里的"0x140869a30=MagicSupports::clear"经 fresh 反编译驳回；
    /// MagicSupports::clear 唯一地址 0x140aa8210。）
    ///
    /// 本实现清 6 个建模集合：petgraph `UnGraphMap`（内部 = 2 张 IndexMap 表 + 2 个
    /// Bucket Vec，对应 +0x18/+0x60 与 +0x08/+0x58）+ 4 个显式映射
    /// （+0x90/+0xc0/+0xf0/+0x150）+ `edge_index`。**U 登记**：+0x120/+0x180 两张未建模表
    /// 的语义，以及 `edge_index` 的原生落点（+0x48 未见独立表头）。
    pub fn clear(&mut self) {
        self.graph.clear();
        self.collider_to_span.clear();
        self.span_to_collider.clear();
        self.collider_to_height.clear();
        self.connections.clear();
        self.edge_index.clear();
    }

    /// `upsert_span`（0x140869d20，O 全解）：
    /// 旧 span 已有 collider → 先解绑（邻边入 connections、逆映射移除、`graph.remove_node(prev)`
    /// 断言、高表移除），再挂接新 collider（节点确保 + 三表插入 + connections）。
    pub fn upsert_span(
        &mut self,
        span: StairGraphEdge,
        collider: StairColliderHandle,
        height: f32,
    ) {
        if let Some(old) = self.span_to_collider.get(&span).copied() {
            let neighbors: Vec<_> = self.graph.neighbors(old).collect();
            for neighbor in neighbors {
                self.connections.insert(neighbor);
            }
            self.collider_to_span.remove(&old);
            let removed = self.graph.remove_node(old);
            assert!(removed, "assertion failed: self.graph.remove_node(prev)");
            self.collider_to_height.remove(&old);
        }
        self.graph.add_node(collider);
        self.collider_to_span.insert(collider, span);
        self.span_to_collider.insert(span, collider);
        self.collider_to_height.insert(collider, height);
        self.connections.insert(collider);
    }

    /// `upsert_support_source_collider`（0x140869f90；Ghidra varnode 失败，合同语义）：
    /// 只加节点/高/visited（无 span 双向映射）。
    pub fn upsert_support_source_collider(&mut self, collider: StairColliderHandle, height: f32) {
        self.graph.add_node(collider);
        self.collider_to_height.insert(collider, height);
        self.connections.insert(collider);
    }

    /// `add_edge_by_span`（0x140869ff0，O 结构）：两 span 的 collider 间加边；
    /// 边已存在 → 早退（索引写 0 语义）；否则边索引表命中写 0，未命中 panic。
    pub fn add_edge_by_span(&mut self, span_a: StairGraphEdge, span_b: StairGraphEdge, weight: ()) {
        let collider_a = *self.span_to_collider.get(&span_a).expect("span_a has collider");
        let collider_b = *self.span_to_collider.get(&span_b).expect("span_b has collider");
        if self.graph.add_edge(collider_a, collider_b, weight).is_some() {
            return;
        }
        let (a, b) = ordered_pair(collider_a, collider_b);
        let entry = self
            .edge_index()
            .get_mut(&(a, b))
            .expect("edge index entry exists");
        *entry = 0;
    }

    /// `add_edge_by_collider`（0x14086a260，O 结构）：直接加边 + 边索引写 0
    /// （键先排序 {slot, index, generation}）。
    pub fn add_edge_by_collider(&mut self, a: StairColliderHandle, b: StairColliderHandle) {
        self.graph.add_edge(a, b, ());
        let (a, b) = ordered_pair(a, b);
        let entry = self
            .edge_index()
            .get_mut(&(a, b))
            .expect("edge index entry exists");
        *entry = 0;
    }

    /// `remove_node_by_collider`（0x14086a320，O）：删邻边入 connections + `remove_node`
    /// 断言；逆映射清理（collider→span 移除后，span→collider 移除断言）。
    pub fn remove_node_by_collider(&mut self, collider: StairColliderHandle) {
        if self.collider_to_height.remove(&collider).is_some() {
            let neighbors: Vec<_> = self.graph.neighbors(collider).collect();
            for neighbor in neighbors {
                self.connections.insert(neighbor);
            }
            let removed = self.graph.remove_node(collider);
            assert!(
                removed,
                "assertion failed: self.graph.remove_node(collider)"
            );
        }
        if let Some(span) = self.collider_to_span.remove(&collider) {
            let removed = self.span_to_collider.remove(&span);
            assert!(
                removed.is_some(),
                "assertion failed: self.span_to_collider_handle.remove(&span).is_some()"
            );
        }
    }

    /// `disconnect_node_by_collider`（0x14086a4c0，O）：枚举邻边移除（保留节点），
    /// 每条边只往 connections 插**一个**端点 = **邻接端点 b**（fresh 机器码 0x14086a58e..a5a8：
    /// petgraph `EdgeMap::edges(node)` 输出 `(a=node, b, &w)`，`[RBP-0x2c]` = b，
    /// `HashMap::insert<SubworldColliderHandle,()>` 插入 b；Vec 收集 `(node,b)` 后逐边
    /// `remove_edge(node,b)`）。边权重字节==0 的邻边跳过（机器码 `*edge_weight != '\0'`；
    /// `StructuralGraphConnection` 未建模，`()` 恒过）。
    pub fn disconnect_node_by_collider(&mut self, collider: StairColliderHandle) {
        let incident: Vec<_> = self
            .graph
            .edges(collider)
            .map(|(a, b, _)| (a, b))
            .collect();
        for (_a, b) in &incident {
            // 机器码：每邻边插邻接端点 b（`[R13]` = 边迭代输出第二分量）。
            self.connections.insert(*b);
        }
        for (a, b) in incident {
            self.graph.remove_edge(a, b);
        }
    }

    /// 边索引表登记（O 语义：`add_edge_by_span`/`add_edge_by_collider` 命中写 0、
    /// 未命中 panic @0x142a96bd0）。**填充源 U**——原始为碰撞体注册面（+0x48 IndexMap），
    /// 本方法供消费端装配前预置；精确类型/来源待取证。
    pub fn register_edge_index(&mut self, a: StairColliderHandle, b: StairColliderHandle) {
        let (a, b) = ordered_pair(a, b);
        self.edge_index.insert((a, b), 1);
    }

    /// 只读访问（供装配/失效系统消费）。
    pub fn graph(&self) -> &UnGraphMap<StairColliderHandle, ()> {
        &self.graph
    }
    pub fn collider_to_span(&self) -> &HashMap<StairColliderHandle, StairGraphEdge> {
        &self.collider_to_span
    }
    pub fn span_to_collider(&self) -> &HashMap<StairGraphEdge, StairColliderHandle> {
        &self.span_to_collider
    }
    pub fn collider_to_height(&self) -> &HashMap<StairColliderHandle, f32> {
        &self.collider_to_height
    }
    pub fn connections(&self) -> &HashSet<StairColliderHandle> {
        &self.connections
    }

    /// +0x48 边索引表（写 0 语义：装配侧标记；精确类型 IndexMap U）。
    fn edge_index(&mut self) -> &mut HashMap<(StairColliderHandle, StairColliderHandle), u8> {
        &mut self.edge_index
    }

    /// 测试辅助：清空 connections，以便验证 disconnect 插入的端点来源。
    #[cfg(test)]
    pub(crate) fn clear_connections_for_test(&mut self) {
        self.connections.clear();
    }
}

fn ordered_pair(
    a: StairColliderHandle,
    b: StairColliderHandle,
) -> (StairColliderHandle, StairColliderHandle) {
    if a > b { (b, a) } else { (a, b) }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn collider(index: u32) -> StairColliderHandle {
        StairColliderHandle::new(index, 1, 3)
    }

    #[test]
    fn upsert_binds_maps_and_connections() {
        let mut graph = StairsStructuralGraph::default();
        let span = StairGraphEdge::new(1, 2);
        let collider = collider(5);
        graph.upsert_span(span, collider, 2.5);
        assert_eq!(graph.span_to_collider()[&span], collider);
        assert_eq!(graph.collider_to_span()[&collider], span);
        assert!((graph.collider_to_height()[&collider] - 2.5).abs() < 1e-6);
        assert!(graph.connections().contains(&collider));
        assert!(graph.graph().contains_node(collider));
    }

    #[test]
    fn upsert_same_span_unbinds_old_collider() {
        let mut graph = StairsStructuralGraph::default();
        let span = StairGraphEdge::new(1, 2);
        graph.upsert_span(span, collider(5), 2.5);
        graph.upsert_span(span, collider(9), 4.0);
        assert!(!graph.graph().contains_node(collider(5)));
        assert!(graph.graph().contains_node(collider(9)));
        assert_eq!(graph.span_to_collider()[&span], collider(9));
        assert!(graph.connections().contains(&collider(5)));
    }

    #[test]
    fn remove_node_cleans_reverse_maps() {
        let mut graph = StairsStructuralGraph::default();
        let span = StairGraphEdge::new(1, 2);
        graph.upsert_span(span, collider(5), 2.5);
        graph.remove_node_by_collider(collider(5));
        assert!(!graph.graph().contains_node(collider(5)));
        assert!(graph.span_to_collider().is_empty());
        assert!(graph.collider_to_span().is_empty());
        assert!(graph.collider_to_height().is_empty());
    }

    #[test]
    fn disconnect_keeps_node_removes_edges() {
        let mut graph = StairsStructuralGraph::default();
        graph.upsert_span(StairGraphEdge::new(1, 2), collider(5), 2.5);
        graph.upsert_span(StairGraphEdge::new(3, 4), collider(9), 2.5);
        // 原版 add_edge_by_span 对新边做边索引"命中写 0"；索引须由装配面预置。
        graph.register_edge_index(collider(5), collider(9));
        graph.add_edge_by_span(
            StairGraphEdge::new(1, 2),
            StairGraphEdge::new(3, 4),
            (),
        );
        assert!(graph.graph().contains_edge(collider(5), collider(9)));
        // 清空 connections 后断开节点 5：机器码只插入邻接端点 9（单端点，非查询节点）。
        graph.clear_connections_for_test();
        graph.disconnect_node_by_collider(collider(5));
        assert!(!graph.graph().contains_edge(collider(5), collider(9)));
        assert!(graph.graph().contains_node(collider(5)));
        assert!(graph.graph().contains_node(collider(9)));
        assert!(graph.connections().contains(&collider(9)));
        assert!(!graph.connections().contains(&collider(5)));
    }
}
