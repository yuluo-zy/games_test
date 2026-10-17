//! Native program projection checked from declarations using independent cells.
use super::{
    invalid,
    range_oracle::{SpatialLedger, contains},
};
use crate::{
    resource::range_state::AccessScope,
    rg::{
        graph::Descriptor,
        plan::{CompiledPlan, GraphDefinition},
        resource::AccessRange,
        sync_plan::{RangeProgram, RangeUse},
    },
};
use ash::vk;
use std::collections::BTreeSet;

fn native_range(
    range: AccessRange,
    descriptor: Descriptor,
    separate_depth_stencil: bool,
) -> crate::Result<crate::resource::OwnershipRange> {
    use crate::resource::OwnershipRange;
    let normalized = range
        .normalized(descriptor)
        .map_err(|_| invalid("native program contains an invalid declared range"))?;
    match normalized {
        AccessRange::Buffer { offset, size } => Ok(OwnershipRange::Buffer { offset, size }),
        AccessRange::Texture {
            mut aspect_mask,
            base_mip_level,
            level_count,
            base_array_layer,
            layer_count,
        } => {
            if !separate_depth_stencil {
                let AccessRange::Texture {
                    aspect_mask: all, ..
                } = AccessRange::Whole
                    .normalized(descriptor)
                    .map_err(|_| invalid("native image domain is invalid"))?
                else {
                    return Err(invalid("native image has a buffer domain"));
                };
                let both = (vk::ImageAspectFlags::DEPTH | vk::ImageAspectFlags::STENCIL).as_raw();
                if all & both == both && aspect_mask & both != 0 {
                    aspect_mask |= both;
                }
            }
            Ok(OwnershipRange::Image {
                aspects: aspect_mask,
                base_mip: base_mip_level,
                mip_count: level_count,
                base_layer: base_array_layer,
                layer_count,
            })
        }
        AccessRange::Whole => Err(invalid("native range remained opaque after normalization")),
    }
}

#[derive(Clone, Default)]
struct Expected {
    scope: Option<AccessScope>,
    layout: vk::ImageLayout,
    codes: BTreeSet<u8>,
    seen: bool,
}

fn check_uses(
    definition: &GraphDefinition,
    program: &RangeProgram,
    declared: impl IntoIterator<Item = (u32, AccessRange, u8)>,
    actual: &[RangeUse],
) -> crate::Result<()> {
    let mut resources = program
        .resources
        .iter()
        .map(|resource| SpatialLedger::new(resource.range, [(resource.range, Expected::default())]))
        .collect::<crate::Result<Vec<_>>>()?;
    for (resource, range, code) in declared {
        let descriptor = definition
            .resources
            .get(resource as usize)
            .ok_or_else(|| invalid("native use references an unknown declaration"))?
            .descriptor;
        let semantics = definition
            .access_semantics
            .iter()
            .find(|semantics| semantics.code == code)
            .ok_or_else(|| invalid("native use has no declared Vulkan semantics"))?;
        let range = native_range(range, descriptor, program.separate_depth_stencil)?;
        let scope = AccessScope {
            stages: semantics.stages,
            access: semantics.access,
            write: semantics.write,
        };
        let layout = match descriptor {
            Descriptor::Buffer(_) => vk::ImageLayout::UNDEFINED,
            Descriptor::Image(_) => semantics.layout,
        };
        if matches!(descriptor, Descriptor::Image(_))
            && matches!(
                layout,
                vk::ImageLayout::UNDEFINED | vk::ImageLayout::PREINITIALIZED
            )
        {
            return Err(invalid(
                "native image access uses an initial-only target layout",
            ));
        }
        resources[resource as usize].update(range, |old| {
            if old.scope.is_some() && old.layout != layout {
                return Err(invalid(
                    "one native pass requires conflicting image layouts",
                ));
            }
            let mut expected = old.clone();
            expected.scope = Some(match old.scope {
                None => scope,
                Some(old) => AccessScope {
                    stages: old.stages | scope.stages,
                    access: old.access | scope.access,
                    write: old.write || scope.write,
                },
            });
            expected.layout = layout;
            expected.codes.insert(code);
            Ok(expected)
        })?;
    }
    for actual in actual {
        let domain = program
            .resources
            .get(actual.resource as usize)
            .ok_or_else(|| invalid("native program use references an unknown resource"))?;
        if !contains(domain.range, actual.range)? {
            return Err(invalid("native program use exceeds its physical domain"));
        }
        resources[actual.resource as usize].update(actual.range, |expected| {
            if expected.seen
                || expected.scope != Some(actual.scope)
                || expected.layout != actual.layout
                || !expected.codes.contains(&actual.access_code)
            {
                return Err(invalid(
                    "native program does not match declared range semantics",
                ));
            }
            Ok(Expected {
                seen: true,
                ..expected.clone()
            })
        })?;
    }
    for (resource, domain) in resources.iter().zip(&program.resources) {
        resource.visit(domain.range, |expected| {
            if expected.scope.is_some() != expected.seen {
                return Err(invalid("native program omits a declared access range"));
            }
            Ok(())
        })?;
    }
    Ok(())
}

pub(super) fn check_range_program(
    definition: &GraphDefinition,
    plan: &CompiledPlan,
    program: &RangeProgram,
) -> crate::Result<()> {
    if program.separate_depth_stencil != definition.context.separate_depth_stencil_layouts
        || program.resources.len() != definition.resources.len()
        || program.passes.len() != plan.retained_passes.len()
        || program.batches.len() != plan.batches.len()
    {
        return Err(invalid("native program shape differs from its pure plan"));
    }
    for (actual, resource) in program.resources.iter().zip(&definition.resources) {
        if actual.sharing != resource.sharing
            || actual.range
                != native_range(
                    AccessRange::Whole,
                    resource.descriptor,
                    program.separate_depth_stencil,
                )?
        {
            return Err(invalid(
                "native program resource domain differs from its descriptor",
            ));
        }
    }
    let mut cursor = 0;
    for (scheduled, (actual, &original)) in
        program.passes.iter().zip(&plan.retained_passes).enumerate()
    {
        if actual.pass != scheduled
            || actual.uses.start != cursor
            || actual.uses.end < cursor
            || actual.uses.end > program.uses.len()
        {
            return Err(invalid(
                "native program pass slices do not partition its uses",
            ));
        }
        check_uses(
            definition,
            program,
            definition.passes[original]
                .accesses
                .iter()
                .map(|access| (access.resource_index, access.range, access.access_code)),
            &program.uses[actual.uses.clone()],
        )?;
        cursor = actual.uses.end;
    }
    if cursor != program.uses.len() {
        return Err(invalid("native program has uses outside scheduled passes"));
    }
    check_uses(
        definition,
        program,
        definition
            .exports
            .iter()
            .map(|export| (export.resource_index, export.range, export.access_code)),
        &program.exports,
    )?;
    for (index, (actual, expected)) in program.batches.iter().zip(&plan.batches).enumerate() {
        if actual.domain != plan.queue_schedule.domains[index]
            || actual.passes != expected.pass_range
        {
            return Err(invalid(
                "native program batch differs from queue scheduling",
            ));
        }
    }
    let mut edges = vec![Vec::new(); program.batches.len()];
    let mut seen = BTreeSet::new();
    for &(source, destination) in &program.dependencies {
        if source >= destination
            || destination >= edges.len()
            || !seen.insert((source, destination))
            || program.batches[source].domain.queue_index
                == program.batches[destination].domain.queue_index
        {
            return Err(invalid(
                "native program contains an invalid batch dependency",
            ));
        }
        edges[source].push(destination);
    }
    for (source, earlier) in program.batches.iter().enumerate() {
        for (destination, later) in program.batches.iter().enumerate().skip(source + 1) {
            if earlier.domain.queue_index == later.domain.queue_index {
                edges[source].push(destination);
            }
        }
    }
    let mut reachability = vec![BTreeSet::new(); edges.len()];
    for source in 0..edges.len() {
        let mut pending = edges[source].clone();
        while let Some(destination) = pending.pop() {
            if reachability[source].insert(destination) {
                pending.extend(&edges[destination]);
            }
        }
    }
    for (destination, required) in plan.queue_schedule.dependencies.iter().enumerate() {
        if required
            .iter()
            .any(|&source| !reachability[source].contains(&destination))
        {
            return Err(invalid("native program omits a graph queue dependency"));
        }
    }
    for (source, earlier) in program.batches.iter().enumerate() {
        for (destination, later) in program.batches.iter().enumerate().skip(source + 1) {
            let source_uses = earlier.passes.clone().flat_map(|pass| {
                program.uses[program.passes[pass].uses.clone()]
                    .iter()
                    .copied()
            });
            let mut conflict = false;
            for a in source_uses {
                for pass in later.passes.clone() {
                    for b in &program.uses[program.passes[pass].uses.clone()] {
                        if a.resource == b.resource
                            && ranges_overlap(a.range, b.range)?
                            && (a.scope.write || b.scope.write || a.layout != b.layout)
                        {
                            conflict = true;
                        }
                    }
                }
            }
            if conflict && !reachability[source].contains(&destination) {
                return Err(invalid("native range hazard has no batch dependency path"));
            }
        }
    }
    Ok(())
}

pub(super) fn ranges_overlap(
    a: crate::resource::OwnershipRange,
    b: crate::resource::OwnershipRange,
) -> crate::Result<bool> {
    // The variants provide their finite extents directly; production range
    // splitting/intersection are intentionally not used by this checker.
    match (a, b) {
        (
            crate::resource::OwnershipRange::Buffer { offset: x, size: n },
            crate::resource::OwnershipRange::Buffer { offset: y, size: m },
        ) => {
            let end_x = x
                .checked_add(n)
                .ok_or_else(|| invalid("buffer range overflows"))?;
            let end_y = y
                .checked_add(m)
                .ok_or_else(|| invalid("buffer range overflows"))?;
            Ok(x < end_y && y < end_x)
        }
        (
            crate::resource::OwnershipRange::Image {
                aspects: a,
                base_mip: am,
                mip_count: an,
                base_layer: al,
                layer_count: ac,
            },
            crate::resource::OwnershipRange::Image {
                aspects: b,
                base_mip: bm,
                mip_count: bn,
                base_layer: bl,
                layer_count: bc,
            },
        ) => Ok(a & b != 0
            && u64::from(am) < u64::from(bm) + u64::from(bn)
            && u64::from(bm) < u64::from(am) + u64::from(an)
            && u64::from(al) < u64::from(bl) + u64::from(bc)
            && u64::from(bl) < u64::from(al) + u64::from(ac)),
        _ => Err(invalid("native resource changes range type")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        device::QueueClass,
        raw::{buffer::BufferDesc, texture::TextureDesc},
        resource::{QueueFamilies, ResourceSharing},
        rg::{
            graph::RenderGraph,
            plan::{PlanContext, compile_definition},
            queue_schedule::QueueDomain,
            vulkan::{AccessTable, ResourceSpec},
        },
    };

    fn context(separate: bool) -> PlanContext {
        PlanContext {
            separate_depth_stencil_layouts: separate,
            queue_domains: Some(std::array::from_fn(|index| QueueDomain {
                queue_index: index as u32,
                family: index as u32,
            })),
            ..Default::default()
        }
    }
    fn compile(graph: RenderGraph, context: PlanContext) -> CompiledPlan {
        let definition = GraphDefinition::from_declaration(
            &graph.build().declaration,
            &mut AccessTable::original(),
            context,
        )
        .unwrap();
        compile_definition(definition).unwrap()
    }
    fn texture_range(aspect: vk::ImageAspectFlags, mip: u32) -> AccessRange {
        AccessRange::Texture {
            aspect_mask: aspect.as_raw(),
            base_mip_level: mip,
            level_count: 1,
            base_array_layer: 0,
            layer_count: 1,
        }
    }
    #[test]
    fn program_cannot_omit_union_ranges_or_change_native_scopes_or_resource_bounds() {
        let mut graph = RenderGraph::new();
        let resource = graph
            .import_unowned(
                "read sources",
                ResourceSpec::Buffer(BufferDesc::new_cpu_to_gpu(
                    64,
                    vk::BufferUsageFlags::TRANSFER_SRC | vk::BufferUsageFlags::UNIFORM_BUFFER,
                ))
                .descriptor(),
                1,
                0x26,
            )
            .unwrap();
        let buffer = graph.buffer_id(resource).unwrap();
        let mut pass = graph.add_pass("overlapping readers").with_side_effect();
        pass.read_buffer(
            buffer,
            0x25,
            AccessRange::Buffer {
                offset: 0,
                size: 32,
            },
        )
        .unwrap();
        pass.read_buffer(
            buffer,
            0x18,
            AccessRange::Buffer {
                offset: 16,
                size: 32,
            },
        )
        .unwrap();
        pass.render(|_| true);
        let plan = compile(graph, PlanContext::default());
        assert_eq!(plan.range_program.uses.len(), 3);
        let check = |program: &RangeProgram| check_range_program(&plan.definition, &plan, program);
        check(&plan.range_program).unwrap();
        let mut broken = plan.range_program.clone();
        broken.uses[0].scope.stages = vk::PipelineStageFlags::empty();
        assert!(check(&broken).is_err());
        let mut broken = plan.range_program.clone();
        broken.uses.pop();
        assert!(check(&broken).is_err());
        let mut broken = plan.range_program.clone();
        broken.resources[0].range = crate::resource::OwnershipRange::Buffer {
            offset: 0,
            size: 63,
        };
        assert!(check(&broken).is_err());
        let mut broken = plan.range_program.clone();
        broken.uses[0].access_code = 0x34;
        assert!(check(&broken).is_err());
    }

    fn two_image_readers(
        format: vk::Format,
        first: AccessRange,
        second: AccessRange,
    ) -> RenderGraph {
        let mut graph = RenderGraph::new();
        let image = graph
            .import_unowned_with_sharing(
                "read layouts",
                ResourceSpec::Image(
                    TextureDesc::new_2d(8, 8, format)
                        .mip_levels(2)
                        .usage(vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::TRANSFER_SRC),
                )
                .descriptor(),
                1,
                0x22,
                ResourceSharing::Concurrent(QueueFamilies::new(&[0, 1]).unwrap()),
            )
            .unwrap();
        let image = graph.texture_id(image).unwrap();
        for (queue, code, range) in [
            (QueueClass::Graphics, 0x22, first),
            (QueueClass::Compute, 0x25, second),
        ] {
            let mut pass = graph
                .add_pass("layout reader")
                .with_side_effect()
                .with_queue(queue);
            pass.read_texture(image, code, range).unwrap();
            pass.render(|_| true);
        }
        graph
    }

    #[test]
    fn read_read_layout_hazards_require_real_batch_paths_only_for_overlapping_mips() {
        let first = texture_range(vk::ImageAspectFlags::COLOR, 0);
        let overlapping = compile(
            two_image_readers(vk::Format::R8G8B8A8_UNORM, first, first),
            context(false),
        );
        assert!(overlapping.range_program.dependencies.contains(&(0, 1)));
        let mut broken = overlapping.range_program.clone();
        broken.dependencies.clear();
        assert!(check_range_program(&overlapping.definition, &overlapping, &broken).is_err());
        let disjoint = compile(
            two_image_readers(
                vk::Format::R8G8B8A8_UNORM,
                first,
                texture_range(vk::ImageAspectFlags::COLOR, 1),
            ),
            context(false),
        );
        assert!(disjoint.range_program.dependencies.is_empty());
        check_range_program(&disjoint.definition, &disjoint, &disjoint.range_program).unwrap();
    }

    #[test]
    fn coupled_depth_stencil_layouts_conflict_and_enabled_separate_layouts_preserve_parallelism() {
        let make = || {
            two_image_readers(
                vk::Format::D24_UNORM_S8_UINT,
                texture_range(vk::ImageAspectFlags::DEPTH, 0),
                texture_range(vk::ImageAspectFlags::STENCIL, 0),
            )
        };
        let coupled = compile(make(), context(false));
        assert!(coupled.range_program.dependencies.contains(&(0, 1)));
        let mut broken = coupled.range_program.clone();
        broken.separate_depth_stencil = true;
        assert!(check_range_program(&coupled.definition, &coupled, &broken).is_err());
        let separate = compile(make(), context(true));
        assert!(separate.range_program.dependencies.is_empty());
        check_range_program(&separate.definition, &separate, &separate.range_program).unwrap();
    }
}
