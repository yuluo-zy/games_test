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
        command: &super::command_buffer::VulkanCommandBuffer,
    ) -> crate::Result<()> {
        if self.is_empty() {
            return Ok(());
        }
        // SAFETY: the command is recording on this device and every barrier
        // resource is retained by it through completion.
        unsafe { command.pool.device.issue_barriers(command.handle, self) }
    }
    fn legacy_stages(&self) -> (ash::vk::PipelineStageFlags, ash::vk::PipelineStageFlags) {
        use ash::vk::PipelineStageFlags;
        // synchronization2 NONE has an empty execution scope. Legacy barriers
        // cannot use zero masks without that feature: TOP as a source and
        // BOTTOM as a destination preserve those empty scopes respectively.
        (
            if self.source_stages.is_empty() {
                PipelineStageFlags::TOP_OF_PIPE
            } else {
                self.source_stages
            },
            if self.destination_stages.is_empty() {
                PipelineStageFlags::BOTTOM_OF_PIPE
            } else {
                self.destination_stages
            },
        )
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
    pub range: crate::resource::OwnershipRange,
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
        range: crate::resource::OwnershipRange,
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
        transfer: &crate::resource::range_state::RangeTransfer,
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
        let crate::resource::OwnershipRange::Buffer { offset, size } = self.range else {
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
    range: crate::resource::OwnershipRange,
) -> Option<ash::vk::ImageSubresourceRange> {
    let crate::resource::OwnershipRange::Image {
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
    generation: crate::device::DeviceGeneration,
    cmd_pipeline_barrier2: Option<ash::vk::PFN_vkCmdPipelineBarrier2>,
}
impl BarrierDispatch {
    pub(crate) fn new(device: &super::device::Device) -> Self {
        let cmd_pipeline_barrier2 = device
            .caps()
            .is_enabled(crate::device::Feature::Synchronization2)
            .then(|| match &device.synchronization2 {
                Some(extension) => extension.fp().cmd_pipeline_barrier2_khr,
                None => device.device.fp_v1_3().cmd_pipeline_barrier2,
            });
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
        device: &super::device::Device,
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
        if let Some(cmd_pipeline_barrier2) = self.cmd_pipeline_barrier2 {
            let barriers = Barrier2Scope::new(scope, &device.allocator_stats)?;
            let dependency = vk::DependencyInfo::default()
                .memory_barriers(&barriers.memory)
                .buffer_memory_barriers(&barriers.buffers)
                .image_memory_barriers(&barriers.images);
            unsafe {
                cmd_pipeline_barrier2(command, &dependency);
            }
        } else {
            let (source, destination) = scope.legacy_stages();
            unsafe {
                device.device.cmd_pipeline_barrier(
                    command,
                    source,
                    destination,
                    vk::DependencyFlags::empty(),
                    &scope.memory,
                    &scope.buffers,
                    &scope.images,
                );
            }
        }
        Ok(())
    }
}

/// API conversion only: all access/layout/range/ownership decisions have already
/// been made in the shared symbolic scope. Larger batches spill fallibly and
/// keep their actual CPU capacity charged through the native call.
struct Barrier2Scope {
    memory: super::submit::SubmitScratch<[ash::vk::MemoryBarrier2<'static>; 1]>,
    buffers: super::submit::SubmitScratch<[ash::vk::BufferMemoryBarrier2<'static>; 4]>,
    images: super::submit::SubmitScratch<[ash::vk::ImageMemoryBarrier2<'static>; 4]>,
}
impl Barrier2Scope {
    fn new(
        scope: &SynchronizationScope,
        stats: &std::sync::Arc<super::allocator::AllocatorStats>,
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
        let memory = super::submit::SubmitScratch::try_collect(
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
        let buffers = super::submit::SubmitScratch::try_collect(
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
        let images = super::submit::SubmitScratch::try_collect(
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
mod synchronization2_tests {
    use super::*;
    use ash::vk::{self, Handle};
    use std::sync::Arc;

    #[test]
    fn range_barriers_map_ranges_and_reject_the_wrong_resource_kind() {
        let image_range = crate::resource::OwnershipRange::Image {
            aspects: vk::ImageAspectFlags::COLOR.as_raw(),
            base_mip: 2,
            mip_count: 3,
            base_layer: 4,
            layer_count: 5,
        };
        let barrier = RangeBarrier::local(
            image_range,
            vk::AccessFlags::SHADER_WRITE,
            vk::AccessFlags::SHADER_READ,
            vk::ImageLayout::GENERAL,
            vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
        );
        let image = barrier.image(vk::Image::from_raw(7)).unwrap();
        assert_eq!(image.src_queue_family_index, vk::QUEUE_FAMILY_IGNORED);
        assert_eq!(image.dst_queue_family_index, vk::QUEUE_FAMILY_IGNORED);
        assert_eq!(
            (image.old_layout, image.new_layout),
            (
                vk::ImageLayout::GENERAL,
                vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL
            )
        );
        let range = image.subresource_range;
        assert_eq!(
            (
                range.base_mip_level,
                range.level_count,
                range.base_array_layer,
                range.layer_count
            ),
            (2, 3, 4, 5)
        );
        assert!(barrier.buffer(vk::Buffer::from_raw(7)).is_err());
        let mut scope = SynchronizationScope::default();
        assert!(
            scope
                .push(BarrierTarget::Buffer(vk::Buffer::from_raw(7)), &barrier)
                .is_err()
        );
        assert!(scope.is_empty());
        scope
            .push(BarrierTarget::Image(vk::Image::from_raw(7)), &barrier)
            .unwrap();
        assert_eq!(scope.images.len(), 1);
    }

    #[test]
    fn conversion_preserves_accesses_layouts_ranges_and_ownership_pairs() {
        let stats = Arc::new(super::super::allocator::AllocatorStats::default());
        let mut scope = SynchronizationScope::new(
            vk::PipelineStageFlags::TRANSFER | vk::PipelineStageFlags::HOST,
            vk::PipelineStageFlags::COMPUTE_SHADER,
        );
        scope.memory.push(
            vk::MemoryBarrier::default()
                .src_access_mask(vk::AccessFlags::TRANSFER_WRITE)
                .dst_access_mask(vk::AccessFlags::SHADER_READ),
        );
        scope.buffers.push(
            vk::BufferMemoryBarrier::default()
                .src_access_mask(vk::AccessFlags::TRANSFER_WRITE)
                .dst_access_mask(vk::AccessFlags::SHADER_READ)
                .src_queue_family_index(1)
                .dst_queue_family_index(3)
                .buffer(vk::Buffer::from_raw(101))
                .offset(123)
                .size(456),
        );
        scope.images.push(
            vk::ImageMemoryBarrier::default()
                .src_access_mask(vk::AccessFlags::SHADER_WRITE)
                .dst_access_mask(vk::AccessFlags::TRANSFER_READ)
                .old_layout(vk::ImageLayout::GENERAL)
                .new_layout(vk::ImageLayout::TRANSFER_SRC_OPTIMAL)
                .src_queue_family_index(3)
                .dst_queue_family_index(1)
                .image(vk::Image::from_raw(202))
                .subresource_range(
                    vk::ImageSubresourceRange::default()
                        .aspect_mask(vk::ImageAspectFlags::DEPTH | vk::ImageAspectFlags::STENCIL)
                        .base_mip_level(1)
                        .level_count(2)
                        .base_array_layer(3)
                        .layer_count(4),
                ),
        );
        let converted = Barrier2Scope::new(&scope, &stats).unwrap();
        assert_eq!(
            converted.memory[0].src_stage_mask.as_raw(),
            u64::from(scope.source_stages.as_raw())
        );
        assert_eq!(
            converted.memory[0].dst_stage_mask,
            vk::PipelineStageFlags2::COMPUTE_SHADER
        );
        assert_eq!(
            converted.memory[0].src_access_mask,
            vk::AccessFlags2::TRANSFER_WRITE
        );
        assert_eq!(
            converted.memory[0].dst_access_mask,
            vk::AccessFlags2::SHADER_READ
        );
        let buffer = converted.buffers[0];
        assert_eq!(buffer.buffer.as_raw(), 101);
        assert_eq!(
            (buffer.src_queue_family_index, buffer.dst_queue_family_index),
            (1, 3)
        );
        assert_eq!((buffer.offset, buffer.size), (123, 456));
        assert_eq!(buffer.src_access_mask, vk::AccessFlags2::TRANSFER_WRITE);
        assert_eq!(buffer.dst_access_mask, vk::AccessFlags2::SHADER_READ);
        let image = converted.images[0];
        assert_eq!(image.image.as_raw(), 202);
        assert_eq!(
            (image.src_queue_family_index, image.dst_queue_family_index),
            (3, 1)
        );
        assert_eq!(
            (image.old_layout, image.new_layout),
            (
                vk::ImageLayout::GENERAL,
                vk::ImageLayout::TRANSFER_SRC_OPTIMAL
            )
        );
        assert_eq!(image.src_access_mask, vk::AccessFlags2::SHADER_WRITE);
        assert_eq!(image.dst_access_mask, vk::AccessFlags2::TRANSFER_READ);
        let range = image.subresource_range;
        assert_eq!(
            range.aspect_mask,
            vk::ImageAspectFlags::DEPTH | vk::ImageAspectFlags::STENCIL
        );
        assert_eq!(
            (
                range.base_mip_level,
                range.level_count,
                range.base_array_layer,
                range.layer_count
            ),
            (1, 2, 3, 4)
        );
        assert_eq!(
            stats.cpu_memory_usage().total(),
            0,
            "common barriers remain inline"
        );
    }

    #[test]
    fn stage_only_scope_retains_a_real_execution_dependency() {
        let stats = Arc::new(super::super::allocator::AllocatorStats::default());
        let scope = SynchronizationScope::new(
            vk::PipelineStageFlags::ALL_COMMANDS,
            vk::PipelineStageFlags::ALL_COMMANDS,
        );
        let converted = Barrier2Scope::new(&scope, &stats).unwrap();
        assert_eq!(converted.memory.len(), 1);
        assert_eq!(
            converted.memory[0].src_stage_mask,
            vk::PipelineStageFlags2::ALL_COMMANDS
        );
        assert_eq!(
            converted.memory[0].dst_stage_mask,
            vk::PipelineStageFlags2::ALL_COMMANDS
        );
        assert!(converted.memory[0].src_access_mask.is_empty());
        assert!(converted.memory[0].dst_access_mask.is_empty());
    }

    #[test]
    fn legacy_none_scopes_use_legal_empty_scope_equivalents() {
        let stats = Arc::new(super::super::allocator::AllocatorStats::default());
        let scope = SynchronizationScope::new(
            vk::PipelineStageFlags::empty(),
            vk::PipelineStageFlags::empty(),
        );
        assert_eq!(
            scope.legacy_stages(),
            (
                vk::PipelineStageFlags::TOP_OF_PIPE,
                vk::PipelineStageFlags::BOTTOM_OF_PIPE,
            )
        );
        let converted = Barrier2Scope::new(&scope, &stats).unwrap();
        assert!(converted.memory[0].src_stage_mask.is_empty());
        assert!(converted.memory[0].dst_stage_mask.is_empty());
        let scope = SynchronizationScope::new(
            vk::PipelineStageFlags::TRANSFER,
            vk::PipelineStageFlags::COMPUTE_SHADER,
        );
        assert_eq!(
            scope.legacy_stages(),
            (scope.source_stages, scope.destination_stages)
        );
    }

    #[test]
    fn later_spill_admission_failure_releases_already_prepared_capacity() {
        let stats = Arc::new(super::super::allocator::AllocatorStats::default());
        let mut scope = SynchronizationScope::default();
        scope
            .buffers
            .extend([vk::BufferMemoryBarrier::default(); 5]);
        scope.images.extend([vk::ImageMemoryBarrier::default(); 5]);
        stats
            .set_total_budget((5 * size_of::<vk::BufferMemoryBarrier2<'static>>()) as u64)
            .unwrap();
        assert!(matches!(
            Barrier2Scope::new(&scope, &stats),
            Err(crate::Error::OutOfHostMemory)
        ));
        assert_eq!(stats.cpu_memory_usage().total(), 0);
        stats.set_total_budget(4096).unwrap();
        let converted = Barrier2Scope::new(&scope, &stats).unwrap();
        assert!(
            stats.cpu_memory_usage().live
                >= (5
                    * (size_of::<vk::BufferMemoryBarrier2<'static>>()
                        + size_of::<vk::ImageMemoryBarrier2<'static>>())) as u64
        );
        drop(converted);
        assert_eq!(stats.cpu_memory_usage().total(), 0);
    }
}
