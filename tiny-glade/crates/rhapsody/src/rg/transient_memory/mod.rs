//! Physical transient-memory planning using driver requirements and existing
//! dependency proofs. The production backend queries these facts, binds each
//! multi-member backing once, and initializes each alias at its first pass.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Placement {
    pub resource: u32,
    pub backing: usize,
    pub offset: u64,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PlanStats {
    pub allocated_bytes: u64,
    pub unaliased_bytes: u64,
    pub reuse_bytes: u64,
    pub allocation_count: usize,
}

/// Immutable physical facts queried with vkGet{Buffer,Image}MemoryRequirements2.
/// Pass IDs refer to the existing dependency graph, never recording order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MemoryRequirement {
    pub sharing: crate::tracking::ResourceSharing,
    pub resource: u32,
    /// Linear buffers and optimal images use different classes, avoiding the
    /// bufferImageGranularity constraint entirely. Allocation flags also match.
    pub compatibility: u64,
    pub size: u64,
    pub alignment: u64,
    pub memory_type_bits: u32,
    pub allocation_flags: u32,
    pub requires_dedicated: bool,
    pub prefers_dedicated: bool,
    pub uses: Vec<usize>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PhysicalBacking {
    pub sharing: crate::tracking::ResourceSharing,
    /// Filled only by physical preparation after a real Vulkan allocation.
    pub allocation_id: Option<u64>,
    pub memory_type: Option<u32>,
    pub size: u64,
    pub alignment: u64,
    pub memory_type_bits: u32,
    pub allocation_flags: u32,
    pub resources: Vec<u32>,
}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PhysicalMemoryPlan {
    pub dependency_edges: Vec<(usize, usize)>,
    pub requirements: Vec<MemoryRequirement>,
    pub backings: Vec<PhysicalBacking>,
    pub placements: Vec<Placement>,
    pub stats: PlanStats,
}
impl PhysicalMemoryPlan {
    pub fn dump(&self) -> String {
        use std::fmt::Write;
        let mut out = format!(
            "policy=PreserveParallelism added_edges=0 planned_bytes={} unaliased={} saved={} allocations={}\n",
            self.stats.allocated_bytes,
            self.stats.unaliased_bytes,
            self.stats.reuse_bytes,
            self.stats.allocation_count
        );
        for (index, b) in self.backings.iter().enumerate() {
            let _ = writeln!(
                out,
                "backing={index} size={} alignment={} memory_bits={:#x} flags={:#x} allocation_id={:?} memory_type={:?}",
                b.size,
                b.alignment,
                b.memory_type_bits,
                b.allocation_flags,
                b.allocation_id,
                b.memory_type
            );
            for id in &b.resources {
                let fact = self
                    .requirements
                    .iter()
                    .find(|f| f.resource == *id)
                    .expect("planned resource");
                let _ = writeln!(
                    out,
                    "  resource={id} offset=0 size={} alignment={} uses={:?} dedicated={} preferred={}",
                    fact.size,
                    fact.alignment,
                    fact.uses,
                    fact.requires_dedicated,
                    fact.prefers_dedicated
                );
            }
        }
        out
    }
}
/// Greedy alias coloring with a proof of ordering in the pre-existing DAG.
/// It never adds a dependency, including between independent passes recorded in
/// different command buffers. Required dedicated allocations always stand alone.
pub fn plan_memory_requirements(
    requirements: &[MemoryRequirement],
    dependency_edges: &[(usize, usize)],
) -> crate::Result<PhysicalMemoryPlan> {
    use std::collections::{BTreeMap, BTreeSet};
    let mut facts = BTreeMap::new();
    for fact in requirements {
        if fact.size == 0
            || fact.alignment == 0
            || !fact.alignment.is_power_of_two()
            || fact.memory_type_bits == 0
            || fact.uses.is_empty()
            || facts.insert(fact.resource, fact).is_some()
        {
            return Err(crate::Error::invalid_usage(
                "invalid or duplicate transient memory requirement",
            ));
        }
    }
    let mut successors: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for &(from, to) in dependency_edges {
        if from >= to {
            return Err(crate::Error::invalid_usage(
                "transient dependencies must follow the graph DAG",
            ));
        }
        successors.entry(from).or_default().push(to);
    }
    let mut reachable: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();
    for pass in requirements.iter().flat_map(|r| r.uses.iter()).copied() {
        reachable.entry(pass).or_insert_with(|| {
            let mut result = BTreeSet::new();
            let mut pending = successors.get(&pass).cloned().unwrap_or_default();
            while let Some(next) = pending.pop() {
                if result.insert(next) {
                    pending.extend(successors.get(&next).into_iter().flatten().copied());
                }
            }
            result
        });
    }
    let ordered = |a: &MemoryRequirement, b: &MemoryRequirement| {
        a.uses.iter().all(|source| {
            b.uses
                .iter()
                .all(|target| reachable[source].contains(target))
        })
    };
    let aligned = |size: u64, alignment: u64| {
        crate::collections::align::align_up(size, alignment)
            .ok_or_else(|| crate::Error::invalid_usage("transient memory requirement overflows"))
    };
    let mut order = requirements.iter().collect::<Vec<_>>();
    order.sort_by_key(|f| (std::cmp::Reverse(f.size), f.resource));
    let mut result = PhysicalMemoryPlan {
        dependency_edges: dependency_edges.to_vec(),
        requirements: requirements.to_vec(),
        ..Default::default()
    };
    for fact in order {
        let size = aligned(fact.size, fact.alignment)?;
        result.stats.unaliased_bytes = result
            .stats
            .unaliased_bytes
            .checked_add(size)
            .ok_or_else(|| crate::Error::invalid_usage("transient memory total overflows"))?;
        let target = (!fact.requires_dedicated)
            .then(|| {
                result.backings.iter().position(|backing| {
                    backing.memory_type_bits & fact.memory_type_bits != 0
                        && backing.allocation_flags == fact.allocation_flags
                        && backing.sharing == fact.sharing
                        && backing.resources.iter().all(|id| {
                            let placed = facts[id];
                            !placed.requires_dedicated
                                && placed.compatibility == fact.compatibility
                                && (ordered(placed, fact) || ordered(fact, placed))
                        })
                })
            })
            .flatten();
        let backing = match target {
            Some(index) => {
                let b = &mut result.backings[index];
                b.alignment = b.alignment.max(fact.alignment);
                b.size = aligned(b.size.max(size), b.alignment)?;
                b.memory_type_bits &= fact.memory_type_bits;
                b.resources.push(fact.resource);
                index
            }
            None => {
                let index = result.backings.len();
                result.backings.push(PhysicalBacking {
                    sharing: fact.sharing,
                    allocation_id: None,
                    memory_type: None,
                    size,
                    alignment: fact.alignment,
                    memory_type_bits: fact.memory_type_bits,
                    allocation_flags: fact.allocation_flags,
                    resources: vec![fact.resource],
                });
                index
            }
        };
        result.placements.push(Placement {
            resource: fact.resource,
            backing,
            offset: 0,
        });
    }
    result.stats.allocated_bytes = result
        .backings
        .iter()
        .try_fold(0u64, |sum, b| sum.checked_add(b.size))
        .ok_or_else(|| crate::Error::invalid_usage("transient memory total overflows"))?;
    result.stats.reuse_bytes = result.stats.unaliased_bytes - result.stats.allocated_bytes;
    result.stats.allocation_count = result.backings.len();
    Ok(result)
}

#[cfg(test)]
mod physical_tests;
