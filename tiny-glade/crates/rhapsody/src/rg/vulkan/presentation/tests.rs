use super::*;
use crate::rg::vulkan::AccessTable;
use crate::{device::QueueClass, rg::{graph::{Descriptor, RenderGraph}, plan::{GraphDefinition, PlanContext, compile_definition}, queue_schedule::QueueDomain, sync_plan::{SynchronizationPlanner, initial_native_state}}, sync::QueueId};
use crate::tracking::range_state::RangeOwner;
use crate::rg::resource_registry::BufferDesc;
use crate::native::AccessScope;

fn domains() -> [QueueDomain; 3] {
    [
        QueueDomain {
            queue_index: 0,
            family: 0,
        },
        QueueDomain {
            queue_index: 1,
            family: 1,
        },
        QueueDomain {
            queue_index: 2,
            family: 2,
        },
    ]
}
fn image_descriptor() -> Descriptor {
    crate::rg::vulkan::ResourceSpec::Image(crate::resource::texture::TextureDesc::new_2d(
        8,
        8,
        vk::Format::B8G8R8A8_UNORM,
    ))
    .descriptor()
}

/// Bind a presentation graph whose first use of the swapchain image is on
/// a compute queue of another family, followed by a graphics pass.
fn bound_multi_family(
    compute_first: bool,
) -> (
    crate::rg::plan::CompiledPlan,
    Arc<SynchronizationPlanner>,
    u32,
) {
    bound_presentation(compute_first, true)
}
/// `overlay` adds a graphics pass after the compute write; without it the
/// compute queue is also the last user and the export must hand the image
/// back to the presenting family.
fn bound_presentation(
    compute_first: bool,
    overlay: bool,
) -> (
    crate::rg::plan::CompiledPlan,
    Arc<SynchronizationPlanner>,
    u32,
) {
    let mut graph = RenderGraph::new();
    let target = graph
        .import_unowned("swapchain", image_descriptor(), 7, 0)
        .unwrap();
    let other = graph
        .import_unowned(
            "other",
            Descriptor::Buffer(BufferDesc {
                word_at_00: 64,
                word_at_08: 0,
                bytes_at_0c_through_0e: [0; 3],
            }),
            8,
            0x25,
        )
        .unwrap();
    {
        let mut pass = graph.add_pass("unrelated").with_side_effect();
        pass.add_resource_access(other, 0x25, 0).unwrap();
        pass.render(|_| true);
    }
    {
        let mut pass = graph
            .add_pass("compute write")
            .with_side_effect()
            .with_queue(if compute_first {
                QueueClass::Compute
            } else {
                QueueClass::Graphics
            });
        // Storage-image write (GENERAL) from the compute shader stage.
        pass.add_resource_access(target, 0x32, 0).unwrap();
        pass.render(|_| true);
    }
    if overlay {
        let mut pass = graph.add_pass("raster overlay");
        pass.add_resource_access(target, 0x2e, 0).unwrap();
        pass.render(|_| true);
    }
    graph.export(target, PRESENT_ACCESS).unwrap();
    let built = graph.build();
    let context = PlanContext {
        queue_domains: Some(domains()),
        ..Default::default()
    };
    let definition = GraphDefinition::from_declaration(
        &built.declaration,
        &AccessTable::original(),
        context,
    )
    .unwrap();
    let plan = compile_definition(definition).unwrap();
    let mut planner = SynchronizationPlanner::new_shared(
        Arc::new(crate::accounting::AllocatorStats::default()),
        Default::default(),
    )
    .unwrap();
    {
        let planner = Arc::get_mut(&mut planner).unwrap();
        planner.begin();
        for (logical, resource) in plan.range_program.resources.iter().enumerate() {
            let state = initial_native_state(
                AccessScope::EMPTY,
                vk::ImageLayout::UNDEFINED,
                QueueId::new(0).unwrap(),
                RangeOwner::Owned(0),
            );
            let cells = [crate::tracking::range_state::RangeCell {
                range: resource.range,
                state,
            }];
            planner.add_initial(logical as u32, &cells).unwrap();
        }
        planner
            .bind(&plan.range_program, |family| {
                domains()
                    .into_iter()
                    .find(|domain| domain.family == family)
                    .ok_or_else(|| crate::Error::unsupported("family"))
            })
            .unwrap();
    }
    let resource = target.raw();
    (plan, planner, resource)
}

#[test]
fn single_queue_presentation_waits_first_and_signals_last_touching_batch() {
    let (plan, planner, resource) = bound_multi_family(false);
    let bound = planner.bound();
    let schedule = schedule_presentation(bound, &plan.range_program, resource);
    let first = schedule.first.unwrap();
    let last = schedule.last.unwrap();
    let order = bound.execution_order();
    let position = |batch| order.iter().position(|&b| b == batch).unwrap();
    assert!(position(first) <= position(last));
    assert_eq!(last, *order.last().unwrap(), "export batch is the last use");
    assert_eq!(
        schedule.wait_stages,
        vk::PipelineStageFlags::COMPUTE_SHADER,
        "acquire wait sits at the first use only"
    );
    // The final transition to PRESENT_SRC_KHR exists in the last batch.
    assert!(bound.transitions().iter().any(|t| t.batch == last
        && t.resource == resource
        && t.new_layout == vk::ImageLayout::PRESENT_SRC_KHR));
}

fn transfers(
    bound: &crate::rg::sync_plan::BoundSynchronization,
    resource: u32,
    source: u32,
    destination: u32,
) -> (usize, usize) {
    let count = |release: bool| {
        bound
            .transitions()
            .iter()
            .filter(|t| {
                t.resource == resource
                    && t.source_family == source
                    && t.destination_family == destination
                    && match t.kind {
                        crate::rg::sync_plan::BarrierKind::Release { .. } => release,
                        crate::rg::sync_plan::BarrierKind::Acquire { .. } => !release,
                        crate::rg::sync_plan::BarrierKind::Memory => false,
                    }
            })
            .count()
    };
    (count(true), count(false))
}

#[test]
fn discarded_first_use_on_a_foreign_family_needs_no_ownership_transfer() {
    let (plan, planner, resource) = bound_multi_family(true);
    let bound = planner.bound();
    let schedule = schedule_presentation(bound, &plan.range_program, resource);
    let (first, last) = (schedule.first.unwrap(), schedule.last.unwrap());
    assert_ne!(first, last);
    let order = bound.execution_order();
    let position = |batch| order.iter().position(|&b| b == batch).unwrap();
    assert!(position(first) < position(last));
    // The image ends on the presenting family (0) in PRESENT_SRC_KHR.
    assert_eq!(bound.batches()[last].domain.family, 0);
    let present = bound
        .transitions()
        .iter()
        .filter(|t| t.resource == resource && t.new_layout == vk::ImageLayout::PRESENT_SRC_KHR)
        .collect::<Vec<_>>();
    assert!(!present.is_empty());
    assert!(present.iter().all(|t| t.batch == last));
    // The first use is the compute pass itself: no ownership prelude, no
    // release from the home family, just UNDEFINED -> GENERAL on queue
    // family 1; the acquire wait therefore targets the compute stage.
    assert!(matches!(
        bound.batches()[first].kind,
        BoundBatchKind::Graph { .. }
    ));
    assert_eq!(bound.batches()[first].domain.family, 1);
    assert_eq!(schedule.wait_stages, vk::PipelineStageFlags::COMPUTE_SHADER);
    assert!(bound.transitions().iter().any(|t| t.batch == first
        && t.resource == resource
        && t.old_layout == vk::ImageLayout::UNDEFINED
        && t.new_layout == vk::ImageLayout::GENERAL
        && t.kind == crate::rg::sync_plan::BarrierKind::Memory
        && t.source_family == vk::QUEUE_FAMILY_IGNORED));
    assert_eq!(
        transfers(bound, resource, 0, 1),
        (0, 0),
        "discarded contents are ownerless: no 0 -> 1 pair"
    );
    // Written contents are preserved for the later graphics pass.
    let (releases, acquires) = transfers(bound, resource, 1, 0);
    assert!(
        releases == 1 && acquires == 1,
        "{releases} releases, {acquires} acquires"
    );
    assert_eq!(
        position(first),
        1,
        "the unrelated batch does not wait on acquire"
    );
}

#[test]
fn final_handover_to_the_presenting_family_keeps_its_ownership_transfer() {
    // The compute queue is both first and last user; the image must still
    // return to family 0 before PRESENT_SRC_KHR, and only that pair exists.
    let (plan, planner, resource) = bound_presentation(true, false);
    let bound = planner.bound();
    assert_eq!(transfers(bound, resource, 0, 1), (0, 0));
    assert_eq!(transfers(bound, resource, 1, 0), (1, 1));
    let schedule = schedule_presentation(bound, &plan.range_program, resource);
    let last = schedule.last.unwrap();
    assert_eq!(bound.batches()[last].domain.family, 0);
    assert!(bound.transitions().iter().any(|t| t.batch == last
        && t.resource == resource
        && t.new_layout == vk::ImageLayout::PRESENT_SRC_KHR));
}

#[test]
fn discarded_ownerless_binding_passes_the_independent_checkers() {
    for overlay in [true, false] {
        let (plan, planner, _) = bound_presentation(true, overlay);
        crate::rg::plan_check::check_bound_synchronization(
            &plan.range_program,
            planner.bound(),
        )
        .unwrap();
    }
}

#[test]
fn acquire_patch_chains_only_the_first_batch_transitions_of_the_image() {
    let (plan, planner, resource) = bound_multi_family(false);
    let _ = plan;
    let transition = *planner
        .bound()
        .transitions()
        .iter()
        .find(|t| t.resource == resource)
        .unwrap();
    let patch = PresentPatch {
        token: 7,
        batch: 2,
        phase: transition.phase,
        stages: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
    };
    let mut scope = crate::command::barrier::SynchronizationScope::default();
    let at = |batch| super::super::super::sync_plan::BoundTransition {
        batch,
        ..transition
    };
    PresentPatch::apply(Some(patch), &mut scope, 7, &at(1));
    PresentPatch::apply(Some(patch), &mut scope, 8, &at(2));
    PresentPatch::apply(None, &mut scope, 7, &at(2));
    PresentPatch::apply(
        Some(PresentPatch {
            phase: BarrierPhase::Export,
            ..patch
        }),
        &mut scope,
        7,
        &super::super::super::sync_plan::BoundTransition {
            phase: BarrierPhase::BeforePass(99),
            ..at(2)
        },
    );
    assert_eq!(scope.source_stages, vk::PipelineStageFlags::TOP_OF_PIPE);
    PresentPatch::apply(Some(patch), &mut scope, 7, &at(2));
    assert_eq!(
        scope.source_stages,
        vk::PipelineStageFlags::TOP_OF_PIPE | vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT
    );
}
