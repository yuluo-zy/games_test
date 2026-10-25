// 候选实现迁移来源：reconstruction/render/work-packages/compiled-state/lib.rs.
// 这是行为重建，不是原作者源码。
use std::ops::Range;

pub const NO_ACCESS: u32 = u32::MAX;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Batch {
    pub pass_range: Range<usize>,
    pub export_barriers: bool,
}

#[derive(Debug, Default)]
pub struct BatchCompiler {
    pub batches: Vec<Batch>,
    pub current_range: Range<usize>,
    pub current_export_barriers: bool,
}

impl BatchCompiler {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn flush_batch(&mut self, boundary: usize) {
        if self.current_range.start == boundary {
            return;
        }
        self.current_range.end = boundary;
        let pass_range = std::mem::take(&mut self.current_range);
        let export_barriers = std::mem::take(&mut self.current_export_barriers);
        self.batches.push(Batch {
            pass_range,
            export_barriers,
        });
        self.current_range.start = boundary;
    }

    pub fn finish(mut self, pass_count: usize) -> Vec<Batch> {
        self.flush_batch(pass_count);
        if let Some(last) = self.batches.last_mut() {
            last.export_barriers = true;
        }
        self.batches
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResourceLifetime {
    pub first_access_index: u32,
    pub last_access_index: u32,
}

pub fn pending_initial_transition_resources(resources: &[ResourceLifetime]) -> Vec<u32> {
    resources
        .iter()
        .enumerate()
        .filter_map(|(resource_index, lifetime)| {
            (lifetime.first_access_index != NO_ACCESS).then_some(resource_index as u32)
        })
        .collect()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResourceAccess {
    pub resource_index: u32,
    pub previous_access_index: u32,
    pub synchronization_group: u32,
    pub access_code: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RuntimeResourceState {
    pub has_initial_state: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BarrierTransition {
    pub resource_index: u32,
    pub previous_access_code: u8,
    pub next_access_code: u8,
}

pub fn pre_pass_barrier_transitions(
    pass_access_range: Range<usize>,
    accesses: &[ResourceAccess],
    resources: &[RuntimeResourceState],
    mut can_be_concurrent: impl FnMut(&ResourceAccess, &ResourceAccess) -> bool,
) -> Vec<BarrierTransition> {
    let pass_accesses = &accesses[pass_access_range];
    let mut barriers = Vec::new();
    for current in pass_accesses {
        let resource = resources[current.resource_index as usize];
        let previous_access_code = if current.previous_access_index == NO_ACCESS {
            if !resource.has_initial_state {
                continue;
            }
            0
        } else {
            let previous = &accesses[current.previous_access_index as usize];
            if can_be_concurrent(current, previous) {
                continue;
            }
            previous.access_code
        };
        barriers.push(BarrierTransition {
            resource_index: current.resource_index,
            previous_access_code,
            next_access_code: current.access_code,
        });
    }
    barriers
}

#[derive(Debug)]
pub struct CompiledGraphSnapshot<Graph, Registry, Pass> {
    pub graph: Graph,
    pub recorded_passes: Vec<Pass>,
    pub registry: Registry,
    pub pending_initial_transitions: Vec<u32>,
}

impl<Graph, Registry, Pass> CompiledGraphSnapshot<Graph, Registry, Pass> {
    pub fn new(
        graph: Graph,
        recorded_passes: Vec<Pass>,
        registry: Registry,
        resource_lifetimes: &[ResourceLifetime],
    ) -> Self {
        Self {
            graph,
            recorded_passes,
            registry,
            pending_initial_transitions: pending_initial_transition_resources(resource_lifetimes),
        }
    }

    pub fn take_initial_transitions(&mut self) -> Vec<u32> {
        std::mem::take(&mut self.pending_initial_transitions)
    }
}
