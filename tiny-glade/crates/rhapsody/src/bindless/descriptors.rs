//! Writing descriptors for buffers, images and samplers.
use ash::vk;
use crate::Error;
use crate::Result;
use crate::pipeline::shader::VulkanPipeline;
use crate::profile::RecordCount;
use crate::profile::RecordPhase;
use crate::profile::RecordSpan;
use crate::profile::record_count;
use crate::resource::buffer::Buffer;
use crate::resource::sampler::Sampler;
use crate::resource::texture::TextureView;
use std::sync::Arc;
use super::BindlessHeap;
use super::kind::DescriptorKind;
use super::lease::BindlessLease;
use super::lease::Owner;

impl BindlessHeap {
    pub fn storage_buffer(
        self: &Arc<Self>,
        buffer: Arc<Buffer>,
        offset: u64,
        size: u64,
    ) -> Result<Arc<BindlessLease>> {
        self.buffer_binding(buffer, offset, size, false)
    }

    /// Only the immutable block allocated by BindlessParameters::finish uses
    /// this role. It is framework upload data rather than a graph resource.
    pub(crate) fn parameter_buffer(
        self: &Arc<Self>,
        chunk: &crate::resource::dynamic_constants::DynamicConstantsChunk,
    ) -> Result<Arc<BindlessLease>> {
        if !chunk.buffer.is_dynamic_constants_page() {
            return Err(Error::invalid_usage(
                "parameter storage must come from the formal constants ring",
            ));
        }
        self.buffer_binding(chunk.buffer.clone(), chunk.offset, chunk.size, true)
    }

    pub(crate) fn uniform_upload_buffer(
        self: &Arc<Self>,
        buffer: Arc<Buffer>,
        offset: u64,
        size: u64,
    ) -> Result<Arc<BindlessLease>> {
        if !buffer.is_dynamic_constants_page() {
            return Err(Error::invalid_usage(
                "uniform upload must come from the formal constants ring",
            ));
        }
        self.buffer_binding(buffer, offset, size, true)
    }

    fn buffer_binding(
        self: &Arc<Self>,
        buffer: Arc<Buffer>,
        offset: u64,
        size: u64,
        parameters: bool,
    ) -> Result<Arc<BindlessLease>> {
        if self.device.is_device_lost() {
            return Err(Error::DeviceLost);
        }
        let limits = self.device.limits();
        if !Arc::ptr_eq(&buffer.device, &self.device)
            || !buffer
                .desc
                .usage
                .contains(vk::BufferUsageFlags::STORAGE_BUFFER)
            || size == 0
            || offset
                .checked_add(size)
                .is_none_or(|end| end > buffer.desc.size)
            || !offset.is_multiple_of(limits.min_storage_buffer_offset_alignment.max(1))
            || size > u64::from(limits.max_storage_buffer_range)
        {
            return Err(Error::invalid_usage(
                "invalid bindless storage buffer device/usage/range",
            ));
        }
        let info = [vk::DescriptorBufferInfo::default()
            .buffer(buffer.handle)
            .offset(offset)
            .range(size)];
        let lease = self.reserve(
            DescriptorKind::StorageBuffer,
            Owner::Buffer {
                buffer,
                offset,
                size,
                parameters,
            },
        )?;
        let write = [vk::WriteDescriptorSet::default()
            .dst_set(self.set.handle)
            .dst_binding(2)
            .dst_array_element(lease.slot.index.index)
            .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
            .buffer_info(&info)];
        {
            let _update_driver = RecordSpan::new(RecordPhase::UpdateDriver);
            unsafe {
                self.device.core.device.update_descriptor_sets(&write, &[]);
            }
        }
        record_count(RecordCount::StorageBufferWrites, 1);
        Ok(lease)
    }

    pub fn sampled_image(
        self: &Arc<Self>,
        view: Arc<TextureView>,
        layout: vk::ImageLayout,
    ) -> Result<Arc<BindlessLease>> {
        self.image(DescriptorKind::SampledImage, view, layout)
    }

    pub fn storage_image(self: &Arc<Self>, view: Arc<TextureView>) -> Result<Arc<BindlessLease>> {
        self.image(DescriptorKind::StorageImage, view, vk::ImageLayout::GENERAL)
    }

    fn image(
        self: &Arc<Self>,
        kind: DescriptorKind,
        view: Arc<TextureView>,
        layout: vk::ImageLayout,
    ) -> Result<Arc<BindlessLease>> {
        if self.device.is_device_lost() {
            return Err(Error::DeviceLost);
        }
        let usage = if kind == DescriptorKind::StorageImage {
            vk::ImageUsageFlags::STORAGE
        } else {
            vk::ImageUsageFlags::SAMPLED
        };
        if !Arc::ptr_eq(&view.texture.device, &self.device)
            || !view.desc.usage.contains(usage)
            || (layout != vk::ImageLayout::GENERAL
                && layout != vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL
                // 深度/模板图同时作只读附件与采样时，访问计划的布局就是
                // DEPTH_STENCIL_READ_ONLY_OPTIMAL（access 0x1e/0x14），描述符必须同布局。
                && !(kind == DescriptorKind::SampledImage
                    && layout == vk::ImageLayout::DEPTH_STENCIL_READ_ONLY_OPTIMAL
                    && view
                        .desc
                        .aspect
                        .intersects(vk::ImageAspectFlags::DEPTH | vk::ImageAspectFlags::STENCIL)))
        {
            return Err(Error::invalid_usage(
                "invalid bindless image device/usage/layout",
            ));
        }
        let info = [vk::DescriptorImageInfo::default()
            .image_view(view.handle)
            .image_layout(layout)];
        let lease = self.reserve(kind, Owner::View { view, layout })?;
        let write = [vk::WriteDescriptorSet::default()
            .dst_set(self.set.handle)
            .dst_binding(kind as u32)
            .dst_array_element(lease.slot.index.index)
            .descriptor_type(kind.ty())
            .image_info(&info)];
        {
            let _update_driver = RecordSpan::new(RecordPhase::UpdateDriver);
            unsafe {
                self.device.core.device.update_descriptor_sets(&write, &[]);
            }
        }
        record_count(
            if kind == DescriptorKind::SampledImage {
                RecordCount::SampledImageWrites
            } else {
                RecordCount::StorageImageWrites
            },
            1,
        );
        Ok(lease)
    }

    pub fn sampler(self: &Arc<Self>, sampler: Arc<Sampler>) -> Result<Arc<BindlessLease>> {
        if self.device.is_device_lost() {
            return Err(Error::DeviceLost);
        }
        if !Arc::ptr_eq(&sampler.device, &self.device) {
            return Err(Error::invalid_usage(
                "bindless sampler belongs to another device",
            ));
        }
        let info = [vk::DescriptorImageInfo::default().sampler(sampler.handle)];
        let lease = self.reserve(DescriptorKind::Sampler, Owner::Sampler(sampler))?;
        let write = [vk::WriteDescriptorSet::default()
            .dst_set(self.set.handle)
            .dst_binding(3)
            .dst_array_element(lease.slot.index.index)
            .descriptor_type(vk::DescriptorType::SAMPLER)
            .image_info(&info)];
        {
            let _update_driver = RecordSpan::new(RecordPhase::UpdateDriver);
            unsafe {
                self.device.core.device.update_descriptor_sets(&write, &[]);
            }
        }
        record_count(RecordCount::SamplerWrites, 1);
        Ok(lease)
    }

    pub(crate) fn validate_binding(
        &self,
        pipeline: &VulkanPipeline,
        set: u32,
        leases: &[Arc<BindlessLease>],
    ) -> Result<()> {
        let _trace = RecordSpan::new(RecordPhase::ManagedValidate);
        record_count(RecordCount::BindingChecks, 1);
        record_count(RecordCount::BindingLeaseInputs, leases.len());

        if self.device.is_device_lost() {
            return Err(Error::DeviceLost);
        }
        if !Arc::ptr_eq(&pipeline.device, &self.device)
            || pipeline
                .layout
                .sets
                .get(set as usize)
                .is_none_or(|layout| layout.handle != self.layout.handle)
            || leases.iter().any(|lease| !std::ptr::eq(self, &*lease.slot.heap))
        {
            return Err(Error::invalid_usage(
                "bindless table/lease/pipeline mismatch",
            ));
        }
        // A lease's private index is immutable and only its final Arc drop
        // retires the slot. Holding these Arcs already proves slot liveness.
        Ok(())
    }
}
