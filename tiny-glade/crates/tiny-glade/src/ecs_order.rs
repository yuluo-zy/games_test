//! Deterministic conflict dependencies for the parallel ECS executor.
//!
//! Each schedule is built once from its declared dependencies. Conflicting
//! accesses receive explicit edges in that topological order; independent
//! systems remain parallel. Rebuilds honor new explicit dependencies before
//! preferring the recorded order. Deferred visibility continues to require
//! explicit business dependencies and sync points.
use bevy_ecs::{
    schedule::{
        graph::DiGraph, ExecutorKind, NodeId, ScheduleBuildError, ScheduleBuildPass,
        ScheduleGraph, Schedules, SystemSet,
    },
    world::World,
};
use std::collections::HashMap;

/// Adds the recorded conflict orderings to the flattened dependency graph.
#[derive(Debug, Default)]
struct PinnedOrder {
    order: HashMap<NodeId, usize>,
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
        graph: &mut ScheduleGraph,
        dependency_flattened: &mut DiGraph,
    ) -> Result<(), ScheduleBuildError> {
        // The flattened graph is fresh: generated edges are never fed back
        // into the explicit dependency graph, so changed business ordering
        // cannot form cycles with last build's generated order.
        let nodes: Vec<_> = dependency_flattened.nodes().collect();
        let mut incoming: HashMap<_, usize> = nodes.iter().map(|&id| (id, 0)).collect();
        for (_, to) in dependency_flattened.all_edges() {
            *incoming.get_mut(&to).expect("graph node") += 1;
        }
        let mut ready: Vec<_> = nodes.iter().copied().filter(|id| incoming[id] == 0).collect();
        let mut ordered = Vec::with_capacity(nodes.len());
        while !ready.is_empty() {
            let index = (0..ready.len()).min_by_key(|&i| {
                (self.order.get(&ready[i]).copied().unwrap_or(usize::MAX), ready[i].index())
            }).expect("nonempty");
            let node = ready.swap_remove(index);
            ordered.push(node);
            for next in dependency_flattened.neighbors(node) {
                let count = incoming.get_mut(&next).expect("graph node");
                *count -= 1;
                if *count == 0 { ready.push(next); }
            }
        }
        if ordered.len() != nodes.len() {
            // Preserve Bevy's own cycle diagnostics without adding any edges.
            return Ok(());
        }
        for (index, &before) in ordered.iter().enumerate() {
            let Some(a) = graph.get_system_at(before) else { continue };
            for &after in &ordered[index + 1..] {
                let Some(b) = graph.get_system_at(after) else { continue };
                if a.is_exclusive() || b.is_exclusive()
                    || !a.component_access().is_compatible(b.component_access())
                    || (a.has_deferred() && b.has_deferred())
                {
                    dependency_flattened.add_edge(before, after);
                }
            }
        }
        Ok(())
    }
}

/// Configuring this set marks a schedule's graph changed, so the next run
/// rebuilds it with `PinnedOrder`.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct Repin;

/// Pin every schedule of `world` and switch it to the multi-threaded executor.
pub fn pin_and_parallelize(world: &mut World) -> anyhow::Result<()> {
    // Keep Schedules in the world: Schedule::initialize reads its ignored
    // ambiguity registry and will recreate it if resource_scope removed it.
    let labels: Vec<_> = world.resource::<Schedules>().iter().map(|(_, schedule)| schedule.label()).collect();
    for label in labels {
        world.schedule_scope(label, |world, schedule| -> anyhow::Result<()> {
            schedule.remove_build_pass::<PinnedOrder>();
            schedule.configure_sets(Repin);
            schedule.set_executor_kind(ExecutorKind::MultiThreaded);
            schedule.initialize(world).map_err(|error| anyhow::anyhow!("{label:?}: parallel schedule build failed: {error}"))?;
            let systems = schedule.systems().map_err(|error| anyhow::anyhow!("{label:?}: initialized schedule is unavailable: {error}"))?;
            let order: HashMap<NodeId, usize> =
                systems.enumerate().map(|(index, (id, _))| (id, index)).collect();
            log::info!(target: "tiny_glade::ecs",
                "{label:?}: {} systems, {} reported conflicts; access order will be pinned on every rebuild",
                order.len(), schedule.graph().conflicting_systems().len());
            schedule.add_build_pass(PinnedOrder { order });
            schedule.configure_sets(Repin);
            Ok(())
        })?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_ecs::prelude::*;
    use bevy_ecs::schedule::{IntoScheduleConfigs, Schedule, ScheduleLabel};

    #[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
    struct TestSchedule;
    #[derive(SystemSet, Clone, Debug, PartialEq, Eq, Hash)]
    enum TestSet { First, Second }
    #[derive(Resource, Default)]
    struct Trace(Vec<u8>);
    fn first(mut trace: ResMut<Trace>) { trace.0.push(1); }
    fn second(mut trace: ResMut<Trace>) { trace.0.push(2); }
    fn third(mut trace: ResMut<Trace>) { trace.0.push(3); }
    fn world() -> World {
        let mut world = World::new();
        world.init_resource::<Schedules>();
        world.init_resource::<Trace>();
        let mut schedule = Schedule::new(TestSchedule);
        schedule.add_systems((first.in_set(TestSet::First), second.in_set(TestSet::Second)));
        world.resource_mut::<Schedules>().insert(schedule);
        world
    }

    #[test]
    fn invalid_schedule_returns_its_build_error_without_an_executor_fallback() {
        let mut world = world();
        world.resource_mut::<Schedules>().get_mut(TestSchedule).unwrap().configure_sets((TestSet::First.before(TestSet::Second), TestSet::Second.before(TestSet::First)));
        assert!(pin_and_parallelize(&mut world).is_err());
        assert_eq!(world.resource::<Schedules>().get(TestSchedule).unwrap().get_executor_kind(), ExecutorKind::MultiThreaded);
    }

    #[test]
    fn conflicts_added_after_initial_pinning_are_ordered_on_rebuild() {
        let mut world = world();
        pin_and_parallelize(&mut world).unwrap();
        world.run_schedule(TestSchedule);
        world.resource_mut::<Trace>().0.clear();
        world.resource_mut::<Schedules>().get_mut(TestSchedule).unwrap()
            .add_systems(third);
        world.run_schedule(TestSchedule);
        assert_eq!(world.resource::<Trace>().0.len(), 3);
        assert!(world.resource::<Schedules>().get(TestSchedule).unwrap()
            .graph().conflicting_systems().is_empty());
    }

    #[test]
    fn new_explicit_order_overrides_old_generated_order_without_a_cycle() {
        let mut world = world();
        pin_and_parallelize(&mut world).unwrap();
        world.run_schedule(TestSchedule);
        let previous = world.resource::<Trace>().0.clone();
        world.resource_mut::<Trace>().0.clear();
        let mut schedules = world.resource_mut::<Schedules>();
        let schedule = schedules.get_mut(TestSchedule).unwrap();
        if previous == [1, 2] {
            schedule.configure_sets(TestSet::Second.before(TestSet::First));
        } else {
            schedule.configure_sets(TestSet::First.before(TestSet::Second));
        }
        world.run_schedule(TestSchedule);
        let expected: Vec<_> = previous.into_iter().rev().collect();
        assert_eq!(world.resource::<Trace>().0, expected);
    }

    #[test]
    fn repinning_keeps_explicit_deferred_visibility() {
        #[derive(Resource)]
        struct Created;
        fn create(mut commands: Commands) { commands.insert_resource(Created); }
        fn observe(created: Option<Res<Created>>, mut trace: ResMut<Trace>) {
            trace.0.push(u8::from(created.is_some()));
        }
        let mut world = World::new();
        world.init_resource::<Schedules>();
        world.init_resource::<Trace>();
        let mut schedule = Schedule::new(TestSchedule);
        schedule.add_systems((create, observe).chain());
        world.resource_mut::<Schedules>().insert(schedule);
        pin_and_parallelize(&mut world).unwrap();
        pin_and_parallelize(&mut world).unwrap();
        world.run_schedule(TestSchedule);
        assert_eq!(world.resource::<Trace>().0, [1]);
    }
}
