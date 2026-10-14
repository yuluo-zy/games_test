//! 图资源到描述符的候选转换。资源解析和生命周期持有由真实宿主对象供给。
use crate::raw::binding::{DescriptorSetBinding, ResourceOwner};
use crate::rg::resource::ResourceHandle;
use ash::vk;
#[derive(Clone, Copy, Debug)]
pub enum PassResourceSlot {
    UniformBuffer {
        resource: ResourceHandle,
        offset: u64,
        size: u64,
    },
    StorageBuffer {
        resource: ResourceHandle,
        offset: u64,
        size: u64,
    },
    SampledImage {
        resource: ResourceHandle,
        layout: vk::ImageLayout,
    },
    StorageImage {
        resource: ResourceHandle,
        layout: vk::ImageLayout,
    },
}
impl PassResourceSlot {
    /// 构造 uniform 缓冲槽；uniform=1 语义由 `rg_binding_to_descriptor_binding` 判定。
    pub fn uniform_buffer(resource: ResourceHandle, offset: u64, size: u64) -> Self {
        Self::UniformBuffer {
            resource,
            offset,
            size,
        }
    }
    pub fn storage_buffer(resource: ResourceHandle, offset: u64, size: u64) -> Self {
        Self::StorageBuffer {
            resource,
            offset,
            size,
        }
    }
    pub fn sampled_image(resource: ResourceHandle, layout: vk::ImageLayout) -> Self {
        Self::SampledImage { resource, layout }
    }
    pub fn storage_image(resource: ResourceHandle, layout: vk::ImageLayout) -> Self {
        Self::StorageImage { resource, layout }
    }
}
pub enum ResolvedBinding {
    Buffer {
        buffer: vk::Buffer,
        owner: ResourceOwner,
    },
    Image {
        view: vk::ImageView,
        owner: ResourceOwner,
    },
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BindingError {
    MissingResource,
    WrongKind,
}
pub fn rg_binding_to_descriptor_binding(
    slot: PassResourceSlot,
    mut resolve: impl FnMut(ResourceHandle) -> Option<ResolvedBinding>,
) -> Result<DescriptorSetBinding, BindingError> {
    match slot {
        PassResourceSlot::UniformBuffer {
            resource,
            offset,
            size,
        }
        | PassResourceSlot::StorageBuffer {
            resource,
            offset,
            size,
        } => {
            let ty = if matches!(slot, PassResourceSlot::UniformBuffer { .. }) {
                vk::DescriptorType::UNIFORM_BUFFER
            } else {
                vk::DescriptorType::STORAGE_BUFFER
            };
            match resolve(resource).ok_or(BindingError::MissingResource)? {
                ResolvedBinding::Buffer { buffer, owner } => Ok(DescriptorSetBinding::Buffer {
                    ty,
                    info: vk::DescriptorBufferInfo {
                        buffer,
                        offset,
                        range: size,
                    },
                    owner,
                }),
                _ => Err(BindingError::WrongKind),
            }
        }
        PassResourceSlot::SampledImage { resource, layout }
        | PassResourceSlot::StorageImage { resource, layout } => {
            let ty = if matches!(slot, PassResourceSlot::SampledImage { .. }) {
                vk::DescriptorType::SAMPLED_IMAGE
            } else {
                vk::DescriptorType::STORAGE_IMAGE
            };
            match resolve(resource).ok_or(BindingError::MissingResource)? {
                ResolvedBinding::Image { view, owner } => Ok(DescriptorSetBinding::Image {
                    ty,
                    info: vk::DescriptorImageInfo {
                        sampler: vk::Sampler::null(),
                        image_view: view,
                        image_layout: layout,
                    },
                    owners: vec![owner],
                }),
                _ => Err(BindingError::WrongKind),
            }
        }
    }
}

/// 使用原反射变量的名称和 sparse binding 连接 pass 资源；缺少声明访问立即报错。
pub fn create_descriptor_writes(
    entry: &crate::raw::shader::ReflectedEntry,
    set_index: u32,
    pass: &super::graph::PassDeclaration,
    named_slots: &std::collections::BTreeMap<String, Vec<PassResourceSlot>>,
    mut resolve: impl FnMut(ResourceHandle) -> Option<ResolvedBinding>,
) -> Result<Vec<crate::raw::descriptor_set::DescriptorWrite>, BindingError> {
    let mut writes = Vec::new();
    for descriptor in entry
        .descriptors
        .iter()
        .filter(|d| d.set == set_index && d.descriptor_type != "Sampler")
    {
        let slots = named_slots
            .get(&descriptor.name)
            .ok_or(BindingError::MissingResource)?;
        if descriptor.count != 0 && slots.len() != descriptor.count as usize {
            return Err(BindingError::WrongKind);
        }
        for (element, slot) in slots.iter().copied().enumerate() {
            let resource = match slot {
                PassResourceSlot::UniformBuffer { resource, .. }
                | PassResourceSlot::StorageBuffer { resource, .. }
                | PassResourceSlot::SampledImage { resource, .. }
                | PassResourceSlot::StorageImage { resource, .. } => resource,
            };
            if !pass
                .accesses
                .iter()
                .any(|access| access.resource_index == resource.raw())
            {
                return Err(BindingError::MissingResource);
            }
            let mut value = rg_binding_to_descriptor_binding(slot, &mut resolve)?;
            if let DescriptorSetBinding::Buffer { info, .. } = &value {
                // spirq 的 runtime buffer minimum_bytes=0 不能当实际大小；固定最小 extent 则可检查。
                if descriptor.reflected_type["minimum_bytes"]
                    .as_u64()
                    .is_some_and(|minimum| minimum > info.range)
                {
                    return Err(BindingError::WrongKind);
                }
            }
            // 原 _dyn 名称影响布局，写入的 descriptor type 也必须同步改变。
            if descriptor.name.ends_with("_dyn")
                && let DescriptorSetBinding::Buffer { ty, .. } = &mut value
            {
                *ty = match *ty {
                    vk::DescriptorType::UNIFORM_BUFFER => {
                        vk::DescriptorType::UNIFORM_BUFFER_DYNAMIC
                    }
                    vk::DescriptorType::STORAGE_BUFFER => {
                        vk::DescriptorType::STORAGE_BUFFER_DYNAMIC
                    }
                    _ => *ty,
                };
            }
            writes.push(crate::raw::descriptor_set::DescriptorWrite {
                binding: descriptor.binding,
                array_element: element as u32,
                value,
            });
        }
    }
    Ok(writes)
}

/// # Safety
/// command 正在合法录制；调用者须持有 pipeline/set 直到此提交完成。
pub unsafe fn bind_descriptor_set(
    command: vk::CommandBuffer,
    pipeline: &crate::raw::shader::VulkanPipeline,
    set_index: u32,
    set: &crate::raw::descriptor_set::VulkanDescriptorSet,
    dynamic_offsets: &[u32],
) -> Result<(), vk::Result> {
    let layout = pipeline
        .layout
        .sets
        .get(set_index as usize)
        .ok_or(vk::Result::ERROR_INITIALIZATION_FAILED)?;
    if !std::sync::Arc::ptr_eq(&pipeline.device, &set.pool.device) || layout.plan != set.layout.plan
    {
        return Err(vk::Result::ERROR_INITIALIZATION_FAILED);
    }
    let dynamic_count = layout
        .plan
        .bindings
        .iter()
        .filter(|slot| {
            matches!(
                vk::DescriptorType::from_raw(slot.descriptor_type),
                vk::DescriptorType::UNIFORM_BUFFER_DYNAMIC
                    | vk::DescriptorType::STORAGE_BUFFER_DYNAMIC
            )
        })
        .map(|slot| slot.descriptor_count as usize)
        .sum::<usize>();
    if dynamic_offsets.len() != dynamic_count {
        return Err(vk::Result::ERROR_INITIALIZATION_FAILED);
    }
    unsafe {
        crate::raw::binding::bind_persistent_descriptor_set(
            &pipeline.device,
            command,
            pipeline.point,
            pipeline.layout.handle,
            set_index,
            &[set.handle],
            dynamic_offsets,
        );
    }
    Ok(())
}
