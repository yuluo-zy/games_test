//! B authoritative player stairs graph (0x1408fe7b0..0x140901260).
//! petgraph 0.8.1 owns the graph. The game's self-pair represents an *isolated*
//! node, not an additional persistent edge table. +0x148 is only a dirty set.
//! Original endpoint names/remaining active payload schema remain unproven;
//! the 24-byte endpoint value is preserved without invented variants. Startup
//! writes the unset tag3; its inactive payload bytes have no original value.
use bevy_ecs::resource::Resource;
use hashbrown::{HashMap, HashSet};
use petgraph::{graphmap::UnGraphMap, visit::Bfs};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    sync::atomic::AtomicBool,
};

/// Original B release byte@14362c011 is 01 in initialized .data (RVA362b000,
/// raw size9c00; byte offset1011). load_game applies the actual save-version
/// gate later. Keep one canonical static for its producer and consumers.
pub static VERIFY_STAIR_CHECKSUMS: AtomicBool = AtomicBool::new(true);

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct StairGraphEdge {
    a: u32,
    b: u32,
}
impl StairGraphEdge {
    /// B 0x140901260, shared with StairSegment::new.
    pub fn new(a: u32, b: u32) -> Self {
        Self {
            a: a.min(b),
            b: a.max(b),
        }
    }
    pub fn nodes(self) -> (u32, u32) {
        (self.a, self.b)
    }
    pub fn is_stub(self) -> bool {
        self.a == self.b
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TransientStairEdgeId(pub u32);

#[derive(Clone, Debug, Default)]
pub struct TransientStairEdgeIdAllocator {
    ids: HashMap<StairGraphEdge, TransientStairEdgeId>,
}
impl TransientStairEdgeIdAllocator {
    /// B 0x1408fe5e0: new id is the map's *old length* + 1. There is no separate
    /// counter field in that function; repeated edges preserve their first ID.
    pub fn get_or_allocate(&mut self, edge: StairGraphEdge) -> TransientStairEdgeId {
        let id = TransientStairEdgeId((self.ids.len() as u32).wrapping_add(1));
        *self.ids.entry(edge).or_insert(id)
    }
    pub fn get(&self, edge: StairGraphEdge) -> Option<TransientStairEdgeId> {
        self.ids.get(&edge).copied()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ObservedStairEndpoint([u8; 24]);
impl ObservedStairEndpoint {
    /// B DecoratorPlugin::build@140344287 writes only tag3. Zeroing the
    /// inactive host bytes makes this owned projection initialized; it does
    /// not assert that the native enum's inactive payload was zero-filled.
    pub fn unset_value() -> Self {
        let mut bytes = [0; 24];
        bytes[..8].copy_from_slice(&3u64.to_le_bytes());
        Self(bytes)
    }
    pub fn from_observed(bytes: [u8; 24]) -> Self {
        Self(bytes)
    }
    pub fn observed(&self) -> &[u8; 24] {
        &self.0
    }
    pub fn tag(&self) -> u64 {
        u64::from_le_bytes(self.0[..8].try_into().unwrap())
    }
    pub fn node(&self) -> u32 {
        u32::from_le_bytes(self.0[16..20].try_into().unwrap())
    }
    pub fn is_unset(&self) -> bool {
        self.tag() == 3
    }
    fn unset(&mut self) {
        self.0[..8].copy_from_slice(&3u64.to_le_bytes());
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StairsSnapshot {
    checksum: Option<u64>,
    edges: Vec<(u32, u32)>,
}
impl StairsSnapshot {
    /// B 0x140901120 sorts pairs but does not deduplicate or normalize them.
    pub fn new(mut edges: Vec<(u32, u32)>) -> Self {
        edges.sort_unstable();
        let checksum = Some(hash_edges(&edges));
        Self { checksum, edges }
    }
    pub fn edges(&self) -> &[(u32, u32)] {
        &self.edges
    }
    pub fn compute_checksum(&mut self) -> u64 {
        *self.checksum.get_or_insert_with(|| hash_edges(&self.edges))
    }
}

/// The two checksums and sorted difference vectors from B 0x1409007b0.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StairsHistoryDiff {
    pub before_checksum: u64,
    pub after_checksum: u64,
    pub removed: Vec<(u32, u32)>,
    pub added: Vec<(u32, u32)>,
}
impl StairsHistoryDiff {
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.added.is_empty()
    }
}

#[derive(Resource, Clone, Debug)]
pub struct StairsState {
    graph: UnGraphMap<u32, ()>,
    backup: UnGraphMap<u32, ()>,
    endpoint: ObservedStairEndpoint,
    dirty_edges: HashSet<StairGraphEdge>,
    checksum: Option<u64>,
    backed_up: bool,
}
impl StairsState {
    /// B DecoratorPlugin::build@14034420e/21a constructs two empty graph maps;
    /// @140344275/27c/287 initializes backed_up=false, checksum=None and the
    /// unset endpoint tag3, then @1403442b4 inserts this resource. The dirty
    /// set is empty with a real fresh RandomState, not a fabricated entry.
    pub fn new() -> Self {
        Self::empty(ObservedStairEndpoint::unset_value())
    }

    /// Explicit endpoint construction is retained for known active payloads.
    pub fn empty(endpoint: ObservedStairEndpoint) -> Self {
        Self {
            graph: UnGraphMap::new(),
            backup: UnGraphMap::new(),
            endpoint,
            dirty_edges: HashSet::new(),
            checksum: None,
            backed_up: false,
        }
    }
    pub fn graph(&self) -> &UnGraphMap<u32, ()> {
        &self.graph
    }
    pub fn endpoint(&self) -> ObservedStairEndpoint {
        self.endpoint
    }
    pub fn is_backed_up(&self) -> bool {
        self.backed_up
    }

    /// B 0x1408fe7b0 copies the *whole input endpoint* on success. Its degree
    /// guard is >=2 (7 < count*8-8), not the old reconstruction's >=8.
    pub fn try_set_endpoint(&mut self, candidate: ObservedStairEndpoint) {
        if self.graph.neighbors(candidate.node()).take(2).count() >= 2 {
            self.unset_endpoint();
        } else {
            self.endpoint = candidate;
        }
    }
    pub fn unset_endpoint(&mut self) {
        self.endpoint.unset();
    }

    pub fn add_node(&mut self, node: u32) {
        self.checksum = None;
        self.backed_up = false;
        self.graph.add_node(node);
        self.dirty_edges.insert(StairGraphEdge::new(node, node));
    }
    /// Original self-loop assertion precedes even the duplicate-edge check.
    pub fn add_edge(&mut self, a: u32, b: u32) {
        assert_ne!(a, b, "assertion failed: a != b");
        if self.graph.contains_edge(a, b) {
            return;
        }
        self.checksum = None;
        self.backed_up = false;
        self.graph.add_edge(a, b, ());
        self.dirty_edges.extend([
            StairGraphEdge::new(a, b),
            StairGraphEdge::new(a, a),
            StairGraphEdge::new(b, b),
        ]);
    }
    pub fn disconnect_edge(&mut self, a: u32, b: u32) {
        self.checksum = None;
        let removed = if a == b {
            self.graph.remove_node(a)
        } else {
            self.graph.remove_edge(a, b).is_some()
        };
        if removed {
            self.dirty_edges.insert(StairGraphEdge::new(a, b));
            self.backed_up = false;
        }
    }
    pub fn remove_node(&mut self, node: u32) {
        self.checksum = None;
        self.backed_up = false;
        for neighbor in self.graph.neighbors(node) {
            self.dirty_edges.insert(StairGraphEdge::new(node, neighbor));
        }
        self.dirty_edges.insert(StairGraphEdge::new(node, node));
        self.graph.remove_node(node);
    }
    pub fn get_all_connected_nodes(&self, start: u32) -> Vec<u32> {
        let mut traversal = Bfs::new(&self.graph, start);
        let mut nodes = Vec::new();
        while let Some(node) = traversal.next(&self.graph) {
            nodes.push(node);
        }
        nodes.sort_unstable();
        nodes.dedup();
        nodes
    }
    pub fn contains_edge_or_stub(&self, a: u32, b: u32) -> bool {
        contains(&self.graph, a, b)
    }
    pub fn iter_edges_and_stubs(&self, visit: impl FnMut(u32, u32)) {
        visit_graph(&self.graph, visit);
    }
    pub fn collect_edges(&self) -> Vec<(u32, u32)> {
        graph_edges(&self.graph)
    }
    pub fn drain_dirty_edges(&mut self) -> impl Iterator<Item = StairGraphEdge> + '_ {
        self.dirty_edges.drain()
    }
    pub fn take_snapshot(&self) -> StairsSnapshot {
        StairsSnapshot::new(self.collect_edges())
    }
    pub fn compute_checksum(&mut self) -> u64 {
        if let Some(value) = self.checksum {
            return value;
        }
        let value = self.take_snapshot().compute_checksum();
        self.checksum = Some(value);
        value
    }

    /// B make_backup marks stale artifacts from the old backup before replacing
    /// the backup graph. Dirty artifacts are never used as graph membership.
    pub fn make_backup(&mut self) {
        if self.backed_up {
            return;
        }
        mark_missing(&self.backup, &self.graph, &mut self.dirty_edges);
        self.backup = self.graph.clone();
        self.backed_up = true;
    }
    pub fn restore_backup(&mut self) {
        self.checksum = None;
        mark_missing(&self.graph, &self.backup, &mut self.dirty_edges);
        self.graph = self.backup.clone();
        self.backed_up = true;
    }
    /// B 0x140900010 compares the old backup to current state and then clones
    /// the current graph, even if the two snapshots happen to compare equal.
    pub fn extract_diff_and_make_backup(&mut self) -> StairsHistoryDiff {
        let mut before = StairsSnapshot::new(graph_edges(&self.backup));
        let mut after = self.take_snapshot();
        let diff = Self::reduce_diff(&mut before, &mut after);
        for &(a, b) in &diff.removed {
            self.dirty_edges.insert(StairGraphEdge::new(a, b));
        }
        self.backup = self.graph.clone();
        self.backed_up = true;
        diff
    }
    pub fn reduce_diff(
        before: &mut StairsSnapshot,
        after: &mut StairsSnapshot,
    ) -> StairsHistoryDiff {
        let before_checksum = before.compute_checksum();
        let after_checksum = after.compute_checksum();
        let mut old: HashSet<_> = before.edges.iter().copied().collect();
        let mut new: HashSet<_> = after.edges.iter().copied().collect();
        old.retain(|edge| !new.remove(edge));
        let mut removed: Vec<_> = old.into_iter().collect();
        let mut added: Vec<_> = new.into_iter().collect();
        removed.sort_unstable();
        added.sort_unstable();
        StairsHistoryDiff {
            before_checksum,
            after_checksum,
            removed,
            added,
        }
    }
    /// B apply_diff removes edges, prunes now-isolated removed endpoints, then
    /// inserts new edges/stubs. Undo swaps the lists. The source computes checksums
    /// before and after; no compiled checksum-equality assertion was observed.
    pub fn apply_diff(&mut self, diff: &StairsHistoryDiff, undo: bool) {
        self.compute_checksum();
        let (removed, added) = if undo {
            (&diff.added, &diff.removed)
        } else {
            (&diff.removed, &diff.added)
        };
        for &(a, b) in removed {
            self.disconnect_edge(a, b);
        }
        for &(a, b) in removed {
            for node in [a, b] {
                if self.graph.neighbors(node).next().is_none() {
                    self.graph.remove_node(node);
                }
            }
        }
        for &(a, b) in added {
            if a == b {
                self.add_node(a);
            } else {
                self.add_edge(a, b);
            }
        }
        self.checksum = None;
        self.compute_checksum();
    }
    /// B restore_snapshot applies the real difference, clears the old backup
    /// after marking its obsolete artifacts, and backs up the restored graph.
    pub fn restore_snapshot(&mut self, mut snapshot: StairsSnapshot) {
        let mut current = self.take_snapshot();
        let diff = Self::reduce_diff(&mut current, &mut snapshot);
        self.apply_diff(&diff, false);
        mark_missing(&self.backup, &self.graph, &mut self.dirty_edges);
        self.backup.clear();
        self.backed_up = false;
        self.make_backup();
    }
}

impl Default for StairsState {
    fn default() -> Self {
        Self::new()
    }
}

fn contains(graph: &UnGraphMap<u32, ()>, a: u32, b: u32) -> bool {
    if a == b {
        graph.contains_node(a) && graph.neighbors(a).next().is_none()
    } else {
        graph.contains_edge(a, b)
    }
}
fn visit_graph(graph: &UnGraphMap<u32, ()>, mut visit: impl FnMut(u32, u32)) {
    for node in graph.nodes() {
        let mut neighbors = graph.neighbors(node).peekable();
        if neighbors.peek().is_none() {
            visit(node, node);
        } else {
            for other in neighbors {
                if node <= other {
                    visit(node, other);
                }
            }
        }
    }
}
fn graph_edges(graph: &UnGraphMap<u32, ()>) -> Vec<(u32, u32)> {
    let mut edges = Vec::new();
    visit_graph(graph, |a, b| edges.push((a, b)));
    edges
}
fn mark_missing(
    from: &UnGraphMap<u32, ()>,
    to: &UnGraphMap<u32, ()>,
    dirty: &mut HashSet<StairGraphEdge>,
) {
    visit_graph(from, |a, b| {
        if !contains(to, a, b) {
            dirty.insert(StairGraphEdge::new(a, b));
        }
    });
}
fn hash_edges(edges: &[(u32, u32)]) -> u64 {
    // B 0x14145b330: len as usize, then a and b via Hash<u32>; std owns SipHash.
    let mut state = DefaultHasher::new();
    edges.hash(&mut state);
    state.finish()
}

use crate::resources::walls::decorator::DecoratorId;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct StairSegment {
    a: DecoratorId,
    b: DecoratorId,
}
impl StairSegment {
    /// B @140901260, alias shared with StairGraphEdge::new: unsigned min/max.
    pub fn new(a: DecoratorId, b: DecoratorId) -> Self {
        Self {
            a: DecoratorId(a.0.min(b.0)),
            b: DecoratorId(a.0.max(b.0)),
        }
    }
    pub fn nodes(self) -> (DecoratorId, DecoratorId) {
        (self.a, self.b)
    }
}
impl From<StairSegment> for StairGraphEdge {
    /// Native prepare hashes the same two endpoint words as StairGraphEdge.
    fn from(segment: StairSegment) -> Self {
        Self::new(segment.a.0, segment.b.0)
    }
}
// Same original resource identity; no wrapper/new map is inserted.
impl bevy_ecs::resource::Resource for TransientStairEdgeIdAllocator {}
const _: () = assert!(core::mem::size_of::<StairSegment>() == 8);
