//! Independent verification of symbolic plans before physical frame binding.
//!
//! The planner uses overlapping producer lists. This verifier instead replays
//! disjoint spatial cells carrying their exact last writer, and checks graph
//! reachability rather than requiring the planner's particular edge set.
use super::{
    graph::Descriptor,
    plan::{COMPILER_VERSION, CompiledPlan, GraphDefinition},
    resource::{AccessRange, AccessVersion},
    resource_info::{LifetimeEvent, NO_ACCESS},
};
use std::collections::{BTreeMap, BTreeSet};

mod bound;
mod native_state;
mod range_oracle;
mod range_program;
pub use bound::check_bound_synchronization;
pub use native_state::{StatePublication, check_state_publication};

fn invalid(message: impl Into<std::borrow::Cow<'static, str>>) -> crate::Error {
    crate::Error::invalid_usage(message)
}

fn usage_flags(descriptor: Descriptor, mask: u32) -> crate::Result<u32> {
    use crate::raw::resource_usage::{
        buffer_access_mask_to_usage_flags, image_access_mask_to_usage_flags,
    };
    match descriptor {
        Descriptor::Image(_) => image_access_mask_to_usage_flags(mask).map(|flags| flags.as_raw()),
        Descriptor::Buffer(_) => Ok(buffer_access_mask_to_usage_flags(mask).as_raw()),
    }
}

/// A rectangular cell: bytes for buffers, aspect/mip/layer for images.
/// Whole-only synthetic descriptors use a single opaque unit cell.
#[derive(Clone, Copy, Debug)]
struct Cell([(u64, u64); 3]);
impl Cell {
    fn intersects(self, other: Self) -> bool {
        self.0
            .iter()
            .zip(other.0)
            .all(|(&(a, b), (c, d))| a < d && c < b)
    }

    fn without(self, other: Self) -> Vec<Self> {
        if !self.intersects(other) {
            return vec![self];
        }
        let mut middle = self;
        let mut outside = Vec::new();
        // Peel slabs off each axis, leaving exactly the intersection behind.
        for axis in 0..3 {
            let (low, high) = middle.0[axis];
            let start = low.max(other.0[axis].0);
            let end = high.min(other.0[axis].1);
            if low < start {
                let mut slab = middle;
                slab.0[axis].1 = start;
                outside.push(slab);
            }
            if end < high {
                let mut slab = middle;
                slab.0[axis].0 = end;
                outside.push(slab);
            }
            middle.0[axis] = (start, end);
        }
        outside
    }
}

struct Domain {
    descriptor: Descriptor,
    opaque: bool,
    content: Vec<(Cell, Option<usize>)>,
    version: u32,
}
impl Domain {
    fn cells(&self, range: AccessRange) -> crate::Result<Vec<Cell>> {
        range
            .validate(self.descriptor)
            .map_err(|_| invalid("plan contains an invalid access range"))?;
        if self.opaque {
            return Ok(vec![Cell([(0, 1); 3])]);
        }
        match range
            .normalized(self.descriptor)
            .map_err(|_| invalid("plan range cannot be normalized"))?
        {
            AccessRange::Buffer { offset, size } => {
                Ok(vec![Cell([(offset, offset + size), (0, 1), (0, 1)])])
            }
            AccessRange::Texture {
                aspect_mask,
                base_mip_level,
                level_count,
                base_array_layer,
                layer_count,
            } => Ok((0..u32::BITS)
                .filter(|bit| aspect_mask & (1u32 << bit) != 0)
                .map(|bit| {
                    Cell([
                        (u64::from(bit), u64::from(bit) + 1),
                        (
                            u64::from(base_mip_level),
                            u64::from(base_mip_level) + u64::from(level_count),
                        ),
                        (
                            u64::from(base_array_layer),
                            u64::from(base_array_layer) + u64::from(layer_count),
                        ),
                    ])
                })
                .collect()),
            AccessRange::Whole => unreachable!("normalized range must be concrete"),
        }
    }

    fn sources(&self, range: AccessRange) -> crate::Result<BTreeSet<usize>> {
        self.sources_with_undefined(range, false)
    }

    // Independent checker still collects all available writers. Only a caller's
    // explicit undefined-content read may leave cells without a content source.
    fn sources_with_undefined(&self, range: AccessRange, allow_undefined: bool)
        -> crate::Result<BTreeSet<usize>> {
        let mut missing = self.cells(range)?;
        let mut sources = BTreeSet::new();
        for &(defined, writer) in &self.content {
            if missing.iter().any(|cell| cell.intersects(defined)) {
                sources.extend(writer);
            }
            missing = missing
                .into_iter()
                .flat_map(|cell| cell.without(defined))
                .collect();
        }
        if missing.is_empty() || allow_undefined {
            Ok(sources)
        } else {
            Err(invalid("plan reads content without an initialized source"))
        }
    }

    fn write(&mut self, range: AccessRange, writer: usize) -> crate::Result<()> {
        for written in self.cells(range)? {
            self.content = self
                .content
                .drain(..)
                .flat_map(|(cell, source)| {
                    cell.without(written)
                        .into_iter()
                        .map(move |remaining| (remaining, source))
                })
                .collect();
            self.content.push((written, Some(writer)));
        }
        Ok(())
    }
}

struct Provenance {
    dependencies: Vec<BTreeSet<usize>>,
    roots: BTreeSet<usize>,
}

fn inspect_definition(definition: &GraphDefinition) -> crate::Result<Provenance> {
    if definition.compiler_version != COMPILER_VERSION {
        return Err(invalid(
            "graph definition uses an incompatible compiler version",
        ));
    }
    if definition.resources.len() > usize::from(u16::MAX)
        || definition.passes.len() > super::graph::MAX_PASSES
    {
        return Err(invalid("graph definition exceeds packed index limits"));
    }
    let mut semantics = BTreeMap::new();
    for entry in &definition.access_semantics {
        if usize::from(entry.code) >= crate::raw::access_table::ACCESS_CODE_COUNT
            || semantics.insert(entry.code, entry).is_some()
        {
            return Err(invalid("graph access semantics are invalid or duplicated"));
        }
    }
    let mut opaque = vec![true; definition.resources.len()];
    for access in definition.passes.iter().flat_map(|pass| &pass.accesses) {
        let entry = opaque
            .get_mut(access.resource_index as usize)
            .ok_or_else(|| invalid("graph access references an unknown resource"))?;
        *entry &= access.range == AccessRange::Whole;
    }
    for export in &definition.exports {
        let entry = opaque
            .get_mut(export.resource_index as usize)
            .ok_or_else(|| invalid("graph export references an unknown resource"))?;
        *entry &= export.range == AccessRange::Whole;
    }
    let mut domains: Vec<_> = definition
        .resources
        .iter()
        .zip(opaque)
        .map(|(resource, opaque)| Domain {
            descriptor: resource.descriptor,
            opaque,
            content: Vec::new(),
            version: 0,
        })
        .collect();
    for (domain, resource) in domains.iter_mut().zip(&definition.resources) {
        if resource.is_external() {
            domain.content = domain
                .cells(AccessRange::Whole)?
                .into_iter()
                .map(|cell| (cell, None))
                .collect();
        }
    }
    let mut dependencies = vec![BTreeSet::new(); definition.passes.len()];
    let mut roots = BTreeSet::new();
    let mut access_count = 0usize;
    for (index, pass) in definition.passes.iter().enumerate() {
        if pass.graph_id.is_some() {
            return Err(invalid(
                "pure graph definition contains a frame graph identity",
            ));
        }
        if pass.side_effect {
            roots.insert(index);
        }
        access_count = access_count
            .checked_add(pass.accesses.len())
            .ok_or_else(|| invalid("graph access count overflows"))?;
        if access_count >= NO_ACCESS as usize {
            return Err(invalid("graph access count exceeds packed index limits"));
        }
        for access in &pass.accesses {
            let domain = domains
                .get_mut(access.resource_index as usize)
                .ok_or_else(|| invalid("graph access references an unknown resource"))?;
            let semantic = semantics
                .get(&access.access_code)
                .ok_or_else(|| invalid("graph access has no classification"))?;
            usage_flags(domain.descriptor, semantic.mask)?;
            domain.cells(access.range)?;
            if access.contents_unused && (semantic.write || access.reads_previous) {
                return Err(invalid(
                    "inactive contents intent requires a non-writing access",
                ));
            }
            if access.allow_undefined_contents
                && (semantic.write || access.reads_previous || access.contents_unused) {
                return Err(invalid("undefined contents permission requires a real non-writing read"));
            }
            if access.reads_previous {
                if !semantic.write {
                    return Err(invalid(
                        "a read access cannot also declare a read-modify-write dependency",
                    ));
                }
                dependencies[index].extend(
                    domain
                        .sources(access.range)?
                        .into_iter()
                        .filter(|&source| source != index),
                );
            }
            match (semantic.write, access.version) {
                (true, AccessVersion::Write { previous, current })
                    if previous == domain.version && previous.checked_add(1) == Some(current) =>
                {
                    domain.write(access.range, index)?;
                    domain.version = current;
                }
                (false, AccessVersion::Read(version)) if version == domain.version => {
                    if !access.contents_unused {
                        dependencies[index].extend(
                            domain
                                .sources_with_undefined(access.range, access.allow_undefined_contents)?
                                .into_iter()
                                .filter(|&source| source != index),
                        );
                    }
                }
                _ => {
                    return Err(invalid(
                        "graph access has an unresolved, stale or incorrectly classified version",
                    ));
                }
            }
        }
    }
    for export in &definition.exports {
        let domain = domains
            .get(export.resource_index as usize)
            .ok_or_else(|| invalid("graph export references an unknown resource"))?;
        if export.tag != domain.descriptor.kind_code()
            || !semantics.contains_key(&export.access_code)
        {
            return Err(invalid(
                "graph export has an invalid kind or unclassified access",
            ));
        }
        usage_flags(domain.descriptor, semantics[&export.access_code].mask)?;
        roots.extend(domain.sources(export.range)?);
    }
    super::subgraph::validate_subgraphs(definition)?;
    Ok(Provenance {
        dependencies,
        roots,
    })
}

/// Validates public pure declarations before the planner may index them.
/// This intentionally does not invoke graph construction or contract resolution.
pub fn check_definition(definition: &GraphDefinition) -> crate::Result<()> {
    inspect_definition(definition).map(|_| ())
}

/// Checks declaration contracts, liveness, ordering and the executable projection.
/// Safe conservative retention and transitive dependency paths are accepted.
pub fn check_plan(definition: &GraphDefinition, plan: &CompiledPlan) -> crate::Result<()> {
    if definition != &plan.definition {
        return Err(invalid("plan is bound to a different graph definition"));
    }
    let provenance = inspect_definition(definition)?;
    let mut positions = vec![None; definition.passes.len()];
    for (position, &pass) in plan.retained_passes.iter().enumerate() {
        let entry = positions
            .get_mut(pass)
            .ok_or_else(|| invalid("plan schedules an unknown pass"))?;
        if entry.replace(position).is_some() {
            return Err(invalid("plan schedules a pass more than once"));
        }
    }
    let mut needed = provenance.roots;
    // Extra retained passes must also retain the producers of their own reads.
    needed.extend(plan.retained_passes.iter().copied());
    let mut work: Vec<_> = needed.iter().copied().collect();
    while let Some(pass) = work.pop() {
        for &source in &provenance.dependencies[pass] {
            if needed.insert(source) {
                work.push(source);
            }
        }
    }
    if needed.iter().any(|&pass| positions[pass].is_none()) {
        return Err(invalid(
            "plan culled a required output, side effect or read producer",
        ));
    }

    let mut edges = vec![Vec::new(); definition.passes.len()];
    for &(from, to) in &plan.dependency_edges {
        let source = positions.get(from).and_then(|p| *p);
        let target = positions.get(to).and_then(|p| *p);
        if !matches!((source, target), (Some(a), Some(b)) if a < b) {
            return Err(invalid(
                "plan dependency is missing a scheduled endpoint or points backwards",
            ));
        }
        edges[from].push(to);
    }
    for (earlier, pass) in definition
        .passes
        .iter()
        .enumerate()
        .filter(|(index, _)| positions[*index].is_some())
    {
        let mut required = BTreeSet::new();
        for (later, next) in definition
            .passes
            .iter()
            .enumerate()
            .skip(earlier + 1)
            .filter(|(index, _)| positions[*index].is_some())
        {
            if pass.accesses.iter().any(|a| {
                next.accesses.iter().any(|b| {
                    a.resource_index == b.resource_index
                        && a.range.overlaps(b.range)
                        && (definition.is_write(a.access_code)
                            || definition.is_write(b.access_code))
                })
            }) {
                required.insert(later);
            }
        }
        if required.is_empty() {
            continue;
        }
        let mut visited = BTreeSet::new();
        let mut pending = edges[earlier].clone();
        while let Some(next) = pending.pop() {
            if visited.insert(next) {
                pending.extend(&edges[next]);
            }
        }
        if !required.is_subset(&visited) {
            return Err(invalid(
                "plan lacks a dependency path for conflicting resource accesses",
            ));
        }
    }
    check_projection(definition, plan)?;
    range_program::check_range_program(definition, plan, &plan.range_program)
}

fn check_projection(definition: &GraphDefinition, plan: &CompiledPlan) -> crate::Result<()> {
    let resources = definition.resources.len();
    let passes = plan.retained_passes.len();
    if plan.info.pass_access_ranges.len() != passes
        || plan.info.lifetimes.len() != resources
        || plan.info.image_usage_flags.len() != resources
        || plan.info.buffer_usage_flags.len() != resources
    {
        return Err(invalid(
            "plan executable table dimensions do not match declarations",
        ));
    }
    let mut previous = vec![NO_ACCESS; resources];
    let mut first = vec![NO_ACCESS; resources];
    let mut pass_bounds = vec![None; resources];
    let mut required_usage: Vec<_> = definition
        .resources
        .iter()
        .map(|resource| {
            if resource.is_external() {
                0
            } else {
                resource.descriptor.usage()
            }
        })
        .collect();
    let masks: BTreeMap<_, _> = definition
        .access_semantics
        .iter()
        .map(|semantic| (semantic.code, semantic.mask))
        .collect();
    let mut cursor = 0usize;
    for (scheduled, &original) in plan.retained_passes.iter().enumerate() {
        let (start, end) = plan.info.pass_access_ranges[scheduled];
        let accesses = &definition.passes[original].accesses;
        if start.0 as usize != cursor || end.0 as usize != cursor + accesses.len() {
            return Err(invalid(
                "plan pass access mapping differs from its declaration",
            ));
        }
        for access in accesses {
            let actual = plan
                .info
                .accesses
                .get(cursor)
                .ok_or_else(|| invalid("plan is missing a resource access"))?;
            let resource = access.resource_index as usize;
            required_usage[resource] |= usage_flags(
                definition.resources[resource].descriptor,
                masks[&access.access_code],
            )?;
            let bounds = pass_bounds[resource].get_or_insert((scheduled, scheduled));
            bounds.1 = scheduled;
            let prior = previous[resource];
            if actual.resource_index != access.resource_index
                || actual.access_code != access.access_code
                || actual.synchronization_group != access.synchronization_group
                || actual.previous_access_index != prior
            {
                return Err(invalid(
                    "plan resource access chain differs from its declaration",
                ));
            }
            if prior != NO_ACCESS {
                let before = &plan.info.accesses[prior as usize];
                if before.next_access_index as usize != cursor {
                    return Err(invalid(
                        "plan resource access chain has an invalid successor",
                    ));
                }
            } else {
                first[resource] = cursor as u32;
            }
            previous[resource] = cursor as u32;
            cursor += 1;
        }
    }
    if cursor != plan.info.accesses.len() {
        return Err(invalid("plan contains undeclared resource accesses"));
    }
    for export in &definition.exports {
        let resource = export.resource_index as usize;
        required_usage[resource] |= usage_flags(
            definition.resources[resource].descriptor,
            masks[&export.access_code],
        )?;
    }
    let mut lifetime_events = Vec::new();
    for resource in 0..resources {
        let actual_usage = match definition.resources[resource].descriptor {
            Descriptor::Image(_) => plan.info.image_usage_flags[resource],
            Descriptor::Buffer(_) => plan.info.buffer_usage_flags[resource],
        };
        if actual_usage & required_usage[resource] != required_usage[resource] {
            return Err(invalid(
                "plan resource usage omits a declared access or creation requirement",
            ));
        }
        let lifetime = &plan.info.lifetimes[resource];
        if lifetime.first_access_index != first[resource]
            || lifetime.last_access_index != previous[resource]
        {
            return Err(invalid(
                "plan resource lifetime points to the wrong accesses",
            ));
        }
        let bounds = if definition.resources[resource].is_external() && passes != 0 {
            Some((0, passes - 1))
        } else {
            pass_bounds[resource]
        };
        if let Some((start, end)) = bounds {
            if lifetime.first_pass != Some(start as u32) || lifetime.last_pass != Some(end as u32) {
                return Err(invalid(
                    "plan resource lifetime does not span its scheduled accesses",
                ));
            }
            for (pass, release) in [(start, false), (end, true)] {
                lifetime_events.push(LifetimeEvent {
                    pass: pass as u32,
                    release,
                    resource: resource as u16,
                });
            }
        } else if lifetime.last_pass.is_some()
            || lifetime.first_pass.is_some() != definition.resources[resource].is_external()
        {
            return Err(invalid(
                "plan has an active lifetime for an unused resource",
            ));
        }
        if previous[resource] != NO_ACCESS
            && plan.info.accesses[previous[resource] as usize].next_access_index != NO_ACCESS
        {
            return Err(invalid(
                "plan resource access chain has a successor after its last access",
            ));
        }
    }
    lifetime_events.sort_unstable();
    if lifetime_events != plan.info.lifetime_events {
        return Err(invalid(
            "plan lifetime events do not match resource lifetimes",
        ));
    }
    let expected_exports: BTreeSet<_> = definition
        .exports
        .iter()
        .map(|export| export.resource_index as u16)
        .collect();
    if expected_exports != plan.info.exported_indices.iter().copied().collect() {
        return Err(invalid(
            "plan exported resource set differs from its declaration",
        ));
    }
    if plan.queue_schedule.domains.len() != plan.batches.len()
        || plan.queue_schedule.dependencies.len() != plan.batches.len()
    {
        return Err(invalid("queue schedule and batches have different lengths"));
    }
    let mut scheduled_batch = vec![0; passes];
    let mut next_pass = 0;
    for (index, batch) in plan.batches.iter().enumerate() {
        let empty_export = index + 1 == plan.batches.len()
            && !definition.exports.is_empty()
            && next_pass == passes
            && batch.pass_range.end == next_pass
            && plan.queue_schedule.domains[index]
                == definition
                    .context
                    .queue_domain(crate::device::QueueClass::Graphics);
        if batch.pass_range.start != next_pass
            || (batch.pass_range.end <= next_pass && !empty_export)
            || batch.pass_range.end > passes
            || batch.export_barriers != (index + 1 == plan.batches.len())
        {
            return Err(invalid(
                "plan batches do not cover each scheduled pass exactly once",
            ));
        }
        for scheduled in batch.pass_range.clone() {
            scheduled_batch[scheduled] = index;
            let original = plan.retained_passes[scheduled];
            if plan.queue_schedule.domains[index]
                != definition
                    .context
                    .queue_domain(definition.passes[original].queue_class)
            {
                return Err(invalid(
                    "batch contains a pass assigned to another physical queue",
                ));
            }
        }
        for &source in &plan.queue_schedule.dependencies[index] {
            if source >= index
                || plan.queue_schedule.domains[source].queue_index
                    == plan.queue_schedule.domains[index].queue_index
            {
                return Err(invalid(
                    "batch semaphore dependency is cyclic or redundant on an aliased queue",
                ));
            }
        }
        next_pass = batch.pass_range.end;
    }
    if next_pass != passes {
        return Err(invalid("plan has scheduled passes outside its batches"));
    }
    let original_batch = plan
        .retained_passes
        .iter()
        .enumerate()
        .map(|(scheduled, &original)| (original, scheduled_batch[scheduled]))
        .collect::<BTreeMap<_, _>>();
    for &(source, destination) in &plan.dependency_edges {
        let (source, destination) = (original_batch[&source], original_batch[&destination]);
        if source != destination
            && plan.queue_schedule.domains[source].queue_index
                != plan.queue_schedule.domains[destination].queue_index
            && !plan.queue_schedule.dependencies[destination].contains(&source)
        {
            return Err(invalid(
                "cross-queue graph dependency has no semaphore batch edge",
            ));
        }
    }
    if let Some(last) = plan.batches.len().checked_sub(1) {
        for (scheduled, &original) in plan.retained_passes.iter().enumerate() {
            if definition.passes[original]
                .accesses
                .iter()
                .any(|access| expected_exports.contains(&(access.resource_index as u16)))
            {
                let source = scheduled_batch[scheduled];
                if source != last
                    && plan.queue_schedule.domains[source].queue_index
                        != plan.queue_schedule.domains[last].queue_index
                    && !plan.queue_schedule.dependencies[last].contains(&source)
                {
                    return Err(invalid(
                        "graphics export batch does not wait for every exported-resource user",
                    ));
                }
            }
        }
    }
    check_snapshot_projection(definition, plan)
}

fn check_snapshot_projection(
    definition: &GraphDefinition,
    plan: &CompiledPlan,
) -> crate::Result<()> {
    let snapshot = &plan.snapshot;
    let info = &plan.info;
    if !snapshot.pass_order.iter().map(String::as_str).eq(plan
        .retained_passes
        .iter()
        .map(|&index| definition.passes[index].name.as_ref()))
        || !snapshot.pass_ranges.iter().cloned().eq(info
            .pass_access_ranges
            .iter()
            .map(|(start, end)| start.0 as usize..end.0 as usize))
        || snapshot.accesses != info.accesses
        || snapshot.lifetimes != info.lifetimes
        || snapshot.image_usage != info.image_usage_flags
        || snapshot.buffer_usage != info.buffer_usage_flags
        || snapshot.lifetime_events != info.lifetime_events
        || snapshot.batches != plan.batches
        || !snapshot.imports.iter().copied().eq(definition
            .resources
            .iter()
            .enumerate()
            .filter_map(|(index, resource)| resource.is_external().then_some(index as u32)))
        || snapshot.exports.len() != info.exported_indices.len()
        || snapshot.exports.windows(2).any(|pair| pair[0] >= pair[1])
        || snapshot
            .exports
            .iter()
            .any(|index| !info.exported_indices.contains(index))
    {
        return Err(invalid("shared snapshot differs from the executable plan"));
    }
    Ok(())
}
