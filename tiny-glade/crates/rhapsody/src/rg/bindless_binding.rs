//! Binding of bindless descriptor heaps and parameter blocks into a recording pass.
//!
//! The heap and parameter types live in [`crate::bindless`] and know nothing about the
//! render graph; the pass-recording glue (coverage against the pass's declared accesses,
//! lease validation, binding into the command) belongs to the layer that owns passes.
use crate::bindless::{BindlessLease, Owner, *};
use crate::profile::{RecordCount, RecordPhase, RecordSpan, record_count};
use crate::pipeline::shader::VulkanPipeline;
use crate::{Error, Result};
use ash::vk::{self, Handle};
use std::sync::Arc;

impl BindlessHeap {
    /// # Safety
    /// In addition to bind_command's contract, declare all dynamically accessed
    /// resources and subranges in this graph pass before recording it.
    pub unsafe fn bind_pass(
        self: &Arc<Self>,
        context: &crate::rg::executor::PassContext<'_>,
        pipeline: &Arc<VulkanPipeline>,
        set: u32,
        leases: &[Arc<BindlessLease>],
    ) -> Result<()> {
        self.validate_binding(pipeline, set, leases)?;
        if context.compiled.plan.definition.context.device_generation
            != self.device.generation().value()
        {
            return Err(Error::invalid_usage(
                "bindless pass belongs to another device generation",
            ));
        }
        for lease in leases {
            Self::validate_pass_lease(context, lease)?;
        }
        let retain_trace = RecordSpan::new(RecordPhase::Retain);
        context.retain_object(self.clone())?;
        record_count(RecordCount::RetainSuccesses, 1);
        context.retain_object(pipeline.clone())?;
        record_count(RecordCount::RetainSuccesses, 1);
        for lease in leases {
            if let Owner::Buffer {
                buffer,
                offset,
                size,
                parameters: true,
            } = &lease.owner
            {
                context.retain_upload_read(buffer.clone(), *offset, *size, Self::upload_scope())?;
                record_count(RecordCount::RetainSuccesses, 1);
            }
            // SAFETY: validate_pass_lease checked every memory range and role.
            unsafe {
                context.retain_recorded(lease.clone())?;
                record_count(RecordCount::RetainSuccesses, 1);
            }
        }
        drop(retain_trace);
        let _native_bind = RecordSpan::new(RecordPhase::NativeBind);
        unsafe { self.device.core.device.cmd_bind_descriptor_sets(
            vk::CommandBuffer::from_raw(context.command.handle()), pipeline.point,
            pipeline.layout.handle, set, &[self.set.handle], &[]); }
        record_count(RecordCount::NativeBindCalls, 1);
        Ok(())
    }
    /// Every declared access of the pass that a shader may read through a
    /// bindless descriptor: its native scope is a shader memory access and the
    /// logical resource resolves to a physical one. Stable-sorted by physical
    /// resource so one lease only visits its own accesses.
    fn build_coverage(
        context: &crate::rg::executor::PassContext<'_>,
        managed: &crate::rg::pass_bindings::ManagedPassResources,
        pass: &crate::rg::graph::PassDeclaration,
    ) -> Vec<crate::rg::executor::CoverageEntry> {
        use crate::rg::executor::CoverageEntry;
        let semantics = &context.compiled.plan.definition.access_semantics;
        let mut entries = Vec::with_capacity(pass.accesses.len());
        for access in &pass.accesses {
            let Some(native) = semantics
                .iter()
                .find(|entry| entry.code == access.access_code)
            else {
                continue;
            };
            if !Self::is_shader_memory_access(native.scope.access) {
                continue;
            }
            if let Ok(buffer) = managed.buffer_for(access.resource_index) {
                entries.push(CoverageEntry {
                    resource: buffer.resource() as *const _ as usize,
                    layout: None,
                    range: access.range,
                });
            } else if let Ok(texture) = managed.texture_for(access.resource_index) {
                entries.push(CoverageEntry {
                    resource: texture.resource() as *const _ as usize,
                    layout: Some(native.layout),
                    range: access.range,
                });
            }
        }
        entries.sort_by_key(|entry| entry.resource);
        entries
    }
    fn validate_pass_lease(
        context: &crate::rg::executor::PassContext<'_>,
        lease: &BindlessLease,
    ) -> Result<()> {
        let _trace = RecordSpan::new(RecordPhase::ManagedValidate);
        record_count(RecordCount::PassLeaseChecks, 1);

        use crate::rg::{resource::AccessRange, vulkan::ResourceSpec};
        let (range, descriptor) = match &lease.owner {
            Owner::Sampler(_)
            | Owner::Buffer {
                parameters: true, ..
            } => return Ok(()),
            Owner::Buffer {
                buffer,
                offset,
                size,
                ..
            } => (
                AccessRange::Buffer {
                    offset: *offset,
                    size: *size,
                },
                ResourceSpec::Buffer(buffer.desc).descriptor(),
            ),
            Owner::View { view, .. } => (
                AccessRange::Texture {
                    aspect_mask: view.desc.aspect.as_raw(),
                    base_mip_level: view.desc.base_mip_level,
                    level_count: view.desc.level_count,
                    base_array_layer: view.desc.base_layer,
                    layer_count: view.desc.layer_count,
                },
                ResourceSpec::Image(view.texture.desc).descriptor(),
            ),
        };
        let managed = context.managed.ok_or_else(|| {
            Error::invalid_usage(
                "bindless graph resources require the production managed projection",
            )
        })?;
        let pass = context
            .compiled
            .declarations
            .passes
            .get(context.pass_index)
            .ok_or_else(|| Error::invalid_usage("bindless pass index is invalid"))?;
        let entries = context
            .coverage
            .entries(|| Self::build_coverage(context, managed, pass));
        // Entries are sorted by resource address; only this resource's accesses matter.
        let (key, layout) = match &lease.owner {
            Owner::Buffer { buffer, .. } => (Arc::as_ptr(buffer) as usize, None),
            Owner::View { view, layout } => (Arc::as_ptr(&view.texture) as usize, Some(*layout)),
            Owner::Sampler(_) => return Ok(()),
        };
        let first = entries.partition_point(|entry| entry.resource < key);
        record_count(RecordCount::CoverageAccesses, entries.len());
        let ranges = entries[first..]
            .iter()
            .take_while(|entry| entry.resource == key)
            .filter(|entry| entry.layout == layout)
            .map(|entry| entry.range);
        if !crate::rg::resource_contracts::covers_iter(ranges, range, descriptor)
            .map_err(|_| Error::invalid_usage("bindless lease range is invalid"))?
        {
            return Err(Error::invalid_usage(
                "bindless lease range or image layout is not covered by this pass's shader declarations",
            ));
        }
        Ok(())
    }
}

impl PreparedBindlessParameters {
    /// # Safety
    /// All resources the shader actually indexes must be bound and graph access
    /// declarations/layout barriers must describe those accesses.
    pub unsafe fn bind_pass(
        &self,
        context: &crate::rg::executor::PassContext<'_>,
        pipeline: &Arc<VulkanPipeline>,
    ) -> Result<()> {
        self.validate_pipeline(pipeline)?;
        unsafe {
            self.heap
                .bind_pass(context, pipeline, 0, &self.leases)?;
        }
        self.push(
            vk::CommandBuffer::from_raw(context.command.handle()),
            pipeline,
        );
        Ok(())
    }
}
