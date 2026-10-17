use ash::vk::{self, Handle};
use serde::{Deserialize, Serialize};
// 原 sparse descriptor 键、资源 tag/数组/动态 offset 和 compute tuple 绑定闭包。
// 来源 render/work-packages/descriptor-binding-r8；图物理资源 producer 接线独立验收。
use crate::raw::descriptor_set::{
    Binding, LayoutPlan, PersistentPoolPlan, descriptor_set_layout, persistent_pool_plan,
    with_compute_pipeline_layout,
};

/// Private BindingIndex key is read as a u32 by the sparse layout and write consumers.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct BindingIndex(pub u32);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SparseBinding {
    pub index: BindingIndex,
    #[serde(flatten)]
    pub binding: Binding,
}

/// Consumer at 0x1418c1a10. Input order is the caller's HashMap iterator order.
/// Duplicates are excluded by the original HashMap producer, not checked here.
pub fn sparse_descriptor_set_layout(
    entries: &[SparseBinding],
    stage: vk::ShaderStageFlags,
    device_limit: u32,
) -> crate::Result<LayoutPlan> {
    let mut output = LayoutPlan {
        flags: 0,
        binding_flags: Vec::with_capacity(entries.len()),
        bindings: Vec::with_capacity(entries.len()),
    };
    for entry in entries {
        let mut converted = descriptor_set_layout(
            std::slice::from_ref(&entry.binding),
            entry.index.0 as u64,
            stage,
            device_limit,
        )?;
        output.flags |= converted.flags;
        output.binding_flags.append(&mut converted.binding_flags);
        output.bindings.append(&mut converted.bindings);
    }
    Ok(output)
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ImageInfo {
    pub sampler: u64,
    pub image_view: u64,
    pub image_layout: i32,
}
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct BufferInfo {
    pub buffer: u64,
    pub offset: u64,
    pub range: u64,
}

/// Caller-resolved DescriptorSetBinding ABI projection. Shader declarations and
/// actual resource bindings have different private tag sets.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ResourceBinding {
    pub tag: u32,
    #[serde(default)]
    pub dynamic_offset: u32,
    #[serde(default)]
    pub images: Vec<ImageInfo>,
    pub buffer: Option<BufferInfo>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BindingWrite {
    pub binding: BindingIndex,
    pub array_element: u32,
    pub resource: ResourceBinding,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct WritePlan {
    pub dst_set: u64,
    pub dst_binding: u32,
    pub dst_array_element: u32,
    pub descriptor_count: u32,
    pub descriptor_type: i32,
    pub images: Vec<ImageInfo>,
    pub buffers: Vec<BufferInfo>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum WriteError {
    UnsupportedTag(u32),
    MissingImage,
    MissingBuffer,
    SlotOutOfBounds { slot: u8, length: usize },
}

/// Actual compute-pass tuple mapper observed at 0x1416a6245..0x1416a628e.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PassResourceBinding {
    pub binding: BindingIndex,
    pub slot: u8,
}

/// First graph-resource side of the original compute-pass chained iterator.
/// The caller has already resolved PassResourceSlot to DescriptorSetBinding.
pub fn compute_pass_bind_plan(
    context: &BindContext,
    tuples: &[PassResourceBinding],
    resources: &[ResourceBinding],
) -> Result<BindPlan, WriteError> {
    if context.set_index >= context.set_count {
        return Ok(BindPlan {
            update_call: false,
            writes: Vec::new(),
            dynamic_offsets: Vec::new(),
        });
    }
    let mut inputs = Vec::with_capacity(tuples.len());
    for tuple in tuples {
        let resource = resources
            .get(tuple.slot as usize)
            .ok_or(WriteError::SlotOutOfBounds {
                slot: tuple.slot,
                length: resources.len(),
            })?;
        inputs.push(BindingWrite {
            binding: tuple.binding,
            array_element: 0,
            resource: resource.clone(),
        });
    }
    descriptor_bind_plan(context, &inputs)
}

/// Shared VkWriteDescriptorSet constructor at 0x140b06020. Resource data has
/// already been resolved to Vulkan handles/offsets/layouts by the caller.
pub fn descriptor_write(
    set: u64,
    input: &BindingWrite,
    offsets: &mut Vec<u32>,
) -> Result<WritePlan, WriteError> {
    let resource = &input.resource;
    let (ty, images, buffers, count) = match resource.tag {
        0 | 1 | 4 => {
            let image = resource
                .images
                .first()
                .ok_or(WriteError::MissingImage)?
                .clone();
            let ty = match resource.tag {
                0 => vk::DescriptorType::SAMPLED_IMAGE,
                1 => vk::DescriptorType::STORAGE_IMAGE,
                _ => vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
            };
            (ty, vec![image], Vec::new(), 1)
        }
        2 | 3 => (
            if resource.tag == 2 {
                vk::DescriptorType::SAMPLED_IMAGE
            } else {
                vk::DescriptorType::STORAGE_IMAGE
            },
            resource.images.clone(),
            Vec::new(),
            resource.images.len() as u32,
        ),
        5 | 6 | 8 | 9 => {
            let buffer = resource
                .buffer
                .as_ref()
                .ok_or(WriteError::MissingBuffer)?
                .clone();
            let ty = match resource.tag {
                5 => vk::DescriptorType::STORAGE_BUFFER,
                6 => vk::DescriptorType::UNIFORM_BUFFER,
                8 => vk::DescriptorType::UNIFORM_BUFFER_DYNAMIC,
                _ => vk::DescriptorType::STORAGE_BUFFER_DYNAMIC,
            };
            if matches!(resource.tag, 8 | 9) {
                offsets.push(resource.dynamic_offset);
            }
            (ty, Vec::new(), vec![buffer], 1)
        }
        tag => return Err(WriteError::UnsupportedTag(tag)),
    };
    Ok(WritePlan {
        dst_set: set,
        dst_binding: input.binding.0,
        dst_array_element: input.array_element,
        descriptor_count: count,
        descriptor_type: ty.as_raw(),
        images,
        buffers,
    })
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BindContext {
    pub command_buffer: u64,
    pub bind_point: i32,
    pub pipeline_layout: u64,
    pub set_index: u32,
    pub set_count: u32,
    pub descriptor_set: u64,
    pub declared_bindings: Vec<BindingIndex>,
}

/// Caller-selected compiled shader interface. Selection/reflection production is
/// an unresolved private boundary; this type does not infer a game pass binding.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ComputePipelineInterface {
    pub descriptor_sets: Vec<Vec<SparseBinding>>,
    pub push_constant_bytes: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ComputeBindTarget {
    pub command_buffer: u64,
    pub pipeline_layout: u64,
    pub set_index: u32,
    pub descriptor_set: u64,
}

#[derive(Clone, Debug)]
pub struct PreparedComputePass {
    pub context: BindContext,
    pub layout: Option<LayoutPlan>,
    pub pool: Option<PersistentPoolPlan>,
    pub push_constant_bytes: u32,
    pub bind_plan: BindPlan,
}

#[derive(Debug)]
pub enum PrepareError {
    Layout(crate::Error),
    Binding(WriteError),
}

/// Framework consumer entry: derive sparse declared keys and set count from the
/// selected interface, then map actual resource slots into writes/update/bind.
/// Layout/pool creation and descriptor set allocation are external callbacks.
pub fn prepare_compute_pass(
    interface: &ComputePipelineInterface,
    target: &ComputeBindTarget,
    tuples: &[PassResourceBinding],
    resources: &[ResourceBinding],
    device_limit: u32,
) -> Result<PreparedComputePass, PrepareError> {
    let selected = interface.descriptor_sets.get(target.set_index as usize);
    let context = BindContext {
        command_buffer: target.command_buffer,
        bind_point: vk::PipelineBindPoint::COMPUTE.as_raw(),
        pipeline_layout: target.pipeline_layout,
        set_index: target.set_index,
        set_count: interface.descriptor_sets.len() as u32,
        descriptor_set: target.descriptor_set,
        declared_bindings: selected
            .map(|entries| entries.iter().map(|e| e.index).collect())
            .unwrap_or_default(),
    };
    let layout = selected
        .map(|entries| {
            sparse_descriptor_set_layout(entries, vk::ShaderStageFlags::COMPUTE, device_limit)
        })
        .transpose()
        .map_err(PrepareError::Layout)?;
    let pool = layout
        .as_ref()
        .map(|layout| persistent_pool_plan(layout, device_limit));
    let bind_plan =
        compute_pass_bind_plan(&context, tuples, resources).map_err(PrepareError::Binding)?;
    Ok(PreparedComputePass {
        context,
        layout,
        pool,
        push_constant_bytes: interface.push_constant_bytes,
        bind_plan,
    })
}

impl ComputePipelineInterface {
    pub fn with_pipeline_layout<R>(
        &self,
        layouts: &[vk::DescriptorSetLayout],
        call: impl FnOnce(&vk::PipelineLayoutCreateInfo<'_>) -> R,
    ) -> R {
        with_compute_pipeline_layout(layouts, self.push_constant_bytes, call)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct BindPlan {
    pub update_call: bool,
    pub writes: Vec<WritePlan>,
    pub dynamic_offsets: Vec<u32>,
}

/// Membership filtering and update-before-bind semantics from 0x1416a5380.
/// Resource class is not checked against reflection class in this consumer.
pub fn descriptor_bind_plan(
    context: &BindContext,
    inputs: &[BindingWrite],
) -> Result<BindPlan, WriteError> {
    let mut plan = BindPlan {
        update_call: false,
        writes: Vec::new(),
        dynamic_offsets: Vec::new(),
    };
    if context.set_index >= context.set_count {
        return Ok(plan);
    }
    plan.update_call = true;
    for input in inputs {
        if context.declared_bindings.contains(&input.binding) {
            plan.writes.push(descriptor_write(
                context.descriptor_set,
                input,
                &mut plan.dynamic_offsets,
            )?);
        }
    }
    Ok(plan)
}

impl BindPlan {
    /// Owns the backing arrays throughout update and bind. It also preserves an
    /// update with zero writes and increments the caller's wrapping update counter.
    pub fn execute(
        &self,
        context: &BindContext,
        counter: &mut u64,
        update: impl FnOnce(&[vk::WriteDescriptorSet<'_>]),
        bind: impl FnOnce(
            vk::CommandBuffer,
            vk::PipelineBindPoint,
            vk::PipelineLayout,
            u32,
            vk::DescriptorSet,
            &[u32],
        ),
    ) {
        if !self.update_call {
            return;
        }
        let images: Vec<Vec<vk::DescriptorImageInfo>> = self
            .writes
            .iter()
            .map(|w| {
                w.images
                    .iter()
                    .map(|x| vk::DescriptorImageInfo {
                        sampler: vk::Sampler::from_raw(x.sampler),
                        image_view: vk::ImageView::from_raw(x.image_view),
                        image_layout: vk::ImageLayout::from_raw(x.image_layout),
                    })
                    .collect()
            })
            .collect();
        let buffers: Vec<Vec<vk::DescriptorBufferInfo>> = self
            .writes
            .iter()
            .map(|w| {
                w.buffers
                    .iter()
                    .map(|x| vk::DescriptorBufferInfo {
                        buffer: vk::Buffer::from_raw(x.buffer),
                        offset: x.offset,
                        range: x.range,
                    })
                    .collect()
            })
            .collect();
        let writes: Vec<_> = self
            .writes
            .iter()
            .enumerate()
            .map(|(i, w)| {
                let mut raw = vk::WriteDescriptorSet::default()
                    .dst_set(vk::DescriptorSet::from_raw(w.dst_set))
                    .dst_binding(w.dst_binding)
                    .dst_array_element(w.dst_array_element)
                    .descriptor_type(vk::DescriptorType::from_raw(w.descriptor_type))
                    .descriptor_count(w.descriptor_count);
                if matches!(w.descriptor_type, 1..=3) {
                    raw.p_image_info = images[i].as_ptr();
                } else {
                    raw.p_buffer_info = buffers[i].as_ptr();
                }
                raw
            })
            .collect();
        update(&writes);
        *counter = counter.wrapping_add(1);
        bind(
            vk::CommandBuffer::from_raw(context.command_buffer),
            vk::PipelineBindPoint::from_raw(context.bind_point),
            vk::PipelineLayout::from_raw(context.pipeline_layout),
            context.set_index,
            vk::DescriptorSet::from_raw(context.descriptor_set),
            &self.dynamic_offsets,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sparse_keys_are_not_ordinal_positions() {
        let entries = [3, 23].map(|index| SparseBinding {
            index: BindingIndex(index),
            binding: Binding {
                tag: 1,
                name: "u".into(),
                runtime: false,
                count: 1,
                sampler: 0,
            },
        });
        let plan =
            sparse_descriptor_set_layout(&entries, vk::ShaderStageFlags::COMPUTE, 16384).unwrap();
        assert_eq!(
            plan.bindings.iter().map(|b| b.binding).collect::<Vec<_>>(),
            [3, 23]
        );
    }
    #[test]
    fn unsupported_resource_is_an_explicit_candidate_boundary() {
        let input = BindingWrite {
            binding: BindingIndex(0),
            array_element: 0,
            resource: ResourceBinding {
                tag: 7,
                dynamic_offset: 0,
                images: vec![],
                buffer: None,
            },
        };
        assert_eq!(
            descriptor_write(0, &input, &mut vec![]),
            Err(WriteError::UnsupportedTag(7))
        );
    }
}
