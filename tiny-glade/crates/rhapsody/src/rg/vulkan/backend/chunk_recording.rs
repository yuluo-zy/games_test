//! Per-chunk command recording view and its barrier bookkeeping.
use ash::vk;
use ash::vk::Handle;
use crate::device::Device;
use super::TimelineSignal;
use super::TimelineWait;
use super::super::presentation;
use super::super::resource_spec::GpuResource;

/// Pending barrier scopes of the commands being recorded. At most a handful of
/// commands are open at once, so a short vector beats a `BTreeMap` whose
/// ~600-byte values forced a node allocation each time a command got its first
/// barrier.
#[derive(Default)]
pub(super) struct CommandBarriers(Vec<(u64, crate::command::barrier::SynchronizationScope)>);

impl CommandBarriers {
    pub(super) fn entry(&mut self, command: u64) -> &mut crate::command::barrier::SynchronizationScope {
        let at = match self.0.iter().position(|(handle, _)| *handle == command) {
            Some(at) => at,
            None => {
                self.0.push((command, Default::default()));
                self.0.len() - 1
            }
        };
        &mut self.0[at].1
    }
    pub(super) fn remove(&mut self, command: u64) -> Option<crate::command::barrier::SynchronizationScope> {
        let at = self.0.iter().position(|(handle, _)| *handle == command)?;
        Some(self.0.swap_remove(at).1)
    }
}

#[derive(Default)]
pub(in crate::rg::vulkan) struct OwnedBatchSync {
    pub(in crate::rg::vulkan) waits: Vec<TimelineWait>,
    pub(in crate::rg::vulkan) signals: Vec<TimelineSignal>,
}

/// Emit exactly the native range chosen by the bound synchronization program.
pub(super) fn add_transition(
    scope: &mut crate::command::barrier::SynchronizationScope,
    resources: &crate::collections::token_map::TokenMap<GpuResource>,
    token: u64,
    transition: &crate::rg::sync_plan::BoundTransition,
    present: Option<presentation::PresentPatch>,
) -> crate::Result<()> {
    use crate::command::barrier::{BarrierTarget, RangeBarrier};
    presentation::PresentPatch::apply(present, scope, token, transition);
    // Alias (shared backing) dependencies are ordinary transitions here: the
    // planner models them as a synthetic previous access of the new member.
    scope.source_stages |= transition.source.stages;
    scope.destination_stages |= transition.destination.stages;
    let target = match resources.get(&token) {
        Some(GpuResource::Buffer(buffer)) => BarrierTarget::Buffer(buffer.handle),
        Some(GpuResource::Image(image)) => BarrierTarget::Image(image.handle),
        None => {
            return Err(crate::Error::invalid_usage(
                "native barrier resource is missing",
            ));
        }
    };
    scope.push(
        target,
        &RangeBarrier {
            range: transition.range,
            source_access: transition.source.access,
            destination_access: transition.destination.access,
            old_layout: transition.old_layout,
            new_layout: transition.new_layout,
            source_family: transition.source_family,
            destination_family: transition.destination_family,
        },
    )
}

/// 并行录制时各块共享的只读视图：只读访问表与资源表，直接把每个 pass 的屏障写入块自己的命令缓冲。
pub(super) struct VulkanChunkView<'a> {
    pub(super) device: &'a ash::Device,
    pub(super) present_patch: Option<presentation::PresentPatch>,
    pub(super) barrier_device: &'a Device,
    pub(super) barrier_dispatch: crate::command::barrier::BarrierDispatch,
    pub(super) resources: &'a crate::collections::token_map::TokenMap<GpuResource>,
    pub(super) cpu_arenas: &'a crate::command::CpuArenaPool,
    pub(super) owner_pool: &'a crate::collections::temp_list_pool::TempListPool<crate::command::binding::RetainedResource>,
}

impl crate::rg::executor::ChunkRecorder for VulkanChunkView<'_> {
    unsafe fn write_pass_timestamp(
        &self,
        lease: &mut crate::rg::executor::ChunkLease<'_>,
        pair: crate::rg::profiling::GpuPassTimestamp,
        end: bool,
    ) {
        pair.write(
            self.device,
            vk::CommandBuffer::from_raw(lease.command()),
            end,
        );
    }
    fn owner_storage_pool(
        &self,
    ) -> Option<&crate::collections::temp_list_pool::TempListPool<crate::command::binding::RetainedResource>>
    {
        Some(self.owner_pool)
    }

    fn cpu_arena_pool(&self) -> Option<&crate::command::CpuArenaPool> {
        Some(self.cpu_arenas)
    }
    fn record_transitions(
        &self,
        lease: &mut crate::rg::executor::ChunkLease<'_>,
        transitions: &[crate::rg::executor::ChunkTransition],
        owners: &mut crate::rg::owner_storage::ResourceOwners,
    ) -> crate::Result<()> {
        if transitions.is_empty() {
            return Ok(());
        }
        let mut scope = crate::command::barrier::SynchronizationScope::default();
        for t in transitions {
            // These references are covered by the exact native program and
            // its preflight StatePatches, rather than opaque whole-resource uses.
            match self.resources.get(&t.token) {
                Some(GpuResource::Buffer(buffer)) => owners.add_recorded(buffer.clone())?,
                Some(GpuResource::Image(image)) => owners.add_recorded(image.clone())?,
                None => {
                    return Err(crate::Error::invalid_usage(
                        "parallel barrier resource is missing",
                    ));
                }
            }
            add_transition(
                &mut scope,
                self.resources,
                t.token,
                &t.transition,
                self.present_patch,
            )?;
        }
        // 每个 pass 一次屏障命令。
        unsafe {
            self.barrier_dispatch.issue(
                self.barrier_device,
                vk::CommandBuffer::from_raw(lease.command()),
                &scope,
            )
        }
    }
}
