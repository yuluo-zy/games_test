//! Execution cursors for the one bound range program. Physical layouts, owners,
//! scopes and revisions live exclusively in the actual resource SyncCell table.
use super::*;
use crate::{
    resource::{
        ResourceSharing, SyncCell,
        range_state::{RangeCell, RangeOwner, RangeStateConfig, Storage},
    },
    rg::{
        executor::QueuePrelude,
        queue_schedule::QueueDomain,
        sync_plan::{
            self, BoundBatchKind, BoundSynchronization, DependencyRef, SynchronizationPlanner,
        },
    },
    sync::{SyncPoint, SyncSet},
};
pub(super) struct QueueRuntime {
    pub multi_queue: bool,
    planner: Arc<SynchronizationPlanner>,
    tokens: Storage<Option<u64>>,
    points: Storage<Option<SyncPoint>>,
    cursor: usize,
    active_batch: Option<usize>,
    queue_flags: [vk::QueueFlags; crate::sync::MAX_QUEUES],
}
impl QueueRuntime {
    pub fn new(device: &Device) -> crate::Result<Self> {
        let stats = &device.allocator_stats;
        let families = unsafe {
            device
                .instance
                .instance
                .get_physical_device_queue_family_properties(device.physical)
        };
        let queue_flags = std::array::from_fn(|index| {
            let family = if index == 0 {
                Some(device.queue_family)
            } else {
                device
                    .additional_queues
                    .get(index - 1)
                    .map(|queue| queue.family)
            };
            family
                .and_then(|family| families.get(family as usize))
                .map_or(vk::QueueFlags::empty(), |family| family.queue_flags)
        });
        Ok(Self {
            multi_queue: false,
            planner: SynchronizationPlanner::new_shared(stats.clone(), Default::default())?,
            tokens: Storage::new(0, stats)?,
            points: Storage::new(0, stats)?,
            cursor: 0,
            active_batch: None,
            queue_flags,
        })
    }
    pub(super) fn select_main_batch(&mut self, batch: usize) -> crate::Result<()> {
        if self.planner.bound().execution_order().get(self.cursor) != Some(&batch) {
            return Err(crate::Error::invalid_usage(
                "native graph batch did not follow its ownership preludes",
            ));
        }
        self.active_batch = Some(batch);
        Ok(())
    }
    pub(super) fn planner(&self) -> Arc<SynchronizationPlanner> {
        self.planner.clone()
    }
}
impl GpuResource {
    pub(super) fn owner_cell(&self) -> Arc<SyncCell> {
        match self {
            Self::Buffer(buffer) => buffer.last_use.clone(),
            Self::Image(image) => image.last_use.clone(),
        }
    }
    pub(super) fn sharing_mode(&self) -> ResourceSharing {
        match self {
            Self::Buffer(buffer) => buffer.sharing(),
            Self::Image(image) => image.sharing(),
        }
    }
}
impl VulkanGraphBackend {
    pub fn bound_synchronization(&self) -> &BoundSynchronization {
        self.queue_runtime.planner.bound()
    }
    pub fn resource_native_state<R>(
        &self,
        token: u64,
        run: impl FnOnce(&[RangeCell]) -> R,
    ) -> crate::Result<R> {
        let resource = self.resources.get(&token).ok_or_else(|| {
            crate::Error::invalid_usage("native-state query names an absent physical resource")
        })?;
        resource
            .owner_cell()
            .with_existing_range_state(|table| Ok(run(table.cells())))?
            .ok_or_else(|| {
                crate::Error::invalid_usage("physical resource has no tracked native state yet")
            })
    }
    pub(super) fn domain_for_class(&self, class: crate::device::QueueClass) -> QueueDomain {
        let index = if class == crate::device::QueueClass::Graphics {
            match self.queue {
                recording::Queue::Primary => 0,
                recording::Queue::Auxiliary(index) => index + 1,
            }
        } else {
            self.device.queue_plan.queue(class).index() as u32
        };
        QueueDomain {
            queue_index: index,
            family: if index == 0 {
                self.device.queue_family
            } else {
                self.device.additional_queues[index as usize - 1].family
            },
        }
    }
    fn domains(&self) -> [Option<QueueDomain>; crate::sync::MAX_QUEUES] {
        std::array::from_fn(|index| {
            if index == 0 {
                Some(QueueDomain {
                    queue_index: 0,
                    family: self.device.queue_family,
                })
            } else {
                self.device
                    .additional_queues
                    .get(index - 1)
                    .map(|queue| QueueDomain {
                        queue_index: index as u32,
                        family: queue.family,
                    })
            }
        })
    }
    pub(super) fn prepare_queue_plan(
        &mut self,
        plan: &super::super::plan::CompiledPlan,
        _graph: &graph::GraphDeclaration,
    ) -> crate::Result<()> {
        let home = self.domain_for_class(crate::device::QueueClass::Graphics);
        self.queue_runtime.multi_queue = plan
            .queue_schedule
            .domains
            .iter()
            .any(|domain| *domain != home);
        self.queue_runtime.cursor = 0;
        self.queue_runtime.active_batch = None;
        self.queue_runtime.tokens.values.clear();
        self.queue_runtime.points.values.clear();
        if Arc::get_mut(&mut self.queue_runtime.planner).is_none() {
            return Err(crate::Error::Timeout);
        }
        for batch in &plan.range_program.batches {
            let flags = *self
                .queue_runtime
                .queue_flags
                .get(batch.domain.queue_index as usize)
                .ok_or_else(|| crate::Error::invalid_usage("native queue index is absent"))?;
            let universal = vk::PipelineStageFlags::TOP_OF_PIPE
                | vk::PipelineStageFlags::BOTTOM_OF_PIPE
                | vk::PipelineStageFlags::HOST
                | vk::PipelineStageFlags::ALL_COMMANDS
                | vk::PipelineStageFlags::TRANSFER;
            let compute =
                vk::PipelineStageFlags::COMPUTE_SHADER | vk::PipelineStageFlags::DRAW_INDIRECT;
            for pass in batch.passes.clone() {
                for access in &plan.range_program.uses[plan.range_program.passes[pass].uses.clone()]
                {
                    if !flags.contains(vk::QueueFlags::GRAPHICS)
                        && !(access.scope.stages
                            & !(universal
                                | if flags.contains(vk::QueueFlags::COMPUTE) {
                                    compute
                                } else {
                                    vk::PipelineStageFlags::empty()
                                }))
                        .is_empty()
                    {
                        return Err(crate::Error::unsupported(
                            "native access stages are unsupported by the actual queue family",
                        ));
                    }
                }
            }
        }
        Ok(())
    }
    pub(super) fn bind_queue_resources(
        &mut self,
        compiled: &graph::FrameInstance,
    ) -> crate::Result<()> {
        let domains = self.domains();
        let stats = &self.device.allocator_stats;
        let runtime = &mut self.queue_runtime;
        let planner = Arc::get_mut(&mut runtime.planner).ok_or(crate::Error::Timeout)?;
        planner.begin();
        let count = compiled.snapshot.physical_mapping.len();
        runtime.tokens.grow(
            count,
            runtime.points.bytes(),
            RangeStateConfig {
                max_cells: 65_536,
                max_cpu_bytes: 4 << 20,
            },
            stats,
        )?;
        runtime.tokens.values.resize(count, None);
        for (logical, mapping) in compiled.snapshot.physical_mapping.iter().enumerate() {
            let Some(token) = mapping.token else {
                continue;
            };
            runtime.tokens.values[logical] = Some(token);
            let object = self.resources.get(&token).ok_or_else(|| {
                crate::Error::invalid_usage("native resource disappeared during binding")
            })?;
            let (scope, mut layout) = graph::AccessDependency::native_semantics(
                &mut self.accesses,
                mapping.initial_access,
            );
            if matches!(object, GpuResource::Buffer(_)) {
                layout = vk::ImageLayout::UNDEFINED;
            }
            let family = self
                .import_families
                .get(&token)
                .copied()
                .unwrap_or(self.family);
            let initial_queue = domains
                .iter()
                .flatten()
                .find(|domain| domain.family == family)
                .map(|domain| domain.queue_index)
                .unwrap_or(0);
            let initial = sync_plan::initial_native_state(
                scope,
                layout,
                crate::sync::QueueId::new(initial_queue as usize).expect("validated device queue"),
                if object.sharing_mode().is_concurrent()
                    || matches!(
                        compiled.declarations.resources[logical].source,
                        graph::OriginSource::Create
                    )
                {
                    RangeOwner::Unclaimed
                } else {
                    RangeOwner::Owned(family)
                },
            );
            object
                .owner_cell()
                .with_range_state(initial, stats, |table| {
                    planner.add_initial(logical as u32, table.cells())
                })?;
        }
        planner.bind(&compiled.plan.range_program, |family| {
            domains
                .iter()
                .flatten()
                .find(|domain| domain.family == family)
                .copied()
                .ok_or_else(|| {
                    crate::Error::unsupported(
                        "exclusive native owner family has no configured device queue",
                    )
                })
        })?;
        runtime.points.grow(
            planner.bound().batches().len(),
            runtime.tokens.bytes(),
            RangeStateConfig {
                max_cells: 65_536,
                max_cpu_bytes: 4 << 20,
            },
            stats,
        )?;
        runtime
            .points
            .values
            .resize(planner.bound().batches().len(), None);
        runtime.multi_queue |=
            planner.bound().batches().iter().any(|batch| {
                batch.domain.queue_index != compiled.plan.definition.context.queue_index
            });
        Ok(())
    }
    pub(super) fn next_prelude(
        &mut self,
        _compiled: &graph::FrameInstance,
        batch: usize,
    ) -> crate::Result<Option<QueuePrelude>> {
        let runtime = &mut self.queue_runtime;
        let Some(&next) = runtime
            .planner
            .bound()
            .execution_order()
            .get(runtime.cursor)
        else {
            return Ok(None);
        };
        if next == batch {
            return Ok(None);
        }
        let entry = &runtime.planner.bound().batches()[next];
        if !matches!(entry.kind, BoundBatchKind::OwnershipPrelude) {
            return Err(crate::Error::invalid_usage(
                "unexpected native batch execution order",
            ));
        }
        runtime.active_batch = Some(next);
        Ok(Some(QueuePrelude {
            domain: entry.domain,
            tag: next as u64,
        }))
    }
    pub(super) fn record_prelude(
        &mut self,
        command: u64,
        prelude: QueuePrelude,
    ) -> crate::Result<()> {
        let planner = self.queue_runtime.planner.clone();
        for transition in planner
            .bound()
            .transitions()
            .iter()
            .filter(|transition| transition.batch == prelude.tag as usize)
        {
            let token = self.queue_runtime.tokens.values[transition.resource as usize].ok_or_else(
                || crate::Error::invalid_usage("ownership transition has no physical token"),
            )?;
            if !ExecutionBackend::transition(self, command, token, transition) {
                return Err(self.recording_error.take().unwrap_or_else(|| {
                    crate::Error::invalid_usage("ownership transition recording failed")
                }));
            }
        }
        self.attach_range_patches(&[command], prelude.tag as usize)
    }
    pub(super) fn attach_range_patches(
        &mut self,
        commands: &[u64],
        batch: usize,
    ) -> crate::Result<()> {
        let last = commands
            .last()
            .ok_or_else(|| crate::Error::invalid_usage("native state batch contains no command"))?;
        let runtime = &self.queue_runtime;
        let target = self
            .commands
            .get_mut(last)
            .ok_or_else(|| crate::Error::invalid_usage("native state command is missing"))?;
        for patch in runtime
            .planner
            .bound()
            .patches()
            .iter()
            .filter(|patch| patch.batch == batch)
        {
            let token = runtime.tokens.values[patch.resource as usize]
                .ok_or_else(|| crate::Error::invalid_usage("native patch has no physical token"))?;
            let cell = self
                .resources
                .get(&token)
                .ok_or_else(|| crate::Error::invalid_usage("native patch resource is missing"))?
                .owner_cell();
            target.record_state_patch(
                cell,
                patch.resolve(|source| runtime.points.values.get(source).copied().flatten())?,
            )?;
        }
        Ok(())
    }
    pub(super) fn queue_submission_dependencies(&self) -> crate::Result<SyncSet> {
        let mut result = SyncSet::EMPTY;
        let Some(batch) = self.queue_runtime.active_batch else {
            return Ok(result);
        };
        for wait in self
            .queue_runtime
            .planner
            .bound()
            .waits()
            .iter()
            .filter(|wait| wait.batch == batch)
        {
            result.insert(match wait.dependency {
                DependencyRef::Point(point) => point,
                DependencyRef::Batch(source) => self
                    .queue_runtime
                    .points
                    .values
                    .get(source)
                    .copied()
                    .flatten()
                    .ok_or_else(|| {
                        crate::Error::invalid_usage("native wait refers to an unsuccessful batch")
                    })?,
            });
        }
        Ok(result)
    }
    pub(super) fn queue_submitted(&mut self, point: SyncPoint) {
        let Some(batch) = self.queue_runtime.active_batch.take() else {
            return;
        };
        self.queue_runtime.points.values[batch] = Some(point);
        self.queue_runtime.cursor += 1;
    }
    pub(super) fn queue_timeline_wait(&self, point: SyncPoint) -> crate::Result<GraphTimelineWait> {
        let semaphore = self
            .queue_timelines
            .get(point.queue.index())
            .ok_or_else(|| crate::Error::invalid_usage("native wait queue is absent"))?;
        Ok(GraphTimelineWait {
            semaphore: semaphore.handle,
            value: point.value,
            stage: vk::PipelineStageFlags::ALL_COMMANDS,
        })
    }
}
