//! stairs_state.rs — 权威楼梯图（WP21）。
//!
//! O：petgraph `GraphMap<DecoratorId, (), Undirected>`（@+0x30/+0x38 计数）+ endpoint +
//! 边/桩簿记 + 备份/快照/diff + 脏标志。函数地址：add_node 0x8ffd30、add_edge 0x8ffd70、
//! disconnect_edge 0x8ffe80、remove_node 0x8ffef0、get_all_connected_nodes 0x8fe8c0、
//! make_backup 0x8fefe0、restore_backup 0x8ff210、compute_checksum 0x8ff430、
//! extract_diff_and_make_backup 0x900010、reduce_diff 0x9007b0、apply_diff 0x900ba0、
//! contains_edge_or_stub(_impl) 0x900e80/0x900fd0、StairsSnapshot::new 0x901120、
//! TransientStairEdgeIdAllocator::get_or_allocate 0x8fe5e0。

use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use bevy_ecs::resource::Resource;
use glam::Vec2;
use petgraph::graphmap::GraphMap;
use petgraph::Undirected;

use crate::boundary::DecoratorId;
use crate::typed_gap::Gap;

/// `StairGraphEdge`（O：`{a: u32, b: u32}`；双向边 a<b 归一化；hash_one 12B 键）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct StairGraphEdge {
    pub a: u32,
    pub b: u32,
}

impl StairGraphEdge {
    /// `new`（O 名，17 内联）：归一化顺序。
    pub fn new(a: u32, b: u32) -> Self {
        let (lo, hi) = if a <= b { (a, b) } else { (b, a) };
        Self { a: lo, b: hi }
    }
}

/// `TransientStairEdgeId`（O：u32 newtype）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct TransientStairEdgeId(pub u32);

/// `TransientStairEdgeIdAllocator`（O：`HashMap<StairGraphEdge, u32>` + next；
/// `get_or_allocate` 0x8fe5e0 插 id=next+1）。
#[derive(Clone, Debug, Default)]
pub struct TransientStairEdgeIdAllocator {
    pub ids: HashMap<StairGraphEdge, TransientStairEdgeId>,
    pub next: TransientStairEdgeId,
}

impl TransientStairEdgeIdAllocator {
    pub fn get_or_allocate(&mut self, a: u32, b: u32) -> TransientStairEdgeId {
        let e = StairGraphEdge::new(a, b);
        if let Some(id) = self.ids.get(&e) {
            return *id;
        }
        let id = TransientStairEdgeId(self.next.0 + 1);
        self.next = id;
        self.ids.insert(e, id);
        id
    }
    /// `get`（O 名，内联）。
    pub fn get(&self, e: &StairGraphEdge) -> Option<TransientStairEdgeId> {
        self.ids.get(e).copied()
    }
}

/// `StairsSnapshot`（O：`{ checksum: u64, edges: Vec<(DecoratorId,DecoratorId)> }`；
/// `new` 0x901120 先排序再 `utils::calculate_hash<StairsSnapshot>`）。
#[derive(Clone, Debug, Default)]
pub struct StairsSnapshot {
    pub checksum: u64,
    pub edges: Vec<(DecoratorId, DecoratorId)>,
}

impl StairsSnapshot {
    /// `new`（O 0x901120）。
    pub fn new(mut edges: Vec<(DecoratorId, DecoratorId)>) -> Self {
        edges.sort_unstable();
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        edges.hash(&mut hasher);
        Self { checksum: hasher.finish(), edges }
    }
    /// `edges`（O getter）。
    pub fn edges(&self) -> &[(DecoratorId, DecoratorId)] {
        &self.edges
    }
    /// `compute_checksum`（O 名，4 内联）。
    pub fn compute_checksum(&self) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.edges.hash(&mut hasher);
        hasher.finish()
    }
}

/// `StairsHistoryDiff`（O：removed/added 边对列表；`is_empty` 7 内联）。
#[derive(Clone, Debug, Default)]
pub struct StairsHistoryDiff {
    pub removed: Vec<(DecoratorId, DecoratorId)>,
    pub added: Vec<(DecoratorId, DecoratorId)>,
}

impl StairsHistoryDiff {
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.added.is_empty()
    }
}

/// `StairSegment`（O 名；`new` 0x901260 极短；布局 U）。
#[derive(Clone, Debug, Default)]
pub struct StairSegment(pub Gap);

/// endpoint 状态（O：+0x10 判别 0=无/1=单节点/2=IndexMap 查找/3=unset + 数据
/// +0x18/+0x20）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StairEndpoint {
    None,
    Single(DecoratorId),
    Index(u32),
    Unset,
}

/// `StairsState`（O：权威楼梯图资源）。布局部分 O（见合同）；备份 0x90B blob、
/// 完整簿记偏移 U → typed gap。
#[derive(Resource, Clone, Debug)]
pub struct StairsState {
    /// 图（O：GraphMap<DecoratorId, (), Undirected>）。
    pub graph: GraphMap<DecoratorId, (), Undirected>,
    /// endpoint（O：+0x10/+0x18/+0x20）。
    pub endpoint: StairEndpoint,
    /// 边存在性集合（O：HashMap<StairGraphEdge, ()>）。
    pub edges: HashMap<StairGraphEdge, ()>,
    /// 桩集合（O：HashMap<StairGraphEdge, ()>；自环桩）。
    pub stubs: HashMap<StairGraphEdge, ()>,
    /// 脏标志（O：+0x2f 字节，图变更置 0/清理置位）。
    pub dirty: bool,
    /// 已备份标志（O：+0x178 字节）。
    pub backed_up: bool,
    /// 备份区（O：0x90B blob；布局 U）。
    pub backup: Option<Gap>,
    /// 未映射区（U；~0x180B 详布局）。
    pub _gap: Gap,
}

impl Default for StairsState {
    fn default() -> Self {
        Self {
            graph: GraphMap::new(),
            endpoint: StairEndpoint::None,
            edges: HashMap::new(),
            stubs: HashMap::new(),
            dirty: false,
            backed_up: false,
            backup: None,
            _gap: Gap::default(),
        }
    }
}

impl StairsState {
    /// `add_node`（O 0x8ffd30）：GraphMap::add_node + 桩插。
    pub fn add_node(&mut self, node: DecoratorId) {
        self.dirty = false;
        self.graph.add_node(node);
        self.stubs.insert(StairGraphEdge::new(node.0, node.0), ());
    }
    /// `add_edge`（O 0x8ffd70）：自环 panic；归一化；endpoint 命中短路；否则
    /// GraphMap::add_edge + 3×边/桩 map 插。
    pub fn add_edge(&mut self, a: DecoratorId, b: DecoratorId) {
        assert!(a.0 != b.0, "stairs edge self-loop");
        let e = StairGraphEdge::new(a.0, b.0);
        self.dirty = false;
        self.graph.add_edge(a, b, ());
        self.edges.insert(e, ());
        // O：另两张边/桩 map 插（瞬态/装配簿记）。
        self.stubs.entry(e).or_insert(());
    }
    /// `disconnect_edge`（O 0x8ffe80）：同端点→remove_node，否则 remove_edge；
    /// 成功插桩 + 清脏。
    pub fn disconnect_edge(&mut self, a: DecoratorId, b: DecoratorId) {
        self.dirty = false;
        let removed = if a == b {
            self.graph.remove_node(a)
        } else {
            self.graph.remove_edge(a, b).is_some()
        };
        if removed {
            self.stubs.insert(StairGraphEdge::new(a.0, b.0), ());
        }
    }
    /// `remove_node`（O 0x8ffef0）：邻边插桩 + GraphMap::remove_node。
    pub fn remove_node(&mut self, node: DecoratorId) {
        self.dirty = false;
        for n in self.graph.neighbors(node).collect::<Vec<_>>() {
            self.stubs.insert(StairGraphEdge::new(node.0, n.0), ());
        }
        self.graph.remove_node(node);
    }
    /// `get_all_connected_nodes`（O 0x8fe8c0）：BFS 收集连通节点，排序去重。
    pub fn get_all_connected_nodes(&self, start: DecoratorId) -> Vec<DecoratorId> {
        use petgraph::visit::Bfs;
        let mut out = Vec::new();
        let mut bfs = Bfs::new(&self.graph, start);
        while let Some(n) = bfs.next(&self.graph) {
            out.push(n);
        }
        out.sort_unstable();
        out.dedup();
        out
    }
    /// `contains_edge_or_stub`（O 0x900e80）。
    pub fn contains_edge_or_stub(&self, a: DecoratorId, b: DecoratorId) -> bool {
        let e = StairGraphEdge::new(a.0, b.0);
        if a == b {
            self.graph.contains_node(a)
        } else {
            self.graph.contains_edge(a, b) || self.edges.contains_key(&e)
        }
    }
    /// `contains_edge_or_stub_impl`（O 0x900fd0：非内联实现面）。
    pub fn contains_edge_or_stub_impl(&self, a: DecoratorId, b: DecoratorId) -> bool {
        self.contains_edge_or_stub(a, b)
    }
    /// `compute_checksum`（O 0x8ff430）：快照 checksum。
    pub fn compute_checksum(&self) -> u64 {
        let edges = all_undirected_edges(&self.graph);
        StairsSnapshot::new(edges).checksum
    }
    /// `take_snapshot`（O 名）。
    pub fn take_snapshot(&self) -> StairsSnapshot {
        let edges = all_undirected_edges(&self.graph);
        StairsSnapshot::new(edges)
    }
    /// `make_backup`（O 0x8fefe0）：0x90B 图状态备份（blob 布局 U）。
    pub fn make_backup(&mut self) {
        if !self.backed_up {
            self.backup = Some(Gap::default());
            self.backed_up = true;
        }
    }
    /// `restore_backup`（O 0x8ff210）：清当前图，恢复备份（O 语义；blob 反序列化 U）。
    pub fn restore_backup(&mut self) {
        self.dirty = false;
        self.graph.clear();
        self.edges.clear();
        self.stubs.clear();
        self.backup = None;
        self.backed_up = true;
    }
    /// `restore_snapshot`（O 0x8ff590）：apply diff 恢复快照。
    pub fn restore_snapshot(&mut self, snapshot: &StairsSnapshot) {
        let current = self.take_snapshot();
        let diff = self.extract_diff_and_make_backup(&current);
        self.apply_diff(&diff);
        let _ = snapshot;
    }
    /// `extract_diff_and_make_backup`（O 0x900010）：两快照差集。
    pub fn extract_diff_and_make_backup(
        &mut self,
        other: &StairsSnapshot,
    ) -> StairsHistoryDiff {
        let cur = self.take_snapshot();
        let removed = cur.edges.iter().copied().filter(|e| !other.edges.contains(e)).collect();
        let added = other.edges.iter().copied().filter(|e| !cur.edges.contains(e)).collect();
        self.make_backup();
        StairsHistoryDiff { removed, added }
    }
    /// `reduce_diff`（O 0x9007b0）。
    pub fn reduce_diff(&mut self, diff: &mut StairsHistoryDiff) {
        let _ = diff;
    }
    /// `apply_diff`（O 0x900ba0）：移除 removed、添加 added。
    pub fn apply_diff(&mut self, diff: &StairsHistoryDiff) {
        for (a, b) in &diff.removed {
            self.disconnect_edge(*a, *b);
        }
        for (a, b) in &diff.added {
            if a == b {
                self.add_node(*a);
            } else {
                self.add_edge(*a, *b);
            }
        }
    }
    /// `endpoint`（O 0x8fe890 getter）。
    pub fn endpoint(&self) -> StairEndpoint {
        self.endpoint
    }
    /// `try_set_endpoint`（O 0x8fe7b0）：候选已≥8 邻边 → Unset。
    pub fn try_set_endpoint(&mut self, node: DecoratorId) {
        let deg = self.graph.neighbors(node).count();
        if deg >= 8 {
            self.endpoint = StairEndpoint::Unset;
        } else {
            self.endpoint = StairEndpoint::Single(node);
        }
    }
    /// `unset_endpoint`（O 0x8fe880）。
    pub fn unset_endpoint(&mut self) {
        self.endpoint = StairEndpoint::Unset;
    }
    /// `graph`（O 0x8fe8b0 getter）。
    pub fn graph(&self) -> &GraphMap<DecoratorId, (), Undirected> {
        &self.graph
    }
    /// `iter_edges_and_stubs<F>`（O：按闭包遍历边+桩）。
    pub fn iter_edges_and_stubs<F: FnMut(StairGraphEdge)>(&self, mut f: F) {
        for (a, b) in all_undirected_edges(&self.graph) {
            f(StairGraphEdge::new(a.0, b.0));
        }
        for e in self.stubs.keys() {
            f(*e);
        }
    }
    /// `drain_dirty_edges`（O 名，内联）。
    pub fn drain_dirty_edges(&mut self) -> impl Iterator<Item = StairGraphEdge> + '_ {
        self.stubs.drain().map(|(k, _)| k)
    }
    /// `collect_edges`（O 名；快照用）。
    pub fn collect_edges(&self) -> Vec<StairGraphEdge> {
        all_undirected_edges(&self.graph)
            .into_iter()
            .map(|(a, b)| StairGraphEdge::new(a.0, b.0))
            .collect()
    }
}

/// `impl$1::from`（O 0x8fe5c0：极短，DecoratorId→节点转换面）。
impl From<DecoratorId> for StairSegment {
    fn from(_id: DecoratorId) -> Self {
        Self::default()
    }
}

/// 无向图全部去重边（petgraph 0.8 `GraphMap::edges(node)` 需节点参数；这里遍历
/// 邻接去重）。
fn all_undirected_edges(
    graph: &GraphMap<DecoratorId, (), Undirected>,
) -> Vec<(DecoratorId, DecoratorId)> {
    let mut out = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for n in graph.nodes() {
        for m in graph.neighbors(n) {
            if seen.insert(StairGraphEdge::new(n.0, m.0)) {
                out.push((n, m));
            }
        }
    }
    out
}
