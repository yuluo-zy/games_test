//! The acquired swapchain image as a first-class graph resource.
//!
//! [`VulkanGraphSession::import_swapchain_image`] imports the image with
//! discarded (UNDEFINED) contents and exports it as `Present`, so the planner
//! itself emits the final transition to PRESENT_SRC_KHR — in the graph's final
//! batch on the presenting (primary) queue, including any queue-family
//! ownership transfer from the last using queue. While the graph executes, the
//! backend resolves which bound batches (graph batches and ownership preludes)
//! touch the image and, in execution order:
//! - the first one waits on the binary acquire semaphore at the image's
//!   first-use stages, and its discarding transitions of the image gain those
//!   stages in their source scope so the layout change is chained after the
//!   wait;
//! - the last one signals the per-image binary `rendered` semaphore.
//!
//! This works for any number of batches and queues. Every exit path returns the
//! image exactly once (see [`crate::raw::swapchain::GraphPresentation`]).
use super::*;
use crate::raw::swapchain::{AcquiredImageLease, GraphPresentation, PresentSync, PresentTicket};
use crate::rg::{
    resource::{GraphId, ResourceHandle},
    sync_plan::{BarrierPhase, BoundBatchKind},
};
use std::sync::Weak;

/// Access code of the planner's `Present` access (layout PRESENT_SRC_KHR).
pub const PRESENT_ACCESS: u8 = 0x27;

/// How the presentation graph treats the acquired image's previous contents.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SwapchainContents {
    /// Contents are discarded: the first use transitions from UNDEFINED.
    #[default]
    Discard,
    /// Load what the presentation engine left in the image (PRESENT_SRC_KHR)
    /// when this image was presented before in its generation; otherwise the
    /// contents are undefined and this falls back to [`Self::Discard`].
    Preserve,
}

/// The acquired swapchain image imported into one presentation graph.
/// Use [`Self::resource`] for graph accesses and the raw handles inside pass
/// callbacks; the generation stays alive while any command uses the image.
#[derive(Clone)]
pub struct PresentTarget {
    graph: GraphId,
    resource: ResourceHandle,
    texture: Arc<texture::Texture>,
    image: vk::Image,
    view: vk::ImageView,
    extent: vk::Extent2D,
    format: vk::Format,
    index: u32,
    generation: u64,
}
impl std::fmt::Debug for PresentTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PresentTarget")
            .field("resource", &self.resource)
            .field("image", &self.image)
            .field("extent", &self.extent)
            .field("format", &self.format)
            .field("index", &self.index)
            .field("generation", &self.generation)
            .finish_non_exhaustive()
    }
}
impl PresentTarget {
    /// The graph resource of the swapchain image.
    pub fn resource(&self) -> ResourceHandle {
        self.resource
    }
    pub fn graph_id(&self) -> GraphId {
        self.graph
    }
    /// The borrowed swapchain texture, e.g. for raster attachment descriptors.
    pub fn texture(&self) -> &Arc<texture::Texture> {
        &self.texture
    }
    pub fn image(&self) -> vk::Image {
        self.image
    }
    pub fn image_view(&self) -> vk::ImageView {
        self.view
    }
    pub fn extent(&self) -> vk::Extent2D {
        self.extent
    }
    pub fn format(&self) -> vk::Format {
        self.format
    }
    pub fn index(&self) -> u32 {
        self.index
    }
    pub fn generation(&self) -> u64 {
        self.generation
    }
}

/// Session-owned presentation: imported into `graph`, executed at most once.
pub(super) struct SessionPresentation {
    graph: GraphId,
    token: u64,
    owner: Option<GraphPresentation>,
    /// Dead once the graph was dropped, cancelled or retired.
    alive: Weak<()>,
    executed: bool,
}

/// Source-scope patch for the swapchain image's transitions in the batch that
/// waits on the acquire semaphore.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct PresentPatch {
    pub token: u64,
    pub batch: usize,
    pub phase: BarrierPhase,
    pub stages: vk::PipelineStageFlags,
}
impl PresentPatch {
    pub fn apply(
        patch: Option<Self>,
        scope: &mut crate::raw::barrier::SynchronizationScope,
        token: u64,
        transition: &super::super::sync_plan::BoundTransition,
    ) {
        if let Some(patch) = patch
            && patch.token == token
            && patch.batch == transition.batch
            && patch.phase == transition.phase
        {
            // The acquire wait's second scope is `stages`; including them in
            // the barrier's first scope forms the dependency chain, so the
            // layout transition cannot start before the image is acquired.
            scope.source_stages |= patch.stages;
        }
    }
}

/// Backend view of the presentation while its graph executes.
pub(super) struct BackendPresentation {
    owner: GraphPresentation,
    token: u64,
    sync: PresentSync,
    first: Option<usize>,
    last: Option<usize>,
    first_phase: BarrierPhase,
    wait_stages: vk::PipelineStageFlags,
    /// Bound batch selected for the next queue submission.
    current: Option<usize>,
}

/// Role of one queue submission in the presentation protocol.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) struct SubmitRole {
    pub waits_acquire: bool,
    pub signals_rendered: bool,
    pub touches: bool,
}

/// Which bound batches touch `resource`, and the acquire wait stages.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) struct PresentSchedule {
    pub first: Option<usize>,
    pub last: Option<usize>,
    /// Barrier phase of the image's first transition in `first`.
    pub first_phase: Option<BarrierPhase>,
    pub wait_stages: vk::PipelineStageFlags,
}

/// Resolve the touching batches from the bound program alone: graph batches
/// whose passes use the resource, plus every batch (including ownership
/// preludes and the export batch) holding one of its transitions. First and
/// last are taken in actual submission order.
pub(super) fn schedule_presentation(
    bound: &super::super::sync_plan::BoundSynchronization,
    program: &super::super::sync_plan::RangeProgram,
    resource: u32,
) -> PresentSchedule {
    let touches = |batch: usize| {
        bound
            .transitions()
            .iter()
            .any(|t| t.batch == batch && t.resource == resource)
            || bound
                .batches()
                .get(batch)
                .is_some_and(|entry| match &entry.kind {
                    BoundBatchKind::Graph { passes, .. } => passes.clone().any(|pass| {
                        program.passes.get(pass).is_some_and(|pass| {
                            program.uses[pass.uses.clone()]
                                .iter()
                                .any(|range_use| range_use.resource == resource)
                        })
                    }),
                    BoundBatchKind::OwnershipPrelude => false,
                })
    };
    let mut schedule = PresentSchedule::default();
    for &batch in bound.execution_order() {
        if touches(batch) {
            schedule.first.get_or_insert(batch);
            schedule.last = Some(batch);
        }
    }
    if let Some(first) = schedule.first {
        // The first use (its transitions in the earliest barrier phase) is
        // the acquire wait stage; later uses are ordered after it by barriers.
        let in_first = |t: &&super::super::sync_plan::BoundTransition| {
            t.batch == first && t.resource == resource
        };
        schedule.first_phase = bound.transitions().iter().find(in_first).map(|t| t.phase);
        schedule.wait_stages = bound
            .transitions()
            .iter()
            .filter(in_first)
            .filter(|t| Some(t.phase) == schedule.first_phase)
            .fold(vk::PipelineStageFlags::empty(), |stages, t| {
                stages | t.destination.stages
            });
        // TOP/BOTTOM carry no real work to order against; wait for everything.
        let real = schedule.wait_stages
            & !(vk::PipelineStageFlags::TOP_OF_PIPE | vk::PipelineStageFlags::BOTTOM_OF_PIPE);
        if real.is_empty() {
            schedule.wait_stages = vk::PipelineStageFlags::ALL_COMMANDS;
        }
    }
    schedule
}

impl BackendPresentation {
    fn role(&self, batch: Option<usize>) -> SubmitRole {
        let Some(batch) = batch else {
            return SubmitRole::default();
        };
        let waits_acquire = self.first == Some(batch) && !self.owner.acquire_consumed();
        let signals_rendered = self.last == Some(batch) && self.owner.rendered_point().is_none();
        SubmitRole {
            waits_acquire,
            signals_rendered,
            touches: waits_acquire || signals_rendered || self.owner.acquire_consumed(),
        }
    }
}

impl VulkanGraphBackend {
    pub(super) fn present_patch(&self) -> Option<PresentPatch> {
        let presentation = self.presentation.as_ref()?;
        Some(PresentPatch {
            token: presentation.token,
            batch: presentation.first?,
            phase: presentation.first_phase,
            stages: presentation.wait_stages,
        })
    }
    pub(super) fn select_presentation_batch(&mut self, batch: usize) {
        if let Some(presentation) = &mut self.presentation {
            presentation.current = Some(batch);
        }
    }
    /// After binding: locate the swapchain image and its touching batches.
    pub(super) fn resolve_presentation(
        &mut self,
        compiled: &graph::FrameInstance,
    ) -> crate::Result<()> {
        let Some(presentation) = &mut self.presentation else {
            return Ok(());
        };
        let logical = compiled
            .snapshot
            .physical_mapping
            .iter()
            .position(|mapping| mapping.token == Some(presentation.token));
        let schedule = logical.map_or_else(PresentSchedule::default, |logical| {
            schedule_presentation(
                self.queue_runtime.planner().bound(),
                &compiled.plan.range_program,
                logical as u32,
            )
        });
        presentation.first = schedule.first;
        presentation.last = schedule.last;
        presentation.first_phase = schedule.first_phase.unwrap_or(BarrierPhase::Initial);
        presentation.wait_stages = schedule.wait_stages;
        presentation.current = None;
        Ok(())
    }
    /// Inject the acquire wait / rendered signal into the batch being submitted.
    pub(super) fn amend_presentation_submit(
        &mut self,
        batches: &mut [submit::PreparedSubmit],
    ) -> crate::Result<(SubmitRole, vk::Fence)> {
        let Some(presentation) = &mut self.presentation else {
            return Ok((SubmitRole::default(), vk::Fence::null()));
        };
        let current = presentation.current.take();
        let role = presentation.role(current);
        if !role.waits_acquire && !role.signals_rendered {
            return Ok((role, vk::Fence::null()));
        }
        let [first, ..] = batches else {
            return Err(crate::Error::invalid_usage(
                "presentation batch has no submission",
            ));
        };
        if role.waits_acquire {
            first
                .wait_semaphores
                .try_reserve_exact(1)
                .map_err(|_| crate::Error::OutOfHostMemory)?;
            first
                .wait_values
                .try_reserve_exact(1)
                .map_err(|_| crate::Error::OutOfHostMemory)?;
            first
                .wait_stages
                .try_reserve_exact(1)
                .map_err(|_| crate::Error::OutOfHostMemory)?;
            first.wait_semaphores.push(presentation.sync.acquire);
            first.wait_values.push(0);
            first.wait_stages.push(presentation.wait_stages);
        }
        if role.signals_rendered {
            let [.., last] = batches else {
                unreachable!("nonempty batches")
            };
            last.signal_semaphores
                .try_reserve_exact(1)
                .map_err(|_| crate::Error::OutOfHostMemory)?;
            last.signal_values
                .try_reserve_exact(1)
                .map_err(|_| crate::Error::OutOfHostMemory)?;
            last.signal_semaphores.push(presentation.sync.rendered);
            last.signal_values.push(0);
        }
        let fence = if role.waits_acquire {
            presentation.sync.acquire_fence
        } else {
            vk::Fence::null()
        };
        Ok((role, fence))
    }
    pub(super) fn presentation_submitted(
        &mut self,
        role: SubmitRole,
        point: crate::sync::SyncPoint,
    ) -> crate::Result<()> {
        match &mut self.presentation {
            Some(presentation) if role.touches => {
                presentation
                    .owner
                    .submitted(point, role.waits_acquire, role.signals_rendered)
            }
            _ => Ok(()),
        }
    }
}

impl VulkanGraphSession {
    /// Import the acquired swapchain image into `graph`, which then owns its
    /// acquisition: the graph's execution waits on the acquire, transitions
    /// the image to PRESENT_SRC_KHR after its last use and signals the
    /// present wait. Call [`Self::present`] after executing the graph.
    ///
    /// The backend must record on the device's presenting (primary) queue.
    /// On any error before execution the image is returned to the swapchain.
    pub fn import_swapchain_image(
        &mut self,
        graph: &mut graph::RenderGraph,
        image: AcquiredImageLease,
        contents: SwapchainContents,
    ) -> crate::Result<PresentTarget> {
        self.reap_presentation();
        if self.presentation.is_some() {
            return Err(crate::Error::invalid_usage(
                "another swapchain image is still owned by this session; present or cancel it first",
            ));
        }
        if self.is_poisoned() {
            image.abandon_device_lost();
            return Err(crate::Error::DeviceLost);
        }
        if !Arc::ptr_eq(&self.backend.device, image.device()) {
            return Err(crate::Error::invalid_usage(
                "swapchain image belongs to another device",
            ));
        }
        if self.backend.queue != recording::Queue::Primary {
            return Err(crate::Error::unsupported(
                "presentation graphs must record on the device's presenting queue",
            ));
        }
        let texture = image.texture()?;
        let initial = match contents {
            SwapchainContents::Preserve if image.was_presented() => PRESENT_ACCESS,
            _ => 0,
        };
        let (image_handle, view, extent, format, index, generation) = (
            image.image(),
            image.image_view(),
            image.extent(),
            image.format(),
            image.index(),
            image.generation(),
        );
        let owner = GraphPresentation::arm(image)?;
        // `owner` returns the image if anything below fails.
        let resource =
            self.import_frame_resource(graph, GpuResource::Image(texture.clone()), initial, false)?;
        graph.export(resource, PRESENT_ACCESS)?;
        // The acquire wait is attached to the batch of the image's first use:
        // keep earlier independent passes out of that batch.
        graph.gate_first_use(resource)?;
        let token = match graph.declaration().resources[resource.raw() as usize].source {
            graph::OriginSource::ImportUnowned { token, .. } => token,
            _ => {
                return Err(crate::Error::invalid_usage(
                    "swapchain image was already imported with other ownership",
                ));
            }
        };
        graph.reserve_retirement_callback()?;
        let alive = Arc::new(());
        let weak = Arc::downgrade(&alive);
        graph.on_retire(move || drop(alive));
        self.presentation = Some(SessionPresentation {
            graph: graph.graph_id(),
            token,
            owner: Some(owner),
            alive: weak,
            executed: false,
        });
        Ok(PresentTarget {
            graph: graph.graph_id(),
            resource,
            texture,
            image: image_handle,
            view,
            extent,
            format,
            index,
            generation,
        })
    }
    /// Present the image rendered by `target`'s executed graph.
    pub fn present(&mut self, target: &PresentTarget) -> crate::Result<PresentTicket> {
        let Some(slot) = self
            .presentation
            .as_mut()
            .filter(|slot| slot.graph == target.graph)
        else {
            return Err(crate::Error::invalid_usage(
                "no rendered presentation for this target",
            ));
        };
        if !slot.executed {
            return Err(crate::Error::invalid_usage(
                "execute the presentation graph before presenting",
            ));
        }
        let mut slot = self.presentation.take().expect("checked presentation");
        let result = slot
            .owner
            .as_mut()
            .ok_or_else(|| crate::Error::invalid_usage("presentation already consumed"))?
            .present();
        if result.as_ref().is_err_and(crate::Error::is_device_lost) {
            self.backend.device.queues.timelines.mark_device_lost();
        }
        result
    }
    /// Return a not yet presented swapchain image. A rendered image is
    /// presented (its contents are complete); otherwise the acquisition is
    /// released without presenting, or discarded and presented when a
    /// partially submitted graph already consumed the acquire wait.
    pub fn cancel_presentation(&mut self) -> crate::Result<()> {
        let Some(mut slot) = self.presentation.take() else {
            return Ok(());
        };
        let result = slot
            .owner
            .as_mut()
            .map_or(Ok(()), GraphPresentation::abandon);
        if result.as_ref().is_err_and(crate::Error::is_device_lost) {
            self.backend.device.queues.timelines.mark_device_lost();
        }
        result
    }
    /// Whether a swapchain image is owned by this session.
    pub fn has_presentation(&self) -> bool {
        self.presentation.is_some()
    }
    /// An unexecuted presentation whose graph was dropped or cancelled
    /// returns its image now.
    pub(super) fn reap_presentation(&mut self) {
        if self
            .presentation
            .as_ref()
            .is_some_and(|slot| !slot.executed && slot.alive.upgrade().is_none())
            && let Err(error) = self.cancel_presentation()
        {
            log::warn!(target: "rhapsody::rg::presentation", "abandoned presentation cleanup failed: {error}");
        }
    }
    /// Cancel the presentation bound to a graph that will never execute.
    pub(super) fn cancel_presentation_of(&mut self, graph: GraphId) {
        if self
            .presentation
            .as_ref()
            .is_some_and(|slot| slot.graph == graph && !slot.executed)
            && let Err(error) = self.cancel_presentation()
        {
            log::warn!(target: "rhapsody::rg::presentation", "presentation cancellation failed: {error}");
        }
    }
    /// Lend the presentation to the backend for `graph`'s execution.
    pub(super) fn begin_presentation_execution(&mut self, graph: GraphId) -> crate::Result<()> {
        let Some(slot) = self
            .presentation
            .as_mut()
            .filter(|slot| slot.graph == graph)
        else {
            return Ok(());
        };
        let sync = match (slot.executed, &slot.owner) {
            (false, Some(owner)) => owner.sync(),
            _ => Err(crate::Error::invalid_usage(
                "presentation graph was already executed",
            )),
        };
        let sync = match sync {
            Ok(sync) => sync,
            Err(error) => {
                // Never leave an unusable slot behind: return the image.
                if let Err(cleanup) = self.cancel_presentation() {
                    log::warn!(target: "rhapsody::rg::presentation", "presentation cleanup failed: {cleanup}");
                }
                return Err(error);
            }
        };
        slot.executed = true;
        let owner = slot.owner.take().expect("checked presentation owner");
        self.backend.presentation = Some(BackendPresentation {
            owner,
            token: slot.token,
            sync,
            first: None,
            last: None,
            first_phase: BarrierPhase::Initial,
            wait_stages: vk::PipelineStageFlags::empty(),
            current: None,
        });
        Ok(())
    }
    /// Take the presentation back after execution. Unless `rendered` was
    /// signalled, the image is returned to the swapchain immediately.
    pub(super) fn finish_presentation_execution(&mut self) {
        let Some(backend) = self.backend.presentation.take() else {
            return;
        };
        let Some(slot) = self.presentation.as_mut() else {
            return;
        };
        slot.owner = Some(backend.owner);
        let rendered = slot
            .owner
            .as_ref()
            .is_some_and(|owner| owner.rendered_point().is_some());
        if !rendered && let Err(error) = self.cancel_presentation() {
            log::warn!(target: "rhapsody::rg::presentation", "incomplete presentation graph cleanup failed: {error}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        device::QueueClass,
        resource::range_state::{AccessScope, RangeOwner},
        rg::{
            graph::{BufferDesc, Descriptor, RenderGraph},
            plan::{GraphDefinition, PlanContext, compile_definition},
            queue_schedule::QueueDomain,
            sync_plan::{SynchronizationPlanner, initial_native_state},
        },
        sync::QueueId,
    };

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
        crate::rg::vulkan::ResourceSpec::Image(crate::raw::texture::TextureDesc::new_2d(
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
            &mut AccessTable::original(),
            context,
        )
        .unwrap();
        let plan = compile_definition(definition).unwrap();
        let mut planner = SynchronizationPlanner::new_shared(
            Arc::new(crate::raw::allocator::AllocatorStats::default()),
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
                let cells = [crate::resource::range_state::RangeCell {
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
        let mut scope = crate::raw::barrier::SynchronizationScope::default();
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
}
