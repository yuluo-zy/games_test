//! Native synchronization scopes and capability-selected Vulkan dispatch.
/// One barrier batch: memory, buffer and image barriers sharing one source
/// and destination stage mask, recorded with a single barrier command.
pub struct SynchronizationScope {
    pub memory: smallvec::SmallVec<[ash::vk::MemoryBarrier<'static>; 1]>,
    pub buffers: smallvec::SmallVec<[ash::vk::BufferMemoryBarrier<'static>; 4]>,
    pub images: smallvec::SmallVec<[ash::vk::ImageMemoryBarrier<'static>; 4]>,
    pub source_stages: ash::vk::PipelineStageFlags,
    pub destination_stages: ash::vk::PipelineStageFlags,
}
impl Default for SynchronizationScope {
    fn default() -> Self {
        Self {
            memory: smallvec::SmallVec::new(),
            buffers: smallvec::SmallVec::new(),
            images: smallvec::SmallVec::new(),
            source_stages: ash::vk::PipelineStageFlags::from_raw(1),
            destination_stages: ash::vk::PipelineStageFlags::from_raw(0x2000),
        }
    }
}
impl SynchronizationScope {
    pub(crate) fn new(
        source_stages: ash::vk::PipelineStageFlags,
        destination_stages: ash::vk::PipelineStageFlags,
    ) -> Self {
        Self {
            source_stages,
            destination_stages,
            ..Default::default()
        }
    }
    pub fn is_empty(&self) -> bool {
        self.memory.is_empty() && self.buffers.is_empty() && self.images.is_empty()
    }
    /// Append `barrier` for `target`, fallibly reserving host memory. The
    /// caller widens the batch's stages.
    pub(crate) fn push(
        &mut self,
        target: BarrierTarget,
        barrier: &RangeBarrier,
    ) -> crate::Result<()> {
        match target {
            BarrierTarget::Buffer(buffer) => {
                let barrier = barrier.buffer(buffer)?;
                self.buffers
                    .try_reserve(1)
                    .map_err(|_| crate::Error::OutOfHostMemory)?;
                self.buffers.push(barrier);
            }
            BarrierTarget::Image(image) => {
                let barrier = barrier.image(image)?;
                self.images
                    .try_reserve(1)
                    .map_err(|_| crate::Error::OutOfHostMemory)?;
                self.images.push(barrier);
            }
        }
        Ok(())
    }
    /// Issue this batch into a recording command of its device; an empty
    /// batch records nothing.
    pub(crate) fn issue_into(
        &self,
        command: &crate::command::command_buffer::VulkanCommandBuffer,
    ) -> crate::Result<()> {
        if self.is_empty() {
            return Ok(());
        }
        // SAFETY: the command is recording on this device and every barrier
        // resource is retained by it through completion.
        unsafe { command.pool.device.issue_barriers(command.handle, self) }
    }

}

/// Which half of a queue-family ownership transfer a barrier records.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum OwnershipHalf {
    /// Recorded on the source family: makes `transfer.source` available.
    Release,
    /// Recorded on the destination family: makes the data visible to
    /// `transfer.destination`.
    Acquire,
}

/// The resource a [`RangeBarrier`] applies to.
#[derive(Clone, Copy, Debug)]
pub(crate) enum BarrierTarget {
    Buffer(ash::vk::Buffer),
    Image(ash::vk::Image),
}

/// One memory barrier over a tracked native range: the single place where
/// symbolic range transitions become Vulkan barrier structures. Execution
/// scopes (stages) belong to the [`SynchronizationScope`] batch.
#[derive(Clone, Copy, Debug)]
pub(crate) struct RangeBarrier {
    pub range: crate::tracking::OwnershipRange,
    pub source_access: ash::vk::AccessFlags,
    pub destination_access: ash::vk::AccessFlags,
    pub old_layout: ash::vk::ImageLayout,
    pub new_layout: ash::vk::ImageLayout,
    pub source_family: u32,
    pub destination_family: u32,
}
impl RangeBarrier {
    /// A barrier without a queue-family transfer.
    pub(crate) fn local(
        range: crate::tracking::OwnershipRange,
        source_access: ash::vk::AccessFlags,
        destination_access: ash::vk::AccessFlags,
        old_layout: ash::vk::ImageLayout,
        new_layout: ash::vk::ImageLayout,
    ) -> Self {
        Self {
            range,
            source_access,
            destination_access,
            old_layout,
            new_layout,
            source_family: ash::vk::QUEUE_FAMILY_IGNORED,
            destination_family: ash::vk::QUEUE_FAMILY_IGNORED,
        }
    }
    /// One half of an ownership transfer. Both halves carry identical
    /// families, layouts and range.
    pub(crate) fn ownership(
        transfer: &crate::tracking::range_state::RangeTransfer,
        half: OwnershipHalf,
    ) -> Self {
        let (source_access, destination_access) = match half {
            OwnershipHalf::Release => (transfer.source.access, ash::vk::AccessFlags::empty()),
            OwnershipHalf::Acquire => (ash::vk::AccessFlags::empty(), transfer.destination.access),
        };
        Self {
            range: transfer.range,
            source_access,
            destination_access,
            old_layout: transfer.old_layout,
            new_layout: transfer.new_layout,
            source_family: transfer.source_family,
            destination_family: transfer.destination_family,
        }
    }
    pub(crate) fn buffer(
        &self,
        buffer: ash::vk::Buffer,
    ) -> crate::Result<ash::vk::BufferMemoryBarrier<'static>> {
        let crate::tracking::OwnershipRange::Buffer { offset, size } = self.range else {
            return Err(crate::Error::invalid_usage(
                "buffer barrier of an image range",
            ));
        };
        Ok(ash::vk::BufferMemoryBarrier::default()
            .src_access_mask(self.source_access)
            .dst_access_mask(self.destination_access)
            .src_queue_family_index(self.source_family)
            .dst_queue_family_index(self.destination_family)
            .buffer(buffer)
            .offset(offset)
            .size(size))
    }
    pub(crate) fn image(
        &self,
        image: ash::vk::Image,
    ) -> crate::Result<ash::vk::ImageMemoryBarrier<'static>> {
        let range = subresource_range(self.range)
            .ok_or_else(|| crate::Error::invalid_usage("image barrier of a buffer range"))?;
        Ok(ash::vk::ImageMemoryBarrier::default()
            .src_access_mask(self.source_access)
            .dst_access_mask(self.destination_access)
            .old_layout(self.old_layout)
            .new_layout(self.new_layout)
            .src_queue_family_index(self.source_family)
            .dst_queue_family_index(self.destination_family)
            .image(image)
            .subresource_range(range))
    }
}

/// The Vulkan subresource range of an image ownership range.
pub(crate) fn subresource_range(
    range: crate::tracking::OwnershipRange,
) -> Option<ash::vk::ImageSubresourceRange> {
    let crate::tracking::OwnershipRange::Image {
        aspects,
        base_mip,
        mip_count,
        base_layer,
        layer_count,
    } = range
    else {
        return None;
    };
    Some(
        ash::vk::ImageSubresourceRange::default()
            .aspect_mask(ash::vk::ImageAspectFlags::from_raw(aspects))
            .base_mip_level(base_mip)
            .level_count(mip_count)
            .base_array_layer(base_layer)
            .layer_count(layer_count),
    )
}

/// Cache the actual enabled API once, without loading extension entry points
/// for every pass. No Device owner is retained by this Copy dispatcher.
#[derive(Clone, Copy)]
pub(crate) struct BarrierDispatch {
    generation: crate::sync::DeviceGeneration,
    cmd_pipeline_barrier2: ash::vk::PFN_vkCmdPipelineBarrier2,
}
impl BarrierDispatch {
    pub(crate) fn new(device: &crate::device::Device) -> Self {
        let cmd_pipeline_barrier2 = match &device.core.synchronization2 {
            Some(extension) => extension.fp().cmd_pipeline_barrier2_khr,
            None => device.core.device.fp_v1_3().cmd_pipeline_barrier2,
        };
        Self {
            generation: device.generation(),
            cmd_pipeline_barrier2,
        }
    }
    /// # Safety
    /// `command` must be recording on this device with a compatible family;
    /// every resource in the scope must remain owned through its real last use.
    pub(crate) unsafe fn issue(
        &self,
        device: &crate::device::Device,
        command: ash::vk::CommandBuffer,
        scope: &SynchronizationScope,
    ) -> crate::Result<()> {
        use ash::vk;
        if device.is_device_lost() {
            return Err(crate::Error::DeviceLost);
        }
        if self.generation != device.generation() || command == vk::CommandBuffer::null() {
            return Err(crate::Error::invalid_usage(
                "barrier command or device does not match",
            ));
        }
        let barriers = Barrier2Scope::new(scope, &device.memory.stats)?;
        let dependency = vk::DependencyInfo::default()
            .memory_barriers(&barriers.memory)
            .buffer_memory_barriers(&barriers.buffers)
            .image_memory_barriers(&barriers.images);
        unsafe { (self.cmd_pipeline_barrier2)(command, &dependency); }
        Ok(())
    }
}

/// API conversion only: all access/layout/range/ownership decisions have already
/// been made in the shared symbolic scope. Larger batches spill fallibly and
/// keep their actual CPU capacity charged through the native call.
struct Barrier2Scope {
    memory: crate::native::submit::SubmitScratch<[ash::vk::MemoryBarrier2<'static>; 1]>,
    buffers: crate::native::submit::SubmitScratch<[ash::vk::BufferMemoryBarrier2<'static>; 4]>,
    images: crate::native::submit::SubmitScratch<[ash::vk::ImageMemoryBarrier2<'static>; 4]>,
}
impl Barrier2Scope {
    fn new(
        scope: &SynchronizationScope,
        stats: &std::sync::Arc<crate::accounting::AllocatorStats>,
    ) -> crate::Result<Self> {
        use ash::vk;
        if scope.memory.iter().any(|barrier| !barrier.p_next.is_null())
            || scope
                .buffers
                .iter()
                .any(|barrier| !barrier.p_next.is_null())
            || scope.images.iter().any(|barrier| !barrier.p_next.is_null())
        {
            return Err(crate::Error::unsupported(
                "extended legacy barrier structures cannot be converted",
            ));
        }
        // A legacy stage-only execution dependency needs an access-free memory
        // barrier in synchronization2; an empty DependencyInfo would do nothing.
        let execution_only = scope.is_empty();
        let memory_count = scope.memory.len().max(usize::from(execution_only));
        for count in [memory_count, scope.buffers.len(), scope.images.len()] {
            u32::try_from(count)
                .map_err(|_| crate::Error::invalid_usage("barrier count exceeds Vulkan limits"))?;
        }
        let source = vk::PipelineStageFlags2::from_raw(u64::from(scope.source_stages.as_raw()));
        let destination =
            vk::PipelineStageFlags2::from_raw(u64::from(scope.destination_stages.as_raw()));
        let access = |flags: vk::AccessFlags| vk::AccessFlags2::from_raw(u64::from(flags.as_raw()));
        let memory = crate::native::submit::SubmitScratch::try_collect(
            memory_count,
            stats,
            scope
                .memory
                .iter()
                .map(|barrier| {
                    Ok(vk::MemoryBarrier2::default()
                        .src_stage_mask(source)
                        .dst_stage_mask(destination)
                        .src_access_mask(access(barrier.src_access_mask))
                        .dst_access_mask(access(barrier.dst_access_mask)))
                })
                .chain(execution_only.then(|| {
                    Ok(vk::MemoryBarrier2::default()
                        .src_stage_mask(source)
                        .dst_stage_mask(destination))
                })),
        )?;
        let buffers = crate::native::submit::SubmitScratch::try_collect(
            scope.buffers.len(),
            stats,
            scope.buffers.iter().map(|barrier| {
                Ok(vk::BufferMemoryBarrier2::default()
                    .src_stage_mask(source)
                    .dst_stage_mask(destination)
                    .src_access_mask(access(barrier.src_access_mask))
                    .dst_access_mask(access(barrier.dst_access_mask))
                    .src_queue_family_index(barrier.src_queue_family_index)
                    .dst_queue_family_index(barrier.dst_queue_family_index)
                    .buffer(barrier.buffer)
                    .offset(barrier.offset)
                    .size(barrier.size))
            }),
        )?;
        let images = crate::native::submit::SubmitScratch::try_collect(
            scope.images.len(),
            stats,
            scope.images.iter().map(|barrier| {
                Ok(vk::ImageMemoryBarrier2::default()
                    .src_stage_mask(source)
                    .dst_stage_mask(destination)
                    .src_access_mask(access(barrier.src_access_mask))
                    .dst_access_mask(access(barrier.dst_access_mask))
                    .old_layout(barrier.old_layout)
                    .new_layout(barrier.new_layout)
                    .src_queue_family_index(barrier.src_queue_family_index)
                    .dst_queue_family_index(barrier.dst_queue_family_index)
                    .image(barrier.image)
                    .subresource_range(barrier.subresource_range))
            }),
        )?;
        Ok(Self {
            memory,
            buffers,
            images,
        })
    }
}

#[cfg(test)]
mod synchronization2_tests;
