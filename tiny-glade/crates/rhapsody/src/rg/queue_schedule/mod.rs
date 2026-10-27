//! Queue-class batching preserves the graph's dependency DAG. A physical queue
//! alias never creates a new boundary or a semaphore dependency.
use super::{
    compiled::Batch, graph::PassDeclaration, plan::PlanContext, resource_info::ExportDeclaration,
};
use crate::device::QueueClass;
use std::collections::{BTreeMap, BTreeSet};
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct QueueDomain {
    pub queue_index: u32,
    pub family: u32,
}

#[cfg(test)]
mod resource_contract_tests;
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct QueueSchedule {
    pub domains: Vec<QueueDomain>,
    /// Earlier batch indices whose actual queue differs from this batch.
    pub dependencies: Vec<Vec<usize>>,
}
impl PlanContext {
    pub fn queue_domain(&self, class: QueueClass) -> QueueDomain {
        self.queue_domains
            .map(|domains| {
                domains[match class {
                    QueueClass::Graphics => 0,
                    QueueClass::Compute => 1,
                    QueueClass::Transfer => 2,
                }]
            })
            .unwrap_or(QueueDomain {
                queue_index: self.queue_index,
                family: self.queue_family,
            })
    }
}
pub(crate) fn plan_queue_batches(
    passes: &[PassDeclaration],
    retained: &[usize],
    base: &[Batch],
    edges: &[(usize, usize)],
    exports: &[ExportDeclaration],
    context: &PlanContext,
) -> crate::Result<(Vec<Batch>, QueueSchedule)> {
    let invalid = || crate::Error::invalid_usage("invalid queue batch schedule");
    if retained.iter().any(|&pass| pass >= passes.len()) {
        return Err(invalid());
    }
    let mut next = 0;
    for batch in base {
        if batch.pass_range.start != next
            || batch.pass_range.is_empty()
            || batch.pass_range.end > retained.len()
        {
            return Err(invalid());
        }
        next = batch.pass_range.end;
    }
    if next != retained.len() {
        return Err(invalid());
    }
    let mut families = BTreeMap::new();
    for class in QueueClass::ALL {
        let domain = context.queue_domain(class);
        if families
            .insert(domain.queue_index, domain.family)
            .is_some_and(|family| family != domain.family)
        {
            return Err(invalid());
        }
    }
    let used_queues = retained
        .iter()
        .map(|&pass| context.queue_domain(passes[pass].queue_class).queue_index)
        .collect::<BTreeSet<_>>();
    let multi_queue = used_queues.len() > 1;
    let mut batches = Vec::new();
    let mut schedule = QueueSchedule::default();
    let mut pass_batch = vec![0; retained.len()];
    for batch in base {
        if batch.pass_range.is_empty() || batch.pass_range.end > retained.len() {
            return Err(invalid());
        }
        let mut start = batch.pass_range.start;
        while start < batch.pass_range.end {
            let pass = passes.get(retained[start]).ok_or_else(invalid)?;
            let domain = context.queue_domain(pass.queue_class);
            let end = if multi_queue {
                start + 1
            } else {
                (start + 1..batch.pass_range.end)
                    .find(|&index| {
                        context.queue_domain(passes[retained[index]].queue_class) != domain
                    })
                    .unwrap_or(batch.pass_range.end)
            };
            let index = batches.len();
            pass_batch[start..end].fill(index);
            batches.push(Batch {
                pass_range: start..end,
                export_barriers: false,
            });
            schedule.domains.push(domain);
            start = end;
        }
    }
    let home = context.queue_domain(QueueClass::Graphics);
    if !exports.is_empty()
        && (multi_queue
            || schedule
                .domains
                .last()
                .is_some_and(|domain| *domain != home))
    {
        batches.push(Batch {
            pass_range: retained.len()..retained.len(),
            export_barriers: false,
        });
        schedule.domains.push(home);
    }
    if let Some(last) = batches.last_mut() {
        last.export_barriers = true;
    }
    let mut dependencies = vec![BTreeSet::new(); batches.len()];
    let original_to_scheduled = retained
        .iter()
        .enumerate()
        .map(|(scheduled, &original)| (original, scheduled))
        .collect::<BTreeMap<_, _>>();
    let mut add = |source: usize, destination: usize| {
        if source != destination
            && schedule.domains[source].queue_index != schedule.domains[destination].queue_index
        {
            dependencies[destination].insert(source);
        }
    };
    for &(source, destination) in edges {
        let source = *original_to_scheduled.get(&source).ok_or_else(invalid)?;
        let destination = *original_to_scheduled
            .get(&destination)
            .ok_or_else(invalid)?;
        if source >= destination {
            return Err(invalid());
        }
        add(pass_batch[source], pass_batch[destination]);
    }
    if let Some(last) = batches.len().checked_sub(1) {
        let outputs = exports
            .iter()
            .map(|export| export.resource_index)
            .collect::<BTreeSet<_>>();
        for (index, &original) in retained.iter().enumerate() {
            if passes[original]
                .accesses
                .iter()
                .any(|access| outputs.contains(&access.resource_index))
            {
                add(pass_batch[index], last);
            }
        }
    }
    schedule.dependencies = dependencies
        .into_iter()
        .map(|set| set.into_iter().collect())
        .collect();
    Ok((batches, schedule))
}
#[cfg(test)]
mod tests;

/// Overlapping exclusive ownership needs a declared ordering. Disjoint native
/// ranges can have different owners and do not gain artificial dependencies.
pub(crate) fn validate_resource_queue_contracts(
    definition: &super::plan::GraphDefinition,
    retained: &[usize],
    edges: &[(usize, usize)],
) -> crate::Result<()> {
    let mut successors: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for &(source, destination) in edges {
        successors.entry(source).or_default().push(destination);
    }
    let precedes = |source: usize, destination: usize| {
        let mut pending = successors.get(&source).cloned().unwrap_or_default();
        let mut visited = BTreeSet::new();
        while let Some(pass) = pending.pop() {
            if pass == destination {
                return true;
            }
            if visited.insert(pass) {
                pending.extend(successors.get(&pass).into_iter().flatten().copied());
            }
        }
        false
    };
    for (index, resource) in definition.resources.iter().enumerate() {
        let mut previous = Vec::new();
        for &pass in retained {
            let family = definition
                .context
                .queue_domain(definition.passes[pass].queue_class)
                .family;
            for access in definition.passes[pass]
                .accesses
                .iter()
                .filter(|access| access.resource_index as usize == index)
            {
                if !resource.sharing.allows_family(family) {
                    return Err(crate::Error::invalid_usage(
                        "pass queue family is outside concurrent resource's declared family list",
                    ));
                }
                let range = super::sync_plan::native_range(
                    access.range,
                    resource.descriptor,
                    definition.context.separate_depth_stencil_layouts,
                )?;
                if !resource.sharing.is_concurrent()
                    && previous
                        .iter()
                        .any(|&(source, source_family, previous_range)| {
                            source != pass
                                && source_family != family
                                && crate::tracking::range_state::intersection(range, previous_range)
                                    .is_some()
                                && !precedes(source, pass)
                        })
                {
                    return Err(crate::Error::unsupported(
                        "PreserveParallelism cannot move overlapping exclusive ranges between unordered accesses",
                    ));
                }
                previous.push((pass, family, range));
            }
        }
        if definition
            .exports
            .iter()
            .any(|export| export.resource_index as usize == index)
            && !resource
                .sharing
                .allows_family(definition.context.queue_domain(QueueClass::Graphics).family)
        {
            return Err(crate::Error::invalid_usage(
                "export queue is outside concurrent resource's family list",
            ));
        }
    }
    Ok(())
}
