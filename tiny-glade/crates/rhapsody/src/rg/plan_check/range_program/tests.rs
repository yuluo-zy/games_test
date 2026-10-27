use super::*;
use crate::{device::QueueClass, rg::{graph::RenderGraph, plan::{PlanContext, compile_definition}, queue_schedule::QueueDomain, vulkan::{AccessTable, ResourceSpec}}};
use crate::tracking::{QueueFamilies, ResourceSharing};
use crate::resource::buffer::BufferDesc;
use crate::resource::texture::TextureDesc;

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
        &AccessTable::original(),
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
    broken.resources[0].range = crate::tracking::OwnershipRange::Buffer {
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

fn depth_write_then_stencil_read() -> RenderGraph {
    let mut graph = RenderGraph::new();
    let image = graph
        .import_unowned(
            "depth stencil",
            ResourceSpec::Image(
                TextureDesc::new_2d(8, 8, vk::Format::D24_UNORM_S8_UINT).usage(
                    vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT
                        | vk::ImageUsageFlags::SAMPLED,
                ),
            )
            .descriptor(),
            1,
            0x17,
        )
        .unwrap();
    let image = graph.texture_id(image).unwrap();
    let mut writer = graph.add_pass("depth writer").with_side_effect();
    let image = writer
        .write_texture(image, 0x2f, texture_range(vk::ImageAspectFlags::DEPTH, 0))
        .unwrap();
    writer.render(|_| true);
    let mut reader = graph.add_pass("stencil reader").with_side_effect();
    reader
        .read_texture(image, 0x17, texture_range(vk::ImageAspectFlags::STENCIL, 0))
        .unwrap();
    reader.render(|_| true);
    graph
}

/// Bind the compiled program from an UNDEFINED initial state and return
/// every emitted image transition as (aspects, old, new).
fn bound_layouts(plan: &CompiledPlan) -> Vec<(u32, vk::ImageLayout, vk::ImageLayout)> {
    use crate::tracking::range_state::{NativeState, RangeCell};
    use crate::rg::sync_plan::SynchronizationPlanner;
    let program = &plan.range_program;
    let mut planner = SynchronizationPlanner::new(
        std::sync::Arc::new(crate::accounting::AllocatorStats::default()),
        Default::default(),
    )
    .unwrap();
    for (index, resource) in program.resources.iter().enumerate() {
        planner
            .add_initial(
                index as u32,
                &[RangeCell {
                    range: resource.range,
                    state: NativeState::default(),
                }],
            )
            .unwrap();
    }
    planner
        .bind(program, |family| {
            Ok(QueueDomain {
                queue_index: family,
                family,
            })
        })
        .unwrap();
    planner
        .bound()
        .transitions()
        .iter()
        .filter_map(|transition| match transition.range {
            crate::tracking::OwnershipRange::Image { aspects, .. } => {
                Some((aspects, transition.old_layout, transition.new_layout))
            }
            _ => None,
        })
        .collect()
}

#[test]
fn enabled_separate_layouts_use_per_aspect_layouts_with_matching_aspect_masks() {
    let (depth, stencil) = (
        vk::ImageAspectFlags::DEPTH.as_raw(),
        vk::ImageAspectFlags::STENCIL.as_raw(),
    );
    let plan = compile(depth_write_then_stencil_read(), context(true));
    check_range_program(&plan.definition, &plan, &plan.range_program).unwrap();
    let uses = plan
        .range_program
        .uses
        .iter()
        .map(|usage| match usage.range {
            crate::tracking::OwnershipRange::Image { aspects, .. } => (aspects, usage.layout),
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();
    assert_eq!(
        uses,
        [
            (depth, vk::ImageLayout::DEPTH_ATTACHMENT_OPTIMAL),
            (stencil, vk::ImageLayout::STENCIL_READ_ONLY_OPTIMAL),
        ]
    );
    let transitions = bound_layouts(&plan);
    assert!(transitions.contains(&(
        depth,
        vk::ImageLayout::UNDEFINED,
        vk::ImageLayout::DEPTH_ATTACHMENT_OPTIMAL
    )));
    assert!(transitions.contains(&(
        stencil,
        vk::ImageLayout::UNDEFINED,
        vk::ImageLayout::STENCIL_READ_ONLY_OPTIMAL
    )));
    for (aspects, old, new) in transitions {
        for layout in [old, new] {
            let depth_only = matches!(
                layout,
                vk::ImageLayout::DEPTH_ATTACHMENT_OPTIMAL
                    | vk::ImageLayout::DEPTH_READ_ONLY_OPTIMAL
            );
            let stencil_only = matches!(
                layout,
                vk::ImageLayout::STENCIL_ATTACHMENT_OPTIMAL
                    | vk::ImageLayout::STENCIL_READ_ONLY_OPTIMAL
            );
            assert!(
                !depth_only || aspects == depth,
                "{layout:?} on {aspects:#x}"
            );
            assert!(
                !stencil_only || aspects == stencil,
                "{layout:?} on {aspects:#x}"
            );
        }
    }
    // A program claiming combined layouts for split aspects is rejected.
    let mut broken = plan.range_program.clone();
    broken.uses[0].layout = vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL;
    assert!(check_range_program(&plan.definition, &plan, &broken).is_err());
}

#[test]
fn disabled_separate_layouts_fall_back_to_combined_layouts_on_both_aspects() {
    let both = (vk::ImageAspectFlags::DEPTH | vk::ImageAspectFlags::STENCIL).as_raw();
    let plan = compile(depth_write_then_stencil_read(), context(false));
    check_range_program(&plan.definition, &plan, &plan.range_program).unwrap();
    for usage in &plan.range_program.uses {
        assert!(matches!(
            usage.range,
            crate::tracking::OwnershipRange::Image { aspects, .. } if aspects == both
        ));
    }
    assert_eq!(
        plan.range_program
            .uses
            .iter()
            .map(|usage| usage.layout)
            .collect::<Vec<_>>(),
        [
            vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
            vk::ImageLayout::DEPTH_STENCIL_READ_ONLY_OPTIMAL,
        ]
    );
    for (aspects, old, new) in bound_layouts(&plan) {
        assert_eq!(aspects, both);
        for layout in [old, new] {
            assert!(!matches!(
                layout,
                vk::ImageLayout::DEPTH_ATTACHMENT_OPTIMAL
                    | vk::ImageLayout::DEPTH_READ_ONLY_OPTIMAL
                    | vk::ImageLayout::STENCIL_ATTACHMENT_OPTIMAL
                    | vk::ImageLayout::STENCIL_READ_ONLY_OPTIMAL
            ));
        }
    }
}

#[test]
fn native_layout_maps_each_intent_per_mode_and_format() {
    use crate::rg::sync_plan::native_layout;
    use vk::ImageLayout as L;
    let (d, s) = (
        vk::ImageAspectFlags::DEPTH.as_raw(),
        vk::ImageAspectFlags::STENCIL.as_raw(),
    );
    let ds = d | s;
    let cases = [
        (
            L::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
            d,
            ds,
            true,
            L::DEPTH_ATTACHMENT_OPTIMAL,
        ),
        (
            L::DEPTH_STENCIL_READ_ONLY_OPTIMAL,
            d,
            ds,
            true,
            L::DEPTH_READ_ONLY_OPTIMAL,
        ),
        (
            L::DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL,
            s,
            ds,
            true,
            L::STENCIL_READ_ONLY_OPTIMAL,
        ),
        (
            L::DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL,
            s,
            ds,
            true,
            L::STENCIL_ATTACHMENT_OPTIMAL,
        ),
        (
            L::DEPTH_ATTACHMENT_OPTIMAL,
            ds,
            ds,
            false,
            L::DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL,
        ),
        (
            L::STENCIL_ATTACHMENT_OPTIMAL,
            ds,
            ds,
            true,
            L::DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL,
        ),
        (
            L::DEPTH_READ_ONLY_OPTIMAL,
            ds,
            ds,
            false,
            L::DEPTH_STENCIL_READ_ONLY_OPTIMAL,
        ),
        // Depth-only formats keep core combined layouts in both modes.
        (
            L::DEPTH_ATTACHMENT_OPTIMAL,
            d,
            d,
            true,
            L::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
        ),
        (
            L::DEPTH_STENCIL_READ_ONLY_OPTIMAL,
            d,
            d,
            false,
            L::DEPTH_STENCIL_READ_ONLY_OPTIMAL,
        ),
        // Non depth/stencil layouts are untouched.
        (L::GENERAL, d, ds, true, L::GENERAL),
        (
            L::SHADER_READ_ONLY_OPTIMAL,
            d,
            ds,
            true,
            L::SHADER_READ_ONLY_OPTIMAL,
        ),
    ];
    for (layout, aspects, domain, separate, expected) in cases {
        assert_eq!(
            native_layout(layout, aspects, domain, separate).unwrap(),
            expected,
            "{layout:?} aspects {aspects:#x} domain {domain:#x} separate {separate}"
        );
    }
    assert!(native_layout(L::STENCIL_READ_ONLY_OPTIMAL, d, ds, true).is_err());
    assert!(native_layout(L::DEPTH_ATTACHMENT_OPTIMAL, s, ds, true).is_err());
}
