//! Multi-threaded ECS executor with the single-threaded order pinned
//! (reports/render-thread-2026-10-22/ARCHITECTURE.md, step 6).
//!
//! The schedules declare far fewer orderings than the systems' data access
//! needs (a real run reports 525 conflicting, unordered pairs in `Update`).
//! The single-threaded executor resolves each of them by its topological
//! order. Before switching to the multi-threaded executor, every schedule is
//! built once single-threaded, and each conflicting pair receives an explicit
//! edge in exactly that order, so systems that touch the same data keep their
//! present relative order while the rest run in parallel.
//!
//! Not pinned: the application order of deferred commands between systems
//! with disjoint data (commands are applied in executable order at each sync
//! point), and systems added after this runs.
use bevy_ecs::{
    schedule::{
        graph::DiGraph, ExecutorKind, NodeId, ScheduleBuildError, ScheduleBuildPass,
        ScheduleGraph, Schedules, SystemSet,
    },
    world::{Mut, World},
};
use std::collections::HashMap;

/// Adds the recorded conflict orderings to the flattened dependency graph.
#[derive(Debug, Default)]
struct PinnedOrder {
    edges: Vec<(NodeId, NodeId)>,
}

impl ScheduleBuildPass for PinnedOrder {
    type EdgeOptions = ();
    fn add_dependency(&mut self, _from: NodeId, _to: NodeId, _options: Option<&()>) {}
    fn collapse_set(
        &mut self,
        _set: NodeId,
        _systems: &[NodeId],
        _dependency_flattened: &DiGraph,
    ) -> impl Iterator<Item = (NodeId, NodeId)> {
        std::iter::empty()
    }
    fn build(
        &mut self,
        _world: &mut World,
        _graph: &mut ScheduleGraph,
        dependency_flattened: &mut DiGraph,
    ) -> Result<(), ScheduleBuildError> {
        for &(before, after) in &self.edges {
            dependency_flattened.add_edge(before, after);
        }
        Ok(())
    }
}

/// Configuring this set marks a schedule's graph changed, so the next run
/// rebuilds it with `PinnedOrder`.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct Repin;

/// Pin every schedule of `world` and switch it to the multi-threaded executor.
pub fn pin_and_parallelize(world: &mut World) {
    world.resource_scope(|world, mut schedules: Mut<Schedules>| {
        for (label, schedule) in schedules.iter_mut() {
            schedule.set_executor_kind(ExecutorKind::SingleThreaded);
            if let Err(error) = schedule.initialize(world) {
                log::warn!(target: "tiny_glade::ecs", "{label:?}: kept single-threaded, build failed: {error}");
                continue;
            }
            let Ok(systems) = schedule.systems() else {
                continue;
            };
            let order: HashMap<NodeId, usize> =
                systems.enumerate().map(|(index, (id, _))| (id, index)).collect();
            let edges: Vec<(NodeId, NodeId)> = schedule
                .graph()
                .conflicting_systems()
                .iter()
                .filter_map(|(a, b, _)| {
                    let (ia, ib) = (*order.get(a)?, *order.get(b)?);
                    Some(if ia < ib { (*a, *b) } else { (*b, *a) })
                })
                .collect();
            log::info!(target: "tiny_glade::ecs",
                "{label:?}: {} systems, {} conflicting pairs pinned to the single-threaded order",
                order.len(), edges.len());
            schedule.add_build_pass(PinnedOrder { edges });
            schedule.configure_sets(Repin);
            schedule.set_executor_kind(ExecutorKind::MultiThreaded);
        }
    });
}
