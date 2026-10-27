//! Binding the heap into command buffers and shader-memory access checks.
use ash::vk;
use crate::Error;
use crate::Result;
use crate::command::command_buffer::VulkanCommandBuffer;
use crate::pipeline::shader::VulkanPipeline;
use crate::profile::RecordCount;
use crate::profile::RecordPhase;
use crate::profile::RecordSpan;
use crate::profile::record_count;
use std::sync::Arc;
use super::BindlessHeap;
use super::lease::BindlessLease;
use super::lease::Owner;

impl BindlessHeap {
    /// # Safety
    /// Every index dynamically accessed by the shader must have its lease in
    /// `leases`. Image layouts and resource access barriers must match the draw.
    pub unsafe fn bind_command(
        self: &Arc<Self>,
        command: &mut VulkanCommandBuffer,
        pipeline: &Arc<VulkanPipeline>,
        set: u32,
        leases: &[Arc<BindlessLease>],
    ) -> Result<()> {
        self.validate_binding(pipeline, set, leases)?;
        if !command.is_recording() || !Arc::ptr_eq(&command.pool.device, &self.device) {
            return Err(Error::invalid_usage(
                "bindless command is not recording on this device",
            ));
        }
        // The heap and pipeline contain no native image/buffer accesses. Every
        // lease is checked independently against this command's actual patches.
        let retain_trace = RecordSpan::new(RecordPhase::Retain);
        command.retain_object(self.clone())?;
        record_count(RecordCount::RetainSuccesses, 1);
        command.retain_object(pipeline.clone())?;
        record_count(RecordCount::RetainSuccesses, 1);
        for lease in leases {
            if let Owner::Buffer {
                buffer,
                offset,
                size,
                parameters: true,
            } = &lease.owner
            {
                command.retain_upload_read(buffer, *offset, *size, Self::upload_scope())?;
                record_count(RecordCount::RetainSuccesses, 1);
                // SAFETY: the formal upload intent covers this immutable ring range.
                unsafe {
                    command.retain_recorded(lease.clone())?;
                    record_count(RecordCount::RetainSuccesses, 1);
                }
                continue;
            }
            let known = match &lease.owner {
                Owner::Sampler(_) => true,
                Owner::Buffer {
                    buffer,
                    offset,
                    size,
                    ..
                } => command.recorded_access_covers(
                    &buffer.last_use,
                    crate::tracking::OwnershipRange::Buffer {
                        offset: *offset,
                        size: *size,
                    },
                    |scope, _| Self::is_shader_memory_access(scope.access),
                )?,
                Owner::View { view, layout } => command.recorded_access_covers(
                    &view.texture.last_use,
                    crate::tracking::OwnershipRange::Image {
                        aspects: view.desc.aspect.as_raw(),
                        base_mip: view.desc.base_mip_level,
                        mip_count: view.desc.level_count,
                        base_layer: view.desc.base_layer,
                        layer_count: view.desc.layer_count,
                    },
                    |scope, current_layout| {
                        current_layout == *layout && Self::is_shader_memory_access(scope.access)
                    },
                )?,
            };
            if known {
                unsafe {
                    command.retain_recorded(lease.clone())?;
                    record_count(RecordCount::RetainSuccesses, 1);
                }
            } else {
                command.retain_opaque(lease.clone())?;
                record_count(RecordCount::RetainSuccesses, 1);
            }
        }
        drop(retain_trace);
        let _native_bind = RecordSpan::new(RecordPhase::NativeBind);
        unsafe { self.device.core.device.cmd_bind_descriptor_sets(command.handle,
            pipeline.point, pipeline.layout.handle, set, &[self.set.handle], &[]); }
        record_count(RecordCount::NativeBindCalls, 1);
        Ok(())
    }
}

impl BindlessHeap {
    pub(crate) fn is_shader_memory_access(access: vk::AccessFlags) -> bool {
        // MEMORY_* includes the corresponding shader accesses in Vulkan.
        // Host/transfer/attachment scopes alone cannot cover a shader binding.
        access.intersects(
            vk::AccessFlags::SHADER_READ
                | vk::AccessFlags::SHADER_WRITE
                | vk::AccessFlags::MEMORY_READ
                | vk::AccessFlags::MEMORY_WRITE,
        )
    }

    pub(crate) fn upload_scope() -> crate::native::AccessScope {
        crate::native::AccessScope {
            stages: vk::PipelineStageFlags::ALL_COMMANDS,
            access: vk::AccessFlags::SHADER_READ,
            write: false,
        }
    }
}
